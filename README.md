# Happy Home Hospital & Rehab — Web Application

A modern, high-performance web application for **Happy Home Hospital And Rehab** (Bhubaneswar, Odisha), built with **Rust**, **Dioxus 0.7**, and **WebAssembly (WASM)**.

---

## Overview

Happy Home Hospital And Rehab is a specialized 30-bed neuro-rehabilitation and critical care hospital located in Bhubaneswar, Odisha. This web application delivers a reactive single-page experience supporting dynamic brand mode switching (Hospital mode & Neuro-Rehabilitation mode), interactive appointment booking, services browsing, and real-time validation.

---

## Architecture Overview

The project is organized as a Cargo workspace with separation of concerns across multiple crates:

```text
happyhome-web/
├── Dioxus.toml                 # Workspace / project Dioxus configuration
├── Cargo.toml                  # Workspace manifest
├── config/
│   └── site.toml               # Site-wide configuration and content source
├── crates/
│   ├── happyhome-domain/        # Core domain models, TOML config loaders, validation logic
│   ├── happyhome-ui/            # Dioxus components, routing, pages, styles, and WASM shell
│   │   ├── Dioxus.toml         # UI crate Dioxus configuration
│   │   ├── index.html          # HTML shell with meta/OG tags & schema.org JSON-LD
│   │   ├── assets/             # Static assets (CSS, images, icons)
│   │   │   └── images/         # Clinical photography & doctor portraits
│   │   └── src/                # UI components, layouts, hooks, and page definitions
│   └── happyhome-tests/         # Integration, functional, smoke, regression, and infosec tests
```

- **`happyhome-domain`**: Pure business logic, strongly typed config representations (`site.toml`), data validation models (e.g., contact/appointment forms), input sanitization, and security rules.
- **`happyhome-ui`**: Reactive WASM presentation layer using Dioxus 0.7, including reusable UI components, brand toggle states, zero emoji SVG icons, and page routing.
- **`happyhome-tests`**: Complete test suites verifying functional behavior, integration pipelines, smoke checks, regressions, and CIS/NIST/OWASP security assertions.

---

## Prerequisites

1. **Rust Toolchain**: Install stable Rust via [rustup](https://rustup.rs/):
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
2. **Dioxus CLI**: Install `dioxus-cli`:
   ```bash
   cargo install dioxus-cli --version ^0.7.0
   ```

---

## Development & Build

### Run Development Server
Start the local hot-reloading development server:
```bash
# Run from happyhome-web or crates/happyhome-ui
dx serve --platform web
```

### Build for Production
Compile the optimized WebAssembly bundle and static assets:
```bash
dx build --platform web --release
```
The output will be generated inside the `target/dx/happyhome-ui/release/web/public/` directory.

### Run Tests
Execute unit, functional, integration, smoke, regression, and infosec tests:
```bash
cargo test --workspace
```

### Run Linter
Verify zero warnings across all targets:
```bash
cargo clippy --workspace --all-targets -- -D warnings
```

---

## Configuration & Content

All site content, contact info, service lists, doctor profiles, and metadata are driven by:
- **`config/site.toml`**

To update contact details, doctor rosters, service descriptions, or footer links, modify `config/site.toml` without modifying UI component code.

---

## Assets & Design Philosophy

- **Zero Emojis**: Crisp, handcrafted SVG icons in `crates/happyhome-ui/src/components/icons.rs`.
- **Authentic Imagery**: Clinical photos of Bhubaneswar facility and authentic Indian doctors in `crates/happyhome-ui/assets/images/`.
- **Responsive Layout**: Designed for mobile, tablet, and desktop viewports with fluid CSS variables.
