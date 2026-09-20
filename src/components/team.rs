//! Team / Doctors section displaying senior medical consultants and specialists.

use crate::components::common::SectionHeader;
use crate::components::icons::{PhoneIcon, StethoscopeIcon};
use crate::domain::config::SiteConfig;
use crate::state::{BrandMode, BrandModeSignal, ModalDoctorSignal};
use dioxus::prelude::*;

#[component]
pub fn Team() -> Element {
    let config = use_context::<SiteConfig>();
    let brand_signal = use_context::<BrandModeSignal>();
    let mut modal_doctor_signal = use_context::<ModalDoctorSignal>();
    let brand = *brand_signal.0.read();

    let (tag, title, highlight, subtitle, team) = match brand {
        BrandMode::Hospital => (
            "Medical Faculty & Consultants",
            "Meet Our Distinguished",
            "Specialists",
            "AIIMS, SCB, and premier institute trained specialists delivering compassionate clinical care at Happy Home Hospital.",
            &config.hospital.team,
        ),
        BrandMode::Rehab => (
            "Rehabilitation Experts",
            "Our Multidisciplinary",
            "Clinical Team",
            "Led by Dr. Vijay Kumar Sahu, our specialized team delivers personalized neuro-restorative care.",
            &config.rehab.team,
        ),
    };

    rsx! {
        section {
            id: "team",
            class: "py-20 bg-white dark:bg-[#0a101d]",
            "aria-labelledby": "team-heading",

            div { class: "container-x",
                SectionHeader {
                    tag: tag.to_string(),
                    title: title.to_string(),
                    highlight: highlight.to_string(),
                    subtitle: subtitle.to_string(),
                    center: true,
                }

                div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-8",
                    for (idx, doctor) in team.iter().enumerate() {
                        {
                            let delay_cls = match idx % 4 {
                                0 => "delay-100",
                                1 => "delay-200",
                                2 => "delay-300",
                                _ => "delay-400",
                            };
                            rsx! {
                                div {
                                    key: "{doctor.name}",
                                    class: "cure-card overflow-hidden flex flex-col justify-between group peak-in-up {delay_cls}",

                                    // Photo container with realistic image
                                    div { class: "relative aspect-[3/4] overflow-hidden bg-slate-100 dark:bg-slate-800",
                                        img {
                                            src: "{doctor.image_url}",
                                            alt: "{doctor.name}",
                                            class: "w-full h-full object-cover object-top transition-transform duration-500 group-hover:scale-105",
                                            decoding: "async",
                                        }
                                        div { class: "absolute top-3 left-3 bg-white/95 dark:bg-slate-900/95 text-sky-700 dark:text-sky-300 text-[11px] font-bold px-3 py-1 rounded-full shadow-md backdrop-blur-md border border-slate-200/60 dark:border-slate-700/60 max-w-[90%] truncate",
                                            "{doctor.title}"
                                        }
                                    }

                                    // Info block
                                    div { class: "p-6 flex flex-col flex-grow justify-between",
                                        div {
                                            h3 { class: "text-lg font-bold text-slate-900 dark:text-white mb-1 group-hover:text-sky-600 dark:group-hover:text-sky-400 transition-colors",
                                                "{doctor.name}"
                                            }
                                            p { class: "text-xs font-bold text-sky-600 dark:text-sky-400 mb-3",
                                                "{doctor.role}"
                                            }
                                            p { class: "text-xs leading-relaxed text-slate-600 dark:text-slate-300 mb-4 font-medium",
                                                "{doctor.specialty}"
                                            }
                                        }

                                        div { class: "pt-4 border-t border-slate-100 dark:border-slate-800 flex items-center justify-between gap-2",
                                            a {
                                                href: "#appointment",
                                                class: "btn-solid carbon-btn-hover w-full text-center py-2 px-3 rounded-lg text-xs font-semibold inline-flex items-center justify-center gap-1.5 shadow-sm",
                                                onclick: {
                                                     let name = doctor.name.clone();
                                                    move |_| {
                                                        *modal_doctor_signal.0.write() = Some(name.clone());
                                                    }
                                                },
                                                StethoscopeIcon { size: 14, color: "#fff" }
                                                "Book Consult"
                                            }
                                            a {
                                                href: "tel:+918249914673",
                                                class: "p-2 rounded-lg border border-slate-200 dark:border-slate-700 hover:border-sky-500 text-slate-700 dark:text-slate-200 hover:text-sky-600 dark:hover:text-sky-400 transition-colors",
                                                "aria-label": "Call for appointment",
                                                PhoneIcon { size: 16, color: "currentColor" }
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
}
