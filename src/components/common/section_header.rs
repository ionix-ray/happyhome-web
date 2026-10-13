//! Section header with category tag, title, and optional subtitle.
//!
//! Title markup comes from `common::fx::word_masks` — one masked span per
//! word carrying `--i` — so `assets/styles.css` section 8 can cascade a
//! left-to-right masked rise driven by a single `view-timeline` declared on
//! `.sec-head`. Plain text stays intact for assistive tech, since the spans
//! are inline and non-breaking to the accessibility tree.

use crate::components::common::word_masks;
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
        div { class: "{align_class} mb-12 sec-head",
            if !props.tag.is_empty() {
                span { class: "sec-tag mb-3", "{props.tag}" }
            }
            h2 { class: "sec-title text-3xl sm:text-4xl lg:text-5xl font-normal leading-tight mb-4",
                { word_masks(&props.title, &props.highlight) }
            }
            if !props.subtitle.is_empty() {
                p { class: "sec-sub text-base sm:text-lg", style: "color:var(--c-muted);", "{props.subtitle}" }
            }
        }
    }
}
