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

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn start() -> Result<(), JsValue> {
    set_panic_hook();
    Ok(())
}

struct WasmModule {
    #[allow(dead_code)]
    instance: Instance,
    store: Store,
    init: TypedFunction<u32, ()>,
    update: TypedFunction<(u32, u32), u32>,
}

fn create_wasm_module(wasm_bytes: &mut [u8]) -> WasmModule {
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

    println!("Compiling module...");
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

    println!("Instantiating module...");
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

    let update: TypedFunction<(u32, u32), u32> = instance
        .exports
        .get_function("update")
        .expect("failed to get function 'update'")
        .typed(&mut store)
        .expect("failed to get typed version of the function 'update'");

    WasmModule {
        instance,
        store,
        update,
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

impl WasmModule {
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

    fn update(&mut self, data1: u32, data2: u32) -> () {
        match self.update.call(&mut self.store, data1, data2) {
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

    fn get_remaining_points(&mut self) -> MeteringPoints {
        #[cfg(not(target_arch = "wasm32"))]
        let points = get_remaining_points(&mut self.store, &self.instance);

        #[cfg(target_arch = "wasm32")]
        let points = MeteringPoints::Remaining(0);

        points
    }

    #[allow(unused_variables)]
    fn set_remaining_points(&mut self, points: u64) {
        #[cfg(not(target_arch = "wasm32"))]
        set_remaining_points(&mut self.store, &self.instance, points)
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Executor {
    module1: Option<WasmModule>,
    module2: Option<WasmModule>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Executor {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new() -> Executor {
        Executor {
            module1: None,
            module2: None,
        }
    }
    pub fn add_module(&mut self, wasm_bytes: &mut [u8]) {
        if let Some(_existing_module) = &self.module1 {
            if let Some(_existing_module) = &self.module2 {
                panic!("already both modules added");
            } else {
                self.module2 = Some(create_wasm_module(wasm_bytes));
            }
        } else {
            self.module1 = Some(create_wasm_module(wasm_bytes));
        }
    }

    pub fn execute(&mut self) -> u8 {
        let module1 = self.module1.as_mut().unwrap();
        let _module2 = self.module2.as_mut().unwrap();

        module1.set_remaining_points(1000000);

        println!("Calling `init` ...");
        module1.init(0);
        println!("Calling `update` ...");
        module1.update(3, 1);

        println!("Calling `update` again....");
        module1.update(0, 4);

        println!("Calling `update` again(bis)...");
        module1.update(3, 5);

        // Now let's see how we can set a new limit...
        println!("Set new remaining points to 10");
        let new_limit = 10;
        module1.set_remaining_points(new_limit);

        let remaining_points = module1.get_remaining_points();
        // assert_eq!(remaining_points, MeteringPoints::Remaining(new_limit));

        println!("Remaining points: {:?}", remaining_points);

        3
    }
}
