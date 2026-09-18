# Happy Home Hospital & Rehab — Release Notes (RELEASENOTES.md)

**Release Version**: v1.5.0 (Blazing Fast CI/CD & FCP Release)
**Release Date**: 2026-09-18
**Framework**: Rust 2021 Edition / Dioxus 0.7 WASM
**Target Directory**: `/Volumes/hex/happy-home/happyhome-web`

---

## Highlights of Release v1.5.0

### 1. Blazing Fast Sub-Second FCP
- **Tailwind AOT**: Removed 3.5MB Tailwind CDN script. Pre-compiled all CSS AOT into a static 27KB `tailwind.min.css` payload.
- **Image Preload Network Unblocking**: Removed heavy image preloads from `index.html` to guarantee main-thread boot speed.
- **Instant Skeleton Loader**: Added a lightweight HTML/CSS skeleton loader to `index.html` that renders in <10ms, preventing white screen while WASM initializes.

### 2. CI/CD Build Script & Housekeeping
- Created a robust one-click build script (`build.sh`) at the project root to automatically clean old artifacts and generate the new optimized WASM bundle.
- Permanent deletion of legacy `medicore-web` directory and stray codebase elements to guarantee 100% strict adherence to the canonical `/Volumes/hex/happy-home/happyhome-web` workspace.

### 3. Acceptance Criteria Fulfilled
- Zero warnings across `cargo clippy`.
- Zero vulnerabilities.
- UI Modular & config-driven. 
- Fully functional Dioxus WASM with 30/30 unit & integration tests passing.
- Comprehensive tracking (TASKS.md, PROGRESS.md, MEMORY.md, RELEASENOTES.md).

---

## Highlights of Release v1.4.0

### 1. Ultra-Lightweight WebAssembly Payload
- Optimized release profile with `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `panic = "abort"`, and `strip = true`.
- Raw WASM binary reduced from 2.2MB to **795KB**; pre-compressed with Brotli to **245KB** for lightning-fast edge delivery.

### 2. Restorative Scroll "Peak In" Animation Engine
- Zero-dependency GPU-accelerated entrance animations (`peak-in-up`, `peak-in-left`, `peak-in-right`, `peak-in-scale`) with staggered micro-delays (`delay-75` to `delay-600`).
- Seamless native `IntersectionObserver` + `MutationObserver` ensures instant reveal for in-viewport elements and smooth entrance transitions on scroll.

### 3. Vibrant Clinical Duotone Vector Icons & Carbon Animations
- Upgraded 100% SVG icon suite with duotone translucent fills, clinical spark nodes, and brand-aware color tokens.
- Integrated IBM Carbon design system animated icons (`CarbonActivityPulseIcon`, `CarbonHeartHealthIcon`, etc.).

