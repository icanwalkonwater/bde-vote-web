#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;
use crate::app::App;

mod app;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    web_logger::init();
    yew::start_app::<App>();
    Ok(())
}
