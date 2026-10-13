//! Specialities section — original image-card design with a local toggle.
//!
//! Hospital (6 specialties) and Rehabilitation (9 programs) render as
//! `cure-card-split` image cards from the SAME SiteConfig, so the section is
//! independent of the global brand mode. One LOCAL toggle (never global
//! state) picks which wing's grid is on display at every breakpoint — click
//! "Medical" to see the specialities, "Rehabilitation" for the programs.
//! Both grids stay mounted with `hidden` on the inactive one, so the reveal
//! observer attaches once at load; IntersectionObserver refires on the
//! display change and the freshly shown cards rise in.
//!
//! The old Departments section duplicated these offerings almost 1:1 — its
//! facility stats and feature bullets now live on the matching service card
//! (FACILITY map below). Nothing from departments.rs is lost.
//!
//! Card icons come from `CarbonServiceIcon` — multi-colour IBM Carbon glyphs
//! keyed by service slug (one distinct icon per card within a wing).

use crate::components::common::SectionHeader;
use crate::components::icons::{ArrowRightIcon, CarbonServiceIcon, HospitalIcon, StethoscopeIcon};
use crate::domain::config::SiteConfig;
use crate::domain::types::Service;
use crate::state::{BrandMode, BrandModeSignal};
use dioxus::prelude::*;

/// Departments content folded into its matching service card:
/// (slug, facility stat, facility features).
const FACILITY: &[(&str, &str, &[&str])] = &[
    (
        "post-neuro-surgery-rehabilitation",
        "30 Dedicated Beds",
        &[
            "Barrier-free motorized beds",
            "Multi-disciplinary team rounds",
            "Daily customized physio & occupational therapy",
        ],
    ),
    (
        "critical-care-hdu",
        "Continuous Monitoring",
        &[
            "1:1 critical nursing support",
            "Resident anesthesiologist coverage",
            "Step-down tracheostomy weaning",
        ],
    ),
    (
        "advanced-physiotherapy",
        "Fully Outfitted Gym",
        &[
            "Robotic balance training",
            "Sensory reintegration therapy",
            "Fine-motor occupational therapy",
        ],
    ),
    (
        "advanced-bedsore-wound-care",
        "Specialized Mattresses",
        &[
            "Stage 1 to 4 pressure ulcer reversal",
            "Sterile wound dressing protocols",
            "Nutritional healing optimization",
        ],
    ),
    (
        "tracheostomy-respiratory-support-care",
        "Piped Oxygen & Suction",
        &[
            "Piped medical gas at every bed",
            "Chest physiotherapy & nebulization",
            "Specialized speech & swallowing retraining",
        ],
    ),
    (
        "oncology-palliative",
        "Dignified Comfort",
        &[
            "Specialized pain management",
            "Family counseling & attendant comfort",
            "Dignified, peaceful environment",
        ],
    ),
];

fn facility(slug: &str) -> Option<(&'static str, &'static [&'static str])> {
    FACILITY
        .iter()
        .find(|(s, _, _)| *s == slug)
        .map(|(_, stat, feats)| (*stat, *feats))
}

/// Contextual, distinct image for each specialty card.
/// Every single one of the 15 specialties maps to its own unique 16:9 image
/// without any repetition.
fn service_card_image(slug: &str) -> &'static str {
    match slug {
        // Hospital wing (6 specialties)
        "emergency-trauma" => "/assets/images/services/service_emergency_trauma.jpg",
        "critical-care-hdu" => "/assets/images/services/service_critical_care_hdu.jpg",
        "multispecialty-opd" => "/assets/images/services/service_multispecialty_opd.jpg",
        "diagnostic-laboratory" => "/assets/images/services/service_diagnostic_laboratory.jpg",
        "advanced-physiotherapy" => "/assets/images/services/service_advanced_physiotherapy.jpg",
        "oncology-palliative" => "/assets/images/services/service_oncology_palliative.jpg",
        // Rehab wing (9 programs)
        "post-neuro-surgery-rehabilitation" => "/assets/images/services/service_neuro_rehab.jpg",
        "spinal-cord-injury-rehabilitation" => "/assets/images/services/service_spinal_cord_rehab.jpg",
        "traumatic-brain-injury-tbi-care" => "/assets/images/services/service_tbi_care.jpg",
        "advanced-bedsore-wound-care" => "/assets/images/services/service_bedsore_wound_care.jpg",
        "tracheostomy-respiratory-support-care" => "/assets/images/services/service_tracheostomy_respiratory.jpg",
        "cancer-palliative-critical-care-management" => "/assets/images/services/service_cancer_palliative.jpg",
        "parkinsons-disease-dementia-rehabilitation" => "/assets/images/services/service_parkinsons_dementia.jpg",
        "stroke-recovery-paralysis-management" => "/assets/images/services/service_stroke_recovery.jpg",
        "post-general-abdominal-orthopedic-ctvs-surgery-rehabilitation" => {
            "/assets/images/services/service_orthopedic_ctvs.jpg"
        }
        _ => "/assets/images/card_doctor.jpg",
    }
}

/// One speciality card: image half with Carbon icon overlay, text half with
/// the folded-in facility details and the Consult CTA. Pure markup — the
/// caller owns the grid and the reveal-stagger delay.
fn service_card(service: &Service, idx: usize) -> Element {
    let img_src = service_card_image(&service.slug);
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
    let fac = facility(&service.slug);

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
                    loading: "eager",
                    decoding: "async",
                }
                div { class: "cure-card-img-overlay",
                    div { class: "flex items-center justify-between w-full",
                        div { class: "icon-badge {badge_cls} !w-10 !h-10 !rounded-xl text-sm shadow-md",
                            CarbonServiceIcon { slug: service.slug.clone(), size: 20 }
                        }
                        span { class: "text-[10px] font-extrabold uppercase tracking-widest px-2.5 py-1 rounded-full bg-slate-950/75 text-white backdrop-blur-md border border-white/20",
                            "Specialist Care"
                        }
                    }
                }
            }

            // Lower Half: Text, folded facility details, CTA
            div { class: "cure-card-body",
                div {
                    h3 { class: "text-lg font-bold mb-2 text-slate-900 dark:text-white group-hover:text-sky-600 dark:group-hover:text-sky-400 transition-colors",
                        "{service.name}"
                    }
                    p { class: "text-xs sm:text-sm leading-relaxed mb-4 font-medium text-slate-600 dark:text-slate-300",
                        "{service.description}"
                    }
                    if let Some((stat, feats)) = fac {
                        span { class: "inline-flex items-center gap-1.5 text-[10px] font-extrabold uppercase tracking-wider px-2 py-0.5 rounded bg-sky-50 text-sky-700 border border-sky-200 dark:bg-sky-950/60 dark:text-sky-300 dark:border-sky-900 mb-2",
                            span { class: "w-1.5 h-1.5 rounded-full bg-sky-500 shrink-0" }
                            "{stat}"
                        }
                        details { class: "facility-more",
                            summary { "Facility details" }
                            ul { class: "space-y-1",
                                for feat in feats.iter() {
                                    li { class: "flex items-start gap-2 text-[11px] text-slate-600 dark:text-slate-300 font-medium",
                                        span { class: "w-1 h-1 rounded-full bg-sky-500 shrink-0 mt-1.5" }
                                        "{feat}"
                                    }
                                }
                            }
                        }
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

/// One wing's card grid. `visible` toggles the grid classes — NEVER a
/// `reveal-init` class on this element (Dioxus would rewrite it on toggle
/// and wipe the observer's `.fx-in` from its children).
fn service_grid(services: &[Service], visible: bool) -> Element {
    let grid_cls = if visible {
        "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8"
    } else {
        "hidden"
    };
    rsx! {
        div { class: "{grid_cls}",
            for (idx, service) in services.iter().enumerate() {
                { service_card(service, idx) }
            }
        }
    }
}

#[component]
pub fn Services() -> Element {
    let config = use_context::<SiteConfig>();
    let global_brand = use_context::<BrandModeSignal>();
    // Local wing toggle — starts on the visitor's current brand mode, never
    // writes back to the global state (the nav pill keeps its own job).
    let initial_brand = *global_brand.0.read();
    let mut wing = use_signal(move || initial_brand);

    let active = *wing.read();
    let is_hospital = active == BrandMode::Hospital;
    let is_rehab = active == BrandMode::Rehab;

    rsx! {
        section {
            id: "services",
            class: "py-20",
            style: "background: var(--c-bg-2);",
            "aria-labelledby": "services-heading",

            div { class: "container-x",
                SectionHeader {
                    tag: String::new(),
                    title: "Multi Speciality Hospital &".to_string(),
                    highlight: "Rehabilitation Centre".to_string(),
                    subtitle: "Six medical specialties and nine evidence-based recovery programs — pick a wing below and meet the team behind it, around the clock.".to_string(),
                    center: true,
                }

                // Wing toggle — visible at every breakpoint (local state only)
                div { class: "flex justify-center mb-10",
                    div {
                        class: "inline-flex p-1.5 rounded-2xl bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 shadow-md w-full max-w-md",
                        role: "group",
                        "aria-label": "Choose a speciality wing",

                        button {
                            class: if is_hospital {
                                "flex-1 px-4 py-2.5 rounded-xl bg-gradient-to-r from-blue-700 to-sky-600 text-white font-bold text-xs uppercase tracking-wider shadow-sm transition-all flex items-center justify-center gap-2"
                            } else {
                                "flex-1 px-4 py-2.5 rounded-xl text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white font-bold text-xs uppercase tracking-wider transition-all flex items-center justify-center gap-2"
                            },
                            "aria-pressed": "{is_hospital}",
                            onclick: move |_| wing.set(BrandMode::Hospital),
                            HospitalIcon { size: 16, color: "currentColor" }
                            span { "Medical (6 Specialties)" }
                        }
                        button {
                            class: if is_rehab {
                                "flex-1 px-4 py-2.5 rounded-xl bg-gradient-to-r from-teal-700 to-emerald-600 text-white font-bold text-xs uppercase tracking-wider shadow-sm transition-all flex items-center justify-center gap-2"
                            } else {
                                "flex-1 px-4 py-2.5 rounded-xl text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white font-bold text-xs uppercase tracking-wider transition-all flex items-center justify-center gap-2"
                            },
                            "aria-pressed": "{is_rehab}",
                            onclick: move |_| wing.set(BrandMode::Rehab),
                            StethoscopeIcon { size: 16, color: "currentColor" }
                            span { "Rehabilitation (9 Programs)" }
                        }
                    }
                }

                // Both grids mounted; inactive one is display:none
                { service_grid(&config.hospital.services, is_hospital) }
                { service_grid(&config.rehab.services, is_rehab) }
            }
        }
    }
}
