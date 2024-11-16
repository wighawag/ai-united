use std::cell::RefCell;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;
use std::sync::RwLock;

use nalgebra::ArrayStorage;
use nalgebra::Const;
use nalgebra::Matrix;
use nalgebra::Vector3;
use rapier3d::crossbeam;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::JsValue;

#[cfg(target_arch = "wasm32")]
mod utils;
#[cfg(target_arch = "wasm32")]
use utils::set_panic_hook;

use wasmer::RuntimeError;

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Eq, PartialEq)]
pub enum MeteringPoints {
    /// The given number of metering points is left for the execution.
    /// If the value is 0, all points are consumed but the execution
    /// was not terminated.
    Remaining(u64),

    /// The execution was terminated because the metering points were
    /// exhausted.  You can recover from this state by setting the
    /// points via [`set_remaining_points`] and restart the execution.
    Exhausted,
}

#[derive(Copy, Clone, PartialEq)]
enum ObjectType {
    Any,
    Ball,
    Bot1Goal,
    Bot2Goal,
    Bot1,
    Bot2,
}

#[cfg(not(target_arch = "wasm32"))]
use {
    std::sync::Arc,
    wasmer::sys::EngineBuilder,
    wasmer::wasmparser::Operator,
    wasmer::CompilerConfig,
    wasmer_compiler_cranelift::Cranelift,
    wasmer_middlewares::{
        metering::{get_remaining_points, set_remaining_points, MeteringPoints},
        Metering,
    },
};

use wasmer::Function;
use wasmer::{imports, FunctionEnv, FunctionEnvMut, Instance, Module, Store, TypedFunction};

use rapier3d::prelude::*;

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn start() -> Result<(), JsValue> {
    set_panic_hook();
    Ok(())
}

// Function to reconstruct vector from u32
fn reconstruct_action(action: u32) -> [f32; 3] {
    // First, reconstruct [u8; 3] from u32
    let action_u8 = [
        ((action >> 16) & 0xFF) as u8,
        ((action >> 8) & 0xFF) as u8,
        (action & 0xFF) as u8,
    ];

    // Then, convert [u8; 3] back to [f32; 3]
    [
        (action_u8[0] as f32 - 128.0) / 127.0,
        (action_u8[1] as f32 - 128.0) / 127.0,
        (action_u8[2] as f32 - 128.0) / 127.0,
    ]
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

struct BotModule {
    #[allow(dead_code)]
    instance: Instance,
    store: Store,
    init: TypedFunction<u32, ()>,
    compute_actions: TypedFunction<(f32, f32, f32, f32, f32, f32, f32, f32, f32), u32>,
    handle: RigidBodyHandle,
}

const INIT_GAS: u64 = 1_000;
const COMPUTE_ACTIONS_GAS: u64 = 1_000_000;
const MAX_NUM_UPDATES: u64 = 1_000_000;

fn create_bot_module(wasm_bytes: &mut [u8], handle: RigidBodyHandle) -> BotModule {
    #[cfg(not(target_arch = "wasm32"))]
    let mut store = {
        // Let's define our cost function.
        // This function will be called for each `Operator` encountered during
        // the Wasm module execution. It should return the cost of the operator
        // that it received as it first argument.
        let cost_function = |operator: &Operator| -> u64 {
            match operator {
                Operator::LocalGet { .. } | Operator::I32Const { .. } => 1,
                Operator::I32Add { .. } => 2,
                _ => 0,
            }
        };

        // Now let's create our metering middleware.
        //
        // `Metering` needs to be configured with a limit and a cost function.
        //
        // For each `Operator`, the metering middleware will call the cost
        // function and subtract the cost from the remaining points.
        let initial_points = 100;
        let metering = Arc::new(Metering::new(initial_points, cost_function));
        let mut compiler_config = Cranelift::default();
        compiler_config.push_middleware(metering);

        // Create a Store.
        //
        // We use our previously create compiler configuration
        // with the Universal engine.
        Store::new(EngineBuilder::new(compiler_config))
    };

    #[cfg(target_arch = "wasm32")]
    let mut store = { Store::default() };

    println!("Compiling wasm module...");
    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes).expect("failed to create module");

    // Let's define the import object used to import our function
    // into our webassembly sample application.

    struct MyEnv;
    let env = FunctionEnv::new(&mut store, MyEnv {});

    fn print_u32(_env: FunctionEnvMut<MyEnv>, num: u32) {
        // Print it!
        println!("num: {}", num);
    }
    let print_u32 = Function::new_typed_with_env(&mut store, &env, print_u32);

    // this is how you import values:
    // let value = Global::new(&mut store, Value::I32(value)); // there is no u32 but here we can pass as if

    let import_object = imports! {
        "env" => {
            "print_u32" => print_u32,
        },
    };

    println!("Instantiating wasm module...");
    // Let's instantiate the Wasm module.
    let instance =
        Instance::new(&mut store, &module, &import_object).expect("failed to instantiate module");

    // We now have an instance ready to be used.
    //
    // Our module exports a single `add_one`  function. We want to
    // measure the cost of executing this function.
    let init: TypedFunction<u32, ()> = instance
        .exports
        .get_function("init")
        .expect("failed to get function 'init'")
        .typed(&mut store)
        .expect("failed to get typed version of the function 'init'");

    let compute_actions: TypedFunction<(f32, f32, f32, f32, f32, f32, f32, f32, f32), u32> =
        instance
            .exports
            .get_function("compute_actions")
            .expect("failed to get function 'compute_actions'")
            .typed(&mut store)
            .expect("failed to get typed version of the function 'compute_actions'");

    BotModule {
        instance,
        store,
        compute_actions,
        init,
        handle,
    }
}

fn on_runtime_error(e: RuntimeError) {
    #[cfg(target_arch = "wasm32")]
    {
        panic!("RuntimeError: {:?}", e);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("Error message: {}", e.message());
        println!("Error: {}", e);

        let frames = e.trace();
        let frames_len = frames.len();

        for i in 0..frames_len {
            println!(
                "  Frame #{}: {:?}::{:?}",
                frames_len - i,
                frames[i].module_name(),
                frames[i].function_name().or(Some("<func>")).unwrap()
            );
        }
    }
}

impl BotModule {
    fn init(&mut self, seed: u32) -> () {
        match self.init.call(&mut self.store, seed) {
            Ok(_) => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let remaining_points_after_call =
                        get_remaining_points(&mut self.store, &self.instance);
                    // println!(
                    //     "Calling `init` succeeded. points left: {:?}",
                    //     remaining_points_after_call
                    // );
                }
            }
            Err(_) => {
                println!("Calling `init` failed.");

                #[cfg(not(target_arch = "wasm32"))]
                {
                    // Because the last needed more than the remaining points, we should have an error.
                    let remaining_points = get_remaining_points(&mut self.store, &self.instance);

                    match remaining_points {
                        MeteringPoints::Remaining(..) => {
                            panic!("No metering error: there are remaining points");
                        }
                        MeteringPoints::Exhausted => println!("Not enough points remaining"),
                    }
                }
            }
        }
    }

    fn compute_actions(
        &mut self,
        self_x: f32,
        self_y: f32,
        self_z: f32,
        ball_x: f32,
        ball_y: f32,
        ball_z: f32,
        enemy_x: f32,
        enemy_y: f32,
        enemy_z: f32,
    ) -> u32 {
        match self.compute_actions.call(
            &mut self.store,
            self_x,
            self_y,
            self_z,
            ball_x,
            ball_y,
            ball_z,
            enemy_x,
            enemy_y,
            enemy_z,
        ) {
            Ok(res) => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let remaining_points_after_call =
                        get_remaining_points(&mut self.store, &self.instance);
                    // println!(
                    //     "Calling `update` succeeded. points left: {:?}",
                    //     remaining_points_after_call
                    // );
                }
                res
            }
            Err(err) => {
                println!("Calling `update` failed.");

                #[cfg(not(target_arch = "wasm32"))]
                {
                    // Because the last needed more than the remaining points, we should have an error.
                    let remaining_points = get_remaining_points(&mut self.store, &self.instance);

                    match remaining_points {
                        MeteringPoints::Remaining(..) => {
                            on_runtime_error(err);
                        }
                        MeteringPoints::Exhausted => println!("Not enough points remaining"),
                    }
                }
                #[cfg(target_arch = "wasm32")]
                {
                    on_runtime_error(err);
                }
                8421504
            }
        }
    }

    // fn get_remaining_points(&mut self) -> MeteringPoints {
    //     #[cfg(not(target_arch = "wasm32"))]
    //     let points = get_remaining_points(&mut self.store, &self.instance);

    //     #[cfg(target_arch = "wasm32")]
    //     let points = MeteringPoints::Remaining(0);

    //     points
    // }

    #[allow(unused_variables)]
    fn set_remaining_points(&mut self, points: u64) {
        #[cfg(not(target_arch = "wasm32"))]
        set_remaining_points(&mut self.store, &self.instance, points)
    }
}

struct Winner {
    pub winner: u32,
}

// Define a struct to hold our custom event handler
struct CustomEventHandler {
    // pub winner: AtomicI32,
    pub winner: RwLock<u8>,
}

impl EventHandler for CustomEventHandler {
    fn handle_collision_event(
        &self,
        _bodies: &RigidBodySet,
        colliders: &ColliderSet,
        collision_event: CollisionEvent,
        _contact_pair: Option<&ContactPair>,
    ) {
        // println!("collision_event, {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(h1, h2, _) => {
                if let Some(collider1) = colliders.get(h1) {
                    if let Some(collider2) = colliders.get(h2) {
                        let user_data1 = collider1.user_data;
                        let user_data2 = collider2.user_data;

                        if user_data1 == ObjectType::Ball as u128
                            && user_data2 == ObjectType::Bot1 as u128
                        {
                            println!("Ball <-> Bot1");

                            // Add your logic for Bot1's goal here
                        } else if user_data2 == ObjectType::Ball as u128
                            && user_data1 == ObjectType::Bot1 as u128
                        {
                            println!("Ball <-> Bot1");
                        } else if user_data1 == ObjectType::Ball as u128
                            && user_data2 == ObjectType::Bot2Goal as u128
                        {
                            println!("Ball <-> Bot2's goal!");
                            // self.winner.store(1, Ordering::SeqCst);
                            if let Ok(mut winner) = self.winner.write() {
                                *winner = 1;
                            }
                        } else if user_data2 == ObjectType::Ball as u128
                            && user_data1 == ObjectType::Bot2Goal as u128
                        {
                            println!("Ball <-> Bot2's goal!");
                            if let Ok(mut winner) = self.winner.write() {
                                *winner = 1;
                            }
                        }
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {
                // Handle collision end if needed
            }
        }
    }

    fn handle_contact_force_event(
        &self,
        dt: f32,
        bodies: &RigidBodySet,
        colliders: &ColliderSet,
        contact_pair: &ContactPair,
        total_force_magnitude: f32,
    ) {
        println!("contact_force_event")
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Battle {
    bot1: Option<BotModule>,
    bot2: Option<BotModule>,
    collider_set: ColliderSet,
    physics_pipeline: PhysicsPipeline,
    rigid_body_set: RigidBodySet,
    gravity: Matrix<f32, Const<3>, Const<1>, ArrayStorage<f32, 3, 1>>,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhaseMultiSap,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    physics_hooks: (),
    event_handler: (),
    ball: RigidBodyHandle,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Battle {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new() -> Battle {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        /* Create the ground. */
        let collider = ColliderBuilder::cuboid(20.0, 0.1, 20.0)
            .translation(vector![0.0, -0.05, 0.0])
            .build();
        collider_set.insert(collider);

        let collider = ColliderBuilder::cuboid(0.1, 20.0, 20.0)
            .translation(vector![-10.0 - 0.05, 0.0, 0.0])
            .build();
        collider_set.insert(collider);

        let collider = ColliderBuilder::cuboid(20.0, 20.0, 0.1)
            .translation(vector![0.0, 0.0, 10.0 + 0.05])
            .build();
        collider_set.insert(collider);

        let collider = ColliderBuilder::cuboid(0.1, 20.0, 20.0)
            .translation(vector![10.0 + 0.05, 0.0, 0.0])
            .build();
        collider_set.insert(collider);

        let collider = ColliderBuilder::cuboid(20.0, 20.0, 0.1)
            .translation(vector![0.0, 0.0, -10.0 - 0.05])
            .build();
        collider_set.insert(collider);

        // let collider = ColliderBuilder::cone(2.5, 0.5)
        //     .translation(vector![-9.5, 2.5, 9.5])
        //     .build();
        // collider_set.insert(collider);

        // let collider = ColliderBuilder::cone(2.5, 0.5)
        //     .translation(vector![9.5, 2.5, -9.5])
        //     .build();
        // collider_set.insert(collider);

        let collider_for_bot1_goal = ColliderBuilder::cuboid(1.0, 5.0, 5.0)
            .translation(vector![-10.4, 0.0, 0.0])
            .user_data(ObjectType::Bot1Goal as u128)
            .build();
        collider_set.insert(collider_for_bot1_goal);

        let collider_for_bot2_goal = ColliderBuilder::cuboid(1.0, 5.0, 5.0)
            .translation(vector![10.4, 0.0, 0.0])
            .user_data(ObjectType::Bot2Goal as u128)
            .build();
        collider_set.insert(collider_for_bot2_goal);

        // // Define dome parameters
        // let dome_radius = 5.0;

        // // Create a half-sphere shape for the dome
        // let dome_shape = SharedShape::halfspace(Vector::y_axis());

        // // Create a collider descriptor for the dome
        // let dome_collider = ColliderBuilder::new(dome_shape)
        //     .position(Isometry3::translation(0.0, dome_radius, 0.0))
        //     .
        //     .build();

        // // Add the dome collider to the set
        // let dome_handle = collider_set.insert(dome_collider);

        // let collider = ColliderBuilder::ball(10.0).build();
        // collider_set.insert(collider);

        /* Create other structures necessary for the simulation. */
        let gravity = vector![0.0, -9.81, 0.0];
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = DefaultBroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let query_pipeline = QueryPipeline::new();
        let physics_hooks = ();
        let event_handler = ();

        /* Create the bouncing ball. */
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 10.0 - 0.25, 0.0])
            .build();
        let collider = ColliderBuilder::ball(0.5)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .restitution(0.7)
            .user_data(ObjectType::Ball as u128)
            .build();
        let ball = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, ball, &mut rigid_body_set);

        Battle {
            bot1: None,
            bot2: None,
            collider_set,
            physics_pipeline,
            rigid_body_set,
            gravity,
            integration_parameters,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            query_pipeline,
            physics_hooks,
            event_handler,
            ball,
        }
    }

    fn create_bot_handle(
        &mut self,
        position: Position,
        object_type: ObjectType,
    ) -> RigidBodyHandle {
        /* Create the bouncing ball. */
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![position.x, position.y, position.y])
            .build();
        let collider = ColliderBuilder::ball(0.5)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .restitution(0.7)
            .user_data(object_type as u128)
            .build();
        let handle = self.rigid_body_set.insert(rigid_body);
        self.collider_set
            .insert_with_parent(collider, handle, &mut self.rigid_body_set);
        handle
    }

    pub fn get_bot1(&self) -> Position {
        let bot = self.bot1.as_ref().unwrap();
        let body = &self.rigid_body_set[bot.handle];
        Position {
            x: body.translation().x,
            y: body.translation().y,
            z: body.translation().z,
        }
    }

    pub fn get_bot2(&self) -> Position {
        let bot = self.bot2.as_ref().unwrap();
        let body = &self.rigid_body_set[bot.handle];
        Position {
            x: body.translation().x,
            y: body.translation().y,
            z: body.translation().z,
        }
    }

    pub fn get_ball(&self) -> Position {
        let body = &self.rigid_body_set[self.ball];
        Position {
            x: body.translation().x,
            y: body.translation().y,
            z: body.translation().z,
        }
    }

    pub fn add_bot(&mut self, wasm_bytes: &mut [u8]) {
        if let Some(_existing_bot) = &self.bot1 {
            if let Some(_existing_bot) = &self.bot2 {
                panic!("already both bots added");
            } else {
                let handle = self.create_bot_handle(
                    Position {
                        x: 9.0,
                        y: 0.25,
                        z: 0.0,
                    },
                    ObjectType::Bot2,
                );
                self.bot2 = Some(create_bot_module(wasm_bytes, handle));
            }
        } else {
            let handle = self.create_bot_handle(
                Position {
                    x: -9.0,
                    y: 0.25,
                    z: 0.0,
                },
                ObjectType::Bot1,
            );
            self.bot1 = Some(create_bot_module(wasm_bytes, handle));
        }
    }

    pub fn init(&mut self) {
        let bot1 = self.bot1.as_mut().unwrap();
        let bot2 = self.bot2.as_mut().unwrap();

        bot1.set_remaining_points(INIT_GAS);
        bot2.set_remaining_points(INIT_GAS);

        println!("Calling `init` ...");
        bot1.init(0);
        bot2.init(0);

        self.rigid_body_set[bot1.handle].apply_impulse(vector![2.0, 0.0, 0.0], true);
        // self.rigid_body_set[bot2.handle].apply_impulse(vector![-1.0, 1.0, 0.0], true);
        self.rigid_body_set[bot2.handle].apply_impulse(vector![-2.0, 0.0, 1.0], true);
    }

    pub fn update(&mut self) -> u8 {
        let ball_pos = self.get_ball().clone();
        let bot1_pos = self.get_bot1().clone();
        let bot2_pos = self.get_bot2().clone();

        // // Initialize the event collector.
        // let (collision_send, collision_recv) = crossbeam::channel::unbounded();
        // let (contact_force_send, contact_force_recv) = crossbeam::channel::unbounded();
        // let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);

        let bot1 = self.bot1.as_mut().unwrap();
        let bot2 = self.bot2.as_mut().unwrap();

        bot1.set_remaining_points(COMPUTE_ACTIONS_GAS);
        bot2.set_remaining_points(COMPUTE_ACTIONS_GAS);

        // println!("Calling `compute_actions` ...");
        // TODO

        let bot1_vector = Vector3::from(reconstruct_action(bot1.compute_actions(
            bot1_pos.x, bot1_pos.y, bot1_pos.z, ball_pos.x, ball_pos.y, ball_pos.z, bot2_pos.x,
            bot2_pos.y, bot2_pos.z,
        )));
        let bot2_vector = Vector3::from(reconstruct_action(bot2.compute_actions(
            bot2_pos.x, bot2_pos.y, bot2_pos.z, ball_pos.x, ball_pos.y, ball_pos.z, bot1_pos.x,
            bot1_pos.y, bot1_pos.z,
        )));

        self.rigid_body_set[bot1.handle].apply_impulse(bot1_vector, true);
        // self.rigid_body_set[bot2.handle].apply_impulse(vector![-1.0, 1.0, 0.0], true);
        self.rigid_body_set[bot2.handle].apply_impulse(bot2_vector, true);

        let event_handler = CustomEventHandler {
            winner: RwLock::new(0),
        };

        // // Make sure to set up the event handler with the physics pipeline
        // // let mut physics_hooks = ChannelEventCollector::new();
        // self.physics_hooks.handle_collision_event(
        //     &self.rigid_body_set,
        //     &self.collider_set,
        //     event,
        //     contact_pair,
        // );
        // self.physics_hooks
        //     .set_collision_event_handler(Box::new(event_handler));

        /* Run the game loop, stepping the simulation once per frame. */
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &self.physics_hooks,
            &event_handler,
        );

        // while let Ok(collision_event) = collision_recv.try_recv() {
        //     // Handle the collision event.
        //     println!("Received collision event: {:?}", collision_event);
        // }

        // while let Ok(contact_force_event) = contact_force_recv.try_recv() {
        //     // Handle the contact force event.
        //     println!("Received contact force event: {:?}", contact_force_event);
        // }

        // if let Some(ball_body) = self.rigid_body_set.get(self.ball) {
        //     let ball_position = ball_body.position();

        //     let query_filter = QueryFilter::default().exclude_rigid_body(self.ball); // Exclude the ball itself from the query

        //     let collision_with_goal1 = self.query_pipeline.intersection_with_shape(
        //         &self.rigid_body_set,
        //         &self.collider_set,
        //         ball_position,
        //         &Ball::new(0.5), // Assuming the ball has a radius of 0.5, adjust as needed
        //         query_filter,
        //     );

        //     let collision_with_goal2 = self.query_pipeline.intersection_with_shape(
        //         &self.rigid_body_set,
        //         &self.collider_set,
        //         ball_position,
        //         &Ball::new(0.5), // Assuming the ball has a radius of 0.5, adjust as needed
        //         query_filter,
        //     );

        //     let collision_with_goal2 = self.query_pipeline.intersection_with_shape(
        //         &self.rigid_body_set,
        //         &self.collider_set,
        //         ball_position,
        //         &Ball::new(0.5), // Assuming the ball has a radius of 0.5, adjust as needed
        //         query_filter,
        //     );

        //     if let Some(collider_handle) = collision_with_goal1 {
        //         if let Some(collider) = self.collider_set.get(collider_handle) {
        //             match collider.user_data {
        //                 data if data == ObjectType::Bot1 as u128 => {
        //                     println!("Collision Bot1 <-> Bot1Goal");
        //                     // Add your logic for Bot1's goal here
        //                 }
        //                 data if data == ObjectType::Bot2 as u128 => {
        //                     println!("Collision Bot2 <-> Bot1Goal");
        //                     // Add your logic for Bot2's goal here
        //                 }
        //                 data if data == ObjectType::Ball as u128 => {
        //                     println!("Collision Ball <-> Bot1Goal !!!!!!!!!!!!!!!!!");
        //                     // Add your logic for Bot2's goal here
        //                 }
        //                 _ => (), //println!("Collision with unknown object"),
        //             }
        //         }
        //     } else if let Some(collider_handle) = collision_with_goal2 {
        //         if let Some(collider) = self.collider_set.get(collider_handle) {
        //             match collider.user_data {
        //                 data if data == ObjectType::Bot1 as u128 => {
        //                     println!("Collision Bot1 <-> Bot2Goal");
        //                     // Add your logic for Bot1's goal here
        //                 }
        //                 data if data == ObjectType::Bot2 as u128 => {
        //                     println!("Collision Bot2 <-> Bot2Goal");
        //                     // Add your logic for Bot2's goal here
        //                 }
        //                 data if data == ObjectType::Ball as u128 => {
        //                     println!("Collision Ball <-> Bot2Goal !!!!!!!!!!!!!!!!!");
        //                     // Add your logic for Bot2's goal here
        //                 }
        //                 _ => (), //println!("Collision with unknown object"),
        //             }
        //         }
        //     }
        // } else {
        //     println!("Ball not found in rigid body set");
        // }

        // let ball_body = &self.rigid_body_set[self.ball];
        // println!("Ball altitude: {}", ball_body.translation().y);

        // let ball_body: &RigidBody = &self.rigid_body_set[self.ball];
        // println!(
        //     "Ball position: ({}, {})",
        //     ball_body.translation().x,
        //     ball_body.translation().z
        // );

        // if let Some(bot1) = &self.bot1 {
        //     let bot1_body: &RigidBody = &self.rigid_body_set[bot1.handle];
        //     println!(
        //         "Bot1 position: ({}, {}, {})",
        //         bot1_body.translation().x,
        //         bot1_body.translation().y,
        //         bot1_body.translation().z
        //     );
        // }

        // if let Some(bot2) = &self.bot2 {
        //     let bot2_body: &RigidBody = &self.rigid_body_set[bot2.handle];
        //     println!(
        //         "Bot2 position: ({}, {}, {})",
        //         bot2_body.translation().x,
        //         bot2_body.translation().y,
        //         bot2_body.translation().z
        //     );
        // }

        // TODO ball in camp

        let winner = {
            match event_handler.winner.read() {
                Ok(guard) => *guard, // Dereference to get the actual value
                Err(_) => {
                    // Handle potential poisoned lock
                    println!("Warning: RwLock was poisoned. Using default value.");
                    0 // Or any other default value
                }
            }
        };
        winner
    }

    pub fn execute(&mut self) -> u8 {
        println!("battle!");
        let mut counter = 0;
        let mut winner;
        loop {
            winner = self.update();
            if winner != 0 {
                // TODO winner
                println!("WINNER: {winner}");
                break;
            } else {
                counter = counter + 1;
                if counter > MAX_NUM_UPDATES {
                    // TODO draw
                    println!("DRAW: {counter} updates executed.");
                    break;
                }
            }
        }
        winner
    }
}
