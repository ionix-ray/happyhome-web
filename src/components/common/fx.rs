//! Motion primitives — the vayudh.com entrance trio (Wix `motion-revealIn`,
//! `motion-floatIn`, `motion-fadeIn`) plus a masked word rise, orchestrated
//! from Rust.
//!
//! `assets/styles.css` section 8 owns four keyframes and two drivers:
//! * **time driver** (`.fx-time`) — the hero. Above the fold, a scroll
//!   timeline reports "already entered" at load, so entrance must play once
//!   on first paint, staggered by `animation-delay`.
//! * **scroll driver** (`.sec-head`) — below-fold section headings, in
//!   lockstep on one named `view-timeline`; stagger comes from
//!   `animation-range` offsets because `animation-delay` is inert on a
//!   scroll-driven timeline.
//!
//! Choreography lives here: every element picks a motion and its slot in
//! the cascade (`--i`). Both drivers consume the same ordinal — as a delay
//! or as a range offset — so the Rust side never needs to know which
//! timeline a class will run on.

use dioxus::prelude::*;

/// Entrance motion vocabulary.
#[derive(Clone, Copy)]
pub enum Fx {
    /// `motion-revealIn` — hard-edge clip wipe; opacity pinned at 1.
    Reveal,
    /// `motion-floatIn` — fade + gentle rise.
    Float,
    /// `motion-fadeIn` — pure opacity.
    Fade,
}

impl Fx {
    /// CSS hook for the motion (see `assets/styles.css` section 8).
    pub fn class(self) -> &'static str {
        match self {
            Fx::Reveal => "fx-wipe",
            Fx::Float => "fx-float",
            Fx::Fade => "fx-fade",
        }
    }

    /// Pair the motion with its cascade slot. Returns `(class, style)` ready
    /// for `class:` / `style:` attributes; `--i` is consumed by the driver
    /// as `calc(var(--i) * 70ms)` (time) or an `animation-range` offset
    /// (scroll).
    pub fn at(self, step: u32) -> (&'static str, String) {
        (self.class(), format!("--i: {step};"))
    }
}

/// Split on whitespace, dropping empty runs so mask spans never carry
/// leading/trailing spaces that would offset the cascade.
pub fn split_words(input: &str) -> Vec<String> {
    input.split_whitespace().map(str::to_string).collect()
}

/// Ordinals for heading copy: title words then highlight words, each with
/// its global position in the cascade plus a highlight flag. Pure so the
/// stagger contract is testable without a renderer.
pub fn cascade_words(main: &str, highlight: &str) -> Vec<(usize, String, bool)> {
    let main_words = split_words(main);
    let highlight_words = split_words(highlight);
    main_words
        .iter()
        .enumerate()
        .map(|(i, w)| (i, w.clone(), false))
        .chain(
            highlight_words
                .iter()
                .enumerate()
                .map(|(i, w)| (main_words.len() + i, w.clone(), true)),
        )
        .collect()
}

/// Render heading copy as one masked span per word (`.fx-mask` + `.fx-word`)
/// so section 8 can cascade a left-to-right rise. The highlight fragment
/// continues the ordinal, so gradient words join the same cascade as the
/// plain ones.
///
/// Separator spaces are explicit `aria-hidden` inline-blocks BETWEEN the
/// masks: trailing whitespace inside an inline-block is stripped by CSS
/// whitespace processing, which would weld the words together. The spans
/// stay inline and non-breaking, so assistive tech still reads the heading
/// as one plain-text string.
pub fn word_masks(main: &str, highlight: &str) -> Element {
    let words = cascade_words(main, highlight);
    let total = words.len();

    rsx! {
        for (ordinal, word, is_highlight) in words.iter() {
            span {
                key: "w{ordinal}",
                class: "fx-mask",
                style: "--i: {ordinal};",
                span {
                    class: if *is_highlight { "fx-word grad-text" } else { "fx-word" },
                    "{word}"
                }
            }
            if ordinal + 1 < total {
                span { key: "s{ordinal}", class: "fx-space", aria_hidden: "true", " " }
            }
        }
    }
}

/// Ease-out-cubic — fast attack, soft landing. Shared by every Rust-driven
/// roll so motion feels like one family (mirrors the CSS cascade easing).
pub fn ease_out_cubic(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(3)
}

/// Format a rolling counter at its target's precision: integer targets
/// round ("24" at 12.6 → "13"), decimal targets keep their places
/// ("98.6" at 45.32 → "45.3"). The final frame renders `target + suffix`,
/// ending on exactly the static markup value.
pub fn format_counter(target: &str, value: f64) -> String {
    match target.split_once('.') {
        Some((_, frac)) => format!("{value:.*}", frac.len()),
        None => format!("{}", value.round() as i64),
    }
}

/// Roll the hero's `.dynamic-counter` stats: zero them at mount, wait for
/// the stats strip's cascade slot, ease 0 → target over 1.3s, then freeze
/// on the exact static markup value (`target + suffix`) — the DOM ends
/// identical to the no-WASM fallback. Re-running (brand switch) supersedes
/// any in-flight roll via a generation counter.
#[cfg(target_arch = "wasm32")]
pub fn roll_stat_counters(start_after_ms: i32) {
    use wasm_bindgen::JsCast;

    let Some(win) = web_sys::window() else { return };
    let Some(doc) = win.document() else { return };
    if prefers_reduced_motion(&win) {
        return;
    }
    let Ok(list) = doc.query_selector_all("#home .dynamic-counter") else {
        return;
    };

    let mut counters = Vec::new();
    for i in 0..list.length() {
        let Some(node) = list.item(i) else { continue };
        let Ok(el) = node.dyn_into::<web_sys::Element>() else {
            continue;
        };
        let (Some(target), Some(suffix)) = (
            el.get_attribute("data-target"),
            el.get_attribute("data-suffix"),
        ) else {
            continue;
        };
        if target.parse::<f64>().is_err() {
            continue;
        }
        // Zero immediately so the pills enter the cascade reading "0".
        el.set_text_content(Some(&format_counter(&target, 0.0)));
        counters.push((el, target, suffix));
    }
    if counters.is_empty() {
        return;
    }

    ROLL_GENERATION.fetch_add(1, Ordering::Relaxed);
    let generation = ROLL_GENERATION.load(Ordering::Relaxed);
    let counters = Rc::new(counters);
    schedule_roll(
        &win,
        generation,
        counters,
        start_after_ms.max(0) as f64,
        0.0,
    );
}

/// One roll step: fires on a timeout chain (no self-referencing rAF
/// closure needed), 16ms cadence, ease-out-cubic over 1300ms.
#[cfg(target_arch = "wasm32")]
fn schedule_roll(
    win: &web_sys::Window,
    generation: u32,
    counters: Rc<Vec<(web_sys::Element, String, String)>>,
    timeout_ms: f64,
    elapsed_ms: f64,
) {
    use wasm_bindgen::{closure::Closure, JsCast};

    const DURATION_MS: f64 = 1300.0;
    const TICK_MS: i32 = 16;

    let tick_win = win.clone(); // owned: the closure must be 'static
    let cb = Closure::once(move || {
        if generation != ROLL_GENERATION.load(Ordering::Relaxed) {
            return; // superseded by a brand switch — the new roll owns the DOM
        }
        let t = ((elapsed_ms + TICK_MS as f64) / DURATION_MS).min(1.0);
        let eased = ease_out_cubic(t);
        for (el, target, suffix) in counters.iter() {
            if t >= 1.0 {
                // Freeze on the exact static markup value.
                el.set_text_content(Some(&format!("{target}{suffix}")));
            } else {
                let value = target.parse::<f64>().unwrap_or(0.0) * eased;
                el.set_text_content(Some(&format_counter(target, value)));
            }
        }
        if t < 1.0 {
            schedule_roll(
                &tick_win,
                generation,
                counters.clone(),
                TICK_MS as f64,
                elapsed_ms + TICK_MS as f64,
            );
        }
    });
    let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(
        cb.as_ref().unchecked_ref(),
        timeout_ms as i32,
    );
    cb.forget(); // one-shot timer — the closure owns the next tick
}

/// One-shot card entrances: once the WASM app is up, pause every below-fold
/// entrance at frame 0 (`#app[data-fx]`, see styles.css §5) and release each
/// card with `.fx-in` the first time it crosses the viewport — plays once,
/// never reverses on scroll-back, and lands on anchor navigation. Works in
/// any browser with WASM, independent of scroll-driven-animation support.
/// Reduced motion: never marks `data-fx`, so cards keep the static
/// final state.
#[cfg(target_arch = "wasm32")]
pub fn attach_scroll_entrances() {
    use wasm_bindgen::{closure::Closure, JsCast};

    let Some(win) = web_sys::window() else { return };
    let Some(doc) = win.document() else { return };
    if prefers_reduced_motion(&win) {
        return;
    }
    let Some(app) = doc.get_element_by_id("app") else {
        return;
    };
    if app.has_attribute("data-fx") {
        return; // already wired
    }
    const CARDS: &str = ".reveal-init, .peak-in-up, .peak-in-left, .peak-in-right, .peak-in-scale";
    let Ok(nodes) = doc.query_selector_all(CARDS) else {
        return;
    };

    let callback = Closure::wrap(Box::new(
        move |entries: js_sys::Array, observer: web_sys::IntersectionObserver| {
            for entry in entries.iter() {
                let entry: web_sys::IntersectionObserverEntry = entry.unchecked_into();
                if !entry.is_intersecting() {
                    continue;
                }
                let Ok(target) = entry.target().dyn_into::<web_sys::Element>() else {
                    continue;
                };
                let _ = target.class_list().add_1("fx-in");
                observer.unobserve(&target); // one-shot: entrance, then static
            }
        },
    )
        as Box<dyn FnMut(js_sys::Array, web_sys::IntersectionObserver)>);

    // Fire when the card's top crosses ~90% of the viewport height.
    let options = web_sys::IntersectionObserverInit::new();
    options.set_root_margin("0px 0px -10% 0px");
    let Ok(observer) = web_sys::IntersectionObserver::new_with_options(
        callback.as_ref().unchecked_ref(),
        &options,
    ) else {
        return;
    };

    for i in 0..nodes.length() {
        if let Some(node) = nodes.item(i) {
            if let Ok(el) = node.dyn_into::<web_sys::Element>() {
                observer.observe(&el);
            }
        }
    }
    // Only mark the enhancement live once the observer is fully wired.
    let _ = app.set_attribute("data-fx", "");
    std::mem::forget(observer); // page-global singleton, lives for the session
    std::mem::forget(callback);
}

#[cfg(target_arch = "wasm32")]
fn prefers_reduced_motion(win: &web_sys::Window) -> bool {
    win.match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .is_some_and(|m| m.matches())
}

#[cfg(target_arch = "wasm32")]
static ROLL_GENERATION: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use std::sync::atomic::Ordering;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn motion_classes_are_stable_css_hooks() {
        assert_eq!(Fx::Reveal.class(), "fx-wipe");
        assert_eq!(Fx::Float.class(), "fx-float");
        assert_eq!(Fx::Fade.class(), "fx-fade");
    }

    #[test]
    fn cascade_slot_emits_ordinal_custom_property() {
        let (cls, style) = Fx::Float.at(3);
        assert_eq!(cls, "fx-float");
        assert_eq!(style, "--i: 3;");

        // Slot 0 must not be dropped — the eyebrow starts the cascade.
        assert_eq!(Fx::Fade.at(0), ("fx-fade", "--i: 0;".to_string()));
    }

    #[test]
    fn split_words_drops_runs_and_whitespace() {
        assert_eq!(
            split_words("Your health, guided by"),
            vec!["Your", "health,", "guided", "by"]
        );
        assert_eq!(split_words("  a \t b  "), vec!["a", "b"]);
        assert!(split_words("   ").is_empty());
    }

    #[test]
    fn cascade_runs_title_then_highlight_with_dense_ordinals() {
        let words = cascade_words("Your health, guided by", "clinical experts.");
        let texts: Vec<&str> = words.iter().map(|(_, w, _)| w.as_str()).collect();
        assert_eq!(
            texts,
            vec!["Your", "health,", "guided", "by", "clinical", "experts."]
        );
        // Dense 0..=n ordinals — both drivers derive stagger from --i.
        assert_eq!(
            words.iter().map(|(i, _, _)| *i).collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4, 5]
        );
        // Highlight fragment joins the cascade, marked for grad-text.
        assert_eq!(words.iter().filter(|(_, _, hl)| *hl).count(), 2);
        assert!(!words[0].2 && words[4].2);
    }

    #[test]
    fn cascade_without_highlight_stays_contiguous() {
        let words = cascade_words("Helping You To Bring", "");
        assert_eq!(words.len(), 4);
        assert!(words.iter().all(|(_, _, hl)| !*hl));
    }

    #[test]
    fn counter_formats_follow_target_precision() {
        // Integer targets round — "24" mid-roll reads as whole numbers.
        assert_eq!(format_counter("24", 12.6), "13");
        assert_eq!(format_counter("1000", 999.4), "999");
        // Decimal targets keep their place — "98.6" rolls 0.0 → 98.6.
        assert_eq!(format_counter("98.6", 45.32), "45.3");
        assert_eq!(format_counter("98.6", 0.0), "0.0");
        // The final frame is exact: the roll ends on the static markup value.
        assert_eq!(format_counter("24", 24.0), "24");
        assert_eq!(format_counter("98.6", 98.6), "98.6");
    }

    #[test]
    fn ease_out_cubic_is_monotonic_on_the_unit_interval() {
        assert_eq!(ease_out_cubic(0.0), 0.0);
        assert_eq!(ease_out_cubic(1.0), 1.0);
        assert!(ease_out_cubic(0.5) > 0.5); // fast attack
        assert!(ease_out_cubic(0.25) < ease_out_cubic(0.5)); // monotonic
    }
}
