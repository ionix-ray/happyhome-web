//! Card component with soft shadows and hover interactions.

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    #[props(default = String::new())]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        div {
            class: "soft-card p-6 {props.class}",
            {props.children}
        }
    }
}
