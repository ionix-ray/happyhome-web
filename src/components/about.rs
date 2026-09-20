//! About section detailing hospital history, founder credentials, and clinical mission.

use crate::components::common::SectionHeader;
use crate::components::icons::{CheckCircleIcon, StethoscopeIcon};
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    let founder_img = "assets/images/dr_vijay_sahu.jpg";
    let founder_name = "Dr. Vijay Kumar Sahu";
    let founder_title = "Founder & Managing Director";
    let founder_deg = "MPT (Neuro Rehabilitation), Specialized in Neuro & Spine Care";
    let founder_bio = "With specialized training in neurological rehabilitation and acute critical step-down recovery, Dr. Sahu established Happy Home Hospital & Rehab to bridge the vital gap between post-surgical discharge and functional home living in Odisha.";

    let milestones: &[(&str, &str, &str)] = &[
        ("2021", "Foundation & Vision", "Established Happy Home Hospital in Ghatikia, Bhubaneswar to deliver specialized inpatient neuro recovery."),
        ("2022", "Rehabilitation Expansion", "Upgraded facility with dedicated neuro-physiotherapy, tilt tables, and functional electrical stimulation equipment."),
        ("2023", "30-Bedded Specialty Center", "Commissioned 30 barrier-free inpatient beds with central medical gas, ICU step-down telemetry, and air-mattress care."),
        ("2026", "Premier Regional Referral", "Recognized by leading neurosurgeons across Odisha as the benchmark center for post-stroke, TBI, and spinal cord rehabilitation."),
    ];

    rsx! {
        section {
            id: "about",
            class: "py-20 bg-white dark:bg-slate-900",
            "aria-labelledby": "about-heading",

            div { class: "container-x",
                SectionHeader {
                    tag: "About Happy Home".to_string(),
                    title: "Pioneering Neuro & Spine Rehabilitation in".to_string(),
                    highlight: "Odisha".to_string(),
                    subtitle: "Bridging the gap between hospital discharge and full functional independence with round-the-clock medical attention.".to_string(),
                    center: true,
                }

                div { class: "grid grid-cols-1 lg:grid-cols-12 gap-12 items-center mb-16",

                    // Left Column: Story & Milestones (slide from left on scroll)
                    div { class: "lg:col-span-7 space-y-6 reveal-init reveal-stagger-1",
                        p { class: "text-base sm:text-lg text-slate-700 dark:text-slate-300 leading-relaxed",
                            "Happy Home Hospital & Rehab was established in 2021 in Ghatikia, Bhubaneswar, with a singular mission: to provide world-class, multi-disciplinary inpatient rehabilitation for patients recovering from severe neurological insults, spine surgeries, brain trauma, and chronic immobilizing conditions."
                        }
                        p { class: "text-sm sm:text-base text-slate-600 dark:text-slate-400 leading-relaxed",
                            "Unlike general nursing homes or standard outpatient clinics, Happy Home is a purpose-built 30-bed medical rehabilitation facility. Here, resident physicians, neuro-physiotherapists, speech language pathologists, and critical care nurses work together around the clock to prevent secondary complications like bedsores and contractures while accelerating neural recovery."
                        }

                        // Milestones timeline with staggered items
                        div { class: "pt-6 space-y-4",
                            for (_idx, (year, title, desc)) in milestones.iter().enumerate() {
                                div {
                                    key: "{year}",
                                    class: "timeline-item transition-all duration-300 hover:translate-x-1",
                                    div { class: "timeline-dot flex items-center justify-center shadow-sm",
                                        CheckCircleIcon { size: 14, color: "var(--c-brand)" }
                                    }
                                    div { class: "font-bold text-slate-900 dark:text-white text-sm",
                                        span { class: "text-sky-600 dark:text-sky-400 mr-2 font-extrabold", "{year}" }
                                        "{title}"
                                    }
                                    p { class: "text-xs text-slate-600 dark:text-slate-300 mt-1 leading-relaxed font-medium",
                                        "{desc}"
                                    }
                                }
                            }
                        }
                    }

                    // Right Column: Founder Doctor Card (slide from right on scroll)
                    div { class: "lg:col-span-5 reveal-init reveal-stagger-2",
                        div { class: "cure-card p-8 relative overflow-hidden border-2 border-sky-500/30",
                            div { class: "flex flex-col sm:flex-row items-center sm:items-start gap-6 mb-6",
                                img {
                                    src: "{founder_img}",
                                    alt: "{founder_name}",
                                    class: "w-28 h-28 rounded-2xl object-cover object-top shadow-md border-2 border-sky-500 transition-transform duration-500 hover:scale-105",
                                }
                                div { class: "text-center sm:text-left",
                                    div { class: "inline-block text-[11px] font-bold text-sky-600 dark:text-sky-400 uppercase tracking-widest mb-1",
                                        "Leadership & Medical Vision"
                                    }
                                    h3 { class: "text-xl font-bold text-slate-900 dark:text-white",
                                        "{founder_name}"
                                    }
                                    p { class: "text-xs font-bold text-sky-600 dark:text-sky-400 mt-0.5",
                                        "{founder_title}"
                                    }
                                    p { class: "text-[11px] text-slate-600 dark:text-slate-300 font-semibold mt-1",
                                        "{founder_deg}"
                                    }
                                }
                            }

                            p { class: "text-xs sm:text-sm text-slate-700 dark:text-slate-200 leading-relaxed italic mb-6 border-l-2 border-sky-500 pl-4 font-medium",
                                "\"{founder_bio}\""
                            }

                            div { class: "space-y-2 pt-4 border-t border-slate-200 dark:border-slate-700 text-xs font-medium text-slate-700 dark:text-slate-300",
                                div { class: "flex items-center gap-2",
                                    CheckCircleIcon { size: 16, color: "var(--c-brand)" }
                                    span { "Certified Neuro-Rehabilitation Specialist" }
                                }
                                div { class: "flex items-center gap-2",
                                    CheckCircleIcon { size: 16, color: "var(--c-brand)" }
                                    span { "24/7 Clinical Emergency On-Call Team" }
                                }
                                div { class: "flex items-center gap-2",
                                    CheckCircleIcon { size: 16, color: "var(--c-brand)" }
                                    span { "Consultation & Case Reviews: Mon–Sat" }
                                }
                            }

                            div { class: "mt-6",
                                a {
                                    href: "#appointment",
                                    class: "btn-solid carbon-btn-hover w-full py-3 rounded-xl text-xs font-bold flex items-center justify-center gap-2 shadow-md hover:shadow-lg transition-all",
                                    StethoscopeIcon { size: 16, color: "#fff" }
                                    "Consult Dr. Vijay Kumar Sahu"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
