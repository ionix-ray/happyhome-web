//! Badge component for tagging and statuses.

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    pub label: String,
    #[props(default = String::new())]
    pub class: String,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    rsx! {
        span {
            class: "inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-semibold {props.class}",
            "{props.label}"
        }
    }
}
