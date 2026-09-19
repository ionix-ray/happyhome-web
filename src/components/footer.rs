//! Comprehensive Footer component with clinical accreditations and quick links.

use crate::components::icons::{MailIcon, MapPinIcon, PhoneIcon};
use crate::domain::config::SiteConfig;
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    let config = use_context::<SiteConfig>();

    rsx! {
        footer { class: "bg-slate-950 text-slate-300 pt-16 pb-12 border-t border-slate-800",
            div { class: "container-x",
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-10 mb-12",

                    // Col 1: Brand & Mission
                    div {
                        div { class: "mb-5 inline-block",
                            img {
                                src: "/assets/images/happy-home-final-logo.png",
                                alt: "Happy Home Hospital & Rehab",
                                class: "h-14 w-auto object-contain brightness-0 invert",
                            }
                        }
                        p { class: "text-xs text-slate-400 leading-relaxed mb-6 font-medium",
                            "{config.footer.about_text}"
                        }
                        div { class: "text-xs text-slate-400 font-semibold",
                            "Established 2021 · Regd. Healthcare Facility"
                        }
                    }

                    // Col 2: Quick Links
                    div {
                        h4 { class: "text-sm font-bold text-white uppercase tracking-wider mb-4", "Quick Links" }
                        ul { class: "space-y-2.5 text-xs",
                            for link in config.footer.quick_links.iter() {
                                li {
                                    a {
                                        href: "{link.url}",
                                        class: "text-slate-400 hover:text-sky-400 transition-colors font-medium",
                                        "{link.label}"
                                    }
                                }
                            }
                        }
                    }

                    // Col 3: Core Specialties
                    div {
                        h4 { class: "text-sm font-bold text-white uppercase tracking-wider mb-4", "Specialized Care" }
                        ul { class: "space-y-2.5 text-xs text-slate-400 font-medium",
                            li { "Post Neuro-Surgery Inpatient Rehab" }
                            li { "Spinal Cord Trauma & Paraplegia" }
                            li { "Traumatic Brain Injury (TBI) Protocols" }
                            li { "Advanced Bedsore & Air-Bed Healing" }
                            li { "Tracheostomy Weaning & Airway Unit" }
                            li { "24/7 Critical Neuro ICU & HDU" }
                        }
                    }

                    // Col 4: 24/7 Hotline
                    div {
                        h4 { class: "text-sm font-bold text-white uppercase tracking-wider mb-4", "Emergency & Admission" }
                        div { class: "space-y-3 text-xs text-slate-400",
                            div { class: "flex items-start gap-2.5",
                                MapPinIcon { size: 16, color: "var(--c-brand)" }
                                span { "{config.contact.address}" }
                            }
                            div { class: "flex items-center gap-2.5",
                                PhoneIcon { size: 16, color: "var(--c-brand)" }
                                a { href: "tel:{config.emergency.phone}", class: "text-white font-bold hover:text-sky-400 transition-colors", "{config.emergency.phone}" }
                            }
                            div { class: "flex items-center gap-2.5",
                                MailIcon { size: 16, color: "var(--c-brand)" }
                                span { "{config.contact.email}" }
                            }
                        }
                        div { class: "mt-6 p-3 rounded-xl bg-slate-900 border border-slate-800 text-[11px] text-sky-400 font-semibold",
                            "Emergency Trauma Admissions Open 24/7"
                        }
                    }
                }

                // Bottom bar
                div { class: "pt-8 border-t border-slate-800/80 flex flex-col sm:flex-row items-center justify-between text-xs text-slate-500 gap-4",
                    p { "© 2026 Happy Home Hospital And Rehab. All rights reserved." }
                    div { class: "flex flex-wrap gap-6",
                        for link in config.footer.legal_links.iter() {
                            a {
                                href: "{link.url}",
                                class: "hover:text-slate-400 transition-colors",
                                "{link.label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
