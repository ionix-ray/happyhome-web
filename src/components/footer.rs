//! Comprehensive Footer — theme-aware (light/dark), clinical accreditation
//! stamps, quick links, and 24/7 emergency contact.
//!
//! Design notes:
//! • The footer follows the active theme: light mode renders on a pristine
//!   ice-white surface with slate text; dark mode on deep midnight navy.
//! • Accreditations are official SEAL STAMPS (double-ring circular badges
//!   with a slight rubber-stamp rotation) — not a boxed section. Each stamp
//!   carries its own regulatory colour and straightens on hover.

use crate::components::icons::{MailIcon, MapPinIcon, PhoneIcon};
use crate::domain::config::SiteConfig;
use crate::state::ThemeSignal;
use dioxus::prelude::*;

/// One certification stamp: authentic badge seal look with official regulatory emblem.
struct CertStamp {
    name: &'static str,
    sub: &'static str,
    color: &'static str,
    image_src: Asset,
}

#[component]
pub fn Footer() -> Element {
    let config = use_context::<SiteConfig>();
    let theme_signal = use_context::<ThemeSignal>();
    let is_dark = *theme_signal.0.read() == crate::state::Theme::Dark;

    // Light mode: the authentic full-colour logo. Dark mode: the luminous
    // dark-mode variant. One <img>, theme-swapped src — no broken images.
    let logo_src = if is_dark {
        asset!("/assets/images/Logo-happy-home-darkmode.png")
    } else {
        asset!("/assets/images/happy-home-logo-nav.png")
    };

    let stamps = [
        CertStamp {
            name: "ISO 9001:2015",
            sub: "Quality Management",
            color: "#0284c7",
            image_src: asset!("/assets/images/stamps/stamp_iso_9001.png"),
        },
        CertStamp {
            name: "NABH Norms",
            sub: "Patient Safety",
            color: "#059669",
            image_src: asset!("/assets/images/stamps/stamp_nabh.png"),
        },
        CertStamp {
            name: "Govt. of Odisha",
            sub: "Regd. Establishment",
            color: "#2563eb",
            image_src: asset!("/assets/images/stamps/stamp_odisha_govt.png"),
        },
        CertStamp {
            name: "OSPCB Bio-Medical",
            sub: "Infection Control",
            color: "#0d9488",
            image_src: asset!("/assets/images/stamps/stamp_ospcb.png"),
        },
        CertStamp {
            name: "Fire Safety",
            sub: "Odisha Life Safety",
            color: "#d97706",
            image_src: asset!("/assets/images/stamps/stamp_fire_safety.png"),
        },
    ];

    rsx! {
        footer { class: "bg-slate-50 text-slate-600 pt-16 pb-10 border-t border-slate-200 dark:bg-slate-950 dark:text-slate-300 dark:border-slate-800 transition-colors duration-300",
            div { class: "container-x",
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-10 mb-12",

                    // Col 1: Brand & Mission
                    div {
                        a {
                            href: "#home",
                            class: "mb-5 inline-block transition-opacity hover:opacity-90",
                            img {
                                src: logo_src,
                                alt: "Happy Home Hospital & Rehab",
                                class: "h-14 w-auto object-contain",
                            }
                        }
                        p { class: "text-xs text-slate-500 dark:text-slate-400 leading-relaxed mb-6 font-medium",
                            "{config.footer.about_text}"
                        }
                        div { class: "text-xs text-slate-500 dark:text-slate-400 font-semibold",
                            "Established 2021 · Regd. Healthcare Facility"
                        }
                    }

                    // Col 2: Quick Links
                    div {
                        h4 { class: "text-sm font-bold text-slate-900 dark:text-white uppercase tracking-wider mb-4", "Quick Links" }
                        ul { class: "space-y-2.5 text-xs",
                            for link in config.footer.quick_links.iter() {
                                li {
                                    a {
                                        href: "{link.url}",
                                        class: "text-slate-500 dark:text-slate-400 hover:text-sky-600 dark:hover:text-sky-400 transition-colors font-medium",
                                        "{link.label}"
                                    }
                                }
                            }
                        }
                    }

                    // Col 3: Core Specialties
                    div {
                        h4 { class: "text-sm font-bold text-slate-900 dark:text-white uppercase tracking-wider mb-4", "Specialized Care" }
                        ul { class: "space-y-2.5 text-xs text-slate-500 dark:text-slate-400 font-medium",
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
                        h4 { class: "text-sm font-bold text-slate-900 dark:text-white uppercase tracking-wider mb-4", "Emergency & Admission" }
                        div { class: "space-y-3 text-xs text-slate-500 dark:text-slate-400",
                            div { class: "flex items-start gap-2.5",
                                MapPinIcon { size: 16, color: "var(--c-brand)" }
                                span { "{config.contact.address}" }
                            }
                            div { class: "flex items-center gap-2.5",
                                PhoneIcon { size: 16, color: "var(--c-brand)" }
                                a { href: "tel:{config.emergency.phone}", class: "text-slate-900 dark:text-white font-bold hover:text-sky-600 dark:hover:text-sky-400 transition-colors", "{config.emergency.phone}" }
                            }
                            div { class: "flex items-center gap-2.5",
                                MailIcon { size: 16, color: "var(--c-brand)" }
                                span { "{config.contact.email}" }
                            }
                        }
                        div { class: "mt-6 p-3 rounded-xl bg-slate-100 dark:bg-slate-900 border border-slate-200 dark:border-slate-800 text-[11px] text-sky-600 dark:text-sky-400 font-semibold",
                            "Emergency Trauma Admissions Open 24/7"
                        }
                    }
                }

                // ── Certification Stamps — official seal badges, not a section ──
                // Compact strip of double-ring rubber-stamp seals. Each stamp
                // carries its own regulatory colour and straightens on hover.
                div { class: "py-8 mb-8 border-y border-slate-200 dark:border-slate-800/80",
                    div { class: "text-center mb-6",
                        span { class: "text-[11px] font-bold uppercase tracking-[0.25em] text-sky-600 dark:text-sky-400 inline-flex items-center gap-2.5",
                            span { class: "w-6 h-px bg-sky-600/30 dark:bg-sky-400/30" }
                            "Accreditations & Regulatory Certifications"
                            span { class: "w-6 h-px bg-sky-600/30 dark:bg-sky-400/30" }
                        }
                    }
                    div { class: "cert-stamps",
                        for cert in stamps.iter() {
                            div {
                                key: "{cert.name}",
                                class: "cert-stamp group",
                                style: "--stamp-color: {cert.color};",
                                div { class: "cert-stamp-seal",
                                    img {
                                        src: "{cert.image_src}",
                                        alt: "{cert.name} - {cert.sub}",
                                        class: "w-13 h-13 max-w-[50px] max-h-[50px] object-contain select-none transition-transform duration-300 group-hover:scale-110",
                                        loading: "eager",
                                        decoding: "async",
                                        width: "50",
                                        height: "50",
                                    }
                                }
                                div { class: "cert-stamp-title", "{cert.name}" }
                                div { class: "cert-stamp-sub", "{cert.sub}" }
                            }
                        }
                    }
                }

                // Bottom bar
                div { class: "pt-8 border-t border-slate-200 dark:border-slate-800/80 flex flex-col sm:flex-row items-center justify-between text-xs text-slate-400 dark:text-slate-500 gap-4",
                    p { "© 2026 Happy Home Hospital And Rehab. All rights reserved." }
                    div { class: "flex flex-wrap gap-6",
                        for link in config.footer.legal_links.iter() {
                            a {
                                href: "{link.url}",
                                class: "hover:text-slate-600 dark:hover:text-slate-300 transition-colors",
                                "{link.label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
