//! Services section displaying specialized clinical & rehabilitation offerings.

use crate::components::common::SectionHeader;
use crate::components::icons::{ArrowRightIcon, DynamicIcon};
use crate::domain::config::SiteConfig;
use crate::state::{BrandMode, BrandModeSignal};
use dioxus::prelude::*;

#[component]
pub fn Services() -> Element {
    let config = use_context::<SiteConfig>();
    let brand_signal = use_context::<BrandModeSignal>();
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

                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                    for (idx, service) in services.into_iter().enumerate() {
                        {
                            let delay_cls = match idx % 3 {
                                0 => "delay-100",
                                1 => "delay-200",
                                _ => "delay-300",
                            };
                            rsx! {
                                div {
                                    key: "{service.slug}",
                                    class: " p-8 flex flex-col justify-between group peak-in-up {delay_cls}",

                                    div {
                                        div { class: "icon-ring mb-6 group-hover:scale-110 transition-transform duration-300 shadow-sm",
                                            DynamicIcon { name: service.icon_key.clone(), size: 26, color: "var(--c-brand)" }
                                        }
                                        h3 { class: "text-xl font-bold mb-3 text-slate-900 dark:text-white group-hover:text-sky-600 dark:group-hover:text-sky-400 transition-colors",
                                            "{service.name}"
                                        }
                                        p { class: "text-sm leading-relaxed mb-6 font-medium text-slate-700 dark:text-slate-200",
                                            "{service.description}"
                                        }
                                    }

                                    div { class: "pt-4 border-t flex items-center justify-between", style: "border-color: var(--c-line);",
                                        a {
                                            href: "#appointment",
                                            class: "inline-flex items-center gap-2 text-sm font-bold text-sky-600 hover:text-sky-700 dark:text-sky-400 dark:hover:text-sky-300 transition-all duration-200 group-hover:translate-x-1",
                                            "Consult Specialist"
                                            ArrowRightIcon { size: 16, color: "currentColor" }
                                        }
                                        span { class: "text-[11px] font-semibold uppercase tracking-wider px-2.5 py-1 rounded bg-slate-100 dark:bg-slate-800/90 text-slate-600 dark:text-slate-300 flex items-center gap-1.5",
                                            span { class: "w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse" }
                                            "24/7 Care"
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
