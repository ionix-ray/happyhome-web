//! Dual-wing showcase — Hospital & Rehabilitation side by side.
//!
//! The site carries two distinct clinical identities under one roof:
//! the multispecialty HOSPITAL wing (acute care, emergency, diagnostics —
//! Coastal Cyan identity) and the inpatient REHABILITATION centre (neuro &
//! spine recovery — Healing Teal identity). Instead of hiding one behind
//! the global brand toggle, this section presents BOTH wings at once as
//! two colour-coded cards joined by a shared ECG pulse line — one promise,
//! two specialised teams. Each card's CTA flips the global brand mode and
//! scrolls to the matching services grid.

use crate::components::common::SectionHeader;
use crate::components::icons::{
    ArrowRightIcon, CarbonShieldCheckIcon, CheckCircleIcon, HospitalIcon, StethoscopeIcon,
};
use crate::domain::config::SiteConfig;
use crate::state::{BrandMode, BrandModeSignal};
use dioxus::prelude::*;

/// One wing card: identity header, signature services, stat, CTA.
#[component]
fn WingCard(
    brand_mode: BrandMode,
    eyebrow: String,
    name: String,
    tagline: String,
    services: Vec<String>,
    stat: String,
    stat_label: String,
    cta: String,
) -> Element {
    let brand_signal = use_context::<BrandModeSignal>();
    let mut brand = brand_signal.0;

    let is_hospital = brand_mode == BrandMode::Hospital;
    let identity = if is_hospital {
        // Coastal Cyan — acute hospital identity
        (
            "from-sky-500 to-blue-600",
            "bg-sky-50 dark:bg-sky-950/40",
            "text-sky-600 dark:text-sky-400",
        )
    } else {
        // Healing Teal — rehab identity
        (
            "from-teal-500 to-emerald-600",
            "bg-teal-50 dark:bg-teal-950/40",
            "text-teal-600 dark:text-teal-400",
        )
    };
    let (grad_cls, icon_bg, text_cls) = identity;
    let check_color = if is_hospital { "#0284c7" } else { "#0d9488" };
    // Full literal class (single interpolation only — dioxus rsx class
    // attributes accept exactly one {var} or none).
    let card_class = if is_hospital {
        "wing-card-hospital group relative rounded-3xl border border-sky-200 dark:border-sky-500/30 bg-white dark:bg-slate-900 p-8 transition-all duration-300 hover:shadow-2xl hover:-translate-y-1.5 hover:border-sky-400 dark:hover:border-sky-400/60"
    } else {
        "wing-card-rehab group relative rounded-3xl border border-teal-200 dark:border-teal-500/30 bg-white dark:bg-slate-900 p-8 transition-all duration-300 hover:shadow-2xl hover:-translate-y-1.5 hover:border-teal-400 dark:hover:border-teal-400/60"
    };

    rsx! {
        div {
            class: "{card_class}",

            // Identity header
            div { class: "flex items-center gap-4 mb-6",
                div { class: "w-14 h-14 rounded-2xl bg-gradient-to-br {grad_cls} flex items-center justify-center shadow-lg shrink-0",
                    if is_hospital {
                        HospitalIcon { size: 26, color: "#fff" }
                    } else {
                        StethoscopeIcon { size: 26, color: "#fff" }
                    }
                }
                div {
                    div { class: "text-[10px] font-bold uppercase tracking-[0.18em] {text_cls}", "{eyebrow}" }
                    h3 { class: "text-xl font-bold text-slate-900 dark:text-white", "{name}" }
                }
            }

            p { class: "text-sm text-slate-600 dark:text-slate-300 leading-relaxed mb-6 font-medium",
                "{tagline}"
            }

            // Signature services
            ul { class: "space-y-3 mb-8",
                for svc in services.iter() {
                    li { class: "flex items-start gap-2.5 text-xs text-slate-600 dark:text-slate-300 font-medium",
                        span { class: "w-5 h-5 rounded-full {icon_bg} flex items-center justify-center shrink-0 mt-0.5",
                            CheckCircleIcon { size: 13, color: check_color }
                        }
                        span { "{svc}" }
                    }
                }
            }

            // Stat + CTA
            div { class: "flex items-center justify-between gap-4 flex-wrap",
                div {
                    div { class: "text-2xl font-bold text-slate-900 dark:text-white", "{stat}" }
                    div { class: "text-[10px] font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400", "{stat_label}" }
                }
                button {
                    class: "inline-flex items-center gap-2 px-5 py-2.5 rounded-xl text-xs font-bold uppercase tracking-wider text-white bg-gradient-to-r {grad_cls} shadow-md hover:shadow-lg transition-all hover:scale-[1.03] active:scale-95",
                    onclick: move |_| {
                        *brand.write() = brand_mode;
                        #[cfg(target_arch = "wasm32")]
                        {
                            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                                if let Some(el) = document.get_element_by_id("services") {
                                    el.scroll_into_view();
                                }
                            }
                        }
                    },
                    "{cta}"
                    ArrowRightIcon { size: 14, color: "#fff" }
                }
            }
        }
    }
}

#[component]
pub fn Wings() -> Element {
    let config = use_context::<SiteConfig>();

    let hospital_services: Vec<String> = config
        .hospital
        .services
        .iter()
        .take(4)
        .map(|s| s.name.clone())
        .collect();
    let rehab_services: Vec<String> = config
        .rehab
        .services
        .iter()
        .take(4)
        .map(|s| s.name.clone())
        .collect();

    rsx! {
        section {
            id: "wings",
            class: "py-20 bg-white dark:bg-[#0a101d]",
            "aria-labelledby": "wings-heading",

            div { class: "container-x",
                SectionHeader {
                    tag: "One Roof, Two Specialised Wings".to_string(),
                    title: "Two Wings,".to_string(),
                    highlight: "One Promise".to_string(),
                    subtitle: "Acute multispecialty hospital care and dedicated inpatient rehabilitation — distinct teams, distinct protocols, one commitment to your recovery.".to_string(),
                    center: true,
                }

                div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 relative",

                    // Shared ECG pulse connector (desktop): the two wings beat as one
                    div { class: "hidden lg:flex absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 z-10 w-16 h-16 rounded-full bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-700 shadow-xl items-center justify-center",
                        CarbonShieldCheckIcon { size: 26, color: "var(--c-brand)" }
                    }

                    WingCard {
                        brand_mode: BrandMode::Hospital,
                        eyebrow: "Acute & Critical Care".to_string(),
                        name: "Happy Home Hospital".to_string(),
                        tagline: "Multispecialty emergency response, critical care units, diagnostics and OPD consultations — stabilise, diagnose, treat.".to_string(),
                        services: hospital_services,
                        stat: "24/7".to_string(),
                        stat_label: "Emergency & ICU".to_string(),
                        cta: "Explore Hospital".to_string(),
                    }

                    WingCard {
                        brand_mode: BrandMode::Rehab,
                        eyebrow: "Inpatient Recovery".to_string(),
                        name: "Happy Home Rehab".to_string(),
                        tagline: "Dedicated 30-bed neuro & spine inpatient rehabilitation — stroke, TBI, tracheostomy and bedsore recovery programs.".to_string(),
                        services: rehab_services,
                        stat: "30 Beds".to_string(),
                        stat_label: "1:1 Daily Physio".to_string(),
                        cta: "Explore Rehab".to_string(),
                    }
                }
            }
        }
    }
}
