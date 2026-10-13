//! Regenerate SEO artifacts from `config/site.toml`.
//!
//! Run: `cargo run --bin seo_artifacts`
//! Writes `public/{robots.txt,sitemap.xml,llms.txt}` and rewrites the JSON-LD
//! block inside `index.html`. Wired into `build.sh` so every build ships
//! artifacts that match config; `tests/seo_tests.rs` fails on drift.

use happyhome::domain::{config::SiteConfig, seo};

fn main() {
    let raw = std::fs::read_to_string("config/site.toml").expect("read config/site.toml");
    let cfg: SiteConfig = toml::from_str(&raw).expect("config/site.toml must deserialize");

    std::fs::create_dir_all("public").expect("create public/ directory");
    std::fs::write("public/robots.txt", seo::render_robots(&cfg)).expect("write robots.txt");
    std::fs::write("public/sitemap.xml", seo::render_sitemap(&cfg)).expect("write sitemap.xml");
    std::fs::write("public/llms.txt", seo::render_llms_txt(&cfg)).expect("write llms.txt");

    let html_path = "index.html";
    let html = std::fs::read_to_string(html_path).expect("read index.html");
    let injected = seo::inject_json_ld(&html, &seo::render_json_ld(&cfg));
    std::fs::write(html_path, injected).expect("write index.html");

    println!("seo_artifacts: wrote public/robots.txt, public/sitemap.xml, public/llms.txt, index.html JSON-LD");
}
