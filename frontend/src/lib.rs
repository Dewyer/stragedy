#![recursion_limit="512"]

mod app;
mod routes;
mod components;
pub mod api;
pub mod storage;
pub mod handlers;

#[macro_use] extern crate yew;
#[macro_use] extern crate serde_json;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<app::App>();

    Ok(())
}
