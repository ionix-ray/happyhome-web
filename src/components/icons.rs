//! Crisp, accessible SVG icons for Happy Home Hospital & Rehab UI (strictly NO emojis).

use dioxus::prelude::*;

#[component]
pub fn StethoscopeIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            class: "overflow-visible",
            path {
                d: "M4.5 3v5a4 4 0 0 0 8 0V3",
                stroke: "{color}",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle { cx: "4.5", cy: "2.5", r: "1", fill: "{color}" }
            circle { cx: "12.5", cy: "2.5", r: "1", fill: "{color}" }
            path {
                d: "M8.5 10.5v3.5a4 4 0 0 0 8 0v-2",
                stroke: "{color}",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "16.5", cy: "12", r: "3",
                fill: "{color}",
                fill_opacity: "0.15",
                stroke: "{color}",
                stroke_width: "2",
            }
            circle { cx: "16.5", cy: "12", r: "1", fill: "{color}" }
        }
    }
}

#[component]
pub fn HospitalIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            class: "overflow-visible",
            path { d: "M2 22h20", stroke: "{color}", stroke_width: "2", stroke_linecap: "round" }
            path {
                d: "M4 22V5a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v17",
                fill: "{color}",
                fill_opacity: "0.12",
                stroke: "{color}",
                stroke_width: "2",
                stroke_linejoin: "round",
            }
            path { d: "M9 22v-4a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v4", stroke: "{color}", stroke_width: "2" }
            path { d: "M12 7v4m-2-2h4", stroke: "{color}", stroke_width: "2", stroke_linecap: "round" }
            circle { cx: "7.5", cy: "14", r: "1", fill: "{color}" }
            circle { cx: "16.5", cy: "14", r: "1", fill: "{color}" }
        }
    }
}

#[component]
pub fn AmbulanceIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            class: "overflow-visible",
            path {
                d: "M1.5 7a2 2 0 0 1 2-2h11a2 2 0 0 1 2 2v4l3.5 2.5a1.5 1.5 0 0 1 .5 1.1V18a1 1 0 0 1-1 1h-1.5M14 19H8M3.5 19H2a1 1 0 0 1-1-1V7z",
                fill: "{color}",
                fill_opacity: "0.12",
                stroke: "{color}",
                stroke_width: "1.8",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle { cx: "5.5", cy: "19", r: "2.5", stroke: "{color}", stroke_width: "2", fill: "{color}", fill_opacity: "0.2" }
            circle { cx: "16.5", cy: "19", r: "2.5", stroke: "{color}", stroke_width: "2", fill: "{color}", fill_opacity: "0.2" }
            path { d: "M8.5 9v4m-2-2h4", stroke: "{color}", stroke_width: "2", stroke_linecap: "round" }
            path { d: "M8.5 3v1", stroke: "{color}", stroke_width: "2", stroke_linecap: "round" }
        }
    }
}

#[component]
pub fn BrainIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            class: "overflow-visible",
            path {
                d: "M9.5 3A3 3 0 0 0 6.5 6a3.5 3.5 0 0 0-.5 2 3.5 3.5 0 0 0-3 3.5 3.5 3.5 0 0 0 1.5 2.9A4 4 0 0 0 8 20.5h1.5",
                fill: "{color}",
                fill_opacity: "0.14",
                stroke: "{color}",
                stroke_width: "1.8",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M14.5 3a3 3 0 0 1 3 3 3.5 3.5 0 0 1 .5 2 3.5 3.5 0 0 1 3 3.5 3.5 3.5 0 0 1-1.5 2.9A4 4 0 0 1 16 20.5h-1.5",
                fill: "{color}",
                fill_opacity: "0.14",
                stroke: "{color}",
                stroke_width: "1.8",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path { d: "M12 4v16m-2.5-12a2 2 0 0 1 2-2m0 6a2 2 0 0 0 2.5 2", stroke: "{color}", stroke_width: "1.6", stroke_linecap: "round" }
            circle { cx: "7", cy: "10.5", r: "1.2", fill: "{color}" }
            circle { cx: "17", cy: "10.5", r: "1.2", fill: "{color}" }
        }
    }
}

#[component]
pub fn SpineIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            class: "overflow-visible",
            path { d: "M12 2v20", stroke: "{color}", stroke_width: "2", stroke_linecap: "round" }
            ellipse { cx: "12", cy: "5", rx: "4.5", ry: "2", fill: "{color}", fill_opacity: "0.15", stroke: "{color}", stroke_width: "1.8" }
            ellipse { cx: "12", cy: "9.5", rx: "6", ry: "2.2", fill: "{color}", fill_opacity: "0.15", stroke: "{color}", stroke_width: "1.8" }
            ellipse { cx: "12", cy: "14", rx: "6.5", ry: "2.4", fill: "{color}", fill_opacity: "0.15", stroke: "{color}", stroke_width: "1.8" }
            ellipse { cx: "12", cy: "18.5", rx: "5", ry: "2", fill: "{color}", fill_opacity: "0.15", stroke: "{color}", stroke_width: "1.8" }
            circle { cx: "5.5", cy: "9.5", r: "1", fill: "{color}" }
            circle { cx: "18.5", cy: "9.5", r: "1", fill: "{color}" }
            circle { cx: "5", cy: "14", r: "1", fill: "{color}" }
            circle { cx: "19", cy: "14", r: "1", fill: "{color}" }
        }
    }
}

#[component]
pub fn ActivityIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            class: "overflow-visible",
            path {
                d: "M2 12h4.5l2.5-7 4.5 14 3-9 2 4.5H22",
                stroke: "{color}",
                stroke_width: "2.2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle { cx: "11.5", cy: "5", r: "2", fill: "{color}" }
            circle { cx: "16", cy: "19", r: "1.5", fill: "{color}", fill_opacity: "0.6" }
        }
    }
}

#[component]
pub fn ShieldHeartIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            class: "overflow-visible",
            path {
                d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z",
                fill: "{color}",
                fill_opacity: "0.14",
                stroke: "{color}",
                stroke_width: "1.8",
                stroke_linejoin: "round",
            }
            path {
                d: "M12 8.5a2.2 2.2 0 0 0-3.1 0 2.2 2.2 0 0 0 0 3.1l3.1 3.2 3.1-3.2a2.2 2.2 0 0 0 0-3.1 2.2 2.2 0 0 0-3.1 0z",
                stroke: "{color}",
                stroke_width: "1.8",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                fill: "{color}",
                fill_opacity: "0.22",
            }
        }
    }
}

#[component]
pub fn PhoneIcon(
    #[props(default = 20)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07A19.5 19.5 0 0 1 4.69 12 19.79 19.79 0 0 1 1.61 3.44 2 2 0 0 1 3.58 1H6.6a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L7.91 8.6a16 16 0 0 0 6 6l.95-1.03a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" }
        }
    }
}

#[component]
pub fn WhatsAppIcon(
    #[props(default = 20)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "currentColor",
            path { d: "M.057 24l1.687-6.163c-1.041-1.804-1.588-3.849-1.587-5.946.003-6.556 5.338-11.891 11.893-11.891 3.181.001 6.167 1.24 8.413 3.488 2.245 2.248 3.481 5.236 3.48 8.414-.003 6.557-5.338 11.892-11.893 11.892-1.99-.001-3.951-.5-5.688-1.448l-6.305 1.654zm6.597-3.807c1.676.995 3.276 1.591 5.392 1.592 5.448 0 9.886-4.434 9.889-9.885.002-5.462-4.415-9.89-9.881-9.892-5.452 0-9.887 4.434-9.889 9.884-.001 2.225.651 3.891 1.746 5.634l-.999 3.648 3.742-.981zm11.387-5.464c-.074-.124-.272-.198-.57-.347-.297-.149-1.758-.868-2.031-.967-.272-.099-.47-.149-.669.149-.198.297-.768.967-.941 1.165-.173.198-.347.223-.644.074-.297-.149-1.255-.462-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.297-.347.446-.521.151-.172.2-.296.3-.495.099-.198.05-.372-.025-.521-.075-.148-.669-1.611-.916-2.206-.242-.579-.487-.501-.669-.51l-.57-.01c-.198 0-.52.074-.792.372s-1.04 1.016-1.04 2.479 1.065 2.876 1.213 3.074c.149.198 2.095 3.2 5.076 4.487.709.306 1.263.489 1.694.626.712.226 1.36.194 1.872.118.571-.085 1.758-.719 2.006-1.413.248-.695.248-1.29.173-1.414z" }
        }
    }
}

#[component]
pub fn ClockIcon(
    #[props(default = 20)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            class: "overflow-visible",
            circle {
                cx: "12", cy: "12", r: "9.5",
                fill: "{color}",
                fill_opacity: "0.12",
                stroke: "{color}",
                stroke_width: "1.8",
            }
            path {
                d: "M12 6v6l4 2",
                stroke: "{color}",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle { cx: "12", cy: "12", r: "1.5", fill: "{color}" }
            circle { cx: "12", cy: "3.8", r: "0.8", fill: "{color}" }
            circle { cx: "20.2", cy: "12", r: "0.8", fill: "{color}" }
            circle { cx: "12", cy: "20.2", r: "0.8", fill: "{color}" }
            circle { cx: "3.8", cy: "12", r: "0.8", fill: "{color}" }
        }
    }
}

#[component]
pub fn MapPinIcon(
    #[props(default = 20)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z" }
            circle { cx: "12", cy: "10", r: "3" }
        }
    }
}

#[component]
pub fn MailIcon(
    #[props(default = 20)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            rect { width: "20", height: "16", x: "2", y: "4", rx: "2" }
            path { d: "m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" }
        }
    }
}

#[component]
pub fn CheckCircleIcon(
    #[props(default = 20)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M22 11.08V12a10 10 0 1 1-5.93-9.14" }
            polyline { points: "22 4 12 14.01 9 11.01" }
        }
    }
}

#[component]
pub fn StarIcon(
    #[props(default = 16)] size: u32,
    #[props(default = "#f59e0b")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "{color}",
            stroke: "{color}",
            stroke_width: "1",
            polygon { points: "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" }
        }
    }
}

#[component]
pub fn ChevronDownIcon(
    #[props(default = 20)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            polyline { points: "6 9 12 15 18 9" }
        }
    }
}

#[component]
pub fn ArrowRightIcon(
    #[props(default = 18)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "2.5",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M5 12h14" }
            path { d: "m12 5 7 7-7 7" }
        }
    }
}

#[component]
pub fn BedIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            class: "overflow-visible",
            path { d: "M2 4v16M22 10v10", stroke: "{color}", stroke_width: "2", stroke_linecap: "round" }
            path {
                d: "M2 13h18a2 2 0 0 0 2-2v-1a2 2 0 0 0-2-2H8l-6 5z",
                fill: "{color}",
                fill_opacity: "0.15",
                stroke: "{color}",
                stroke_width: "1.8",
                stroke_linejoin: "round",
            }
            circle { cx: "7", cy: "9", r: "1.5", fill: "{color}" }
            path { d: "M2 17h20", stroke: "{color}", stroke_width: "2", stroke_linecap: "round" }
        }
    }
}

#[component]
pub fn WheelchairIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            class: "overflow-visible",
            circle { cx: "12", cy: "4", r: "2", fill: "{color}" }
            path {
                d: "M10 8v5a4 4 0 0 0 4 4h4",
                stroke: "{color}",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path { d: "M10 13h4", stroke: "{color}", stroke_width: "2", stroke_linecap: "round" }
            circle {
                cx: "12", cy: "17", r: "4.5",
                stroke: "{color}",
                stroke_width: "2",
                fill: "{color}",
                fill_opacity: "0.15",
            }
            circle { cx: "12", cy: "17", r: "1.2", fill: "{color}" }
            circle { cx: "18.5", cy: "19.5", r: "1.5", fill: "{color}" }
        }
    }
}

#[component]
pub fn WoundCareIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            class: "overflow-visible",
            circle {
                cx: "12", cy: "12", r: "9.5",
                fill: "{color}",
                fill_opacity: "0.12",
                stroke: "{color}",
                stroke_width: "1.8",
            }
            circle {
                cx: "12", cy: "12", r: "6",
                stroke: "{color}",
                stroke_width: "1.2",
                stroke_dasharray: "3 2",
            }
            path {
                d: "M12 7v10M7 12h10",
                stroke: "{color}",
                stroke_width: "2.2",
                stroke_linecap: "round",
            }
            circle { cx: "12", cy: "12", r: "1.6", fill: "{color}" }
            circle { cx: "8", cy: "8", r: "1", fill: "{color}" }
            circle { cx: "16", cy: "16", r: "1", fill: "{color}" }
        }
    }
}

#[component]
pub fn RespiratoryIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            class: "overflow-visible",
            path {
                d: "M12 2v7m-2-5h4m-2 5l-4 4m4-4l4 4",
                stroke: "{color}",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M7.5 10.5C5 11 3 13 3 16c0 3.5 2 5.5 5 5.5 1.5 0 2.5-.8 3-2.5V11z",
                fill: "{color}",
                fill_opacity: "0.15",
                stroke: "{color}",
                stroke_width: "1.8",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            path {
                d: "M16.5 10.5C19 11 21 13 21 16c0 3.5-2 5.5-5 5.5-1.5 0-2.5-.8-3-2.5V11z",
                fill: "{color}",
                fill_opacity: "0.15",
                stroke: "{color}",
                stroke_width: "1.8",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle { cx: "7", cy: "16", r: "1.2", fill: "{color}" }
            circle { cx: "17", cy: "16", r: "1.2", fill: "{color}" }
        }
    }
}

#[component]
pub fn PalliativeCareIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            class: "overflow-visible",
            path {
                d: "M12 6.5 C10.5 4.5 7.5 4.5 6 6.5 C4.5 8.5 5 11 7 13 L12 17.5 L17 13 C19 11 19.5 8.5 18 6.5 C16.5 4.5 13.5 4.5 12 6.5 Z",
                fill: "{color}",
                fill_opacity: "0.22",
                stroke: "{color}",
                stroke_width: "1.8",
                stroke_linejoin: "round",
            }
            path {
                d: "M2 14c1.5 3 4.5 5.5 8 6.5m12-6.5c-1.5 3-4.5 5.5-8 6.5",
                stroke: "{color}",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            circle { cx: "12", cy: "3", r: "1.5", fill: "{color}" }
        }
    }
}

#[component]
pub fn CarePlanIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            class: "overflow-visible",
            rect {
                x: "4", y: "4", width: "16", height: "18", rx: "3",
                fill: "{color}",
                fill_opacity: "0.12",
                stroke: "{color}",
                stroke_width: "1.8",
            }
            path {
                d: "M9 4V3a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v1",
                stroke: "{color}",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            path { d: "M8 10l1.5 1.5 3-3", stroke: "{color}", stroke_width: "1.8", stroke_linecap: "round", stroke_linejoin: "round" }
            path { d: "M14.5 10h2.5", stroke: "{color}", stroke_width: "1.8", stroke_linecap: "round" }
            path { d: "M8 15l1.5 1.5 3-3", stroke: "{color}", stroke_width: "1.8", stroke_linecap: "round", stroke_linejoin: "round" }
            path { d: "M14.5 15h2.5", stroke: "{color}", stroke_width: "1.8", stroke_linecap: "round" }
        }
    }
}

#[component]
pub fn ParkinsonsIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            class: "overflow-visible",
            path {
                d: "M12 4v6m0 0l-5 4m5-4l5 4m-5 4v6m-5-2l-3 2m13 0l-3-2",
                stroke: "{color}",
                stroke_width: "1.8",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle { cx: "12", cy: "3.5", r: "2", fill: "{color}", stroke: "{color}", stroke_width: "1" }
            circle { cx: "12", cy: "10", r: "2.5", fill: "{color}", fill_opacity: "0.25", stroke: "{color}", stroke_width: "1.5" }
            circle { cx: "6.5", cy: "14.5", r: "2", fill: "{color}" }
            circle { cx: "17.5", cy: "14.5", r: "2", fill: "{color}" }
            circle { cx: "12", cy: "20.5", r: "1.8", fill: "{color}" }
        }
    }
}

#[component]
pub fn SurgeryRehabIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            class: "overflow-visible",
            circle {
                cx: "12", cy: "12", r: "4",
                fill: "{color}",
                fill_opacity: "0.2",
                stroke: "{color}",
                stroke_width: "2",
            }
            path {
                d: "M12 3a9 9 0 0 1 9 9m-18 0a9 9 0 0 1 9-9M12 21a9 9 0 0 1-9-9",
                stroke: "{color}",
                stroke_width: "1.8",
                stroke_linecap: "round",
                stroke_dasharray: "4 2",
            }
            path { d: "M12 5v3m0 8v3M5 12h3m8 0h3", stroke: "{color}", stroke_width: "2", stroke_linecap: "round" }
        }
    }
}

#[component]
pub fn UserIcon(
    #[props(default = 20)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" }
            circle { cx: "12", cy: "7", r: "4" }
        }
    }
}

#[component]
pub fn DynamicIcon(
    #[props(into)] name: String,
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    match name.as_str() {
        "brain" | "tbi" => rsx! { BrainIcon { size, color } },
        "parkinsons" => rsx! { ParkinsonsIcon { size, color } },
        "spine" => rsx! { SpineIcon { size, color } },
        "surgery-rehab" => rsx! { SurgeryRehabIcon { size, color } },
        "ambulance" | "emergency" => rsx! { AmbulanceIcon { size, color } },
        "hospital" | "hospital-full" => rsx! { HospitalIcon { size, color } },
        "activity" | "stroke" | "cardiac" => rsx! { ActivityIcon { size, color } },
        "shield" => rsx! { ShieldHeartIcon { size, color } },
        "wound" => rsx! { WoundCareIcon { size, color } },
        "respiratory" => rsx! { RespiratoryIcon { size, color } },
        "palliative" => rsx! { PalliativeCareIcon { size, color } },
        "plan" => rsx! { CarePlanIcon { size, color } },
        "bed" => rsx! { BedIcon { size, color } },
        "wheelchair" => rsx! { WheelchairIcon { size, color } },
        "clock" | "clock-medical" => rsx! { ClockIcon { size, color } },
        "pin" => rsx! { MapPinIcon { size, color } },
        "phone" => rsx! { PhoneIcon { size, color } },
        _ => rsx! { StethoscopeIcon { size, color } },
    }
}

#[component]
pub fn SunIcon(
    #[props(default = 20)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            circle { cx: "12", cy: "12", r: "4" }
            path { d: "M12 2v2" }
            path { d: "M12 20v2" }
            path { d: "m4.93 4.93 1.41 1.41" }
            path { d: "m17.66 17.66 1.41 1.41" }
            path { d: "M2 12h2" }
            path { d: "M20 12h2" }
            path { d: "m6.34 17.66-1.41 1.41" }
            path { d: "m19.07 4.93-1.41 1.41" }
        }
    }
}

#[component]
pub fn MoonIcon(
    #[props(default = 20)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
        }
    }
}

#[component]
pub fn MenuIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            line { x1: "4", x2: "20", y1: "12", y2: "12" }
            line { x1: "4", x2: "20", y1: "6", y2: "6" }
            line { x1: "4", x2: "20", y1: "18", y2: "18" }
        }
    }
}

#[component]
pub fn CloseIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M18 6 6 18" }
            path { d: "m6 6 12 12" }
        }
    }
}

#[component]
pub fn AlertCircleIcon(
    #[props(default = 20)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            circle { cx: "12", cy: "12", r: "10" }
            line { x1: "12", x2: "12", y1: "8", y2: "12" }
            line { x1: "12", x2: "12.01", y1: "16", y2: "16" }
        }
    }
}

#[component]
pub fn HappyHomeLogo(#[props(default = 42)] height: u32) -> Element {
    let width = height * 320 / 120;
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 320 120",
            width: "{width}",
            height: "{height}",
            fill: "none",
            class: "shrink-0 transition-transform duration-200 hover:scale-[1.02]",

            // Lotus Mandala Yantra Emblem
            g { transform: "translate(60, 60)",
                // Outer 12 Petals
                path { d: "M0 -54 C12 -44 14 -32 0 -22 C-14 -32 -12 -44 0 -54Z", fill: "#36a1d7", opacity: "0.95" }
                path { d: "M0 54 C12 44 14 32 0 22 C-14 32 -12 44 0 54Z", fill: "#134cb3", opacity: "0.95" }
                path { d: "M54 0 C44 12 32 14 22 0 C32 -14 44 -12 54 0Z", fill: "#2563eb", opacity: "0.95" }
                path { d: "M-54 0 C-44 12 -32 14 -22 0 C-32 -14 -44 -12 -54 0Z", fill: "#2563eb", opacity: "0.95" }

                g { transform: "rotate(30)",
                    path { d: "M0 -54 C12 -44 14 -32 0 -22 C-14 -32 -12 -44 0 -54Z", fill: "#36a1d7", opacity: "0.9" }
                    path { d: "M0 54 C12 44 14 32 0 22 C-14 32 -12 44 0 54Z", fill: "#134cb3", opacity: "0.9" }
                    path { d: "M54 0 C44 12 32 14 22 0 C32 -14 44 -12 54 0Z", fill: "#2563eb", opacity: "0.9" }
                    path { d: "M-54 0 C-44 12 -32 14 -22 0 C-32 -14 -44 -12 -54 0Z", fill: "#2563eb", opacity: "0.9" }
                }

                g { transform: "rotate(60)",
                    path { d: "M0 -54 C12 -44 14 -32 0 -22 C-14 -32 -12 -44 0 -54Z", fill: "#36a1d7", opacity: "0.9" }
                    path { d: "M0 54 C12 44 14 32 0 22 C-14 32 -12 44 0 54Z", fill: "#134cb3", opacity: "0.9" }
                    path { d: "M54 0 C44 12 32 14 22 0 C32 -14 44 -12 54 0Z", fill: "#2563eb", opacity: "0.9" }
                    path { d: "M-54 0 C-44 12 -32 14 -22 0 C-32 -14 -44 -12 -54 0Z", fill: "#2563eb", opacity: "0.9" }
                }

                // Concentric circles
                circle { cx: "0", cy: "0", r: "34", stroke: "#2563eb", stroke_width: "2.5", fill: "none" }
                circle { cx: "0", cy: "0", r: "24", stroke: "#36a1d7", stroke_width: "1.5", fill: "none" }

                // Shatkona / Healing Yantra
                polygon { points: "0,-22 19,11 -19,11", stroke: "#36a1d7", stroke_width: "1.8", fill: "none" }
                polygon { points: "0,22 19,-11 -19,-11", stroke: "#36a1d7", stroke_width: "1.8", fill: "none" }

                // Central bindu
                circle { cx: "0", cy: "0", r: "4.5", fill: "#134cb3" }

                // 5 Chakra beads
                circle { cx: "0", cy: "-29", r: "3", fill: "#38bdf8" }
                circle { cx: "-27.5", cy: "-9", r: "3", fill: "#10b981" }
                circle { cx: "27.5", cy: "-9", r: "3", fill: "#f59e0b" }
                circle { cx: "-17", cy: "23.5", r: "3", fill: "#ea580c" }
                circle { cx: "17", cy: "23.5", r: "3", fill: "#a855f7" }
            }

            // Typography
            g {
                // HAPPY
                text {
                    x: "130", y: "44",
                    font_family: "'Plus Jakarta Sans', system-ui, sans-serif",
                    font_weight: "800",
                    font_size: "34",
                    letter_spacing: "1.5",
                    fill: "#2d8f7b",
                    "HAPPY"
                }

                // HOME
                text {
                    x: "130", y: "78",
                    font_family: "'Plus Jakarta Sans', system-ui, sans-serif",
                    font_weight: "800",
                    font_size: "36",
                    letter_spacing: "1.5",
                    fill: "#36a1d7",
                    "HOME"
                }

                // HOSPITAL (dynamic theme class)
                text {
                    x: "132", y: "97",
                    font_family: "'Plus Jakarta Sans', system-ui, sans-serif",
                    font_weight: "800",
                    font_size: "14.5",
                    letter_spacing: "5",
                    class: "fill-blue-900 dark:fill-white font-extrabold",
                    "HOSPITAL"
                }

                // Slogan
                g { transform: "translate(130, 108)",
                    line { x1: "2", y1: "-2", x2: "22", y2: "-2", stroke: "#36a1d7", stroke_width: "1.5" }
                    circle { cx: "27", cy: "-2", r: "2.5", fill: "#ea580c" }
                    text {
                        x: "35", y: "1",
                        font_family: "'Plus Jakarta Sans', system-ui, sans-serif",
                        font_weight: "700",
                        font_size: "8.5",
                        letter_spacing: "1.2",
                        fill: "#36a1d7",
                        "WHERE HEALING BEGINS"
                    }
                    circle { cx: "163", cy: "-2", r: "2.5", fill: "#ea580c" }
                    line { x1: "168", y1: "-2", x2: "188", y2: "-2", stroke: "#36a1d7", stroke_width: "1.5" }
                }
            }
        }
    }
}

// ── Carbon Design System Animated Icons ──────────────────────────────────────

#[component]
pub fn CarbonActivityPulseIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "var(--c-brand)")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 32 32",
            fill: "none",
            class: "overflow-visible",
            path {
                d: "M2 16h6l4-10 6 20 4-12 3 6h5",
                stroke: "{color}",
                stroke_width: "2.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                class: "animate-ecg-line",
            }
            circle {
                cx: "18", cy: "26", r: "2",
                fill: "{color}",
                class: "animate-pulse",
            }
        }
    }
}

#[component]
pub fn CarbonHeartHealthIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "#f43f5e")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 32 32",
            fill: "none",
            class: "animate-heartbeat shrink-0 inline-block",
            // Radiant cardiac heart chamber in soft rose
            path {
                d: "M16 28 C14 26 4 18 4 11 A7 7 0 0 1 16 7 A7 7 0 0 1 28 11 C28 18 18 26 16 28 Z",
                fill: "{color}",
                fill_opacity: "0.18",
                stroke: "{color}",
                stroke_width: "2",
                stroke_linejoin: "round",
            }
            // Precision ECG waveform pulse across the cardiac chamber
            path {
                d: "M9 16h3.5l2-5 3.5 10 2-5h3",
                stroke: "#38bdf8",
                stroke_width: "2.2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            // Vital rhythm indicator pulse node
            circle { cx: "16", cy: "16", r: "1.5", fill: "#fbbf24" }
        }
    }
}

#[component]
pub fn CarbonShieldCheckIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "var(--c-brand-teal)")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 32 32",
            fill: "none",
            path {
                d: "M16 3L5 7v8c0 7.5 4.7 13.3 11 15 6.3-1.7 11-7.5 11-15V7L16 3z",
                fill: "{color}",
                fill_opacity: "0.12",
                stroke: "{color}",
                stroke_width: "2",
                stroke_linejoin: "round",
            }
            path {
                d: "M11 16l3.5 3.5 7-7",
                stroke: "{color}",
                stroke_width: "2.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        }
    }
}

#[component]
pub fn CarbonHealingSparkIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "var(--c-brand)")] color: &'static str,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 32 32",
            fill: "{color}",
            class: "animate-float-1",
            path {
                d: "M16 2 C16 9 23 16 30 16 C23 16 16 23 16 30 C16 23 9 16 2 16 C9 16 16 9 16 2 Z",
            }
        }
    }
}

/// IBM Carbon Renew / Surgical Post-Op Rehabilitation Cycle — Multi-colour
#[component]
pub fn CarbonRenewIcon(
    #[props(default = 24)] size: u32,
    #[props(default = "currentColor")] color: &'static str,
) -> Element {
    let _ = color;
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 32 32",
            fill: "none",
            class: "shrink-0 inline-block",
            // Top renewal arc in Coastal Cyan
            path {
                d: "M26 12A12 12 0 0 0 7.5 8.5L4 12",
                stroke: "#38bdf8",
                stroke_width: "2.5",
                stroke_linecap: "round",
            }
            path {
                d: "M4 4v8h8",
                stroke: "#0284c7",
                stroke_width: "2.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            // Bottom recovery arc in Emerald Teal
            path {
                d: "M6 20a12 12 0 0 0 18.5 3.5L28 20",
                stroke: "#34d399",
                stroke_width: "2.5",
                stroke_linecap: "round",
            }
            path {
                d: "M28 28v-8h-8",
                stroke: "#059669",
                stroke_width: "2.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            // Central rehabilitation balance node in Warm Amber
            circle { cx: "16", cy: "16", r: "2.5", fill: "#f59e0b" }
        }
    }
}

/// IBM Carbon Alarm / Siren — Multi-colour (Emergency ICU)
#[component]
pub fn CarbonEmergencySirenIcon(#[props(default = 16)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 32 32",
            fill: "none",
            class: "shrink-0 inline-block",
            // Base stand in sky cyan
            path { d: "M10 28h12v2H10z", fill: "#38bdf8" }
            // Main siren dome in vivid ruby red
            path {
                d: "M16 4a9 9 0 0 0-9 9v7h18v-7a9 9 0 0 0-9-9zm-7 14v-5a7 7 0 0 1 14 0v5H9z",
                fill: "#ef4444",
            }
            // Warning flash rays in warm amber
            rect { x: "15", y: "0", width: "2", height: "3", rx: "1", fill: "#f59e0b" }
            rect { x: "4", y: "7", width: "2", height: "3", rx: "1", transform: "rotate(-45 5 8.5)", fill: "#f59e0b" }
            rect { x: "26", y: "7", width: "2", height: "3", rx: "1", transform: "rotate(45 27 8.5)", fill: "#f59e0b" }
            // Inner core beacon glow
            circle { cx: "16", cy: "15", r: "3", fill: "#fca5a5" }
        }
    }
}

/// IBM Carbon Hospital Bed — Multi-colour (Dedicated Inpatient Beds)
#[component]
pub fn CarbonHospitalBedIcon(#[props(default = 16)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 32 32",
            fill: "none",
            class: "shrink-0 inline-block",
            // Bed frame & posts in deep ocean teal
            path {
                d: "M25 16H17a2.002 2.002 0 0 0-2 2v6H4V14H2v16h2v-4h24v4h2V21a5.006 5.006 0 0 0-5-5zm3 8H17v-6h8a3.003 3.003 0 0 1 3 3z",
                fill: "#0d9488",
            }
            // Patient head / pillow in vibrant sky cyan
            circle { cx: "9.5", cy: "18.5", r: "2.5", fill: "#38bdf8" }
            path { d: "M9.5 15a3.5 3.5 0 1 0 3.5 3.5A3.5 3.5 0 0 0 9.5 15zm0 5a1.5 1.5 0 1 1 1.5-1.5 1.502 1.502 0 0 1-1.5 1.5z", fill: "#0284c7" }
            // Medical cross above bed in vibrant coral rose
            polygon {
                points: "21 6 17 6 17 2 15 2 15 6 11 6 11 8 15 8 15 12 17 12 17 8 21 8 21 6",
                fill: "#f43f5e",
            }
        }
    }
}

/// IBM Carbon Chart Line Trend — Multi-colour (Patient Recovery Rate)
#[component]
pub fn CarbonChartTrendIcon(#[props(default = 16)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 32 32",
            fill: "none",
            class: "shrink-0 inline-block",
            // Grid axes in indigo
            path { d: "M2 2v26a2 2 0 0 0 2 2h26v-2H4V2H2z", fill: "#6366f1" }
            // Dynamic upward recovery curve in bright emerald
            path {
                d: "M4.67 28l6.39-12 7.3 6.49a2 2 0 0 0 1.7.47 2 2 0 0 0 1.42-1.07L27 10.9 25.18 10 19.69 21l-7.3-6.49a2 2 0 0 0-1.68-.51 2 2 0 0 0-1.42 1L4 25v3h.67z",
                fill: "#10b981",
            }
            // Peak achievement star / node in bright amber
            circle { cx: "26.5", cy: "10.5", r: "2.5", fill: "#f59e0b" }
        }
    }
}

/// IBM Carbon Shield Quality / Security Check — Multi-colour (NABH Quality Norms)
#[component]
pub fn CarbonShieldQualityIcon(#[props(default = 16)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 32 32",
            fill: "none",
            class: "shrink-0 inline-block",
            // Shield perimeter in royal sapphire blue
            path {
                d: "M16 30L9.82 26.71A10.98 10.98 0 0 1 4 17V4a2.002 2.002 0 0 1 2-2h20a2.002 2.002 0 0 1 2 2v17a10.98 10.98 0 0 1-5.82 9.71zM6 4v13a8.99 8.99 0 0 0 4.77 7.94L16 27.73l5.23-2.79A8.99 8.99 0 0 0 26 17V4H6z",
                fill: "#2563eb",
            }
            // Soft protective shield body fill
            path {
                d: "M6 4v13a8.99 8.99 0 0 0 4.77 7.94L16 27.73l5.23-2.79A8.99 8.99 0 0 0 26 17V4H6z",
                fill: "#3b82f6",
                fill_opacity: "0.18",
            }
            // Certified checkmark in vibrant emerald
            polygon {
                points: "14 16.59 11.41 14 10 15.41 14 19.41 22 11.41 20.59 10 14 16.59",
                fill: "#10b981",
            }
        }
    }
}

/// IBM Carbon Cognitive / Neural Brain — Multi-colour (Specialized Neuro & Spine Rehab)
#[component]
pub fn CarbonCognitiveIcon(#[props(default = 16)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 32 32",
            fill: "none",
            class: "shrink-0 inline-block",
            // Brain neural contours in violet/purple
            path {
                d: "M30 13a11 11 0 0 0-11-11h-8a9 9 0 0 0-9 9v3a5 5 0 0 0 5 5h1.1A5 5 0 0 0 13 23h1.38l4 7 1.73-1-4-6.89A2 2 0 0 0 14.38 21H13a3 3 0 0 1 0-6h1v-2h-1a5 5 0 0 0-4.9 4H7a3 3 0 0 1-3-3v-1h2a3 3 0 0 0 3-3V8H7v1a1 1 0 0 1-1 1H4.08A7 7 0 0 1 11 4h6v2a1 1 0 0 1-1 1h-2v2h2a3 3 0 0 0 3-3V4a9 9 0 0 1 8.05 5H26a3 3 0 0 0-3 3v1h2v-1a1 1 0 0 1 1-1h1.77A8.76 8.76 0 0 1 28 13v1a5 5 0 0 1-5 5h-3v2h3a7 7 0 0 0 3-.68V21a3 3 0 0 1-3 3h-1v2h1a5 5 0 0 0 5-5v-2.11A7 7 0 0 0 30 14z",
                fill: "#8b5cf6",
            }
            // Synapse node sparks in bright electric cyan
            circle { cx: "16", cy: "11", r: "2", fill: "#06b6d4" }
            circle { cx: "21", cy: "14", r: "1.5", fill: "#38bdf8" }
            circle { cx: "10", cy: "14", r: "1.5", fill: "#a78bfa" }
        }
    }
}

/// IBM Carbon Partnership / Collaborative Care — Multi-colour (1:1 Daily Physiotherapy)
#[component]
pub fn CarbonPartnershipIcon(#[props(default = 16)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 32 32",
            fill: "none",
            class: "shrink-0 inline-block",
            // Patient head in cyan
            circle { cx: "8", cy: "5", r: "3", fill: "#0284c7" }
            circle { cx: "8", cy: "5", r: "1.5", fill: "#bae6fd" }
            // Therapist head in emerald teal
            circle { cx: "24", cy: "5", r: "3", fill: "#0d9488" }
            circle { cx: "24", cy: "5", r: "1.5", fill: "#99f6e4" }
            // Supporting therapeutic handshake & link in warm amber + teal
            path {
                d: "M26 30h-4a2 2 0 0 1-2-2v-7h2v7h4v-9h2v-6a1 1 0 0 0-1-1h-2.42L16 20l-4.58-8H5a1 1 0 0 0-1 1v6h2v9h4v-7h2v7a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-7a2 2 0 0 1-2-2v-6a3 3 0 0 1 3-3h7.58L16 16l3.42-6H27a3 3 0 0 1 3 3v6a2 2 0 0 1-2 2v7a2 2 0 0 1-2 2z",
                fill: "#0891b2",
            }
            // Clasp clasp node in warm amber
            circle { cx: "16", cy: "17", r: "2", fill: "#f59e0b" }
        }
    }
}

/// IBM Carbon Smiles & Healing Heart — Multi-colour (1,000+ Smiles Restored)
#[component]
pub fn CarbonSmilesHeartIcon(#[props(default = 16)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 32 32",
            fill: "none",
            class: "shrink-0 inline-block",
            // Radiant healing heart in rose
            path {
                d: "M16 29s-12-7.5-12-16a7 7 0 0 1 12-4.5A7 7 0 0 1 28 13c0 8.5-12 16-12 16z",
                fill: "#f43f5e",
            }
            // Inner smile curve in golden sunshine amber
            path {
                d: "M11 16c1.2 2.5 3.3 3.5 5 3.5s3.8-1 5-3.5",
                stroke: "#fef08a",
                stroke_width: "2",
                stroke_linecap: "round",
            }
            // Eyes of the smile
            circle { cx: "12.5", cy: "13.5", r: "1.2", fill: "#ffffff" }
            circle { cx: "19.5", cy: "13.5", r: "1.2", fill: "#ffffff" }
        }
    }
}

/// IBM Carbon Time & Clock — Multi-colour (OPD & Clinic Hours)
#[component]
pub fn CarbonTimeClockIcon(#[props(default = 14)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 32 32",
            fill: "none",
            class: "shrink-0 inline-block",
            // Top stopwatch button in sky cyan
            rect { x: "13", y: "2", width: "6", height: "2", rx: "1", fill: "#38bdf8" }
            // Clock outer dial ring in brilliant sky blue
            path {
                d: "M28 9l-1.42-1.41-2.25 2.25a10.94 10.94 0 1 0 1.18 1.65zM16 26a9 9 0 1 1 9-9 9 9 0 0 1-9 9z",
                fill: "#38bdf8",
            }
            // Dial center glow
            circle { cx: "16", cy: "17", r: "7", fill: "#0284c7", fill_opacity: "0.2" }
            // Clock hands pointing in golden amber
            rect { x: "15", y: "11", width: "2", height: "7", rx: "1", fill: "#f59e0b" }
            rect { x: "16", y: "16", width: "5", height: "2", rx: "1", fill: "#f59e0b" }
            circle { cx: "16", cy: "17", r: "1.5", fill: "#ffffff" }
        }
    }
}

/// IBM Carbon Spinal Column — Multi-colour (Spinal Cord Injury Rehab)
#[component]
pub fn CarbonSpineIcon(#[props(default = 16)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 32 32",
            fill: "none",
            class: "shrink-0 inline-block",
            // Nerve roots in warm amber
            path { d: "M11 5.5H6M11 13H5.5M11 20.5H6M11 27.5H6.5", stroke: "#f59e0b", stroke_width: "1.75", stroke_linecap: "round" }
            path { d: "M21 5.5h5M21 13h5.5M21 20.5h5M21 27.5h4.5", stroke: "#f59e0b", stroke_width: "1.75", stroke_linecap: "round" }
            // Vertebral bodies alternating indigo / violet
            rect { x: "12", y: "3", width: "8", height: "5", rx: "2", fill: "#6366f1" }
            rect { x: "12", y: "10.5", width: "8", height: "5", rx: "2", fill: "#818cf8" }
            rect { x: "12", y: "18", width: "8", height: "5", rx: "2", fill: "#6366f1" }
            rect { x: "12", y: "25.5", width: "8", height: "4.5", rx: "2", fill: "#818cf8" }
            // Intervertebral discs in sky cyan
            rect { x: "13", y: "8.3", width: "6", height: "1.9", rx: "0.95", fill: "#38bdf8" }
            rect { x: "13", y: "15.8", width: "6", height: "1.9", rx: "0.95", fill: "#38bdf8" }
            rect { x: "13", y: "23.3", width: "6", height: "1.9", rx: "0.95", fill: "#38bdf8" }
        }
    }
}

/// IBM Carbon Lungs — Multi-colour (Tracheostomy & Respiratory Support)
#[component]
pub fn CarbonLungsIcon(#[props(default = 16)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 32 32",
            fill: "none",
            class: "shrink-0 inline-block",
            // Air particles in warm amber
            circle { cx: "6.5", cy: "9", r: "1.5", fill: "#f59e0b" }
            circle { cx: "25.5", cy: "7.5", r: "1.2", fill: "#fbbf24" }
            // Trachea + bronchi in emerald teal
            path {
                d: "M16 4v7M16 11l-3.5 4M16 11l3.5 4",
                stroke: "#14b8a6",
                stroke_width: "3",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            // Left lobe in sky cyan
            path {
                d: "M12.5 15C9 15 5.5 18 4.5 22.5C4 25 5.7 27.2 8 27.2C11.5 27.2 14 24 14 20.3V15Z",
                fill: "#38bdf8",
            }
            // Right lobe in deep sky blue
            path {
                d: "M19.5 15C23 15 26.5 18 27.5 22.5C28 25 26.3 27.2 24 27.2C20.5 27.2 18 24 18 20.3V15Z",
                fill: "#0ea5e9",
            }
        }
    }
}

/// IBM Carbon Adhesive Bandage — Multi-colour (Bedsore & Wound Care)
#[component]
pub fn CarbonWoundCareIcon(#[props(default = 16)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 32 32",
            fill: "none",
            class: "shrink-0 inline-block",
            // Strips rotated into a tilted adhesive bandage
            rect { x: "2", y: "12", width: "28", height: "8", rx: "4", fill: "#f59e0b", transform: "rotate(-35 16 16)" }
            // Sterile pad in cream
            rect { x: "11", y: "13.5", width: "10", height: "5", rx: "1.5", fill: "#fef3c7", transform: "rotate(-35 16 16)" }
            // Medical cross in rose, rotating with the strip
            rect { x: "14", y: "15.25", width: "4", height: "1.5", rx: "0.75", fill: "#f43f5e", transform: "rotate(-35 16 16)" }
            rect { x: "15.25", y: "14", width: "1.5", height: "4", rx: "0.75", fill: "#f43f5e", transform: "rotate(-35 16 16)" }
        }
    }
}

/// IBM Carbon Tremor Hand — Multi-colour (Parkinson's & Dementia Rehab)
#[component]
pub fn CarbonTremorHandIcon(#[props(default = 16)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 32 32",
            fill: "none",
            class: "shrink-0 inline-block",
            // Tremor motion arcs in sky cyan
            path { d: "M26.5 10a5 5 0 0 1 0 8", stroke: "#38bdf8", stroke_width: "2", stroke_linecap: "round", fill: "none" }
            path { d: "M29 7.5a9 9 0 0 1 0 13", stroke: "#7dd3fc", stroke_width: "2", stroke_linecap: "round", fill: "none" }
            // Thumb in violet
            rect { x: "4", y: "17", width: "6", height: "3.5", rx: "1.75", fill: "#a78bfa", transform: "rotate(-30 7 18.75)" }
            // Fingers alternating violet / indigo
            rect { x: "9", y: "6", width: "3", height: "11", rx: "1.5", fill: "#a78bfa" }
            rect { x: "13", y: "4", width: "3", height: "13", rx: "1.5", fill: "#818cf8" }
            rect { x: "17", y: "5", width: "3", height: "12", rx: "1.5", fill: "#a78bfa" }
            rect { x: "21", y: "8", width: "3", height: "9", rx: "1.5", fill: "#818cf8" }
            // Palm in deep indigo
            rect { x: "8", y: "15", width: "17", height: "13", rx: "5", fill: "#6366f1" }
        }
    }
}

/// IBM Carbon Stroke Brain — Multi-colour (Stroke Recovery & Paralysis)
#[component]
pub fn CarbonStrokeBrainIcon(#[props(default = 16)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 32 32",
            fill: "none",
            class: "shrink-0 inline-block",
            // Left hemisphere in soft rose
            path {
                d: "M15 6C11 6 8 9 8 12.5C5 14 4 17 5 19.5C4.2 22 5.5 24.5 7.5 25.5C8.4 27.7 10.5 29 13 29C14.2 29 15 28.5 15 28V6Z",
                fill: "#fb7185",
            }
            // Right hemisphere in vivid rose
            path {
                d: "M17 6C21 6 24 9 24 12.5C27 14 28 17 27 19.5C27.8 22 26.5 24.5 24.5 25.5C23.6 27.7 21.5 29 19 29C17.8 29 17 28.5 17 28V6Z",
                fill: "#f43f5e",
            }
            // Central fissure in blush
            path { d: "M16 6V29", stroke: "#fecdd3", stroke_width: "1.5", stroke_linecap: "round" }
            // Gyri folds in blush
            path { d: "M11 9.5c-1.8 1.6-1.8 3.9 0 5.5", stroke: "#ffe4e6", stroke_width: "1.5", stroke_linecap: "round", fill: "none" }
            path { d: "M21 9.5c1.8 1.6 1.8 3.9 0 5.5", stroke: "#ffe4e6", stroke_width: "1.5", stroke_linecap: "round", fill: "none" }
            // Stroke lightning bolt in warm amber
            path {
                d: "M18.5 10.5L13.5 18.5H17L15.5 24.5L21 16H17.5Z",
                fill: "#f59e0b",
                stroke: "#fef3c7",
                stroke_width: "0.75",
            }
        }
    }
}

/// IBM Carbon Vitals Monitor — Multi-colour (Critical Care & HDU)
#[component]
pub fn CarbonVitalsMonitorIcon(#[props(default = 16)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 32 32",
            fill: "none",
            class: "shrink-0 inline-block",
            // Stand in muted slate
            rect { x: "13", y: "26", width: "6", height: "3", rx: "1", fill: "#64748b" }
            rect { x: "9", y: "29", width: "14", height: "2.5", rx: "1.25", fill: "#94a3b8" }
            // Monitor frame in deep slate
            rect { x: "3", y: "4", width: "26", height: "23", rx: "3", fill: "#334155" }
            // Screen glass in white
            rect { x: "5", y: "6", width: "22", height: "15", rx: "2", fill: "#f8fafc" }
            // ECG waveform in emerald
            path {
                d: "M7 14H10.5L12.5 9.5L15.5 18.5L17.5 14H25",
                stroke: "#10b981",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                fill: "none",
            }
            // Control bars + alarm LED
            rect { x: "5", y: "22.5", width: "6", height: "2.5", rx: "1.25", fill: "#38bdf8" }
            rect { x: "13", y: "22.5", width: "5", height: "2.5", rx: "1.25", fill: "#10b981" }
            circle { cx: "24", cy: "23.75", r: "1.75", fill: "#f59e0b" }
        }
    }
}

/// IBM Carbon Laboratory Flask — Multi-colour (Diagnostics & Laboratory)
#[component]
pub fn CarbonLabFlaskIcon(#[props(default = 16)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            width: "{size}", height: "{size}",
            view_box: "0 0 32 32",
            fill: "none",
            class: "shrink-0 inline-block",
            // Evaporating sample dots in amber / rose
            circle { cx: "25", cy: "8", r: "1.5", fill: "#f59e0b" }
            circle { cx: "27.5", cy: "12.5", r: "1", fill: "#f43f5e" }
            // Reagent liquid in emerald
            path {
                d: "M11.6 21.5H20.4L22.3 25.5A3 3 0 0 1 20 30H12A3 3 0 0 1 9.6 25.5L11.6 21.5Z",
                fill: "#10b981",
            }
            // Bubbles inside the liquid
            circle { cx: "13", cy: "25", r: "1.6", fill: "#fef08a" }
            circle { cx: "17.5", cy: "26.5", r: "1.2", fill: "#ffffff" }
            circle { cx: "15.5", cy: "23", r: "1", fill: "#f59e0b" }
            // Glass outline in sky cyan
            path {
                d: "M12 4H20M14.5 4V15L9.4 25.6A3 3 0 0 0 12 30H20A3 3 0 0 0 22.6 25.6L17.5 15V4",
                stroke: "#38bdf8",
                stroke_width: "2.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                fill: "none",
            }
        }
    }
}

/// Multi-colour Carbon icon for a speciality card, keyed by SiteConfig slug.
/// Every single one of the 15 specialties across Hospital and Rehab wings
/// has a unique, distinct IBM Carbon Design clinical SVG glyph.
#[component]
pub fn CarbonServiceIcon(slug: String, #[props(default = 20)] size: u32) -> Element {
    match slug.as_str() {
        // Hospital wing (6 specialties)
        "emergency-trauma" => rsx! { CarbonEmergencySirenIcon { size } },
        "critical-care-hdu" => rsx! { CarbonVitalsMonitorIcon { size } },
        "multispecialty-opd" => rsx! { CarbonTimeClockIcon { size } },
        "diagnostic-laboratory" => rsx! { CarbonLabFlaskIcon { size } },
        "advanced-physiotherapy" => rsx! { CarbonPartnershipIcon { size } },
        "oncology-palliative" => rsx! { CarbonSmilesHeartIcon { size } },
        // Rehab wing (9 programs)
        "post-neuro-surgery-rehabilitation" => rsx! { CarbonHospitalBedIcon { size } },
        "spinal-cord-injury-rehabilitation" => rsx! { CarbonSpineIcon { size } },
        "traumatic-brain-injury-tbi-care" => rsx! { CarbonCognitiveIcon { size } },
        "advanced-bedsore-wound-care" => rsx! { CarbonWoundCareIcon { size } },
        "tracheostomy-respiratory-support-care" => rsx! { CarbonLungsIcon { size } },
        "cancer-palliative-critical-care-management" => rsx! { CarbonHeartHealthIcon { size } },
        "parkinsons-disease-dementia-rehabilitation" => rsx! { CarbonTremorHandIcon { size } },
        "stroke-recovery-paralysis-management" => rsx! { CarbonStrokeBrainIcon { size } },
        "post-general-abdominal-orthopedic-ctvs-surgery-rehabilitation" => {
            rsx! { CarbonRenewIcon { size } }
        }
        _ => rsx! { CarbonShieldQualityIcon { size } },
    }
}

/// Multi-colour Carbon icon for Why Us cards, keyed by icon_key.
#[component]
pub fn CarbonWhyUsIcon(icon_key: String, #[props(default = 20)] size: u32) -> Element {
    match icon_key.as_str() {
        "ambulance" | "emergency" => rsx! { CarbonEmergencySirenIcon { size } },
        "shield" | "quality" => rsx! { CarbonShieldQualityIcon { size } },
        "hospital-full" | "hospital" => rsx! { CarbonHospitalBedIcon { size } },
        "plan" | "chart" => rsx! { CarbonChartTrendIcon { size } },
        "clock-medical" | "clock" => rsx! { CarbonTimeClockIcon { size } },
        "wound" => rsx! { CarbonWoundCareIcon { size } },
        _ => rsx! { CarbonShieldCheckIcon { size } },
    }
}
