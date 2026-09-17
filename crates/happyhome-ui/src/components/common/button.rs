//! Button component with solid, outline, and ghost variants.

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Solid,
    Outline,
    Ghost,
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    pub label: String,
    #[props(default)]
    pub variant: ButtonVariant,
    #[props(default = String::new())]
    pub href: String,
    #[props(default = String::new())]
    pub class: String,
    pub onclick: Option<EventHandler<MouseEvent>>,
    pub children: Option<Element>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let variant_class = match props.variant {
        ButtonVariant::Solid => "btn-solid",
        ButtonVariant::Outline => "btn-outline",
        ButtonVariant::Ghost => "btn-ghost",
    };

    if !props.href.is_empty() {
        rsx! {
            a {
                class: "{variant_class} inline-flex items-center justify-center gap-2 px-6 py-3 rounded-xl font-semibold transition-all {props.class}",
                href: "{props.href}",
                "{props.label}"
                {props.children}
            }
        }
    } else {
        rsx! {
            button {
                class: "{variant_class} inline-flex items-center justify-center gap-2 px-6 py-3 rounded-xl font-semibold transition-all {props.class}",
                onclick: move |e| {
                    if let Some(ref handler) = props.onclick {
                        handler.call(e);
                    }
                },
                "{props.label}"
                {props.children}
            }
        }
    }
}
