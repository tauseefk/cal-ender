use crate::prelude::*;

#[allow(non_snake_case)]
pub fn Root(cx: Scope) -> Element {
    cx.render(rsx! {
        url_input::UrlInput {
            url: String::from(""),
        }
    })
}
