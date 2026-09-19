//! Section header with category tag, title, and optional subtitle.

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SectionHeaderProps {
    pub tag: String,
    pub title: String,
    #[props(default = String::new())]
    pub highlight: String,
    #[props(default = String::new())]
    pub subtitle: String,
    #[props(default = true)]
    pub center: bool,
}

#[component]
pub fn SectionHeader(props: SectionHeaderProps) -> Element {
    let align_class = if props.center {
        "text-center max-w-2xl mx-auto"
    } else {
        "max-w-2xl"
    };

    rsx! {
        div { class: "{align_class} mb-12 peak-in-down",
            span { class: "sec-tag mb-3", "{props.tag}" }
            h2 { class: "text-3xl sm:text-4xl lg:text-5xl font-normal leading-tight mb-4",
                "{props.title} "
                if !props.highlight.is_empty() {
                    span { class: "grad-text", "{props.highlight}" }
                }
            }
            if !props.subtitle.is_empty() {
                p { class: "text-base sm:text-lg", style: "color:var(--c-muted);", "{props.subtitle}" }
            }
        }
    }
}
