//! Contact and location component with interactive directions and emergency channels.

use crate::components::common::SectionHeader;
use crate::components::icons::{ClockIcon, MapPinIcon, PhoneIcon, WhatsAppIcon};
use crate::domain::config::SiteConfig;
use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    let config = use_context::<SiteConfig>();

    let phone_primary = &config.contact.phone_primary;
    let phone_secondary = &config.contact.phone_secondary;
    let whatsapp = &config.contact.phone_whatsapp;
    let address = &config.contact.address;
    let email = &config.contact.email;
    let hours = &config.contact.hours;

    let maps_link = "https://maps.google.com/?q=Happy+Home+Hospital+Kalinga+Nagar+Bhubaneswar";
    let whatsapp_link = format!(
        "https://wa.me/{}?text=Hello%20Happy%20Home%20Hospital,%20I%20have%20an%20enquiry.",
        whatsapp.trim_start_matches('+')
    );

    rsx! {
        section {
            id: "contact",
            class: "py-20 bg-white dark:bg-slate-900",
            "aria-labelledby": "contact-heading",

            div { class: "container-x",
                SectionHeader {
                    tag: "Location & Direct Reach".to_string(),
                    title: "Visit Our Facility in".to_string(),
                    highlight: "Bhubaneswar".to_string(),
                    subtitle: "Conveniently situated in Kalinga Nagar, Ghatikia. Open 24 hours a day for trauma and emergency admissions.".to_string(),
                    center: true,
                }

                div { class: "grid grid-cols-1 lg:grid-cols-3 gap-8 mb-12",

                    // Card 1: Address
                    div { class: " p-8 flex flex-col justify-between group peak-in-up delay-100",
                        div {
                            div { class: "icon-ring mb-4 group-hover:scale-110 transition-transform duration-300 shadow-sm",
                                MapPinIcon { size: 24, color: "var(--c-brand)" }
                            }
                            h3 { class: "text-lg font-bold text-slate-900 dark:text-white mb-2 group-hover:text-sky-600 transition-colors", "Hospital Address" }
                            p { class: "text-sm text-slate-700 dark:text-slate-200 leading-relaxed mb-4 font-medium", "{address}" }
                        }
                        a {
                            href: "{maps_link}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "text-xs font-bold text-sky-600 hover:text-sky-700 dark:text-sky-400 flex items-center gap-1 group-hover:translate-x-1 transition-transform",
                            "Get Directions on Google Maps →"
                        }
                    }

                    // Card 2: Contact Numbers
                    div { class: " p-8 flex flex-col justify-between group peak-in-up delay-200",
                        div {
                            div { class: "icon-ring mb-4 group-hover:scale-110 transition-transform duration-300 shadow-sm",
                                PhoneIcon { size: 24, color: "var(--c-brand)" }
                            }
                            h3 { class: "text-lg font-bold text-slate-900 dark:text-white mb-2 group-hover:text-sky-600 transition-colors", "Phone & WhatsApp" }
                            p { class: "text-sm text-slate-700 dark:text-slate-200 leading-relaxed mb-2 font-medium",
                                "Emergency: "
                                a { href: "tel:{phone_primary}", class: "font-bold text-sky-600 dark:text-sky-400 hover:underline", "{phone_primary}" }
                            }
                            p { class: "text-sm text-slate-700 dark:text-slate-200 leading-relaxed mb-4 font-medium",
                                "Alternative: "
                                a { href: "tel:{phone_secondary}", class: "font-bold text-sky-600 dark:text-sky-400 hover:underline", "{phone_secondary}" }
                            }
                        }
                        a {
                            href: "{whatsapp_link}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "btn-solid py-2.5 px-4 rounded-lg text-xs font-bold inline-flex items-center justify-center gap-2 carbon-btn-hover",
                            WhatsAppIcon { size: 16, color: "#fff" }
                            "Open WhatsApp Chat"
                        }
                    }

                    // Card 3: Hours & Email
                    div { class: " p-8 flex flex-col justify-between group peak-in-up delay-300",
                        div {
                            div { class: "icon-ring mb-4 group-hover:scale-110 transition-transform duration-300 shadow-sm",
                                ClockIcon { size: 24, color: "var(--c-brand)" }
                            }
                            h3 { class: "text-lg font-bold text-slate-900 dark:text-white mb-2 group-hover:text-sky-600 transition-colors", "Hours & Support" }
                            p { class: "text-sm text-slate-700 dark:text-slate-200 leading-relaxed mb-2 font-semibold", "{hours}" }
                            p { class: "text-sm text-slate-700 dark:text-slate-200 leading-relaxed mb-4 font-medium",
                                "Email: "
                                a { href: "mailto:{email}", class: "text-sky-600 dark:text-sky-400 underline font-semibold", "{email}" }
                            }
                        }
                        div { class: "inline-flex items-center gap-2 text-xs font-bold text-sky-600 dark:text-sky-400",
                            span { class: "w-2 h-2 rounded-full bg-amber-400 animate-ping" }
                            "Emergency Trauma Staff on Duty"
                        }
                    }
                }

                // Visual Google Maps placeholder / banner
                div { class: "rounded-3xl overflow-hidden border border-slate-200 dark:border-slate-800 shadow-md h-72 bg-slate-100 dark:bg-slate-800 relative flex items-center justify-center peak-in-scale delay-200",
                    img {
                        src: "assets/images/facility.jpg",
                        alt: "Happy Home Hospital Facility",
                        class: "w-full h-full object-cover opacity-80",
                    }
                    div { class: "absolute inset-0 bg-gradient-to-t from-slate-950/80 via-slate-950/30 to-transparent flex items-end p-8",
                        div { class: "flex flex-col sm:flex-row sm:items-center justify-between w-full gap-4",
                            div {
                                div { class: "text-white text-xl font-bold", "Happy Home Hospital & Inpatient Rehab Centre" }
                                div { class: "text-white/80 text-sm", "Plot no.223, K-1 Kalinga Nagar, Ghatikia, Bhubaneswar 751029" }
                            }
                            a {
                                href: "{maps_link}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                class: "btn-solid py-2.5 px-6 rounded-xl text-sm font-bold shrink-0 inline-flex items-center gap-2",
                                MapPinIcon { size: 16, color: "#fff" }
                                "Navigate with GPS"
                            }
                        }
                    }
                }
            }
        }
    }
}
