use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::router::Router;
pub fn Home(cx: Scope) -> Element {
    render! {
        h1{
            "Home page"
        }
        Link{
            to:Router::Login{},
            "Login"
        }
    }
}
