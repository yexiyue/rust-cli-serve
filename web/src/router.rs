use crate::page::{Home::Home, Login::Login};
use dioxus::{html::h2, prelude::*};
use dioxus_router::prelude::*;
use tracing::info;

#[rustfmt::skip]
#[derive(Debug, Clone,PartialEq, Routable)]
pub enum Router{
    #[route("/login")]
    Login{},

    #[route("/home")]
    Home{},
 
    #[route("/**")]
    NotFound{},
}

fn NotFound(cx: Scope) -> Element {
    cx.render(rsx!(
      h1 { "404 Not Found" }
      p { "页面不存在" }
    ))
}
