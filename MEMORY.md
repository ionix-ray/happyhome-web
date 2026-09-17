# Happy Home Hospital & Rehab — Memory & Architectural Decisions (MEMORY.md)

**Project**: Happy Home Hospital & Inpatient Rehab Centre Web Application  
**Root Path**: `/Volumes/hex/happy-home/happyhome-web`  

---

## 1. Project Context & Origin

- **Original Request**: Recover and revamp the Happy Home Hospital website into a modern Dioxus (Rust WASM) single page application based on prototype `mediCore-v2.html` and live website `https://happyhomehospital.com`.
- **Location Constraints**: Single canonical root directory on `/Volumes/hex/happy-home/happyhome-web`. Zero duplicate directories or symlinks.
- **Restoration Source**: Recovered baseline files and test definitions from historical Antigravity session transcripts.

---

## 2. Key Architectural Decisions (ADRs)

### ADR-01: Dual-Brand Mode Architecture
- **Decision**: Provide instantaneous client-side switching between **Hospital Mode** and **Rehab Mode** without page reload.
- **Rationale**: The facility operates as both an acute 24/7 medical hospital (ICU, trauma, general OPD) and a specialized long-term neuro/spine inpatient rehabilitation centre. Providing a dual toggle allows both patient types to see relevant specialties, doctor rosters, and recovery packages.
- **Implementation**: Managed globally via `BrandModeSignal(Signal<BrandMode>)` context. Components read this context and conditionally render services, highlights, and CTAs.

### ADR-02: Compile-Time Embedded Configuration
- **Decision**: Embed `config/site.toml` at compile-time via `include_str!` into the WASM binary.
- **Rationale**: Eliminates asynchronous network fetch latency on page load, enables static type validation of all clinical data at build time, and ensures 100% offline availability in PWA/WASM environments.

### ADR-03: Strict Zero Unicode Emojis
- **Decision**: Replace all unicode emojis in user-facing components with dedicated SVG vector components in `components/icons.rs`.
- **Rationale**: Preserves clinical gravity, prevents operating system dependent emoji font inconsistencies, and ensures crisp visual rendering across all displays.

### ADR-04: Dioxus 0.7 RSX Lifetime Resolution
- **Problem**: In Dioxus 0.7, iterating over borrowed vectors from component context inside `rsx!` triggers `E0597: borrowed value does not live long enough` because RSX closures require `'static` bounds.
- **Decision**: Component-level vector data extracted from `use_context::<SiteConfig>()` must be cloned into local owned variables (`.clone()`) and consumed with owned iteration (`for item in vec`).
- **Component Props**: Renamed conflicting prop `key` to `name` in `DynamicIcon` and enabled `#[props(into)] name: String`.

### ADR-05: Input Sanitization & Security Boundary
- **Decision**: Implement domain-level sanitization (`sanitize_text`), strict length caps (100 chars for names, 1000 for messages), and proactive XSS payload filtering before creating the strongly-typed `ContactForm` model.
- **Rationale**: Defends against stored and reflected Cross-Site Scripting, buffer exhaustion, and malformed inputs.

### ADR-06: Vector SVG Branding & Theme Adaptability
- **Decision**: Render official 12-petal lotus emblem and typography in clean vector SVG (`HappyHomeLogo` component and `assets/images/logo.svg`).
- **Rationale**: Replaces raster pixelated assets with crisp scalable vectors. Ensures logo sub-elements adapt seamlessly between light mode (`#f3f7fb`) and dark mode (`#0a101d`) while maintaining brand purity.

### ADR-07: WCAG Contrast & Semantic CSS Variables
- **Decision**: Standardize on semantic CSS variables (`--c-brand: #36a1d7`, `--c-brand-dark: #134cb3`, `--c-bg: #f3f7fb`, `--c-text: #0f172a`, dark mode `--c-bg: #0a101d`, `--c-text: #f8fafc`).
- **Rationale**: Eliminates washed-out text on dark or translucent surfaces. Guarantees WCAG AAA compliance (≥7:1 body, ≥15:1 headings).

### ADR-08: Authentic Clinical Badging & Uniform Alignment
- **Decision**: Generate photorealistic clinical portraits with custom-embroidered "Happy Home Hospital" & "Happy Home Rehab" lab coats and chest emblems. Purge all third-party hospital names ("Manipal") from code, configs, and assets.
- **Rationale**: Delivers authentic brand representation matching the live Bhubaneswar institution.

### ADR-09: Live Logo Super-Resolution & Theme Presentation
- **Decision**: Directly adopt official emblem from `https://happyhomehospital.com/wp-content/uploads/2026/05/Logo-happy-home.jpg` upscaled 4x with Lanczos filtering (`Logo-happy-home-superres.png`). Encapsulate within a clean white/glass rounded container in navbar and footer.
- **Rationale**: Preserves 100% font and emblem fidelity without pixelation on Retina displays while guaranteeing maximum contrast and visibility in both light and dark themes.

### ADR-10: Organic Healing Cure Animation Architecture
- **Decision**: Implement a CSS-driven restorative animation system (`healingAura`, `cure-light`, `healingPulse`) using organic easing curves and subtle ambient color oscillations.
- **Rationale**: Infuses the interface with an emotional sense of biological healing, recovery, and clinical calm without incurring JavaScript execution overhead on scroll.

### ADR-11: Canonical Single Workspace Directory Consolidation
- **Decision**: Consolidate workspace strictly at `/Volumes/hex/happy-home/happyhome-web`. Eliminate old `medicore-web` directory and symlinks.
- **Rationale**: Prevents multi-folder ambiguity, ensures cargo targets and Dioxus config operate on a single, deterministic file tree.

### ADR-12: High-Performance Lightweight WASM & Restorative Scroll Peak-In
- **Decision**: Configure `Cargo.toml` and `Dioxus.toml` with `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `panic = "abort"`, `strip = true`, `debug = false`, and Brotli pre-compression. Implement GPU-accelerated scroll entrance peak-in animations with native `IntersectionObserver` + `MutationObserver`. Upgrade vector icons to duotone with Carbon design system animations.
- **Rationale**: Decreases WebAssembly payload to 795KB (245KB Brotli), eliminates lazy-loading pop-in lag, and provides smooth, organic page transitions with zero third-party JavaScript dependencies.


