//! Doctor detail and fast-booking modal dialog.

use dioxus::prelude::*;
use crate::state::ModalDoctorSignal;
use crate::components::icons::{StethoscopeIcon, CloseIcon};

#[component]
pub fn DoctorModal() -> Element {
    let mut modal_signal = use_context::<ModalDoctorSignal>();
    let doc_opt = modal_signal.0.read().clone();

    if let Some(doc_name) = doc_opt {
        rsx! {
            div {
                class: "fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-950/60 backdrop-blur-sm animate-fade-in",
                onclick: move |_| *modal_signal.0.write() = None,

                div {
                    class: "bg-white dark:bg-slate-900 rounded-3xl max-w-lg w-full p-8 shadow-2xl border border-slate-200 dark:border-slate-800 relative",
                    onclick: move |e| e.stop_propagation(),

                    button {
                        class: "absolute top-6 right-6 text-slate-400 hover:text-slate-700 dark:hover:text-white flex items-center justify-center",
                        onclick: move |_| *modal_signal.0.write() = None,
                        "aria-label": "Close modal",
                        CloseIcon { size: 20, color: "currentColor" }
                    }

                    div { class: "flex items-center gap-4 mb-6",
                        div { class: "w-14 h-14 rounded-2xl bg-sky-100 dark:bg-sky-950/60 flex items-center justify-center text-sky-700 dark:text-sky-300 border border-sky-300/40 dark:border-sky-800/40",
                            StethoscopeIcon { size: 28, color: "var(--c-brand)" }
                        }
                        div {
                            h3 { class: "text-xl font-bold text-slate-900 dark:text-white", "{doc_name}" }
                            p { class: "text-xs font-bold text-sky-600 dark:text-sky-400", "Consultant Specialist" }
                        }
                    }

                    p { class: "text-sm text-slate-700 dark:text-slate-200 leading-relaxed mb-6 font-medium",
                        "You are scheduling a consultation with {doc_name}. Please continue to our online booking desk to choose your preferred morning or evening OPD slot."
                    }

                    div { class: "flex gap-3",
                        a {
                            href: "#appointment",
                            class: "btn-solid flex-1 text-center py-3 rounded-xl font-bold text-sm",
                            onclick: move |_| *modal_signal.0.write() = None,
                            "Proceed to Booking Desk"
                        }
                        button {
                            class: "btn-outline py-3 px-5 rounded-xl font-semibold text-sm",
                            onclick: move |_| *modal_signal.0.write() = None,
                            "Close"
                        }
                    }
                }
            }
        }
    } else {
        rsx! {}
    }
}
