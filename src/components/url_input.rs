use crate::prelude::*;

#[derive(PartialEq, Props)]
pub struct UrlInputProps {
    url: String,
}

#[allow(non_snake_case)]
pub fn UrlInput(cx: Scope<UrlInputProps>) -> Element {
    let url = use_state(&cx, || "".to_string());
    let is_invalid = url.get().starts_with("invalid");
    let class_names = match is_invalid {
        true => "invalid url-input",
        false => "valid url-input",
    };

    let error_message = match is_invalid {
        true => rsx!(ErrorMessage {
            message: "Error: The URL you entered is invalid.".to_string(),
            severity: ErrorSeverity::Error
        }),
        false => rsx!(EmptyElement {}),
    };

    cx.render(rsx!(
        rsx! {
            div {
                input {
                    id: "url-input",
                    class: "{class_names}",
                    value: "{url}",
                    oninput: move |e| url.set(e.value.clone())
            }
        }},
        rsx! {
            div {
               rsx!(error_message)
            }
        }
    ))
}

#[derive(PartialEq)]
enum ErrorSeverity {
    Error,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorSeverity::Error => write!(f, "error"),
        }
    }
}

#[derive(Props, PartialEq)]
struct ErrorMessageProps {
    message: String,
    severity: ErrorSeverity,
}

#[allow(non_snake_case)]
fn ErrorMessage(cx: Scope<ErrorMessageProps>) -> Element {
    cx.render(rsx! {
        div {
            span {
                class: "error-message {cx.props.severity}",
                "{cx.props.message}"
            }
        }
    })
}

#[allow(non_snake_case)]
fn EmptyElement(_: Scope) -> Element {
    None
}
