//! Global state management for Happy Home Hospital & Rehab WASM application.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BrandMode {
    #[default]
    Hospital,
    Rehab,
}

impl BrandMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Hospital => "hospital",
            Self::Rehab => "rehab",
        }
    }

    #[allow(dead_code)]
    pub fn toggle(&self) -> Self {
        match self {
            Self::Hospital => Self::Rehab,
            Self::Rehab => Self::Hospital,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}

#[derive(Clone, Copy)]
pub struct BrandModeSignal(pub Signal<BrandMode>);

#[derive(Clone, Copy)]
pub struct ThemeSignal(pub Signal<Theme>);

#[derive(Clone, Copy)]
pub struct MobileNavSignal(pub Signal<bool>);

#[derive(Clone, Copy)]
pub struct ModalDoctorSignal(pub Signal<Option<String>>);

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct ActiveSectionSignal(pub Signal<String>);

#[derive(Clone, Copy)]
pub struct ToastSignal(pub Signal<Option<String>>);

/// Initializes global signals and provides them as context.
pub fn init_global_state() -> (
    BrandModeSignal,
    ThemeSignal,
    MobileNavSignal,
    ModalDoctorSignal,
    ActiveSectionSignal,
    ToastSignal,
) {
    let brand_mode = BrandModeSignal(use_signal(|| BrandMode::Hospital));
    let theme = ThemeSignal(use_signal(|| Theme::Light));
    let mobile_nav = MobileNavSignal(use_signal(|| false));
    let modal_doctor = ModalDoctorSignal(use_signal(|| None));
    let active_section = ActiveSectionSignal(use_signal(|| "home".to_string()));
    let toast = ToastSignal(use_signal(|| None));

    use_context_provider(|| brand_mode);
    use_context_provider(|| theme);
    use_context_provider(|| mobile_nav);
    use_context_provider(|| modal_doctor);
    use_context_provider(|| active_section);
    use_context_provider(|| toast);

    (
        brand_mode,
        theme,
        mobile_nav,
        modal_doctor,
        active_section,
        toast,
    )
}
