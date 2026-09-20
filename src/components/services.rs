//! Services section displaying specialized clinical & rehabilitation offerings.

use crate::components::common::SectionHeader;
use crate::components::icons::{ArrowRightIcon, DynamicIcon, HospitalIcon, StethoscopeIcon};
use crate::domain::config::SiteConfig;
use crate::state::{BrandMode, BrandModeSignal};
use dioxus::prelude::*;

#[component]
pub fn Services() -> Element {
    let config = use_context::<SiteConfig>();
    let mut brand_signal = use_context::<BrandModeSignal>();
    let brand = *brand_signal.0.read();

    let (tag, title, highlight, subtitle, services) = match brand {
        BrandMode::Hospital => (
            "Specialized Medical Services",
            "Comprehensive Clinical",
            "Specialties",
            "Emergency, critical care, and advanced surgical recovery led by senior consultants.",
            config.hospital.services.clone(),
        ),
        BrandMode::Rehab => (
            "Inpatient Neuro-Rehabilitation",
            "Evidence-Based Recovery",
            "Programs",
            "Restoring independence through specialized neuro, spinal, trauma, and critical post-surgical care.",
            config.rehab.services.clone(),
        ),
    };

    rsx! {
        section {
            id: "services",
            class: "py-20",
            style: "background: var(--c-bg-2);",
            "aria-labelledby": "services-heading",

            div { class: "container-x",
                SectionHeader {
                    tag: tag.to_string(),
                    title: title.to_string(),
                    highlight: highlight.to_string(),
                    subtitle: subtitle.to_string(),
                    center: true,
                }

                // In-Page Interactive Dual-Track Explorer Tabs
                div { class: "flex justify-center mb-12",
                    div {
                        class: "inline-flex p-1.5 rounded-2xl bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 shadow-md",
                        role: "tablist",
                        "aria-label": "Select clinical track",

                        button {
                            class: if brand == BrandMode::Hospital {
                                "px-5 sm:px-6 py-2.5 rounded-xl bg-gradient-to-r from-blue-700 to-sky-600 text-white font-bold text-xs uppercase tracking-wider shadow-sm transition-all flex items-center gap-2"
                            } else {
                                "px-5 sm:px-6 py-2.5 rounded-xl text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white font-bold text-xs uppercase tracking-wider transition-all flex items-center gap-2"
                            },
                            onclick: move |_| *brand_signal.0.write() = BrandMode::Hospital,
                            HospitalIcon { size: 16, color: "currentColor" }
                            span { "Hospital Wing (6 Specialties)" }
                        }
                        button {
                            class: if brand == BrandMode::Rehab {
                                "px-5 sm:px-6 py-2.5 rounded-xl bg-gradient-to-r from-teal-700 to-emerald-600 text-white font-bold text-xs uppercase tracking-wider shadow-sm transition-all flex items-center gap-2"
                            } else {
                                "px-5 sm:px-6 py-2.5 rounded-xl text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white font-bold text-xs uppercase tracking-wider transition-all flex items-center gap-2"
                            },
                            onclick: move |_| *brand_signal.0.write() = BrandMode::Rehab,
                            StethoscopeIcon { size: 16, color: "currentColor" }
                            span { "Neuro-Rehab Centre (9 Programs)" }
                        }
                    }
                }

                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                    for (idx, service) in services.into_iter().enumerate() {
                        {
                            let img_src = match idx % 4 {
                                0 => "/assets/images/card_doctor.jpg",
                                1 => "/assets/images/card_icu.jpg",
                                2 => "/assets/images/card_rehab.jpg",
                                _ => "/assets/images/card_corridor.jpg",
                            };
                            let delay_cls = match idx % 3 {
                                0 => "reveal-stagger-1",
                                1 => "reveal-stagger-2",
                                _ => "reveal-stagger-3",
                            };
                            let badge_cls = match idx % 6 {
                                0 => "badge-cyan",
                                1 => "badge-teal",
                                2 => "badge-indigo",
                                3 => "badge-amber",
                                4 => "badge-rose",
                                _ => "badge-purple",
                            };
                            rsx! {
                                div {
                                    key: "{service.slug}",
                                    class: "cure-card cure-card-split group reveal-init {delay_cls}",

                                    // Upper Image Half with text & icon overlay
                                    div { class: "cure-card-img-wrap",
                                        img {
                                            src: "{img_src}",
                                            alt: "{service.name}",
                                            class: "cure-card-img",
                                            loading: "lazy",
                                        }
                                        div { class: "cure-card-img-overlay",
                                            div { class: "flex items-center justify-between w-full",
                                                div { class: "icon-badge {badge_cls} !w-10 !h-10 !rounded-xl text-sm shadow-md",
                                                    DynamicIcon { name: service.icon_key.clone(), size: 20, color: "currentColor" }
                                                }
                                                span { class: "text-[10px] font-extrabold uppercase tracking-widest px-2.5 py-1 rounded-full bg-slate-950/70 text-white backdrop-blur-md border border-white/20",
                                                    "Specialist Care"
                                                }
                                            }
                                        }
                                    }

                                    // Lower Half: Text, description, CTA
                                    div { class: "cure-card-body",
                                        div {
                                            h3 { class: "text-lg font-bold mb-2 text-slate-900 dark:text-white group-hover:text-sky-600 dark:group-hover:text-sky-400 transition-colors",
                                                "{service.name}"
                                            }
                                            p { class: "text-xs sm:text-sm leading-relaxed mb-6 font-medium text-slate-600 dark:text-slate-300",
                                                "{service.description}"
                                            }
                                        }

                                        div { class: "pt-4 border-t flex items-center justify-between", style: "border-color: var(--c-line);",
                                            a {
                                                href: "#appointment",
                                                class: "inline-flex items-center gap-1.5 text-xs font-bold text-sky-600 hover:text-sky-700 dark:text-sky-400 dark:hover:text-sky-300 transition-all group-hover:translate-x-1",
                                                "Consult Specialist"
                                                ArrowRightIcon { size: 14, color: "currentColor" }
                                            }
                                            span { class: "text-[10px] font-semibold uppercase tracking-wider px-2 py-0.5 rounded bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-300 flex items-center gap-1.5",
                                                span { class: "w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse" }
                                                "24/7 Ready"
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
