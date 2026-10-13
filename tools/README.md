# tools/ — build-time-only asset generators (NEVER shipped)

JavaScript in this repository lives **only** here (dev-time generators) and in
Playwright test cases (`test_e2e.mjs`, `tests_e2e_suite.mjs`,
`test-artifacts/`, `happyhome-social/tests/`). The shipped application
(Dioxus/Rust + `index.html` + CSS) contains **zero** JavaScript — enforced by
`tests/lib.rs::test_index_html_contains_no_script_tags`.

## Scripts

| Script | Purpose | Output (same filename, never renamed) |
|---|---|---|
| `render_premium_heroes.mjs` | Premium 1920×640 heroes, canonical logo `happy-home-logo-transparent.png` | `assets/images/hero_hospital_ambulance.jpg`, `assets/images/hero_banner.jpg` |
| `create_hero_banner.mjs` | Legacy 1200×900 graphic hero (superseded, kept for reference) | `assets/images/hero_hospital_ambulance.jpg` |
| `scratch_render_hero.mjs` | Test render of `compose_hero.html` overlays to a scratch path | scratch path only |

## Run

```bash
# from the web root:
node tools/render_premium_heroes.mjs
```

Requires `/Applications/Google Chrome.app` + the `playwright` npm package.
All paths resolve from the file location (`import.meta.dirname`), so the
scripts work regardless of the invoking working directory.
