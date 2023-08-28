use dioxus::html::{p, button};
use dioxus::prelude::EventHandler;
use dioxus::prelude::*;
use tracing::info;

pub fn Login(cx: Scope) -> Element {
    let username = use_state(cx, String::default);
    let password=use_state(cx, String::default);
    render! {
        div{
            class: "h-screen w-screen flex justify-center items-center bg-no-repeat bg-cover bg-center",
            style: "background-image:url(/images/login-bg3.png)",
            section{
                class: " w-1/4 rounded-xl flex flex-col h-48 text-center items-center backdrop-blur bg-transparent",
                p{  
                    class: " my-4 text-blue-400",
                    "管理员登录"
                }
                Input{
                    class: " my-1 w-full",
                    name:"账号",
                    value:username,
                    on_change:|v|{
                        username.set(v);
                    }
                }
                Input{
                    class: " my-1 w-full",
                    name:"密码",
                    value:password,
                    on_change:|v|{
                        password.set(v);
                    }
                }
                button{
                    class: " mt-3 outline-none border-none w-1/3 bg-blue-300 text-white hover:bg-green-300 border transition h-9 rounded-xl",
                    "登录"
                }
            }
        }
    }
}
#[inline_props]
pub fn Input<'a>(
    cx: Scope,
    class: Option<&'a str>,
    name: &'a str,
    value: &'a str,
    on_change: EventHandler<'a, String>,
) -> Element {
    let c=match class {
        Some(v)=>v,
        None=>"",
    };
    render! {
        div{
            class:"{c}",
            div{
                class: "h-8 w-full flex items-center",
                label{
                    class: " px-4 text-sm text-blue-300",
                    r#for: "{name}",
                    "{name}"
                }
                input{
                    class: " outline-none pl-2 mr-4 bg-transparent placeholder:text-green-300 border-solid border placeholder:text-sm flex-1 rounded transition focus:border-green-300  hover:border-blue-300",
                    name:"{name}",
                    value: "{value}",
                    placeholder:"输入{name}",
                    oninput: |e|{
                        on_change.call(e.value.clone());
                    },
                }
            }
        }
    }
}
