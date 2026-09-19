use crate::components::common::SectionHeader;
use dioxus::prelude::*;

struct DeptItem {
    name: &'static str,
    tag: &'static str,
    beds: &'static str,
    description: &'static str,
    features: &'static [&'static str],
}

#[component]
pub fn Departments() -> Element {
    let departments: &[DeptItem] = &[
        DeptItem {
            name: "Neuro & Spine Rehabilitation Ward",
            tag: "Inpatient Centre",
            beds: "30 Dedicated Beds",
            description: "Specialized inpatient recovery division for post-craniotomy, spinal trauma, and hemiplegia recovery.",
            features: &["Barrier-free motorized beds", "Multi-disciplinary team rounds", "Daily customized physio & occupational therapy"],
        },
        DeptItem {
            name: "Critical Care ICU & High Dependency Unit",
            tag: "24/7 Intensive Care",
            beds: "Continuous Monitoring",
            description: "Equipped with advanced multi-para monitoring, arterial line blood pressure tracking, central oxygen, and crash cart readiness.",
            features: &["1:1 critical nursing support", "Resident anesthesiologist coverage", "Step-down tracheostomy weaning"],
        },
        DeptItem {
            name: "Advanced Neuro-Physiotherapy & Gait Lab",
            tag: "Therapy Center",
            beds: "Fully Outfitted Gym",
            description: "State-of-the-art rehabilitation gym equipped with tilt tables, functional electrical stimulation (FES), and parallel bars.",
            features: &["Robotic balance training", "Sensory reintegration therapy", "Fine-motor occupational therapy"],
        },
        DeptItem {
            name: "Advanced Bedsore & Air-Bed Healing Clinic",
            tag: "Wound Management",
            beds: "Specialized Mattresses",
            description: "Negative Pressure Wound Therapy (NPWT), enzymatic debridement, and dynamic alternating pressure air mattresses.",
            features: &["Stage 1 to 4 pressure ulcer reversal", "Sterile wound dressing protocols", "Nutritional healing optimization"],
        },
        DeptItem {
            name: "Tracheostomy & Respiratory Support Unit",
            tag: "Airway Care",
            beds: "Piped Oxygen & Suction",
            description: "Round-the-clock suctioning, decannulation therapy, and airway safety for long-term critical respiratory patients.",
            features: &["Piped medical gas at every bed", "Chest physiotherapy & nebulization", "Specialized speech & swallowing retraining"],
        },
        DeptItem {
            name: "Palliative Oncology & Chronic Care Ward",
            tag: "Supportive Medicine",
            beds: "Dignified Comfort",
            description: "Holistic pain relief, compassionate symptom alleviation, and emotional counselling for end-stage cancer and geriatric patients.",
            features: &["Specialized pain management", "Family counseling & attendant comfort", "Dignified, peaceful environment"],
        },
    ];

    rsx! {
        section {
            id: "departments",
            class: "py-20",
            style: "background: var(--c-bg-2);",
            "aria-labelledby": "departments-heading",

            div { class: "container-x",
                SectionHeader {
                    tag: "Clinical Infrastructure".to_string(),
                    title: "Specialized Hospital & Rehab".to_string(),
                    highlight: "Departments".to_string(),
                    subtitle: "Engineered specifically for complex neuro recovery, critical patient stabilization, and round-the-clock inpatient healing.".to_string(),
                    center: true,
                }

                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                    for (idx, dept) in departments.iter().enumerate() {
                        {
                            let delay_cls = match idx % 3 {
                                0 => "delay-100",
                                1 => "delay-200",
                                _ => "delay-300",
                            };
                            rsx! {
                                div {
                                    key: "{dept.name}",
                                    class: " p-8 flex flex-col justify-between group hover:border-sky-500 transition-all peak-in-up {delay_cls}",

                                    div {
                                        div { class: "flex items-center justify-between mb-4",
                                            span { class: "text-xs font-bold uppercase tracking-wider text-sky-700 dark:text-sky-300 bg-sky-50 dark:bg-sky-950/60 px-3 py-1 rounded-full",
                                                "{dept.tag}"
                                            }
                                            span { class: "text-xs font-semibold text-slate-600 dark:text-slate-300",
                                                "{dept.beds}"
                                            }
                                        }

                                        h3 { class: "text-xl font-bold text-slate-900 dark:text-white mb-3 group-hover:text-sky-600 dark:group-hover:text-sky-400 transition-colors",
                                            "{dept.name}"
                                        }

                                        p { class: "text-sm text-slate-700 dark:text-slate-300 mb-6 leading-relaxed font-medium",
                                            "{dept.description}"
                                        }

                                        ul { class: "space-y-2.5 mb-6",
                                            for feat in dept.features.iter() {
                                                li { class: "flex items-center gap-2 text-xs font-medium text-slate-700 dark:text-slate-300",
                                                    span { class: "w-1.5 h-1.5 rounded-full bg-sky-500 shrink-0" }
                                                    "{feat}"
                                                }
                                            }
                                        }
                                    }

                                    div { class: "pt-4 border-t border-slate-100 dark:border-slate-800",
                                        a {
                                            href: "#appointment",
                                            class: "text-xs font-bold text-sky-600 hover:text-sky-700 dark:text-sky-400 flex items-center justify-between group-hover:translate-x-1 transition-transform",
                                            span { "Check Inpatient Admission" }
                                            span { "→" }
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
