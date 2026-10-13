//! Hero section with dual-brand dynamic copy, stats pills, and hospital visuals.

use crate::components::common::{split_words, word_masks, Fx};
use crate::components::icons::{
    ArrowRightIcon, CarbonActivityPulseIcon, CarbonChartTrendIcon, CarbonCognitiveIcon,
    CarbonEmergencySirenIcon, CarbonHospitalBedIcon, CarbonLabFlaskIcon, CarbonLungsIcon,
    CarbonPartnershipIcon, CarbonShieldCheckIcon, CarbonShieldQualityIcon, CarbonSmilesHeartIcon,
    CarbonSpineIcon, CarbonStrokeBrainIcon, CarbonVitalsMonitorIcon, CarbonWoundCareIcon,
    StethoscopeIcon, WhatsAppIcon,
};
use crate::domain::config::SiteConfig;
use crate::state::{BrandMode, BrandModeSignal};
use dioxus::prelude::*;

/// Biomedical double-helix DNA animation component.
/// Rendered using scalable SVG vectors and pure CSS sinusoidal oscillation keyframes.
#[component]
fn GeneHelix() -> Element {
    let rungs: [(f32, &str, &str, &str); 16] = [
        (20.0, "0.00s", "#38bdf8", "#2dd4bf"),
        (55.0, "0.22s", "#38bdf8", "#34d399"),
        (90.0, "0.45s", "#0284c7", "#14b8a6"),
        (125.0, "0.68s", "#0ea5e9", "#10b981"),
        (160.0, "0.90s", "#38bdf8", "#2dd4bf"),
        (195.0, "1.12s", "#0284c7", "#34d399"),
        (230.0, "1.35s", "#0ea5e9", "#14b8a6"),
        (265.0, "1.58s", "#38bdf8", "#10b981"),
        (300.0, "1.80s", "#0284c7", "#2dd4bf"),
        (335.0, "2.02s", "#0ea5e9", "#34d399"),
        (370.0, "2.25s", "#38bdf8", "#14b8a6"),
        (405.0, "2.48s", "#0284c7", "#10b981"),
        (440.0, "2.70s", "#0ea5e9", "#2dd4bf"),
        (475.0, "2.92s", "#38bdf8", "#34d399"),
        (510.0, "3.15s", "#0284c7", "#14b8a6"),
        (545.0, "3.38s", "#0ea5e9", "#10b981"),
    ];

    rsx! {
        div { class: "gene-helix-stream pointer-events-none select-none absolute -top-8 left-1/2 -translate-x-1/2 w-full max-w-5xl h-24 overflow-hidden opacity-30 dark:opacity-20 -z-0",
            "aria-hidden": "true",
            svg {
                view_box: "0 0 570 60",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                class: "w-full h-full",
                preserve_aspect_ratio: "xMidYMid meet",

                for (x, delay, color_a, color_b) in rungs.iter() {
                    g {
                        key: "{x}",
                        line {
                            x1: "{x}",
                            y1: "18",
                            x2: "{x}",
                            y2: "42",
                            stroke: "url(#helixGrad)",
                            stroke_width: "1.2",
                            stroke_dasharray: "2 2",
                            class: "helix-rung",
                            style: "animation-delay: {delay};",
                        }
                        circle {
                            cx: "{x}",
                            cy: "30",
                            r: "3.2",
                            fill: "{color_a}",
                            class: "helix-node-a",
                            style: "animation-delay: {delay};",
                        }
                        circle {
                            cx: "{x}",
                            cy: "30",
                            r: "3.2",
                            fill: "{color_b}",
                            class: "helix-node-b",
                            style: "animation-delay: {delay};",
                        }
                    }
                }
                defs {
                    linearGradient {
                        id: "helixGrad",
                        x1: "0%",
                        y1: "0%",
                        x2: "100%",
                        y2: "100%",
                        stop { offset: "0%", stop_color: "#38bdf8", stop_opacity: "0.8" }
                        stop { offset: "100%", stop_color: "#2dd4bf", stop_opacity: "0.8" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Hero() -> Element {
    let config = use_context::<SiteConfig>();
    let brand_signal = use_context::<BrandModeSignal>();
    let brand = *brand_signal.0.read();

    let (title_main, title_highlight, lead) = match brand {
        BrandMode::Hospital => (
            "Your health, guided by",
            "clinical experts.",
            "Modern multispecialty hospital with 30 beds, advanced critical care ICU, digital diagnostics, and 24/7 emergency response in Bhubaneswar.",
        ),
        BrandMode::Rehab => (
            "Helping You To Bring",
            "Back Your Smile.",
            "Dedicated 30-bed neuro and spine recovery center in Bhubaneswar. Transforming lives through specialized stroke, TBI, tracheostomy, and bedsore inpatient management.",
        ),
    };

    let phone = &config.emergency.phone;
    let whatsapp_url = format!(
        "https://api.whatsapp.com/send?phone={}&text=Hello%20Happy%20Home%20Hospital,%20I%20would%20like%20to%20enquire%20about%20care.",
        phone.trim_start_matches('+').replace([' ', '-'], "")
    );

    let stats: &[(&str, &str, &str, &str)] = match brand {
        BrandMode::Hospital => &[
            ("24/7", "24", "/7", "Emergency ICU"),
            ("30+", "30", "+", "Dedicated Beds"),
            ("98.6%", "98.6", "%", "Patient Recovery"),
            ("2021", "2021", "", "Serving Odisha"),
        ],
        BrandMode::Rehab => &[
            ("30", "30", "", "Specialized Beds"),
            ("1:1", "1", ":1", "Therapist Attention"),
            ("1000+", "1000", "+", "Smiles Restored"),
            ("24/7", "24", "/7", "Medical Supervision"),
        ],
    };

    let (
        hero_img,
        hero_img_alt,
        hero_card_title,
        hero_card_sub,
        is_hospital,
    ) = match brand {
        BrandMode::Hospital => (
            asset!("/assets/images/facility.jpg"),
            "Happy Home Hospital Bhubaneswar — 24/7 Emergency Ambulance & Critical Care Entrance",
            "Happy Home Multispecialty Hospital & Emergency",
            "Plot no.223, K-1 Kalinga Nagar, Bhubaneswar · 24/7 Trauma & Ambulance",
            true,
        ),
        BrandMode::Rehab => (
            asset!("/assets/images/card_rehab.jpg"),
            "Happy Home Inpatient Neuro-Rehabilitation Center Bhubaneswar",
            "Happy Home Inpatient Neuro & Spine Rehab",
            "Plot no.223, K-1 Kalinga Nagar, Bhubaneswar · Dedicated 30-Bed Recovery Facility",
            false,
        ),
    };
    let hero_img_pos = if is_hospital {
        "object-left"
    } else {
        "object-center"
    };

    let title_steps = (split_words(title_main).len() + split_words(title_highlight).len()) as u32;
    let (eyebrow_cls, eyebrow_st) = Fx::Fade.at(0);
    let (lead_cls, lead_st) = Fx::Reveal.at(title_steps + 1);
    let (cta_cls, cta_st) = Fx::Float.at(title_steps + 2);
    let (stat_cls, stat_st) = Fx::Float.at(title_steps + 3);
    let (visual_cls, visual_st) = Fx::Float.at(2);

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        let _brand = *brand_signal.0.read();
        let delay_ms = ((title_steps + 3) * 70) as i32;
        crate::components::common::fx::roll_stat_counters(delay_ms);
    });

    rsx! {
        section {
            id: "home",
            class: "hero-wrap pt-12 sm:pt-16 pb-16 overflow-hidden relative fx-time",
            role: "banner",
            "aria-labelledby": "hero-heading",

            // Ambient Biomedical Gene Helix Animation Ribbon
            GeneHelix {}

            div { class: "container-x relative z-10",
                div { class: "grid grid-cols-1 lg:grid-cols-12 gap-12 items-center",

                    // ── Left Column: Copy, CTAs & Live Stats ───────────
                    div { class: "lg:col-span-7 flex flex-col items-start",

                        // Institutional masthead kicker (authentic clinical registry)
                        div { class: "flex items-center gap-2.5 mb-5 font-mono text-[11px] font-bold tracking-widest text-slate-500 dark:text-slate-400 uppercase select-none {eyebrow_cls}",
                            style: "{eyebrow_st}",
                            CarbonActivityPulseIcon { size: 14, color: "var(--c-brand)" }
                            span { class: "text-sky-700 dark:text-sky-300 font-extrabold tracking-wider", "Bhubaneswar Medical Campus" }
                            span { class: "text-slate-300 dark:text-slate-700", "/" }
                            span { "Govt. Regd. Estd. 2021" }
                        }

                        // Headline — one masked span per word (shared fx cascade)
                        h1 { id: "hero-heading", class: "text-4xl sm:text-5xl lg:text-6xl font-normal leading-[1.1] mb-6 text-slate-900 dark:text-white",
                            { word_masks(title_main, title_highlight) }
                        }

                        // Subtitle — vayudh motion-revealIn hard-edge wipe
                        p { class: "text-base sm:text-lg text-slate-700 dark:text-slate-200 leading-relaxed mb-8 max-w-2xl font-medium {lead_cls}",
                            style: "{lead_st}",
                            "{lead}"
                        }

                        // Action buttons
                        div { class: "flex flex-wrap items-center gap-4 mb-10 w-full sm:w-auto {cta_cls}",
                            style: "{cta_st}",
                            a {
                                class: "btn-solid carbon-btn-hover px-8 py-3.5 rounded-xl text-sm font-bold inline-flex items-center justify-center gap-2 shadow-lg hover:shadow-xl transition-all",
                                href: "#appointment",
                                StethoscopeIcon { size: 18, color: "#fff" }
                                "Book an Appointment"
                                ArrowRightIcon { size: 16, color: "#fff" }
                            }
                            a {
                                class: "btn-outline carbon-btn-hover px-6 py-3.5 rounded-xl text-sm font-semibold inline-flex items-center justify-center gap-2 hover:bg-sky-50 dark:hover:bg-slate-800 transition-all",
                                href: "{whatsapp_url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                WhatsAppIcon { size: 18, color: "var(--c-brand)" }
                                "WhatsApp Consultation"
                            }
                        }

                        // Stats strip with staggered delay & dynamic rolling counters
                        div { class: "grid grid-cols-2 sm:grid-cols-4 gap-3 w-full max-w-xl {stat_cls}",
                            style: "{stat_st}",
                            for (_idx, (fallback, target, suffix, lbl)) in stats.iter().enumerate() {
                                div {
                                    key: "{lbl}",
                                    class: "stat-pill rounded-2xl p-4 text-center transition-all duration-300 hover:-translate-y-1 hover:shadow-md",
                                    div {
                                        class: "num text-2xl sm:text-3xl font-normal dynamic-counter",
                                        "data-target": "{target}",
                                        "data-suffix": "{suffix}",
                                        "{fallback}"
                                    }
                                    div { class: "text-[11px] font-bold uppercase tracking-wider text-slate-600 dark:text-slate-400 mt-1", "{lbl}" }
                                }
                            }
                        }
                    }

                    // ── Right Column: Central Visual surrounded by Floating Capability Consoles ──
                    div { class: "lg:col-span-5 relative mt-6 lg:mt-0 {visual_cls}",
                        style: "{visual_st}",

                        // Ambient glow aura behind visual
                        div { class: "absolute -inset-4 sm:-inset-6 bg-gradient-to-tr from-sky-500/20 via-teal-500/15 to-transparent rounded-[2.5rem] filter blur-2xl pointer-events-none -z-10" }

                        // Central facility image showcase
                        div { class: "relative rounded-[2rem] overflow-hidden border-4 border-white dark:border-slate-800 shadow-2xl aspect-[4/5] bg-cover bg-center group z-10",
                            img {
                                src: "{hero_img}",
                                alt: "{hero_img_alt}",
                                class: "w-full h-full object-cover {hero_img_pos} group-hover:scale-105 transition-transform duration-700",
                                loading: "eager",
                                decoding: "async",
                            }

                            // Gradient overlay with hospital title
                            div { class: "absolute bottom-0 inset-x-0 p-6 bg-gradient-to-t from-slate-950/95 via-slate-950/50 to-transparent text-white z-10",
                                div { class: "text-base font-bold", "{hero_card_title}" }
                                div { class: "text-xs text-white/90 mt-1", "{hero_card_sub}" }
                            }
                        }

                        // ── Desktop Floating Capability Consoles (Surrounding Central Visual) ──
                        // Console 1: Top-Left (24x7 Emergency / Stroke Recovery)
                        div { class: "absolute -top-7 -left-6 xl:-left-12 z-20 hidden lg:flex items-center gap-3 bg-white/95 dark:bg-slate-900/95 backdrop-blur-md px-3.5 py-2.5 rounded-2xl shadow-xl border border-slate-200/90 dark:border-slate-700/80 animate-float-slow hover:scale-105 transition-all duration-300",
                            div { class: "w-9 h-9 rounded-xl bg-rose-50 dark:bg-rose-950/70 border border-rose-200 dark:border-rose-800/60 flex items-center justify-center shrink-0 shadow-inner",
                                if is_hospital {
                                    CarbonEmergencySirenIcon { size: 18 }
                                } else {
                                    CarbonStrokeBrainIcon { size: 18 }
                                }
                            }
                            div { class: "pr-1",
                                div { class: "flex items-center gap-1.5",
                                    span { class: "text-xs font-black text-slate-900 dark:text-white whitespace-nowrap",
                                        if is_hospital { "24×7 Emergency" } else { "Stroke Recovery" }
                                    }
                                    span { class: "inline-block w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse" }
                                }
                                div { class: "text-[10px] font-bold text-slate-500 dark:text-slate-400 whitespace-nowrap",
                                    if is_hospital { "Rapid Triage" } else { "Motor Relearning" }
                                }
                            }
                        }

                        // Console 2: Top-Right (Critical Neuro ICU / Spine Injury Rehab — Replaces Dr. Sambit Dash card)
                        div { class: "absolute -top-9 -right-6 xl:-right-10 z-20 hidden lg:flex items-center gap-3 bg-white/95 dark:bg-slate-900/95 backdrop-blur-md px-3.5 py-2.5 rounded-2xl shadow-xl border border-slate-200/90 dark:border-slate-700/80 animate-float-delayed hover:scale-105 transition-all duration-300",
                            div { class: "w-9 h-9 rounded-xl bg-sky-50 dark:bg-sky-950/70 border border-sky-200 dark:border-sky-800/60 flex items-center justify-center shrink-0 shadow-inner",
                                if is_hospital {
                                    CarbonVitalsMonitorIcon { size: 18 }
                                } else {
                                    CarbonSpineIcon { size: 18 }
                                }
                            }
                            div { class: "pr-1",
                                div { class: "flex items-center gap-1.5",
                                    span { class: "text-xs font-black text-slate-900 dark:text-white whitespace-nowrap",
                                        if is_hospital { "Critical Neuro ICU" } else { "Spine Injury Rehab" }
                                    }
                                    span { class: "inline-block w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse" }
                                }
                                div { class: "text-[10px] font-bold text-slate-500 dark:text-slate-400 whitespace-nowrap",
                                    if is_hospital { "Level-III Telemetry" } else { "Gait & Posture Gym" }
                                }
                            }
                        }

                        // Console 3: Mid-Left (Digital Diagnostics / Tracheostomy Weaning)
                        div { class: "absolute top-1/2 -translate-y-1/2 -left-8 xl:-left-14 z-20 hidden lg:flex items-center gap-3 bg-white/95 dark:bg-slate-900/95 backdrop-blur-md px-3.5 py-2.5 rounded-2xl shadow-xl border border-slate-200/90 dark:border-slate-700/80 animate-float hover:scale-105 transition-all duration-300",
                            div { class: "w-9 h-9 rounded-xl bg-amber-50 dark:bg-amber-950/70 border border-amber-200 dark:border-amber-800/60 flex items-center justify-center shrink-0 shadow-inner",
                                if is_hospital {
                                    CarbonLabFlaskIcon { size: 18 }
                                } else {
                                    CarbonLungsIcon { size: 18 }
                                }
                            }
                            div { class: "pr-1",
                                div { class: "flex items-center gap-1.5",
                                    span { class: "text-xs font-black text-slate-900 dark:text-white whitespace-nowrap",
                                        if is_hospital { "Digital Diagnostics" } else { "Tracheostomy Weaning" }
                                    }
                                    span { class: "inline-block w-1.5 h-1.5 rounded-full bg-amber-500 animate-pulse" }
                                }
                                div { class: "text-[10px] font-bold text-slate-500 dark:text-slate-400 whitespace-nowrap",
                                    if is_hospital { "Automated Pathology" } else { "Pulmonary Protocol" }
                                }
                            }
                        }

                        // Console 4: Bottom-Left (Trauma Stabilization / Advanced Bedsore Care)
                        div { class: "absolute -bottom-8 -left-6 xl:-left-12 z-20 hidden lg:flex items-center gap-3 bg-white/95 dark:bg-slate-900/95 backdrop-blur-md px-3.5 py-2.5 rounded-2xl shadow-xl border border-slate-200/90 dark:border-slate-700/80 animate-float-slow hover:scale-105 transition-all duration-300",
                            div { class: "w-9 h-9 rounded-xl bg-emerald-50 dark:bg-emerald-950/70 border border-emerald-200 dark:border-emerald-800/60 flex items-center justify-center shrink-0 shadow-inner",
                                if is_hospital {
                                    CarbonShieldQualityIcon { size: 18 }
                                } else {
                                    CarbonWoundCareIcon { size: 18 }
                                }
                            }
                            div { class: "pr-1",
                                div { class: "flex items-center gap-1.5",
                                    span { class: "text-xs font-black text-slate-900 dark:text-white whitespace-nowrap",
                                        if is_hospital { "Trauma Stabilization" } else { "Advanced Bedsore Care" }
                                    }
                                    span { class: "inline-block w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse" }
                                }
                                div { class: "text-[10px] font-bold text-slate-500 dark:text-slate-400 whitespace-nowrap",
                                    if is_hospital { "Immediate Care" } else { "Tissue Regeneration" }
                                }
                            }
                        }

                        // Console 5: Bottom-Right (NABH Quality Norms / 1:1 Daily Physio)
                        div { class: "absolute -bottom-8 -right-4 xl:-right-8 z-20 hidden lg:flex items-center gap-3 bg-white/95 dark:bg-slate-900/95 backdrop-blur-md px-3.5 py-2.5 rounded-2xl shadow-xl border border-slate-200/90 dark:border-slate-700/80 animate-float-delayed hover:scale-105 transition-all duration-300",
                            div { class: "w-9 h-9 rounded-xl bg-indigo-50 dark:bg-indigo-950/70 border border-indigo-200 dark:border-indigo-800/60 flex items-center justify-center shrink-0 shadow-inner",
                                if is_hospital {
                                    CarbonShieldCheckIcon { size: 18 }
                                } else {
                                    CarbonPartnershipIcon { size: 18 }
                                }
                            }
                            div { class: "pr-1",
                                div { class: "flex items-center gap-1.5",
                                    span { class: "text-xs font-black text-slate-900 dark:text-white whitespace-nowrap",
                                        if is_hospital { "NABH Quality Norms" } else { "1:1 Daily Physio" }
                                    }
                                    span { class: "inline-block w-1.5 h-1.5 rounded-full bg-indigo-500 animate-pulse" }
                                }
                                div { class: "text-[10px] font-bold text-slate-500 dark:text-slate-400 whitespace-nowrap",
                                    if is_hospital { "Certified Safety" } else { "Dedicated Therapist" }
                                }
                            }
                        }

                        // ── Mobile Capability Grid (Responsive docking deck beneath hero image) ──
                        div { class: "lg:hidden grid grid-cols-2 sm:grid-cols-3 gap-2.5 mt-6 w-full z-10",
                            if is_hospital {
                                div { class: "flex items-center gap-2.5 p-2.5 rounded-xl bg-white/90 dark:bg-slate-800/80 border border-slate-200/80 dark:border-slate-700/70 shadow-xs",
                                    div { class: "w-8 h-8 rounded-lg bg-rose-50 dark:bg-rose-950/60 flex items-center justify-center shrink-0",
                                        CarbonEmergencySirenIcon { size: 16 }
                                    }
                                    div { class: "min-w-0",
                                        div { class: "text-[11px] font-extrabold text-slate-900 dark:text-white truncate", "24×7 Emergency" }
                                        div { class: "text-[10px] font-medium text-slate-500 dark:text-slate-400 truncate", "Rapid Triage" }
                                    }
                                }
                                div { class: "flex items-center gap-2.5 p-2.5 rounded-xl bg-white/90 dark:bg-slate-800/80 border border-slate-200/80 dark:border-slate-700/70 shadow-xs",
                                    div { class: "w-8 h-8 rounded-lg bg-sky-50 dark:bg-sky-950/60 flex items-center justify-center shrink-0",
                                        CarbonVitalsMonitorIcon { size: 16 }
                                    }
                                    div { class: "min-w-0",
                                        div { class: "text-[11px] font-extrabold text-slate-900 dark:text-white truncate", "Critical Neuro ICU" }
                                        div { class: "text-[10px] font-medium text-slate-500 dark:text-slate-400 truncate", "Level-III Telemetry" }
                                    }
                                }
                                div { class: "flex items-center gap-2.5 p-2.5 rounded-xl bg-white/90 dark:bg-slate-800/80 border border-slate-200/80 dark:border-slate-700/70 shadow-xs",
                                    div { class: "w-8 h-8 rounded-lg bg-amber-50 dark:bg-amber-950/60 flex items-center justify-center shrink-0",
                                        CarbonLabFlaskIcon { size: 16 }
                                    }
                                    div { class: "min-w-0",
                                        div { class: "text-[11px] font-extrabold text-slate-900 dark:text-white truncate", "Digital Diagnostics" }
                                        div { class: "text-[10px] font-medium text-slate-500 dark:text-slate-400 truncate", "Automated Pathology" }
                                    }
                                }
                                div { class: "flex items-center gap-2.5 p-2.5 rounded-xl bg-white/90 dark:bg-slate-800/80 border border-slate-200/80 dark:border-slate-700/70 shadow-xs",
                                    div { class: "w-8 h-8 rounded-lg bg-emerald-50 dark:bg-emerald-950/60 flex items-center justify-center shrink-0",
                                        CarbonShieldQualityIcon { size: 16 }
                                    }
                                    div { class: "min-w-0",
                                        div { class: "text-[11px] font-extrabold text-slate-900 dark:text-white truncate", "Trauma Stabilization" }
                                        div { class: "text-[10px] font-medium text-slate-500 dark:text-slate-400 truncate", "Immediate Care" }
                                    }
                                }
                                div { class: "flex items-center gap-2.5 p-2.5 rounded-xl bg-white/90 dark:bg-slate-800/80 border border-slate-200/80 dark:border-slate-700/70 shadow-xs col-span-2 sm:col-span-1",
                                    div { class: "w-8 h-8 rounded-lg bg-indigo-50 dark:bg-indigo-950/60 flex items-center justify-center shrink-0",
                                        CarbonShieldCheckIcon { size: 16 }
                                    }
                                    div { class: "min-w-0",
                                        div { class: "text-[11px] font-extrabold text-slate-900 dark:text-white truncate", "NABH Quality Norms" }
                                        div { class: "text-[10px] font-medium text-slate-500 dark:text-slate-400 truncate", "Certified Safety" }
                                    }
                                }
                            } else {
                                div { class: "flex items-center gap-2.5 p-2.5 rounded-xl bg-white/90 dark:bg-slate-800/80 border border-slate-200/80 dark:border-slate-700/70 shadow-xs",
                                    div { class: "w-8 h-8 rounded-lg bg-purple-50 dark:bg-purple-950/60 flex items-center justify-center shrink-0",
                                        CarbonStrokeBrainIcon { size: 16 }
                                    }
                                    div { class: "min-w-0",
                                        div { class: "text-[11px] font-extrabold text-slate-900 dark:text-white truncate", "Stroke Recovery" }
                                        div { class: "text-[10px] font-medium text-slate-500 dark:text-slate-400 truncate", "Motor Relearning" }
                                    }
                                }
                                div { class: "flex items-center gap-2.5 p-2.5 rounded-xl bg-white/90 dark:bg-slate-800/80 border border-slate-200/80 dark:border-slate-700/70 shadow-xs",
                                    div { class: "w-8 h-8 rounded-lg bg-teal-50 dark:bg-teal-950/60 flex items-center justify-center shrink-0",
                                        CarbonSpineIcon { size: 16 }
                                    }
                                    div { class: "min-w-0",
                                        div { class: "text-[11px] font-extrabold text-slate-900 dark:text-white truncate", "Spine Injury Rehab" }
                                        div { class: "text-[10px] font-medium text-slate-500 dark:text-slate-400 truncate", "Gait & Posture Gym" }
                                    }
                                }
                                div { class: "flex items-center gap-2.5 p-2.5 rounded-xl bg-white/90 dark:bg-slate-800/80 border border-slate-200/80 dark:border-slate-700/70 shadow-xs",
                                    div { class: "w-8 h-8 rounded-lg bg-cyan-50 dark:bg-cyan-950/60 flex items-center justify-center shrink-0",
                                        CarbonLungsIcon { size: 16 }
                                    }
                                    div { class: "min-w-0",
                                        div { class: "text-[11px] font-extrabold text-slate-900 dark:text-white truncate", "Tracheostomy Weaning" }
                                        div { class: "text-[10px] font-medium text-slate-500 dark:text-slate-400 truncate", "Pulmonary Protocol" }
                                    }
                                }
                                div { class: "flex items-center gap-2.5 p-2.5 rounded-xl bg-white/90 dark:bg-slate-800/80 border border-slate-200/80 dark:border-slate-700/70 shadow-xs",
                                    div { class: "w-8 h-8 rounded-lg bg-rose-50 dark:bg-rose-950/60 flex items-center justify-center shrink-0",
                                        CarbonWoundCareIcon { size: 16 }
                                    }
                                    div { class: "min-w-0",
                                        div { class: "text-[11px] font-extrabold text-slate-900 dark:text-white truncate", "Advanced Bedsore Care" }
                                        div { class: "text-[10px] font-medium text-slate-500 dark:text-slate-400 truncate", "Tissue Regeneration" }
                                    }
                                }
                                div { class: "flex items-center gap-2.5 p-2.5 rounded-xl bg-white/90 dark:bg-slate-800/80 border border-slate-200/80 dark:border-slate-700/70 shadow-xs col-span-2 sm:col-span-1",
                                    div { class: "w-8 h-8 rounded-lg bg-emerald-50 dark:bg-emerald-950/60 flex items-center justify-center shrink-0",
                                        CarbonPartnershipIcon { size: 16 }
                                    }
                                    div { class: "min-w-0",
                                        div { class: "text-[11px] font-extrabold text-slate-900 dark:text-white truncate", "1:1 Daily Physio" }
                                        div { class: "text-[10px] font-medium text-slate-500 dark:text-slate-400 truncate", "Dedicated Therapist" }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Running Facts & Stats Marquee Ticker (Multi-colour IBM Carbon icons, strictly NO emojis)
            div { class: "mt-12 facts-ticker-wrap select-none",
                div { class: "facts-ticker-content",
                    span { class: "facts-item inline-flex items-center gap-2 text-sky-700 dark:text-sky-300",
                        CarbonEmergencySirenIcon { size: 16 }
                        span { "24/7 Emergency & Neuro ICU" }
                    }
                    span { class: "text-slate-400 dark:text-slate-600", "·" }
                    span { class: "facts-item inline-flex items-center gap-2 text-teal-700 dark:text-teal-300",
                        CarbonHospitalBedIcon { size: 16 }
                        span { "30 Dedicated Inpatient Beds" }
                    }
                    span { class: "text-slate-400 dark:text-slate-600", "·" }
                    span { class: "facts-item inline-flex items-center gap-2 text-indigo-700 dark:text-indigo-300",
                        CarbonChartTrendIcon { size: 16 }
                        span { "98.6% Patient Recovery Rate" }
                    }
                    span { class: "text-slate-400 dark:text-slate-600", "·" }
                    span { class: "facts-item inline-flex items-center gap-2 text-amber-700 dark:text-amber-300",
                        CarbonShieldQualityIcon { size: 16 }
                        span { "NABH Quality Norms" }
                    }
                    span { class: "text-slate-400 dark:text-slate-600", "·" }
                    span { class: "facts-item inline-flex items-center gap-2 text-rose-700 dark:text-rose-300",
                        CarbonCognitiveIcon { size: 16 }
                        span { "Specialized Neuro & Spine Rehab" }
                    }
                    span { class: "text-slate-400 dark:text-slate-600", "·" }
                    span { class: "facts-item inline-flex items-center gap-2 text-teal-700 dark:text-teal-300",
                        CarbonPartnershipIcon { size: 16 }
                        span { "1:1 Daily Physiotherapy" }
                    }
                    span { class: "text-slate-400 dark:text-slate-600", "·" }
                    span { class: "facts-item inline-flex items-center gap-2 text-sky-700 dark:text-sky-300",
                        CarbonSmilesHeartIcon { size: 16 }
                        span { "1,000+ Smiles Restored Since 2021" }
                    }
                    span { class: "text-slate-400 dark:text-slate-600", "·" }

                    // Duplicate items for seamless continuous ticker loop
                    span { class: "facts-item inline-flex items-center gap-2 text-sky-700 dark:text-sky-300",
                        CarbonEmergencySirenIcon { size: 16 }
                        span { "24/7 Emergency & Neuro ICU" }
                    }
                    span { class: "text-slate-400 dark:text-slate-600", "·" }
                    span { class: "facts-item inline-flex items-center gap-2 text-teal-700 dark:text-teal-300",
                        CarbonHospitalBedIcon { size: 16 }
                        span { "30 Dedicated Inpatient Beds" }
                    }
                    span { class: "text-slate-400 dark:text-slate-600", "·" }
                    span { class: "facts-item inline-flex items-center gap-2 text-indigo-700 dark:text-indigo-300",
                        CarbonChartTrendIcon { size: 16 }
                        span { "98.6% Patient Recovery Rate" }
                    }
                    span { class: "text-slate-400 dark:text-slate-600", "·" }
                    span { class: "facts-item inline-flex items-center gap-2 text-amber-700 dark:text-amber-300",
                        CarbonShieldQualityIcon { size: 16 }
                        span { "NABH Quality Norms" }
                    }
                    span { class: "text-slate-400 dark:text-slate-600", "·" }
                    span { class: "facts-item inline-flex items-center gap-2 text-rose-700 dark:text-rose-300",
                        CarbonCognitiveIcon { size: 16 }
                        span { "Specialized Neuro & Spine Rehab" }
                    }
                    span { class: "text-slate-400 dark:text-slate-600", "·" }
                    span { class: "facts-item inline-flex items-center gap-2 text-teal-700 dark:text-teal-300",
                        CarbonPartnershipIcon { size: 16 }
                        span { "1:1 Daily Physiotherapy" }
                    }
                    span { class: "text-slate-400 dark:text-slate-600", "·" }
                    span { class: "facts-item inline-flex items-center gap-2 text-sky-700 dark:text-sky-300",
                        CarbonSmilesHeartIcon { size: 16 }
                        span { "1,000+ Smiles Restored Since 2021" }
                    }
                    span { class: "text-slate-400 dark:text-slate-600", "·" }
                }
            }
        }
    }
}
