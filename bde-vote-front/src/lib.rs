#![recursion_limit = "512"]

use crate::app::App;
use wasm_bindgen::prelude::*;

mod app;
mod vote_btn;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    web_logger::init();
    yew::start_app::<App>();
    Ok(())
}
