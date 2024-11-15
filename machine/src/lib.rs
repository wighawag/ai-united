#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::JsValue;

#[cfg(target_arch = "wasm32")]
mod utils;
#[cfg(target_arch = "wasm32")]
use utils::set_panic_hook;

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn start() -> Result<(), JsValue> {
    set_panic_hook();
    Ok(())
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Executor {}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Executor {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new() -> Executor {
        Executor {}
    }

    pub fn execute(&mut self, payload: &mut [u8]) -> u8 {
        payload[0]
    }
}
