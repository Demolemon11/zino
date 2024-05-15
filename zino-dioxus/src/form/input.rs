use crate::class::Class;
use dioxus::prelude::*;

/// The text input and its variations.
pub fn Input(props: InputProps) -> Element {
    rsx! {
       input {
            class: props.class,
            r#type: "text",
            ..props.attributes,
        }
    }
}

/// The [`Input`] properties struct for the configuration of the component.
#[derive(Clone, PartialEq, Props)]
pub struct InputProps {
    /// The class attribute for the component.
    #[props(into, default = "input".into())]
    pub class: Class,
    /// Spreading the props of the `input` element.
    #[props(extends = input)]
    attributes: Vec<Attribute>,
}
