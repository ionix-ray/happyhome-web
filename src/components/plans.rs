//! Plans component rendering structured inpatient rehabilitation packages.

use crate::components::common::SectionHeader;
use crate::components::icons::{CarbonHealingSparkIcon, CheckCircleIcon};
use dioxus::prelude::*;

struct PlanItem {
    name: &'static str,
    badge: &'static str,
    price: &'static str,
    period: &'static str,
    description: &'static str,
    featured: bool,
    features: &'static [&'static str],
}

#[component]
pub fn Plans() -> Element {
    let plans: &[PlanItem] = &[
        PlanItem {
            name: "Starter Rehabilitation",
            badge: "Post-Discharge Step-Down",
            price: "Custom Package",
            period: "7–14 Days",
            description: "Ideal for patients discharged from acute tertiary care needing initial stabilization and physical therapy.",
            featured: false,
            features: &[
                "Daily physician rounds & vital monitoring",
                "1 daily specialized physiotherapy session",
                "24/7 dedicated nursing & medication care",
                "Basic wound & catheter management",
                "Dietician consultation & customized meals",
            ],
        },
        PlanItem {
            name: "Intensive Neuro & Spine Care",
            badge: "Most Recommended",
            price: "Specialized Inpatient",
            period: "2–6 Weeks",
            description: "Comprehensive multi-disciplinary rehabilitation for stroke, traumatic brain injury, or spinal cord decompression.",
            featured: true,
            features: &[
                "Twice-daily 1:1 neuro-physiotherapy & gait training",
                "Speech therapy & swallowing rehabilitation",
                "Dynamic alternating pressure air-bed included",
                "Full tracheostomy care & respiratory monitoring",
                "Dedicated attendant accommodation with bed",
                "Weekly multidisciplinary recovery milestone review",
            ],
        },
        PlanItem {
            name: "Long-Term Palliative & Critical Care",
            badge: "Comprehensive Extended",
            price: "Monthly Inpatient",
            period: "Long Term",
            description: "Dignified, 24/7 supportive medical care for chronic neurological conditions, cancer palliative, or high-dependency patients.",
            featured: false,
            features: &[
                "Continuous ICU step-down telemetry monitoring",
                "Advanced Negative Pressure Bedsore therapy (NPWT)",
                "Palliative oncology pain management protocols",
                "Family counseling & compassionate spiritual care",
                "Complete barrier-free room & private ambulance support",
            ],
        },
    ];

    rsx! {
        section {
            id: "plans",
            class: "py-20",
            style: "background: var(--c-bg-2);",
            "aria-labelledby": "plans-heading",

            div { class: "container-x",
                SectionHeader {
                    tag: "Inpatient Care Packages".to_string(),
                    title: "Structured Rehabilitation & Healing".to_string(),
                    highlight: "Plans".to_string(),
                    subtitle: "Transparent clinical programs designed to give families clarity, cost predictability, and the highest standards of care.".to_string(),
                    center: true,
                }

                div { class: "grid grid-cols-1 lg:grid-cols-3 gap-8 items-stretch",
                    for (idx, plan) in plans.iter().enumerate() {
                        {
                            let delay_cls = match idx {
                                0 => "delay-100",
                                1 => "delay-200",
                                _ => "delay-300",
                            };
                            rsx! {
                                div {
                                    key: "{plan.name}",
                                    class: if plan.featured {
                                        "plan-card featured p-8 flex flex-col justify-between peak-in-up {delay_cls} shadow-xl"
                                    } else {
                                        "plan-card p-8 flex flex-col justify-between peak-in-up {delay_cls} hover:-translate-y-1 transition-transform"
                                    },

                                    div {
                                        div { class: "flex items-center justify-between mb-4",
                                            span { class: "text-xs font-bold uppercase tracking-wider px-3 py-1 rounded-full flex items-center gap-1.5",
                                                style: if plan.featured { "background: rgba(255,255,255,0.25); color: #fff;" } else { "background: var(--c-brand-soft); color: var(--c-brand);" },
                                                if plan.featured {
                                                    CarbonHealingSparkIcon { size: 12, color: "#fff" }
                                                }
                                                "{plan.badge}"
                                            }
                                            span { class: "text-xs font-semibold opacity-80",
                                                "{plan.period}"
                                            }
                                        }

                                        h3 { class: "text-2xl font-bold mb-2 plan-name",
                                            "{plan.name}"
                                        }

                                        p { class: "text-xs leading-relaxed mb-6 opacity-80",
                                            "{plan.description}"
                                        }

                                        div { class: "mb-6 pb-6 border-b", style: if plan.featured { "border-color: rgba(255,255,255,0.2);" } else { "border-color: var(--c-line);" },
                                            div { class: "text-2xl font-extrabold plan-price", "{plan.price}" }
                                            div { class: "text-xs opacity-75 mt-1", "Tailored to patient clinical assessment" }
                                        }

                                        ul { class: "space-y-3 mb-8",
                                            for feat in plan.features.iter() {
                                                li { class: "flex items-start gap-2.5 text-xs font-medium leading-relaxed",
                                                    span { class: "check mt-0.5",
                                                        CheckCircleIcon { size: 14, color: "currentColor" }
                                                    }
                                                    span { "{feat}" }
                                                }
                                            }
                                        }
                                    }

                                    div {
                                        a {
                                            href: "#appointment",
                                            class: if plan.featured {
                                                "w-full text-center py-3 px-6 rounded-xl font-bold text-sm bg-white text-sky-950 hover:bg-slate-100 transition-all carbon-btn-hover shadow-lg block"
                                            } else {
                                                "w-full text-center py-3 px-6 rounded-xl font-bold text-sm btn-solid carbon-btn-hover block"
                                            },
                                            "Enquire About Admission"
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
