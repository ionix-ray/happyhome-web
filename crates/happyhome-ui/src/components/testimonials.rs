//! Testimonials component rendering patient recovery journeys and family experiences.

use dioxus::prelude::*;
use happyhome_domain::config::SiteConfig;
use crate::state::{BrandMode, BrandModeSignal};
use crate::components::icons::StarIcon;
use crate::components::common::SectionHeader;

#[component]
pub fn Testimonials() -> Element {
    let config = use_context::<SiteConfig>();
    let brand_signal = use_context::<BrandModeSignal>();
    let brand = *brand_signal.0.read();

    let (tag, title, highlight, subtitle, testimonials) = match brand {
        BrandMode::Hospital => (
            "Patient Testimonials",
            "Voices of Hope and",
            "Healing",
            "Real stories from families treated at Happy Home Hospital.",
            &config.hospital.testimonials,
        ),
        BrandMode::Rehab => (
            "Recovery Journeys",
            "Bringing Back Smiles, One",
            "Step at a Time",
            "Inspiring recovery accounts from our neuro and spine rehabilitation inpatients.",
            &config.rehab.testimonials,
        ),
    };

    rsx! {
        section {
            id: "testimonials",
            class: "py-20 bg-white dark:bg-slate-900",
            "aria-labelledby": "testimonials-heading",

            div { class: "container-x",
                SectionHeader {
                    tag: tag.to_string(),
                    title: title.to_string(),
                    highlight: highlight.to_string(),
                    subtitle: subtitle.to_string(),
                    center: true,
                }

                div { class: "grid grid-cols-1 md:grid-cols-3 gap-8",
                    for (idx, testi) in testimonials.iter().enumerate() {
                        {
                            let delay_cls = match idx % 3 {
                                0 => "delay-100",
                                1 => "delay-200",
                                _ => "delay-300",
                            };
                            rsx! {
                                div {
                                    key: "{testi.name}",
                                    class: "testi-card p-8 flex flex-col justify-between pt-12 mt-4 peak-in-up {delay_cls} hover:-translate-y-1 transition-all duration-300 shadow-sm hover:shadow-md",

                                    div {
                                        // Star rating row
                                        div { class: "flex items-center gap-1 mb-4",
                                            for _ in 0..testi.stars {
                                                StarIcon { size: 16, color: "#f59e0b" }
                                            }
                                        }

                                        p { class: "text-sm text-slate-700 dark:text-slate-300 leading-relaxed italic mb-6",
                                            "\"{testi.text}\""
                                        }
                                    }

                                    div { class: "pt-4 border-t border-slate-100 dark:border-slate-800 flex items-center gap-3",
                                        div { class: "w-10 h-10 rounded-full bg-sky-100 dark:bg-sky-950/60 text-sky-800 dark:text-sky-300 font-extrabold flex items-center justify-center text-sm border border-sky-300/40 dark:border-sky-800/40",
                                            "{testi.name.chars().next().unwrap_or('P')}"
                                        }
                                        div {
                                            div { class: "font-bold text-sm text-slate-900 dark:text-white",
                                                "{testi.name}"
                                            }
                                            div { class: "text-xs text-slate-600 dark:text-slate-300 font-medium",
                                                "{testi.role}"
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
}
