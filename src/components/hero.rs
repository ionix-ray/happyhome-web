//! Hero section with dual-brand dynamic copy, stats pills, and hospital visuals.

use crate::components::icons::{
    ArrowRightIcon, CarbonActivityPulseIcon, CarbonChartTrendIcon, CarbonCognitiveIcon,
    CarbonEmergencySirenIcon, CarbonHeartHealthIcon, CarbonHospitalBedIcon, CarbonPartnershipIcon,
    CarbonShieldQualityIcon, CarbonSmilesHeartIcon, StethoscopeIcon, WhatsAppIcon,
};
use crate::domain::config::SiteConfig;
use crate::state::{BrandMode, BrandModeSignal};
use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    let config = use_context::<SiteConfig>();
    let brand_signal = use_context::<BrandModeSignal>();
    let brand = *brand_signal.0.read();

    let (eyebrow, title_main, title_highlight, lead, chips) = match brand {
        BrandMode::Hospital => (
            "Advanced Multispecialty Hospital & Critical Care",
            "Your health, guided by",
            "clinical experts.",
            "Modern multispecialty hospital with 30 beds, advanced critical care ICU, digital diagnostics, and 24/7 emergency response in Bhubaneswar.",
            &["24×7 Emergency", "Critical Neuro ICU", "Digital Diagnostics", "Trauma Stabilization", "NABH Quality Norms"],
        ),
        BrandMode::Rehab => (
            "Exclusive Inpatient Rehabilitation Center",
            "Helping You To Bring",
            "Back Your Smile.",
            "Dedicated 30-bed neuro and spine recovery center in Bhubaneswar. Transforming lives through specialized stroke, TBI, tracheostomy, and bedsore inpatient management.",
            &["Stroke Recovery", "Spine Injury Rehab", "Tracheostomy Weaning", "Advanced Bedsore Care", "1:1 Daily Physio"],
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

    let (hero_img, hero_img_alt, hero_card_title, hero_card_sub, badge_doc_name, badge_doc_role, badge_doc_img, badge_stat_num, badge_stat_lbl, is_hospital) = match brand {
        BrandMode::Hospital => (
            "/assets/images/hero_hospital_ambulance.jpg",
            "Happy Home Hospital Bhubaneswar — 24/7 Emergency Ambulance & Critical Care Entrance",
            "Happy Home Multispecialty Hospital & Emergency",
            "Plot no.223, K-1 Kalinga Nagar, Bhubaneswar · 24/7 Trauma & Ambulance",
            "Dr. Sambit Dash",
            "Chief of Critical Care & ICU",
            "/assets/images/dr_sambit_dash.jpg",
            "24/7 Emergency",
            "Cardiac Trauma Ambulance on Standby",
            true,
        ),
        BrandMode::Rehab => (
            "/assets/images/hero_banner.jpg",
            "Happy Home Inpatient Neuro-Rehabilitation Center Bhubaneswar",
            "Happy Home Inpatient Neuro & Spine Rehab",
            "Plot no.223, K-1 Kalinga Nagar, Bhubaneswar · Dedicated 30-Bed Recovery Facility",
            "Dr. Vijay Kumar Sahu",
            "Founder & Managing Director",
            "/assets/images/dr_vijay_sahu.jpg",
            "30 Inpatient Beds",
            "1:1 Daily Physio & Neuro Rehab",
            false,
        ),
    };

    rsx! {
        section {
            id: "home",
            class: "hero-wrap pt-12 sm:pt-16 pb-16 overflow-hidden relative",
            role: "banner",
            "aria-labelledby": "hero-heading",

            div { class: "container-x relative z-10",
                div { class: "grid grid-cols-1 lg:grid-cols-12 gap-12 items-center",

                    // ── Left Column: Copy & CTAs ──────────────────────────
                    div { class: "lg:col-span-7 flex flex-col items-start peak-in-up",

                        // Eyebrow badge with Carbon ECG pulse
                        div { class: "inline-flex items-center gap-2.5 hero-badge px-4 py-2 rounded-full text-xs font-bold mb-6 shadow-sm border border-sky-400/20 peak-in-down delay-75",
                            CarbonActivityPulseIcon { size: 18, color: "var(--c-brand)" }
                            span { class: "text-slate-800 dark:text-slate-200", "{eyebrow}" }
                        }

                        // Headline
                        h1 { id: "hero-heading", class: "text-4xl sm:text-5xl lg:text-6xl font-normal leading-[1.1] mb-6 text-slate-900 dark:text-white peak-in-up delay-100",
                            "{title_main} "
                            span { class: "grad-text font-normal", "{title_highlight}" }
                        }

                        // Subtitle
                        p { class: "text-base sm:text-lg text-slate-700 dark:text-slate-200 leading-relaxed mb-8 max-w-2xl font-medium peak-in-up delay-150",
                            "{lead}"
                        }

                        // Action buttons
                        div { class: "flex flex-wrap items-center gap-4 mb-8 w-full sm:w-auto peak-in-up delay-200",
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

                        // Quick feature chips
                        div { class: "flex flex-wrap gap-2 mb-10 peak-in-up delay-250",
                            for chip in chips.iter() {
                                span {
                                    key: "{chip}",
                                    class: "provide-chip px-3.5 py-1.5 rounded-full text-xs font-semibold flex items-center gap-1.5 transition-transform duration-300 hover:scale-105",
                                    span { class: "w-1.5 h-1.5 rounded-full bg-sky-500 animate-pulse" }
                                    "{chip}"
                                }
                            }
                        }

                        // Stats strip with staggered delay & dynamic rolling counters
                        div { class: "grid grid-cols-2 sm:grid-cols-4 gap-3 w-full max-w-xl peak-in-up delay-300",
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

                    // ── Right Column: Hospital & Founder Visual ───────────
                    div { class: "lg:col-span-5 relative mt-6 lg:mt-0 reveal-init reveal-stagger-2",

                        // Main facility image card with organic healing breath aura
                        div { class: "relative rounded-[2rem] overflow-hidden border-4 border-white dark:border-slate-800 shadow-2xl aspect-[4/5] bg-cover bg-center group",
                            img {
                                src: "{hero_img}",
                                alt: "{hero_img_alt}",
                                class: "w-full h-full object-cover group-hover:scale-105 transition-transform duration-700",
                            }

                            // Gradient overlay with hospital title
                            div { class: "absolute bottom-0 inset-x-0 p-6 bg-gradient-to-t from-slate-950/90 via-slate-950/40 to-transparent text-white",
                                div { class: "text-base font-bold", "{hero_card_title}" }
                                div { class: "text-xs text-white/90 mt-1", "{hero_card_sub}" }
                            }
                        }

                        // Floating Badge Top Right (Doctor preview)
                        div { class: "absolute -top-6 -right-2 sm:-right-6 bg-white dark:bg-slate-800 rounded-2xl shadow-xl border border-slate-100 dark:border-slate-700 p-3.5 flex items-center gap-3 backdrop-blur-md",
                            img {
                                src: "{badge_doc_img}",
                                alt: "{badge_doc_name}",
                                class: "w-12 h-12 rounded-xl object-cover object-top border-2 border-sky-500",
                            }
                            div {
                                div { class: "font-extrabold text-xs text-slate-900 dark:text-white", "{badge_doc_name}" }
                                div { class: "text-[10px] font-bold text-sky-600 dark:text-sky-400", "{badge_doc_role}" }
                            }
                        }

                        // Floating Badge Bottom Left (Trust badge)
                        div { class: "absolute -bottom-6 -left-2 sm:-left-6 bg-white dark:bg-slate-800 rounded-2xl shadow-xl border border-slate-100 dark:border-slate-700 p-4 flex items-center gap-3",
                            div { class: "icon-ring !w-11 !h-11 shrink-0 bg-rose-50 dark:bg-rose-950/50",
                                if is_hospital {
                                    CarbonEmergencySirenIcon { size: 22 }
                                } else {
                                    CarbonHospitalBedIcon { size: 22 }
                                }
                            }
                            div {
                                div { class: "font-extrabold text-sm text-slate-900 dark:text-white", "{badge_stat_num}" }
                                div { class: "text-[11px] font-semibold text-slate-600 dark:text-slate-300", "{badge_stat_lbl}" }
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
