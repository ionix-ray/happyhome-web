//! SEO contract tests — TDD red phase for the SEO domain module.
//!
//! Guards: config-driven rendering (Rule 11: zero hardcoded values),
//! schema quality gates (no FAQPage/HowTo), NAP consistency across all
//! generated artifacts, Microsoft Clarity injection only when configured.

use happyhome::domain::config::SiteConfig;
use happyhome::domain::seo;

const PROD_TOML: &str = include_str!("../config/site.toml");

fn prod_config() -> SiteConfig {
    toml::from_str(PROD_TOML).expect("config/site.toml must deserialize")
}

// ---------------------------------------------------------------------------
// JSON-LD structured data
// ---------------------------------------------------------------------------

#[test]
fn test_json_ld_is_valid_json_with_graph_and_configured_schema_type() {
    let cfg = prod_config();
    let rendered = seo::render_json_ld(&cfg);
    let value: serde_json::Value =
        serde_json::from_str(&rendered).expect("JSON-LD must be valid JSON");
    assert_eq!(value["@context"], "https://schema.org");
    let graph = value["@graph"].as_array().expect("@graph must be an array");
    let hospital = graph
        .iter()
        .find(|n| n["@type"] == cfg.seo.schema_type)
        .expect("graph must contain node of configured schema_type");
    assert_eq!(hospital["name"], cfg.site.name);
    assert_eq!(hospital["url"], cfg.seo.canonical);
}

#[test]
fn test_json_ld_geo_coordinates_have_five_decimal_places() {
    let cfg = prod_config();
    let rendered = seo::render_json_ld(&cfg);
    let value: serde_json::Value = serde_json::from_str(&rendered).unwrap();
    let graph = value["@graph"].as_array().unwrap();
    let geo = graph
        .iter()
        .find(|n| n["@type"] == cfg.seo.schema_type)
        .unwrap()["geo"]
        .clone();
    let lat = geo["latitude"].as_f64().expect("geo.latitude present");
    let lng = geo["longitude"].as_f64().expect("geo.longitude present");
    let lat_s = format!("{lat}");
    let lng_s = format!("{lng}");
    let lat_decimals = lat_s.split('.').nth(1).map_or(0, |d| d.len());
    let lng_decimals = lng_s.split('.').nth(1).map_or(0, |d| d.len());
    assert!(
        lat_decimals >= 5 && lng_decimals >= 5,
        "geo precision must be >= 5 decimal places, got lat={lat_s} lng={lng_s}"
    );
    // Values must come from config, not be invented in the renderer.
    assert!(cfg.seo.geo_lat != 0.0 && cfg.seo.geo_lng != 0.0);
}

#[test]
fn test_json_ld_contains_nap_and_contact_from_config() {
    let cfg = prod_config();
    let rendered = seo::render_json_ld(&cfg);
    let value: serde_json::Value = serde_json::from_str(&rendered).unwrap();
    let graph = value["@graph"].as_array().unwrap();
    let node = graph
        .iter()
        .find(|n| n["@type"] == cfg.seo.schema_type)
        .unwrap();
    assert_eq!(node["telephone"], cfg.contact.phone_primary);
    assert_eq!(node["email"], cfg.contact.email);
    let addr = &node["address"];
    assert_eq!(addr["streetAddress"], cfg.seo.address.street);
    assert_eq!(addr["addressLocality"], cfg.seo.address.locality);
    assert_eq!(addr["addressRegion"], cfg.seo.address.region);
    assert_eq!(addr["postalCode"], cfg.seo.address.postal_code);
    assert_eq!(addr["addressCountry"], cfg.seo.address.country);
    // NAP consistency: schema street must be a prefix of the visible page address.
    assert!(
        cfg.contact.address.starts_with(&cfg.seo.address.street),
        "visible NAP address and schema streetAddress must agree"
    );
}

#[test]
fn test_json_ld_same_as_covers_every_social_profile() {
    let cfg = prod_config();
    let rendered = seo::render_json_ld(&cfg);
    for url in [
        cfg.social.facebook.as_deref(),
        cfg.social.twitter.as_deref(),
        cfg.social.linkedin.as_deref(),
        cfg.social.youtube.as_deref(),
        cfg.social.instagram.as_deref(),
    ]
    .into_iter()
    .flatten()
    {
        assert!(
            rendered.contains(url),
            "sameAs must include social profile {url}"
        );
    }
}

#[test]
fn test_json_ld_never_emits_deprecated_schema_types() {
    // Quality gates from the seo skill: FAQPage rich results retired
    // (May 2026), HowTo deprecated (Sept 2023).
    let cfg = prod_config();
    let rendered = seo::render_json_ld(&cfg);
    assert!(
        !rendered.contains("FAQPage"),
        "FAQPage schema must not be emitted"
    );
    assert!(
        !rendered.contains("HowTo"),
        "HowTo schema must not be emitted"
    );
}

#[test]
fn test_json_ld_opening_hours_derived_from_config() {
    let cfg = prod_config();
    let rendered = seo::render_json_ld(&cfg);
    let value: serde_json::Value = serde_json::from_str(&rendered).unwrap();
    let graph = value["@graph"].as_array().unwrap();
    let node = graph
        .iter()
        .find(|n| n["@type"] == cfg.seo.schema_type)
        .unwrap();
    let specs = node["openingHoursSpecification"]
        .as_array()
        .expect("openingHoursSpecification must be an array");
    assert!(!specs.is_empty());
    let opd = &specs[0];
    assert_eq!(opd["opens"], cfg.seo.opening_hours.opens);
    assert_eq!(opd["closes"], cfg.seo.opening_hours.closes);
    let days = opd["dayOfWeek"].as_array().unwrap();
    assert!(
        days.len() >= cfg.seo.opening_hours.days.len(),
        "OPD days must come from config"
    );
    let emergency = &specs[1];
    assert_eq!(emergency["opens"], cfg.seo.emergency_hours.opens);
    assert_eq!(emergency["closes"], cfg.seo.emergency_hours.closes);
}

// ---------------------------------------------------------------------------
// robots.txt
// ---------------------------------------------------------------------------

#[test]
fn test_robots_allows_ai_search_crawlers() {
    let cfg = prod_config();
    let robots = seo::render_robots(&cfg);
    for bot in [
        "Googlebot",
        "Google-Extended",
        "bingbot",
        "GPTBot",
        "OAI-SearchBot",
        "ChatGPT-User",
        "ClaudeBot",
        "PerplexityBot",
        "CCBot",
        "Bytespider",
    ] {
        assert!(
            robots.contains(bot),
            "robots.txt must explicitly allow AI/search crawler {bot}"
        );
    }
}

#[test]
fn test_robots_points_to_sitemap_at_canonical_origin() {
    let cfg = prod_config();
    let robots = seo::render_robots(&cfg);
    let origin = cfg.seo.canonical.trim_end_matches('/').to_string();
    assert!(
        robots.contains(&format!("Sitemap: {origin}/sitemap.xml")),
        "robots.txt must reference sitemap on canonical origin"
    );
    assert!(robots.starts_with("User-agent: *"));
    assert!(
        !robots.contains("Disallow: /"),
        "site must not be fully disallowed"
    );
}

// ---------------------------------------------------------------------------
// sitemap.xml
// ---------------------------------------------------------------------------

#[test]
fn test_sitemap_lists_canonical_url_with_configured_lastmod() {
    let cfg = prod_config();
    let sitemap = seo::render_sitemap(&cfg);
    assert!(sitemap.starts_with("<?xml version=\"1.0\""));
    assert!(sitemap.contains("<urlset"));
    assert!(sitemap.contains(&format!("<loc>{}</loc>", cfg.seo.canonical)));
    assert!(sitemap.contains(&format!("<lastmod>{}</lastmod>", cfg.seo.sitemap_lastmod)));
}

#[test]
fn test_sitemap_lastmod_is_iso_date() {
    let cfg = prod_config();
    let lastmod = &cfg.seo.sitemap_lastmod;
    let parts: Vec<&str> = lastmod.split('-').collect();
    assert_eq!(parts.len(), 3, "sitemap_lastmod must be YYYY-MM-DD");
    assert_eq!(parts[0].len(), 4);
    assert_eq!(parts[1].len(), 2);
    assert_eq!(parts[2].len(), 2);
    for p in &parts {
        assert!(p.chars().all(|c| c.is_ascii_digit()));
    }
}

// ---------------------------------------------------------------------------
// llms.txt (AI search citability)
// ---------------------------------------------------------------------------

#[test]
fn test_llms_txt_follows_markdown_convention() {
    let cfg = prod_config();
    let llms = seo::render_llms_txt(&cfg);
    assert!(llms.starts_with("# "), "llms.txt must open with an H1");
    assert!(
        llms.contains("> "),
        "llms.txt must carry a blockquote summary"
    );
    assert!(llms.contains(&cfg.seo.canonical));
    assert!(llms.contains(&cfg.site.name));
}

#[test]
fn test_llms_txt_exposes_nap_and_emergency_contact() {
    let cfg = prod_config();
    let llms = seo::render_llms_txt(&cfg);
    assert!(llms.contains(&cfg.contact.address));
    assert!(llms.contains(&cfg.contact.phone_primary));
    assert!(llms.contains(&cfg.contact.hours));
}

#[test]
fn test_llms_txt_lists_every_service_for_both_brands() {
    let cfg = prod_config();
    let llms = seo::render_llms_txt(&cfg);
    for s in cfg
        .hospital
        .services
        .iter()
        .chain(cfg.rehab.services.iter())
    {
        assert!(
            llms.contains(&s.name),
            "llms.txt must list service {}",
            s.name
        );
    }
}

#[test]
fn test_llms_txt_answers_faq_questions_as_markdown() {
    // FAQ content feeds AI answers as plain markdown — never as FAQPage schema.
    let cfg = prod_config();
    let llms = seo::render_llms_txt(&cfg);
    for faq in cfg.hospital.faqs.iter().chain(cfg.rehab.faqs.iter()) {
        assert!(
            llms.contains(&faq.question),
            "llms.txt must include FAQ: {}",
            faq.question
        );
        assert!(llms.contains(&faq.answer));
    }
}

// ---------------------------------------------------------------------------
// Microsoft Clarity
// ---------------------------------------------------------------------------

#[test]
fn test_clarity_disabled_when_project_id_not_configured() {
    let mut cfg = SiteConfig::default();
    assert_eq!(
        seo::render_clarity_snippet(&cfg),
        None,
        "no analytics when clarity_project_id is empty"
    );
    cfg.analytics.clarity_project_id = "   ".to_string();
    assert_eq!(
        seo::render_clarity_snippet(&cfg),
        None,
        "whitespace id ignored"
    );
}

#[test]
fn test_clarity_snippet_injected_only_from_config() {
    let mut cfg = SiteConfig::default();
    cfg.analytics.clarity_project_id = "abctest123".to_string();
    let snippet = seo::render_clarity_snippet(&cfg).expect("snippet rendered");
    assert!(snippet.contains("clarity.ms/tag/abctest123"));
    assert!(snippet.contains("script"));
}

#[test]
fn test_production_config_keeps_clarity_placeholder_until_id_supplied() {
    // Rule 11: no hardcoded IDs. Production config ships the slot empty;
    // setting the real ID is a one-line TOML edit.
    let cfg = prod_config();
    assert_eq!(
        cfg.analytics.clarity_project_id, "",
        "config/site.toml must carry an empty clarity_project_id placeholder"
    );
    assert_eq!(seo::render_clarity_snippet(&cfg), None);
}

// ---------------------------------------------------------------------------
// index.html JSON-LD injection (markers, idempotency)
// ---------------------------------------------------------------------------

#[test]
fn test_index_html_carries_json_ld_injection_markers() {
    let html = include_str!("../index.html");
    assert!(html.contains(seo::JSONLD_START_MARKER));
    assert!(html.contains(seo::JSONLD_END_MARKER));
    assert!(html.contains("<html lang=\"en-IN\">"));
    assert!(html.contains("<link rel=\"canonical\""));
}

#[test]
fn test_inject_json_ld_replaces_block_and_is_idempotent() {
    let html = include_str!("../index.html");
    let cfg = prod_config();
    let block = seo::render_json_ld(&cfg);
    let once = seo::inject_json_ld(html, &block);
    let twice = seo::inject_json_ld(&once, &block);
    assert_eq!(once, twice, "injection must be idempotent");
    assert_eq!(once.matches(seo::JSONLD_START_MARKER).count(), 1);
    assert_eq!(
        once.matches("\"@context\": \"https://schema.org\"").count(),
        1,
        "exactly one JSON-LD block"
    );
}

#[test]
fn test_inject_json_ld_leaves_html_without_markers_untouched() {
    let bare = "<html><head></head><body></body></html>";
    let out = seo::inject_json_ld(bare, "{}");
    assert_eq!(out, bare, "missing markers must be a no-op, not a panic");
}

#[test]
fn test_inject_json_ld_leaves_unterminated_marker_block_untouched() {
    // Start marker present but end marker missing (or reversed) must not panic.
    let html = "<html><head><!-- SEO:JSONLD:START --></head></html>";
    let out = seo::inject_json_ld(html, "{}");
    assert_eq!(out, html, "unterminated marker block must be a no-op");
    let reversed = "<html><!-- SEO:JSONLD:END --><!-- SEO:JSONLD:START --></html>";
    assert_eq!(seo::inject_json_ld(reversed, "{}"), reversed);
}

// ---------------------------------------------------------------------------
// NAP / canonical consistency across every artifact (cross-source check)
// ---------------------------------------------------------------------------

#[test]
fn test_all_artifacts_agree_on_canonical_and_nap() {
    let cfg = prod_config();
    let canonical = cfg.seo.canonical.trim_end_matches('/').to_string();
    let phone = &cfg.contact.phone_primary;
    let name = &cfg.site.name;
    for artifact in [
        seo::render_robots(&cfg),
        seo::render_sitemap(&cfg),
        seo::render_llms_txt(&cfg),
        seo::render_json_ld(&cfg),
    ] {
        assert!(
            artifact.contains(&canonical) || artifact.contains(&cfg.seo.canonical),
            "artifact must reference canonical URL"
        );
        if !artifact.starts_with("<?xml") && !artifact.starts_with("User-agent") {
            assert!(artifact.contains(name), "artifact must carry business name");
            assert!(artifact.contains(phone), "artifact must carry NAP phone");
        }
    }
}

// ---------------------------------------------------------------------------
// Committed artifacts must match config-derived output (drift gate)
// ---------------------------------------------------------------------------

#[test]
fn test_committed_artifacts_match_config_rendering() {
    let cfg = prod_config();
    assert_eq!(
        include_str!("../public/robots.txt"),
        seo::render_robots(&cfg),
        "public/robots.txt drifted from config — rerun: cargo run --bin seo_artifacts"
    );
    assert_eq!(
        include_str!("../public/sitemap.xml"),
        seo::render_sitemap(&cfg),
        "public/sitemap.xml drifted from config"
    );
    assert_eq!(
        include_str!("../public/llms.txt"),
        seo::render_llms_txt(&cfg),
        "public/llms.txt drifted from config"
    );
}

#[test]
fn test_index_html_embeds_current_json_ld() {
    let cfg = prod_config();
    let html = include_str!("../index.html");
    let block = seo::render_json_ld(&cfg);
    let start = html
        .find(seo::JSONLD_START_MARKER)
        .expect("index.html must contain JSON-LD start marker");
    let end = html
        .find(seo::JSONLD_END_MARKER)
        .expect("index.html must contain JSON-LD end marker");
    let embedded = &html[start..end];
    assert!(
        embedded.contains(&block),
        "index.html JSON-LD drifted from config — rerun: cargo run --bin seo_artifacts"
    );
}

// ---------------------------------------------------------------------------
// Resilience: default (empty) config must render without panicking
// ---------------------------------------------------------------------------

#[test]
fn test_default_config_renders_all_artifacts_without_panic() {
    let cfg = SiteConfig::default();
    assert!(!seo::render_json_ld(&cfg).is_empty());
    assert!(!seo::render_robots(&cfg).is_empty());
    assert!(!seo::render_sitemap(&cfg).is_empty());
    assert!(!seo::render_llms_txt(&cfg).is_empty());
    assert_eq!(seo::render_clarity_snippet(&cfg), None);
}

// ---------------------------------------------------------------------------
// Local SEO: areaServed coverage for AI engines and local pack signals
// ---------------------------------------------------------------------------

#[test]
fn test_json_ld_area_served_covers_configured_localities() {
    let cfg = prod_config();
    let rendered = seo::render_json_ld(&cfg);
    let value: serde_json::Value = serde_json::from_str(&rendered).unwrap();
    let graph = value["@graph"].as_array().unwrap();
    let node = graph
        .iter()
        .find(|n| n["@type"] == cfg.seo.schema_type)
        .unwrap();
    let served = node["areaServed"]
        .as_array()
        .expect("areaServed must be present when configured");
    assert!(
        served.len() >= 5,
        "areaServed must list the full ambulance/transfer coverage, got {served:?}"
    );
    assert!(
        served
            .iter()
            .any(|a| a.as_str() == Some("Bhubaneswar")),
        "areaServed must include the home locality Bhubaneswar"
    );
}

#[test]
fn test_llms_txt_exposes_service_area_for_ai_crawlers() {
    let cfg = prod_config();
    let llms = seo::render_llms_txt(&cfg);
    assert!(
        llms.contains("Service Area") && llms.contains("Bhubaneswar"),
        "llms.txt must carry a Service Area section naming covered localities"
    );
}
