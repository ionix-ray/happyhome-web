//! Organic SVG wave divider for seamless visual transitions between alternating section themes.

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DividerType {
    ToBg2,    // Light/white background -> var(--c-bg-2)
    ToWhite,  // var(--c-bg-2) -> white / slate-900
    ToTeam,   // var(--c-bg-2) -> team background
    ToFooter, // White/slate -> dark footer
}

#[component]
pub fn WaveDivider(variant: DividerType, #[props(default = false)] inverted: bool) -> Element {
    let variant_class = match variant {
        DividerType::ToBg2 => "divider-to-bg2",
        DividerType::ToWhite => "divider-to-white",
        DividerType::ToTeam => "divider-to-team",
        DividerType::ToFooter => "divider-to-footer",
    };

    let path_d = if inverted {
        "M0,0 C400,28 640,60 960,20 C1200,-5 1340,40 1440,24 L1440,68 L0,68 Z"
    } else {
        "M0,0 C320,44 540,16 720,36 C960,56 1200,12 1440,44 L1440,68 L0,68 Z"
    };

    rsx! {
        div {
            class: "section-divider {variant_class}",
            "aria-hidden": "true",
            svg {
                view_box: "0 0 1440 68",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                preserve_aspect_ratio: "none",
                path {
                    d: "{path_d}",
                }
            }
        }
    }
}
