#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use tracing::info;
pub mod page;
pub mod router;

pub fn App(cx: Scope)->Element {
    render! {
        Router::<router::Router>{}
    }
}

pub fn start(){
    tracing_wasm::set_as_global_default();
    dioxus_web::launch(App);
    info!("App launched!");
}