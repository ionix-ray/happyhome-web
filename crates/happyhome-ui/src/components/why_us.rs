//! Why Us component highlighting core institutional strengths and clinical differentiators.

use dioxus::prelude::*;
use happyhome_domain::config::SiteConfig;
use crate::state::{BrandMode, BrandModeSignal};
use crate::components::icons::{DynamicIcon, CarbonShieldCheckIcon};
use crate::components::common::SectionHeader;

#[component]
pub fn WhyUs() -> Element {
    let config = use_context::<SiteConfig>();
    let brand_signal = use_context::<BrandModeSignal>();
    let brand = *brand_signal.0.read();

    let (tag, title, highlight, subtitle, why_us) = match brand {
        BrandMode::Hospital => (
            "Why Choose Us",
            "Setting the Standard for",
            "Inpatient Excellence",
            "Clinical rigor, compassionate doctors, and infection-controlled facilities you can rely on.",
            config.hospital.why_us.clone(),
        ),
        BrandMode::Rehab => (
            "Clinical Advantages",
            "Why Families Trust Our",
            "Rehabilitation Center",
            "We focus exclusively on long-term neurological, spinal, and post-surgical recovery.",
            config.rehab.why_us.clone(),
        ),
    };

    rsx! {
        section {
            id: "why-us",
            class: "py-20 bg-white dark:bg-slate-900",
            "aria-labelledby": "why-us-heading",

            div { class: "container-x",
                SectionHeader {
                    tag: tag.to_string(),
                    title: title.to_string(),
                    highlight: highlight.to_string(),
                    subtitle: subtitle.to_string(),
                    center: true,
                }

                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8",
                    for (idx, item) in why_us.into_iter().enumerate() {
                        {
                            let delay_cls = match idx % 4 {
                                0 => "delay-100",
                                1 => "delay-200",
                                2 => "delay-300",
                                _ => "delay-400",
                            };
                            rsx! {
                                div {
                                    key: "{item.title}",
                                    class: "soft-card p-8 flex flex-col justify-between border-t-4 border-t-sky-500 hover:-translate-y-2 transition-all duration-300 peak-in-up {delay_cls}",

                                    div {
                                        div { class: "flex items-center justify-between mb-6",
                                            div { class: "icon-ring shadow-sm",
                                                DynamicIcon { name: item.icon_key, size: 24, color: "var(--c-brand)" }
                                            }
                                            span { class: "text-2xl font-black text-slate-200 dark:text-slate-800",
                                                "0{idx + 1}"
                                            }
                                        }

                                        h3 { class: "text-lg font-bold text-slate-900 dark:text-white mb-3",
                                            "{item.title}"
                                        }

                                        p { class: "text-sm text-slate-700 dark:text-slate-200 leading-relaxed font-medium",
                                            "{item.description}"
                                        }
                                    }

                                    div { class: "mt-6 pt-4 border-t border-slate-100 dark:border-slate-800 flex items-center gap-2 text-xs font-bold text-teal-600 dark:text-teal-400",
                                        CarbonShieldCheckIcon { size: 18, color: "currentColor" }
                                        span { "Clinical Standard Verified" }
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
