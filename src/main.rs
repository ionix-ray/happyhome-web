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
use components::common::{DividerType, WaveDivider};
use components::icons::{
    CarbonActivityPulseIcon, CarbonTimeClockIcon, CloseIcon, HospitalIcon, MapPinIcon, MenuIcon, MoonIcon,
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

            // Dynamic Site-Wide Multi-Colour Gene Mutation & Neuro-Synapse Canvas
            canvas {
                id: "bio-canvas",
                class: "bio-canvas-fixed",
            }

            // Accessibility skip link
            a { class: "skip-link", href: "#main-content", "Skip to main content" }

            // Sticky Header containing TopBar and NavBar
            header {
                class: "sticky top-0 z-40 shadow-sm transition-all duration-300",
                TopBar {}
                NavBar {}
            }

            // Main sections with organic wave transitions
            main { id: "main-content",
                components::hero::Hero {}
                components::about::About {}
                WaveDivider { variant: DividerType::ToBg2, inverted: false }
                components::services::Services {}
                components::departments::Departments {}
                WaveDivider { variant: DividerType::ToTeam, inverted: true }
                components::team::Team {}
                components::why_us::WhyUs {}
                WaveDivider { variant: DividerType::ToBg2, inverted: false }
                components::plans::Plans {}
                WaveDivider { variant: DividerType::ToWhite, inverted: true }
                components::testimonials::Testimonials {}
                components::appointment::Appointment {}
                WaveDivider { variant: DividerType::ToBg2, inverted: false }
                components::faq::Faq {}
                WaveDivider { variant: DividerType::ToWhite, inverted: true }
                components::contact::Contact {}
                WaveDivider { variant: DividerType::ToFooter, inverted: false }
            }

            // Footer
            components::footer::Footer {}

            // Interactive popups, mobile drawer & toasts
            MobileNavDrawer {}
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
        div {
            class: "topbar py-1.5 px-4 sm:px-6 border-b border-white/10 text-xs font-semibold select-none overflow-hidden",
            // Desktop: Clean single row with dots
            div { class: "container-x hidden md:flex items-center justify-between gap-4 text-white text-[11px]",
                div { class: "flex items-center gap-2",
                    CarbonActivityPulseIcon { size: 14, color: "#fecdd3" }
                    span { class: "text-white/90", "Emergency Helpline:" }
                    a {
                        href: "tel:{config.emergency.phone}",
                        class: "font-bold text-white hover:underline ml-1",
                        "{config.emergency.phone}"
                    }
                }
                div { class: "flex items-center gap-1.5 text-white/80",
                    MapPinIcon { size: 13, color: "currentColor" }
                    span { "Plot no.223, K-1 Kalinga Nagar, Bhubaneswar" }
                }
                div { class: "flex items-center gap-1.5 text-white/90",
                    CarbonTimeClockIcon { size: 14 }
                    span { "{config.contact.hours}" }
                }
            }
            // Mobile: Ticker marquee infinite scroll with 2 copies of deduplicated string
            div { class: "topbar-ticker-wrap md:hidden",
                div { class: "topbar-ticker-content text-white text-[11px] font-medium",
                    span { class: "inline-flex items-center gap-4 pr-8",
                        span { class: "inline-flex items-center gap-1.5 font-bold",
                            CarbonActivityPulseIcon { size: 13, color: "#fecdd3" }
                            "Emergency: "
                            a { href: "tel:{config.emergency.phone}", class: "underline", "{config.emergency.phone}" }
                        }
                        span { "·" }
                        span { class: "inline-flex items-center gap-1",
                            MapPinIcon { size: 12, color: "currentColor" }
                            "Kalinga Nagar, Bhubaneswar"
                        }
                        span { "·" }
                        span { class: "inline-flex items-center gap-1",
                            CarbonTimeClockIcon { size: 12 }
                            span { "{config.contact.hours}" }
                        }
                        span { "·" }
                    }
                    span { class: "inline-flex items-center gap-4 pr-8",
                        span { class: "inline-flex items-center gap-1.5 font-bold",
                            CarbonActivityPulseIcon { size: 13, color: "#fecdd3" }
                            "Emergency: "
                            a { href: "tel:{config.emergency.phone}", class: "underline", "{config.emergency.phone}" }
                        }
                        span { "·" }
                        span { class: "inline-flex items-center gap-1",
                            MapPinIcon { size: 12, color: "currentColor" }
                            "Kalinga Nagar, Bhubaneswar"
                        }
                        span { "·" }
                        span { class: "inline-flex items-center gap-1",
                            CarbonTimeClockIcon { size: 12 }
                            span { "{config.contact.hours}" }
                        }
                        span { "·" }
                    }
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
        ("About", "#about"),
        ("Services", "#services"),
        ("Departments", "#departments"),
        ("Doctors", "#team"),
        ("Why Us", "#why-us"),
        ("Contact", "#contact"),
    ];

    rsx! {
        nav {
            class: "w-full bg-white/95 dark:bg-slate-900/95 backdrop-blur-md transition-all duration-300",
            "aria-label": "Main navigation",

            div { class: "container-x flex items-center justify-between py-2.5 sm:py-3",

                // Official Brand Logo — Real transparent PNG, zero border, zero bg patch
                a {
                    class: "flex items-center shrink-0 focus:outline-none py-1",
                    href: "#home",
                    "aria-label": "Happy Home Hospital And Rehab Home",
                    img {
                        src: "/assets/images/happy-home-logo-nav.png",
                        alt: "Happy Home Hospital And Rehab",
                        height: "56",
                        class: "h-12 sm:h-14 w-auto object-contain select-none dark:brightness-[1.12] dark:contrast-[1.1]",
                        loading: "eager",
                        decoding: "async"
                    }
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

                // Right controls: Compact Brand toggle, Theme toggle, Appointment CTA, Mobile burger
                div { class: "flex items-center gap-2 sm:gap-2.5",

                    // Compact Brand Switcher (Hospital / Rehab)
                    div {
                        class: "brand-toggle-compact",
                        role: "group",
                        "aria-label": "Toggle Hospital or Rehab mode",

                        button {
                            class: if *brand.read() == BrandMode::Hospital { "brand-toggle-btn active-hosp" } else { "brand-toggle-btn" },
                            onclick: move |_| *brand.write() = BrandMode::Hospital,
                            title: "Hospital Wing",
                            "aria-label": "Switch to Hospital Wing",
                            HospitalIcon { size: 14, color: "currentColor" }
                            span { class: "tooltip", "Hospital Wing" }
                        }
                        button {
                            class: if *brand.read() == BrandMode::Rehab { "brand-toggle-btn active-rehab" } else { "brand-toggle-btn" },
                            onclick: move |_| *brand.write() = BrandMode::Rehab,
                            title: "Rehab Centre",
                            "aria-label": "Switch to Rehab Centre",
                            StethoscopeIcon { size: 14, color: "currentColor" }
                            span { class: "tooltip", "Rehab Centre" }
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
                        class: "btn-solid hidden sm:inline-flex items-center gap-2 px-3.5 sm:px-4 py-2 rounded-xl text-xs font-bold uppercase tracking-wider carbon-btn-hover shadow-md",
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
        }
    }
}

// ── Mobile Navigation Drawer ──────────────────────────────────────────────────
#[component]
fn MobileNavDrawer() -> Element {
    let brand_signal = use_context::<BrandModeSignal>();
    let mut mobile_nav_signal = use_context::<MobileNavSignal>();
    let mut brand = brand_signal.0;
    let is_mobile_open = *mobile_nav_signal.0.read();

    let nav_links: &[(&str, &str)] = &[
        ("Home", "#home"),
        ("About", "#about"),
        ("Services", "#services"),
        ("Departments", "#departments"),
        ("Doctors", "#team"),
        ("Why Us", "#why-us"),
        ("Contact", "#contact"),
    ];

    if !is_mobile_open {
        return rsx! {};
    }

    rsx! {
        div {
            class: "mobile-drawer-backdrop lg:hidden",
            onclick: move |_| *mobile_nav_signal.0.write() = false,
        }
        div {
            class: "mobile-drawer-panel lg:hidden p-6 space-y-6",
            div { class: "flex items-center justify-between pb-4 border-b border-slate-200 dark:border-slate-800",
                img {
                    src: "/assets/images/happy-home-logo-nav.png",
                    alt: "Happy Home Hospital",
                    class: "h-10 w-auto object-contain",
                }
                button {
                    class: "p-2 rounded-xl text-slate-500 hover:text-slate-900 dark:hover:text-white",
                    onclick: move |_| *mobile_nav_signal.0.write() = false,
                    "aria-label": "Close navigation menu",
                    CloseIcon { size: 20, color: "currentColor" }
                }
            }

            // Mobile brand toggle pill
            div { class: "p-1 bg-slate-100 dark:bg-slate-800 rounded-xl flex text-xs font-bold",
                button {
                    class: if *brand.read() == BrandMode::Hospital {
                        "flex-1 py-2 rounded-lg bg-gradient-to-r from-blue-700 to-sky-600 text-white flex items-center justify-center gap-1.5 shadow-sm"
                    } else {
                        "flex-1 py-2 text-slate-600 dark:text-slate-400 flex items-center justify-center gap-1.5"
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
                        "flex-1 py-2 rounded-lg bg-gradient-to-r from-teal-700 to-emerald-600 text-white flex items-center justify-center gap-1.5 shadow-sm"
                    } else {
                        "flex-1 py-2 text-slate-600 dark:text-slate-400 flex items-center justify-center gap-1.5"
                    },
                    onclick: move |_| {
                        *brand.write() = BrandMode::Rehab;
                        *mobile_nav_signal.0.write() = false;
                    },
                    StethoscopeIcon { size: 14, color: "currentColor" }
                    span { "Rehab Centre" }
                }
            }

            // Links
            div { class: "flex flex-col space-y-3 text-sm font-bold uppercase tracking-wider text-slate-700 dark:text-slate-200",
                for (name, href) in nav_links.iter() {
                    a {
                        key: "{name}",
                        href: "{href}",
                        onclick: move |_| *mobile_nav_signal.0.write() = false,
                        class: "py-2 hover:text-sky-600 dark:hover:text-sky-400 transition-colors flex items-center justify-between border-b border-slate-100 dark:border-slate-800/60",
                        span { "{name}" }
                        span { class: "text-slate-400 text-xs", "→" }
                    }
                }
            }

            div { class: "pt-2",
                a {
                    class: "btn-solid w-full text-center py-3 rounded-xl text-xs font-bold uppercase tracking-wider block shadow-md",
                    href: "#appointment",
                    onclick: move |_| *mobile_nav_signal.0.write() = false,
                    "Book Appointment"
                }
            }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    dioxus::launch(App);
}
