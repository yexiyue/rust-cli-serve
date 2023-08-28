use crate::page::{Home::Home, Login::Login};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Router {
    #[route("/")]
    Home {},

    #[route("/login")]
    Login {},

    #[route("/:..path")]
    NotFound{path:Vec<String>}
}

#[inline_props]
fn NotFound(cx: Scope, path: Vec<String>) -> Element {
    render!(
      div{
        class: "flex justify-center items-center h-screen w-full flex-col text-center bg-no-repeat bg-cover bg-center",
        style:"background-image:url(/images/error-bg.png)",
        h1 {
          class: " text-4xl",
          "404 Not Found"
        }
        p {
          class: "text-2xl",
          "The page you requested could not be found."
        }
      }
    )
}
