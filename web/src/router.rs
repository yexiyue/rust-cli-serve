use crate::page::{Home::Home, Login::Login};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[rustfmt::skip]
#[derive(Debug, Clone,PartialEq, Routable)]
pub enum Router{
  #[route("/login")]
  Login{},

  #[route("/home")]
  Home{},

  #[route("/:.._path")]
  NotFound{_path:Vec<String>},
}

#[inline_props]
fn NotFound(cx: Scope, _path: Vec<String>) -> Element {
    render!(
      h1 {
        class: " text-2xl text-red-500 bg-slate-500",
        "404 Not Found"
      }
      p { "The page you requested could not be found." }
    )
}
