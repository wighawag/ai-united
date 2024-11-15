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

use rapier2d::prelude::*;

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn start() -> Result<(), JsValue> {
    set_panic_hook();
    Ok(())
}

struct BotModule {
    #[allow(dead_code)]
    instance: Instance,
    store: Store,
    init: TypedFunction<u32, ()>,
    compute_actions: TypedFunction<(u32, u32, u32, u32, u32, u32), u32>,
}

const INIT_GAS: u64 = 1_000;
const COMPUTE_ACTIONS_GAS: u64 = 1_000_000;
const MAX_NUM_UPDATES: u64 = 1_000_000;

fn create_bot_module(wasm_bytes: &mut [u8]) -> BotModule {
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

    let compute_actions: TypedFunction<(u32, u32, u32, u32, u32, u32), u32> = instance
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
                    println!(
                        "Calling `init` succeeded. points left: {:?}",
                        remaining_points_after_call
                    );
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
        self_x: u32,
        self_y: u32,
        ball_x: u32,
        ball_y: u32,
        enemy_x: u32,
        enemy_y: u32,
    ) -> () {
        match self.compute_actions.call(
            &mut self.store,
            self_x,
            self_y,
            ball_x,
            ball_y,
            enemy_x,
            enemy_y,
        ) {
            Ok(_) => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let remaining_points_after_call =
                        get_remaining_points(&mut self.store, &self.instance);
                    println!(
                        "Calling `update` succeeded. points left: {:?}",
                        remaining_points_after_call
                    );
                }
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Battle {
    bot1: Option<BotModule>,
    bot2: Option<BotModule>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Battle {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new() -> Battle {
        Battle {
            bot1: None,
            bot2: None,
        }
    }
    pub fn add_bot(&mut self, wasm_bytes: &mut [u8]) {
        if let Some(_existing_bot) = &self.bot1 {
            if let Some(_existing_bot) = &self.bot2 {
                panic!("already both bots added");
            } else {
                self.bot2 = Some(create_bot_module(wasm_bytes));
            }
        } else {
            self.bot1 = Some(create_bot_module(wasm_bytes));
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
    }

    fn update(&mut self) -> u8 {
        let bot1 = self.bot1.as_mut().unwrap();
        let bot2 = self.bot2.as_mut().unwrap();

        bot1.set_remaining_points(COMPUTE_ACTIONS_GAS);
        bot2.set_remaining_points(COMPUTE_ACTIONS_GAS);

        println!("Calling `compute_actions` ...");
        // TODO
        bot1.compute_actions(0, 0, 0, 0, 0, 0);
        bot2.compute_actions(0, 0, 0, 0, 0, 0);

        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        /* Create the ground. */
        let collider = ColliderBuilder::cuboid(100.0, 0.1).build();
        collider_set.insert(collider);

        /* Create the bouncing ball. */
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 10.0])
            .build();
        let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        let ball_body_handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

        /* Create other structures necessary for the simulation. */
        let gravity = vector![0.0, -9.81];
        let integration_parameters = IntegrationParameters::default();
        let mut physics_pipeline = PhysicsPipeline::new();
        let mut island_manager = IslandManager::new();
        let mut broad_phase = DefaultBroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut impulse_joint_set = ImpulseJointSet::new();
        let mut multibody_joint_set = MultibodyJointSet::new();
        let mut ccd_solver = CCDSolver::new();
        let mut query_pipeline = QueryPipeline::new();
        let physics_hooks = ();
        let event_handler = ();

        /* Run the game loop, stepping the simulation once per frame. */
        for _ in 0..3 {
            physics_pipeline.step(
                &gravity,
                &integration_parameters,
                &mut island_manager,
                &mut broad_phase,
                &mut narrow_phase,
                &mut rigid_body_set,
                &mut collider_set,
                &mut impulse_joint_set,
                &mut multibody_joint_set,
                &mut ccd_solver,
                Some(&mut query_pipeline),
                &physics_hooks,
                &event_handler,
            );

            let ball_body = &rigid_body_set[ball_body_handle];
            println!("Ball altitude: {}", ball_body.translation().y);
        }

        // TODO ball in camp
        0
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
