//! FAQ component rendering accessible question-and-answer accordions.

use crate::components::common::SectionHeader;
use crate::components::icons::ChevronDownIcon;
use crate::domain::config::SiteConfig;
use crate::state::{BrandMode, BrandModeSignal};
use dioxus::prelude::*;

#[component]
pub fn Faq() -> Element {
    let config = use_context::<SiteConfig>();
    let brand_signal = use_context::<BrandModeSignal>();
    let brand = *brand_signal.0.read();

    let mut open_indices = use_signal(|| vec![0]); // First FAQ open by default

    let (tag, title, highlight, subtitle, faqs) = match brand {
        BrandMode::Hospital => (
            "Frequently Asked Questions",
            "Hospital Queries &",
            "Admission Guidelines",
            "Answers to common queries regarding doctor consultations, emergency triage, and facilities.",
            &config.hospital.faqs,
        ),
        BrandMode::Rehab => (
            "Patient & Family Guidance",
            "Inpatient Rehabilitation",
            "FAQ",
            "Detailed answers regarding recovery timeline, attendant stay, and individualized neuro therapy.",
            &config.rehab.faqs,
        ),
    };

    rsx! {
        section {
            id: "faq",
            class: "py-20",
            style: "background: var(--c-bg-2);",
            "aria-labelledby": "faq-heading",

            div { class: "container-x max-w-4xl",
                SectionHeader {
                    tag: tag.to_string(),
                    title: title.to_string(),
                    highlight: highlight.to_string(),
                    subtitle: subtitle.to_string(),
                    center: true,
                }

                div { class: "space-y-4",
                    for (idx, item) in faqs.iter().enumerate() {
                        {
                            let is_open = open_indices.read().contains(&idx);
                            let delay_cls = match idx % 4 {
                                0 => "delay-100",
                                1 => "delay-200",
                                2 => "delay-300",
                                _ => "delay-400",
                            };
                            rsx! {
                                div {
                                    key: "{item.question}",
                                    class: " overflow-hidden transition-all duration-300 peak-in-up {delay_cls}",

                                    button {
                                        class: "w-full p-6 text-left flex items-center justify-between gap-4 font-bold text-slate-900 dark:text-white hover:text-sky-600 dark:hover:text-sky-400 transition-colors",
                                        onclick: move |_| {
                                            let mut list = open_indices.read().clone();
                                            if list.contains(&idx) {
                                                list.retain(|&i| i != idx);
                                            } else {
                                                list.push(idx);
                                            }
                                            *open_indices.write() = list;
                                        },
                                        "aria-expanded": if is_open { "true" } else { "false" },
                                        span { class: "text-base sm:text-lg", "{item.question}" }
                                        span {
                                            class: "transition-transform duration-300 shrink-0",
                                            style: if is_open { "transform: rotate(180deg);" } else { "transform: rotate(0deg);" },
                                            ChevronDownIcon { size: 20, color: "var(--c-brand)" }
                                        }
                                    }

                                    if is_open {
                                        div { class: "px-6 pb-6 text-sm text-slate-700 dark:text-slate-200 leading-relaxed border-t border-slate-100 dark:border-slate-800 pt-4 font-medium",
                                            "{item.answer}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
