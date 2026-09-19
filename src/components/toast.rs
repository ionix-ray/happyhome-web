//! Toast notification component for form success and clinical messages.

use crate::components::icons::{CheckCircleIcon, CloseIcon};
use crate::state::ToastSignal;
use dioxus::prelude::*;

#[component]
pub fn Toast() -> Element {
    let mut toast_signal = use_context::<ToastSignal>();
    let msg_opt = toast_signal.0.read().clone();

    if let Some(msg) = msg_opt {
        rsx! {
            div {
                class: "fixed bottom-6 left-1/2 -translate-x-1/2 z-50 bg-slate-900 text-white px-6 py-4 rounded-2xl shadow-2xl border border-slate-700 flex items-center gap-3 animate-fade-in-up",
                CheckCircleIcon { size: 20, color: "#10b981" }
                span { class: "text-sm font-semibold", "{msg}" }
                button {
                    class: "ml-4 text-slate-400 hover:text-white transition-colors flex items-center justify-center",
                    onclick: move |_| *toast_signal.0.write() = None,
                    "aria-label": "Close notification",
                    CloseIcon { size: 16, color: "currentColor" }
                }
            }
        }
    } else {
        rsx! {}
    }
}
