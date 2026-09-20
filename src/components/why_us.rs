//! Why Us component highlighting core institutional strengths and clinical differentiators.

use crate::components::common::SectionHeader;
use crate::components::icons::{CarbonShieldCheckIcon, DynamicIcon};
use crate::domain::config::SiteConfig;
use crate::state::{BrandMode, BrandModeSignal};
use dioxus::prelude::*;

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

                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 lg:gap-8",
                    for (idx, item) in why_us.into_iter().enumerate() {
                        {
                            let img_src = match idx % 4 {
                                0 => "/assets/images/card_corridor.jpg",
                                1 => "/assets/images/card_icu.jpg",
                                2 => "/assets/images/card_rehab.jpg",
                                _ => "/assets/images/card_doctor.jpg",
                            };
                            let delay_cls = match idx % 4 {
                                0 => "reveal-stagger-1",
                                1 => "reveal-stagger-2",
                                2 => "reveal-stagger-3",
                                _ => "reveal-stagger-4",
                            };
                            let badge_cls = match idx % 4 {
                                0 => "badge-cyan",
                                1 => "badge-teal",
                                2 => "badge-indigo",
                                _ => "badge-amber",
                            };
                            rsx! {
                                div {
                                    key: "{item.title}",
                                    class: "cure-card cure-card-split group reveal-init {delay_cls}",

                                    // Upper Image Half
                                    div { class: "cure-card-img-wrap !h-36",
                                        img {
                                            src: "{img_src}",
                                            alt: "{item.title}",
                                            class: "cure-card-img",
                                            loading: "lazy",
                                        }
                                        div { class: "cure-card-img-overlay",
                                            div { class: "flex items-center justify-between w-full",
                                                div { class: "icon-badge {badge_cls} !w-9 !h-9 !rounded-xl shadow-md",
                                                    DynamicIcon { name: item.icon_key, size: 18, color: "currentColor" }
                                                }
                                                span { class: "text-lg font-black text-white/90 drop-shadow-md",
                                                    "0{idx + 1}"
                                                }
                                            }
                                        }
                                    }

                                    // Lower Half: Text
                                    div { class: "cure-card-body !p-5",
                                        div {
                                            h3 { class: "text-base font-bold text-slate-900 dark:text-white mb-2 group-hover:text-sky-600 dark:group-hover:text-sky-400 transition-colors",
                                                "{item.title}"
                                            }
                                            p { class: "text-xs leading-relaxed text-slate-600 dark:text-slate-300 font-medium",
                                                "{item.description}"
                                            }
                                        }

                                        div { class: "mt-4 pt-3 border-t border-slate-100 dark:border-slate-800 flex items-center gap-1.5 text-[11px] font-bold text-teal-600 dark:text-teal-400",
                                            CarbonShieldCheckIcon { size: 15, color: "currentColor" }
                                            span { "Clinical Standard" }
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
