//! Comprehensive Footer component with clinical accreditations, certifications, and quick links.

use crate::components::icons::{
    CarbonActivityPulseIcon, CarbonShieldCheckIcon, CarbonShieldQualityIcon, CheckCircleIcon,
    HospitalIcon, MailIcon, MapPinIcon, PhoneIcon,
};
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
                        a {
                            href: "#home",
                            class: "mb-5 inline-block transition-opacity hover:opacity-90",
                            img {
                                src: "/assets/images/Logo-happy-home-darkmode.png",
                                alt: "Happy Home Hospital & Rehab",
                                class: "h-14 w-auto object-contain",
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

                // ── Clinical Accreditations & Certifications Strip ──
                div { class: "py-8 my-8 border-y border-slate-800/80",
                    div { class: "text-center mb-6",
                        div { class: "text-[11px] font-bold uppercase tracking-widest text-sky-400 mb-1",
                            "Accreditations & Regulatory Certifications"
                        }
                        h4 { class: "text-lg font-normal text-white serif",
                            "Certified Standards of Clinical Excellence & Patient Safety"
                        }
                    }
                    div { class: "grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-5 gap-4",
                        // Certification 1: ISO 9001:2015
                        div { class: "p-4 rounded-xl bg-slate-900/80 border border-slate-800 flex flex-col items-center text-center group hover:border-sky-500/40 transition-colors",
                            div { class: "w-10 h-10 rounded-lg bg-sky-950/60 border border-sky-500/30 flex items-center justify-center mb-2.5",
                                CarbonShieldQualityIcon { size: 22 }
                            }
                            div { class: "text-xs font-bold text-white", "ISO 9001:2015" }
                            div { class: "text-[10px] text-slate-400 mt-0.5", "Quality Management System" }
                        }
                        // Certification 2: NABH
                        div { class: "p-4 rounded-xl bg-slate-900/80 border border-slate-800 flex flex-col items-center text-center group hover:border-sky-500/40 transition-colors",
                            div { class: "w-10 h-10 rounded-lg bg-emerald-950/60 border border-emerald-500/30 flex items-center justify-center mb-2.5",
                                CarbonShieldCheckIcon { size: 22, color: "#10b981" }
                            }
                            div { class: "text-xs font-bold text-white", "NABH Norms" }
                            div { class: "text-[10px] text-slate-400 mt-0.5", "Hospital Patient Safety" }
                        }
                        // Certification 3: Clinical Establishment
                        div { class: "p-4 rounded-xl bg-slate-900/80 border border-slate-800 flex flex-col items-center text-center group hover:border-sky-500/40 transition-colors",
                            div { class: "w-10 h-10 rounded-lg bg-blue-950/60 border border-blue-500/30 flex items-center justify-center mb-2.5",
                                HospitalIcon { size: 20, color: "#38bdf8" }
                            }
                            div { class: "text-xs font-bold text-white", "Govt. of Odisha" }
                            div { class: "text-[10px] text-slate-400 mt-0.5", "Regd. Clinical Establishment" }
                        }
                        // Certification 4: Infection Control / BMW
                        div { class: "p-4 rounded-xl bg-slate-900/80 border border-slate-800 flex flex-col items-center text-center group hover:border-sky-500/40 transition-colors",
                            div { class: "w-10 h-10 rounded-lg bg-teal-950/60 border border-teal-500/30 flex items-center justify-center mb-2.5",
                                CarbonActivityPulseIcon { size: 20, color: "#2dd4bf" }
                            }
                            div { class: "text-xs font-bold text-white", "OSPCB Bio-Medical" }
                            div { class: "text-[10px] text-slate-400 mt-0.5", "Infection Control Certified" }
                        }
                        // Certification 5: Life Safety & Fire
                        div { class: "p-4 rounded-xl bg-slate-900/80 border border-slate-800 flex flex-col items-center text-center group hover:border-sky-500/40 transition-colors col-span-2 sm:col-span-1",
                            div { class: "w-10 h-10 rounded-lg bg-amber-950/60 border border-amber-500/30 flex items-center justify-center mb-2.5",
                                CheckCircleIcon { size: 20, color: "#f59e0b" }
                            }
                            div { class: "text-xs font-bold text-white", "Fire Safety Certified" }
                            div { class: "text-[10px] text-slate-400 mt-0.5", "Odisha Fire & Life Safety" }
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
