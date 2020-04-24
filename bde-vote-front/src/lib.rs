#![recursion_limit = "512"]

use crate::home::Home;
use wasm_bindgen::prelude::*;

mod app;
mod home;
mod list_panel;
mod vote_btn;
mod confirmed_vote;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    web_logger::init();
    yew::start_app::<Home>();
    Ok(())
}
