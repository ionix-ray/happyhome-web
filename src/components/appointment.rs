//! Appointment booking and admission inquiry form component with domain validation.

use crate::components::common::SectionHeader;
use crate::components::icons::{AlertCircleIcon, PhoneIcon, StethoscopeIcon, WhatsAppIcon};
use crate::domain::config::SiteConfig;
use crate::domain::types::ContactFormInput;
use crate::domain::validation::validate_contact_form;
use crate::state::{BrandMode, BrandModeSignal, ModalDoctorSignal, ToastSignal};
use dioxus::prelude::*;

#[component]
pub fn Appointment() -> Element {
    let config = use_context::<SiteConfig>();
    let brand_signal = use_context::<BrandModeSignal>();
    let modal_doctor_signal = use_context::<ModalDoctorSignal>();
    let mut toast_signal = use_context::<ToastSignal>();

    let brand = *brand_signal.0.read();
    let preselected_doc = modal_doctor_signal.0.read().clone().unwrap_or_default();

    let mut name = use_signal(String::new);
    let mut phone = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut service = use_signal(|| match brand {
        BrandMode::Hospital => "Emergency & Critical Care".to_string(),
        BrandMode::Rehab => "Stroke Recovery & Paralysis Management".to_string(),
    });
    let mut doctor = use_signal(|| {
        if !preselected_doc.is_empty() {
            preselected_doc
        } else {
            "Dr. Vijay Kumar Sahu (Founder & Director)".to_string()
        }
    });
    let mut message = use_signal(String::new);
    let mut error_msg = use_signal(|| None::<String>);

    let phone_link = format!("tel:{}", config.emergency.phone.replace([' ', '-'], ""));
    let whatsapp_link = format!("https://wa.me/{}?text=Hello%20Happy%20Home%20Hospital,%20I%20want%20to%20enquire%20about%20an%20appointment.", config.emergency.whatsapp_phone.trim_start_matches('+'));

    let on_submit = move |_e: Event<FormData>| {
        let input = ContactFormInput {
            name: name.read().clone(),
            phone: phone.read().clone(),
            email: if email.read().is_empty() {
                "patient@happyhomehospital.com".to_string()
            } else {
                email.read().clone()
            },
            service: service.read().clone(),
            message: format!("Doctor: {} | Notes: {}", doctor.read(), message.read()),
        };

        match validate_contact_form(&input) {
            Ok(_) => {
                *error_msg.write() = None;
                *toast_signal.0.write() = Some(
                    "Appointment request registered! Our coordinator will call within 30 minutes."
                        .to_string(),
                );
                *name.write() = String::new();
                *phone.write() = String::new();
                *message.write() = String::new();
            }
            Err(err) => {
                *error_msg.write() = Some(format!("Please verify details: {}", err));
            }
        }
    };

    rsx! {
        section {
            id: "appointment",
            class: "py-20 bg-white dark:bg-slate-900",
            "aria-labelledby": "appointment-heading",

            div { class: "container-x",
                SectionHeader {
                    tag: "Fast Track Booking".to_string(),
                    title: "Schedule Consultation & Inpatient".to_string(),
                    highlight: "Admission".to_string(),
                    subtitle: "Connect directly with Dr. Vijay Kumar Sahu and our clinical care team. Same-day appointments available.".to_string(),
                    center: true,
                }

                div { class: "grid grid-cols-1 lg:grid-cols-12 gap-12 items-start",

                    // Left column: Contact info & emergency highlight
                    div { class: "lg:col-span-5  p-8 text-white flex flex-col justify-between h-full peak-in-left shadow-xl",
                        style: "background: linear-gradient(145deg, #101a29 0%, #134cb3 70%, #1e5cb8 100%); border-color: rgba(255,255,255,0.15);",

                        div {
                            span { class: "inline-flex items-center gap-2 px-3 py-1 rounded-full text-xs font-bold bg-sky-400/20 text-sky-200 border border-sky-400/30 mb-6",
                                span { class: "vital-live-dot" }
                                "24/7 Admissions Helpline"
                            }

                            h3 { class: "text-2xl sm:text-3xl font-bold mb-4 leading-snug text-white",
                                "Need Immediate Medical Assistance or Ambulance?"
                            }

                            p { class: "text-sky-100/90 text-sm leading-relaxed mb-8 font-medium",
                                "Our emergency resuscitation team and inter-hospital ambulance transfer service are operational round the clock across Odisha."
                            }

                            div { class: "space-y-4 mb-8",
                                a {
                                    href: "{phone_link}",
                                    class: "p-4 rounded-xl bg-white/10 hover:bg-white/15 border border-white/15 flex items-center gap-4 transition-all duration-300 hover:translate-x-1",
                                    div { class: "w-10 h-10 rounded-lg bg-sky-500 flex items-center justify-center text-white shrink-0 shadow-md",
                                        PhoneIcon { size: 20, color: "#fff" }
                                    }
                                    div {
                                        div { class: "text-xs text-sky-200 font-medium", "Direct Phone Line" }
                                        div { class: "text-lg font-extrabold tracking-wide", "{config.emergency.phone}" }
                                    }
                                }

                                a {
                                    href: "{whatsapp_link}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "p-4 rounded-xl bg-white/10 hover:bg-white/15 border border-white/15 flex items-center gap-4 transition-all duration-300 hover:translate-x-1",
                                    div { class: "w-10 h-10 rounded-lg bg-teal-600 flex items-center justify-center text-white shrink-0 shadow-md",
                                        WhatsAppIcon { size: 20, color: "#fff" }
                                    }
                                    div {
                                        div { class: "text-xs text-teal-200 font-medium", "Instant WhatsApp Chat" }
                                        div { class: "text-lg font-extrabold tracking-wide", "{config.emergency.whatsapp_phone}" }
                                    }
                                }
                            }
                        }

                        div { class: "pt-6 border-t border-sky-700/50 text-xs text-sky-200/80 leading-relaxed",
                            "Plot no.223, K-1 Kalinga Nagar, Ghatikia, Bhubaneswar 751029. Inpatient admissions open 24×7."
                        }
                    }

                    // Right column: Form
                    div { class: "lg:col-span-7  p-8 sm:p-10 peak-in-right delay-150 shadow-lg",
                        form {
                            onsubmit: on_submit,
                            class: "space-y-6",

                            if let Some(err) = error_msg.read().as_ref() {
                                div { class: "p-4 rounded-xl bg-rose-50 border border-rose-200 text-rose-700 text-sm font-semibold flex items-center gap-2",
                                    AlertCircleIcon { size: 18, color: "#be123c" }
                                    span { "{err}" }
                                }
                            }

                            div { class: "grid grid-cols-1 sm:grid-cols-3 gap-6",
                                div {
                                    label { class: "block text-xs font-bold text-slate-700 dark:text-slate-300 uppercase tracking-wider mb-2",
                                        "Patient Full Name *"
                                    }
                                    input {
                                        id: "patient-name",
                                        name: "patient_name",
                                        class: "inp",
                                        r#type: "text",
                                        required: true,
                                        placeholder: "e.g., Ramesh Mohanty",
                                        value: "{name}",
                                        oninput: move |e| *name.write() = e.value(),
                                    }
                                }

                                div {
                                    label { class: "block text-xs font-bold text-slate-700 dark:text-slate-300 uppercase tracking-wider mb-2",
                                        "Mobile Number *"
                                    }
                                    input {
                                        id: "patient-phone",
                                        name: "patient_phone",
                                        class: "inp",
                                        r#type: "tel",
                                        required: true,
                                        placeholder: "+91 98XXX XXXXX",
                                        value: "{phone}",
                                        oninput: move |e| *phone.write() = e.value(),
                                    }
                                }

                                div {
                                    label { class: "block text-xs font-bold text-slate-700 dark:text-slate-300 uppercase tracking-wider mb-2",
                                        "Email Address"
                                    }
                                    input {
                                        class: "inp",
                                        r#type: "email",
                                        placeholder: "care@example.com",
                                        value: "{email}",
                                        oninput: move |e| *email.write() = e.value(),
                                    }
                                }
                            }

                            div { class: "grid grid-cols-1 sm:grid-cols-2 gap-6",
                                div {
                                    label { class: "block text-xs font-bold text-slate-700 dark:text-slate-300 uppercase tracking-wider mb-2",
                                        "Select Department / Service"
                                    }
                                    select {
                                        class: "inp",
                                        value: "{service}",
                                        onchange: move |e| *service.write() = e.value(),
                                        option { value: "Post Neuro-Surgery Rehabilitation", "Post Neuro-Surgery Rehabilitation" }
                                        option { value: "Stroke Recovery & Paralysis Management", "Stroke Recovery & Paralysis Management" }
                                        option { value: "Spinal Cord Injury Rehabilitation", "Spinal Cord Injury Rehabilitation" }
                                        option { value: "Traumatic Brain Injury (TBI) Care", "Traumatic Brain Injury (TBI) Care" }
                                        option { value: "Advanced Bedsore & Wound Care", "Advanced Bedsore & Wound Care" }
                                        option { value: "Tracheostomy & Respiratory Support", "Tracheostomy & Respiratory Support" }
                                        option { value: "Critical Care ICU & HDU Step-down", "Critical Care ICU & HDU Step-down" }
                                        option { value: "Outpatient Multi-Specialty OPD", "Outpatient Multi-Specialty OPD" }
                                    }
                                }

                                div {
                                    label { class: "block text-xs font-bold text-slate-700 dark:text-slate-300 uppercase tracking-wider mb-2",
                                        "Consulting Doctor"
                                    }
                                    select {
                                        class: "inp",
                                        value: "{doctor}",
                                        onchange: move |e| *doctor.write() = e.value(),
                                        option { value: "Dr. Vijay Kumar Sahu (Founder & Director)", "Dr. Vijay Kumar Sahu (Founder & Director)" }
                                        option { value: "Dr. Sambit Dash (Critical Care Lead)", "Dr. Sambit Dash (Critical Care Lead)" }
                                        option { value: "Dr. Ananya Pattnaik (Senior Neurologist)", "Dr. Ananya Pattnaik (Senior Neurologist)" }
                                        option { value: "Dr. Jajati Keshari Jena (Orthopedic Rehab)", "Dr. Jajati Keshari Jena (Orthopedic Rehab)" }
                                    }
                                }
                            }

                            div {
                                label { class: "block text-xs font-bold text-slate-700 dark:text-slate-300 uppercase tracking-wider mb-2",
                                    "Clinical Condition / Medical Notes"
                                }
                                textarea {
                                    class: "inp",
                                    rows: "3",
                                    placeholder: "Please describe recent surgery, duration of illness, or specific requirements (e.g. tracheostomy, air bed, ambulance transfer)...",
                                    value: "{message}",
                                    oninput: move |e| *message.write() = e.value(),
                                }
                            }

                            div { class: "pt-2",
                                button {
                                    r#type: "submit",
                                    class: "btn-solid carbon-btn-hover w-full py-4 rounded-xl text-base font-bold tracking-wide shadow-xl flex items-center justify-center gap-2 transition-all",
                                    StethoscopeIcon { size: 18, color: "#fff" }
                                    "Confirm Consultation Request"
                                }
                                p { class: "text-[11px] text-center text-slate-400 mt-3",
                                    "Your medical information is held strictly confidential under HIPAA & NABH healthcare norms."
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
