//! Happy Home Hospital & Rehab — Dioxus 0.7.6 WASM SPA entry point.
//!
//! Dual-brand architecture (Hospital & Inpatient Rehab Centre)
//! Fully config-driven via compile-time embedded `site.toml`.
//! Zero emojis, crisp SVG icons, realistic Indian doctors from premier medical institutions.

use dioxus::prelude::*;

pub mod components;
pub mod domain;
mod state;

use crate::domain::config::SiteConfig;
use components::icons::{
    CarbonActivityPulseIcon, CloseIcon, HospitalIcon, MapPinIcon, MenuIcon, MoonIcon,
    StethoscopeIcon, SunIcon,
};
use state::{init_global_state, BrandMode, BrandModeSignal, MobileNavSignal, ThemeSignal};

// ── Compile-time config embedding ────────────────────────────────────────────
fn load_config() -> SiteConfig {
    const TOML_STR: &str = include_str!("../config/site.toml");
    toml::from_str(TOML_STR).expect("site.toml must be valid TOML matching SiteConfig schema")
}

// ── Root application component ────────────────────────────────────────────────
#[component]
fn App() -> Element {
    // Provide compile-time config context
    use_context_provider(load_config);

    let (brand_mode, theme, _mobile_nav, _modal, _active, _toast) = init_global_state();

    let brand_str = brand_mode.0.read().as_str();
    let theme_str = theme.0.read().as_str();

    rsx! {
        div {
            id: "app",
            "data-theme": "{theme_str}",
            "data-brand": "{brand_str}",
            class: if theme_str == "dark" { "dark bg-slate-950 text-slate-100 min-h-screen" } else { "bg-white text-slate-900 min-h-screen" },

            // Accessibility skip link
            a { class: "skip-link", href: "#main-content", "Skip to main content" }

            // Sticky Header containing TopBar and NavBar
            header {
                class: "sticky top-0 z-40 shadow-sm transition-all duration-300",
                TopBar {}
                NavBar {}
            }

            // Main sections
            main { id: "main-content",
                components::hero::Hero {}
                components::about::About {}
                components::services::Services {}
                components::departments::Departments {}
                components::team::Team {}
                components::why_us::WhyUs {}
                components::plans::Plans {}
                components::testimonials::Testimonials {}
                components::appointment::Appointment {}
                components::faq::Faq {}
                components::contact::Contact {}
            }

            // Footer
            components::footer::Footer {}

            // Interactive popups & toasts
            components::modal::DoctorModal {}
            components::toast::Toast {}
        }
    }
}

// ── TopBar ────────────────────────────────────────────────────────────────────
#[component]
fn TopBar() -> Element {
    let config = use_context::<SiteConfig>();

    rsx! {
        div { class: "topbar py-2 px-4 sm:px-6 border-b border-white/10 text-xs font-semibold select-none",
            div { class: "container-x flex flex-wrap items-center justify-between gap-3 text-white",
                div { class: "flex items-center gap-6",
                    span { class: "inline-flex items-center gap-2",
                        CarbonActivityPulseIcon { size: 16, color: "#fecdd3" }
                        span { "{config.emergency.message}: " }
                        a {
                            href: "tel:{config.emergency.phone}",
                            class: "font-bold text-white hover:underline",
                            "{config.emergency.phone}"
                        }
                    }
                    span { class: "hidden md:inline-flex items-center gap-1.5 text-white/80",
                        MapPinIcon { size: 14, color: "currentColor" }
                        span { "{config.contact.address}" }
                    }
                }
                div { class: "hidden sm:flex items-center gap-4 text-white/90 text-[11px]",
                    span { "{config.contact.hours}" }
                }
            }
        }
    }
}

// ── NavBar ────────────────────────────────────────────────────────────────────
#[component]
fn NavBar() -> Element {
    let brand_signal = use_context::<BrandModeSignal>();
    let theme_signal = use_context::<ThemeSignal>();
    let mut mobile_nav_signal = use_context::<MobileNavSignal>();

    let mut brand = brand_signal.0;
    let mut theme = theme_signal.0;
    let is_mobile_open = *mobile_nav_signal.0.read();

    let nav_links: &[(&str, &str)] = &[
        ("Home", "#home"),
        ("About Us", "#about"),
        ("Services", "#services"),
        ("Portfolio", "#appointment"),
        ("Career", "#contact"),
        ("Doctors", "#team"),
        ("Blog", "#contact"),
        ("Contact", "#contact"),
    ];

    rsx! {
        nav {
            class: "w-full bg-white/95 dark:bg-slate-900/95 backdrop-blur-md transition-all duration-300",
            "aria-label": "Main navigation",

            div { class: "container-x flex items-center justify-between py-3",

                // Official Brand Logo — Crisp Transparent Vector SVG
                a {
                    class: "flex items-center gap-2 shrink-0 group focus:outline-none py-1",
                    href: "#home",
                    "aria-label": "Happy Home Hospital And Rehab Home",
                    components::icons::HappyHomeLogo { height: 44 }
                }

                // Desktop Nav links
                div { class: "hidden lg:flex items-center gap-6 text-xs font-bold tracking-wide uppercase text-slate-600 dark:text-slate-300",
                    for (name, href) in nav_links.iter() {
                        a {
                            key: "{name}",
                            href: "{href}",
                            class: "hover:text-sky-600 dark:hover:text-sky-400 transition-colors py-1",
                            "{name}"
                        }
                    }
                }

                // Right controls: Brand toggle, Theme toggle, Appointment CTA, Mobile burger
                div { class: "flex items-center gap-2.5 sm:gap-3",

                    // Brand toggle pill (Hospital vs Rehab mode at end of navbar)
                    div {
                        class: "hidden md:inline-flex p-1 bg-slate-100 dark:bg-slate-800 rounded-xl border border-slate-200 dark:border-slate-700 text-xs font-bold shadow-inner",
                        role: "group",
                        "aria-label": "Select medical mode",

                        button {
                            class: if *brand.read() == BrandMode::Hospital {
                                "px-3.5 py-1.5 rounded-lg bg-gradient-to-r from-blue-700 to-sky-600 text-white shadow-sm transition-all inline-flex items-center gap-1.5 text-[11px] uppercase tracking-wider"
                            } else {
                                "px-3.5 py-1.5 rounded-lg text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white transition-all inline-flex items-center gap-1.5 text-[11px] uppercase tracking-wider"
                            },
                            onclick: move |_| *brand.write() = BrandMode::Hospital,
                            HospitalIcon { size: 13, color: "currentColor" }
                            span { "Hospital" }
                        }
                        button {
                            class: if *brand.read() == BrandMode::Rehab {
                                "px-3.5 py-1.5 rounded-lg bg-gradient-to-r from-teal-700 to-emerald-600 text-white shadow-sm transition-all inline-flex items-center gap-1.5 text-[11px] uppercase tracking-wider"
                            } else {
                                "px-3.5 py-1.5 rounded-lg text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white transition-all inline-flex items-center gap-1.5 text-[11px] uppercase tracking-wider"
                            },
                            onclick: move |_| *brand.write() = BrandMode::Rehab,
                            StethoscopeIcon { size: 13, color: "currentColor" }
                            span { "Rehab Centre" }
                        }
                    }

                    // Theme toggle
                    button {
                        class: "p-2 rounded-xl border border-slate-200 dark:border-slate-700 text-slate-600 dark:text-slate-300 hover:border-sky-500 transition-colors flex items-center justify-center hover:bg-slate-50 dark:hover:bg-slate-800",
                        onclick: move |_| {
                            let cur = *theme.read();
                            *theme.write() = cur.toggle();
                        },
                        "aria-label": "Toggle theme",
                        title: "Toggle Light/Dark",
                        if *theme.read() == state::Theme::Dark {
                            SunIcon { size: 16, color: "currentColor" }
                        } else {
                            MoonIcon { size: 16, color: "currentColor" }
                        }
                    }

                    // Book appointment CTA
                    a {
                        class: "btn-solid hidden sm:inline-flex items-center gap-2 px-4 sm:px-5 py-2.5 rounded-xl text-xs font-bold uppercase tracking-wider carbon-btn-hover shadow-md",
                        href: "#appointment",
                        StethoscopeIcon { size: 14, color: "#fff" }
                        "Book Appointment"
                    }

                    // Mobile Menu Hamburger Button
                    button {
                        class: "lg:hidden p-2 rounded-xl text-slate-700 dark:text-slate-200 flex items-center justify-center hover:bg-slate-100 dark:hover:bg-slate-800",
                        onclick: move |_| *mobile_nav_signal.0.write() = !is_mobile_open,
                        "aria-label": "Toggle mobile navigation",
                        if is_mobile_open {
                            CloseIcon { size: 20, color: "currentColor" }
                        } else {
                            MenuIcon { size: 20, color: "currentColor" }
                        }
                    }
                }
            }

            // Mobile menu drawer
            if is_mobile_open {
                div { class: "lg:hidden border-t border-slate-200 dark:border-slate-800 bg-white/95 dark:bg-slate-900/95 p-6 space-y-4 shadow-xl",

                    // Mobile brand toggle
                    div { class: "flex p-1 bg-slate-100 dark:bg-slate-800 rounded-xl mb-4 text-xs font-bold",
                        button {
                            class: if *brand.read() == BrandMode::Hospital {
                                "flex-1 py-2 rounded-lg bg-emerald-700 text-white text-center flex items-center justify-center gap-1.5"
                            } else {
                                "flex-1 py-2 text-slate-600 dark:text-slate-400 text-center flex items-center justify-center gap-1.5"
                            },
                            onclick: move |_| {
                                *brand.write() = BrandMode::Hospital;
                                *mobile_nav_signal.0.write() = false;
                            },
                            HospitalIcon { size: 14, color: "currentColor" }
                            span { "Hospital" }
                        }
                        button {
                            class: if *brand.read() == BrandMode::Rehab {
                                "flex-1 py-2 rounded-lg bg-emerald-700 text-white text-center flex items-center justify-center gap-1.5"
                            } else {
                                "flex-1 py-2 text-slate-600 dark:text-slate-400 text-center flex items-center justify-center gap-1.5"
                            },
                            onclick: move |_| {
                                *brand.write() = BrandMode::Rehab;
                                *mobile_nav_signal.0.write() = false;
                            },
                            StethoscopeIcon { size: 14, color: "currentColor" }
                            span { "Rehab Centre" }
                        }
                    }

                    div { class: "flex flex-col space-y-3 text-sm font-semibold text-slate-700 dark:text-slate-200",
                        for (name, href) in nav_links.iter() {
                            a {
                                key: "{name}",
                                href: "{href}",
                                onclick: move |_| *mobile_nav_signal.0.write() = false,
                                class: "py-1 hover:text-emerald-700 transition-colors",
                                "{name}"
                            }
                        }
                    }

                    div { class: "pt-4 border-t border-slate-200 dark:border-slate-800",
                        a {
                            class: "btn-solid w-full text-center py-3 rounded-xl text-sm font-bold block",
                            href: "#appointment",
                            onclick: move |_| *mobile_nav_signal.0.write() = false,
                            "Book Appointment"
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    dioxus::launch(App);
}
