//! SEO artifact rendering — pure functions, zero I/O (Rule 12).
//!
//! Single source of truth for everything an indexer or AI crawler reads:
//! JSON-LD structured data, `robots.txt`, `sitemap.xml`, `llms.txt`, and the
//! Microsoft Clarity loader snippet. Every value is derived from
//! `config/site.toml` (Rule 11) so the committed artifacts can be regenerated
//! and drift-detected by tests.

use crate::domain::config::{OpeningHours, SiteConfig};
use serde_json::{json, Value};

pub const JSONLD_START_MARKER: &str = "<!-- SEO:JSONLD:START -->";
pub const JSONLD_END_MARKER: &str = "<!-- SEO:JSONLD:END -->";

/// Search and AI crawlers explicitly granted access in robots.txt.
/// Covers Google, Bing, Brave (Bing index), and the major LLM retrieval bots
/// that power ChatGPT, Perplexity, Claude, Copilot, and Gemini answers.
const AI_CRAWLERS: &[&str] = &[
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
];

fn opening_hours_spec(hours: &OpeningHours) -> Option<Value> {
    if hours.days.is_empty() || hours.opens.is_empty() || hours.closes.is_empty() {
        return None;
    }
    Some(json!({
        "@type": "OpeningHoursSpecification",
        "dayOfWeek": hours.days,
        "opens": hours.opens,
        "closes": hours.closes,
    }))
}

fn same_as_profiles(cfg: &SiteConfig) -> Vec<String> {
    [
        cfg.social.facebook.as_deref(),
        cfg.social.twitter.as_deref(),
        cfg.social.linkedin.as_deref(),
        cfg.social.youtube.as_deref(),
        cfg.social.instagram.as_deref(),
    ]
    .into_iter()
    .flatten()
    .map(str::to_string)
    .collect()
}

/// Full JSON-LD `@graph`: organization node (configured schema.org type),
/// WebSite node, and WebPage node — machine-readable entity evidence for
/// classic search and AI answer engines.
pub fn render_json_ld(cfg: &SiteConfig) -> String {
    let mut hospital = json!({
        "@type": cfg.seo.schema_type,
        "@id": format!("{}#hospital", cfg.seo.canonical),
        "name": cfg.site.name,
        "url": cfg.seo.canonical,
        "description": cfg.seo.description,
        "telephone": cfg.contact.phone_primary,
        "email": cfg.contact.email,
        "image": cfg.seo.image,
        "priceRange": cfg.seo.price_range,
        "address": {
            "@type": "PostalAddress",
            "streetAddress": cfg.seo.address.street,
            "addressLocality": cfg.seo.address.locality,
            "addressRegion": cfg.seo.address.region,
            "postalCode": cfg.seo.address.postal_code,
            "addressCountry": cfg.seo.address.country,
        },
        "geo": {
            "@type": "GeoCoordinates",
            "latitude": cfg.seo.geo_lat,
            "longitude": cfg.seo.geo_lng,
        },
        "sameAs": same_as_profiles(cfg),
        "publicAccess": true,
    });

    let specs: Vec<Value> = [
        opening_hours_spec(&cfg.seo.opening_hours),
        opening_hours_spec(&cfg.seo.emergency_hours),
    ]
    .into_iter()
    .flatten()
    .collect();
    if !specs.is_empty() {
        hospital["openingHoursSpecification"] = Value::Array(specs);
    }
    if !cfg.seo.area_served.is_empty() {
        hospital["areaServed"] = Value::Array(
            cfg.seo.area_served
                .iter()
                .map(|a| Value::String(a.clone()))
                .collect(),
        );
    }

    let graph = json!({
        "@context": "https://schema.org",
        "@graph": [
            hospital,
            {
                "@type": "WebSite",
                "@id": format!("{}#website", cfg.seo.canonical),
                "url": cfg.seo.canonical,
                "name": cfg.site.name,
                "description": cfg.seo.description,
                "inLanguage": "en-IN",
            },
            {
                "@type": "WebPage",
                "@id": format!("{}#webpage", cfg.seo.canonical),
                "url": cfg.seo.canonical,
                "name": cfg.seo.title,
                "description": cfg.seo.description,
                "inLanguage": "en-IN",
                "isPartOf": { "@id": format!("{}#website", cfg.seo.canonical) },
            },
        ],
    });

    serde_json::to_string_pretty(&graph).expect("JSON-LD graph serializes")
}

/// robots.txt: full access for humans and search engines, explicit allow
/// blocks for AI crawlers, sitemap pointer on the canonical origin.
pub fn render_robots(cfg: &SiteConfig) -> String {
    let origin = cfg.seo.canonical.trim_end_matches('/');
    let mut out = String::from("User-agent: *\nAllow: /\n");
    for bot in AI_CRAWLERS {
        out.push_str(&format!("\nUser-agent: {bot}\nAllow: /\n"));
    }
    out.push_str(&format!("\nSitemap: {origin}/sitemap.xml\n"));
    out
}

/// sitemap.xml — single canonical URL for this single-page app.
pub fn render_sitemap(cfg: &SiteConfig) -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n  \
         <url>\n    \
         <loc>{}</loc>\n    \
         <lastmod>{}</lastmod>\n    \
         <changefreq>weekly</changefreq>\n    \
         <priority>1.0</priority>\n  \
         </url>\n\
         </urlset>\n",
        cfg.seo.canonical, cfg.seo.sitemap_lastmod
    )
}

fn llms_brand_section(
    out: &mut String,
    heading: &str,
    cfg: &SiteConfig,
    brand: &crate::domain::config::BrandConfig,
) {
    if brand.services.is_empty() {
        return;
    }
    out.push_str(&format!("\n## {heading}\n"));
    for service in &brand.services {
        out.push_str(&format!(
            "- [{}]({}#services) — {}\n",
            service.name, cfg.seo.canonical, service.description
        ));
    }
    out.push_str(&format!("\n### {heading} — Doctors\n"));
    for member in &brand.team {
        out.push_str(&format!(
            "- {} — {} ({})\n",
            member.name, member.role, member.title
        ));
    }
}

fn llms_faq_section(out: &mut String, brand: &crate::domain::config::BrandConfig) {
    for faq in &brand.faqs {
        out.push_str(&format!("\n### {}\n{}\n", faq.question, faq.answer));
    }
}

/// llms.txt — markdown brief for LLM retrieval (GEO / AI-search citability).
/// Spec: H1 title, blockquote summary, then sectioned markdown.
pub fn render_llms_txt(cfg: &SiteConfig) -> String {
    let mut out = format!("# {}\n\n", cfg.site.name);
    out.push_str(&format!("> {}\n\n", cfg.seo.description));
    out.push_str(&format!("- Canonical: {}\n", cfg.seo.canonical));
    out.push_str(&format!("- Address: {}\n", cfg.contact.address));
    out.push_str(&format!("- Phone: {}\n", cfg.contact.phone_primary));
    out.push_str(&format!("- Email: {}\n", cfg.contact.email));
    out.push_str(&format!("- Hours: {}\n", cfg.contact.hours));
    if !cfg.footer.about_text.is_empty() {
        out.push_str(&format!("\n## About\n{}\n", cfg.footer.about_text));
    }
    if !cfg.seo.area_served.is_empty() {
        out.push_str(&format!(
            "\n## Service Area\nAmbulance and transfer coverage: {}\n",
            cfg.seo.area_served.join(", ")
        ));
    }
    llms_brand_section(&mut out, "Hospital Services", cfg, &cfg.hospital);
    llms_brand_section(&mut out, "Rehabilitation Services", cfg, &cfg.rehab);
    if !cfg.hospital.faqs.is_empty() {
        out.push_str("\n## Frequently Asked Questions\n");
    }
    llms_faq_section(&mut out, &cfg.hospital);
    llms_faq_section(&mut out, &cfg.rehab);
    if !cfg.emergency.phone.is_empty() {
        out.push_str(&format!(
            "\n## Emergency\n24/7 emergency line: {}\nWhatsApp: {}\n",
            cfg.emergency.phone, cfg.emergency.whatsapp_phone
        ));
    }
    out
}

/// Microsoft Clarity loader — rendered only when a project ID is configured.
/// Returns `None` when the TOML slot is empty or whitespace (no tracking).
pub fn render_clarity_snippet(cfg: &SiteConfig) -> Option<String> {
    let id = cfg.analytics.clarity_project_id.trim();
    if id.is_empty() {
        return None;
    }
    Some(format!(
        "(function(c,l,a,r,i){{c[a]=c[a]||function(){{(c[a].q=c[a].q||[]).push(arguments)}};\
         var t=l.createElement(r);t.async=1;t.src=\"https://www.clarity.ms/tag/{id}\";\
         var y=l.getElementsByTagName(r)[0];y.parentNode.insertBefore(t,y);}})\
         (window,document,\"clarity\",\"script\");"
    ))
}

/// Replace the JSON-LD block between markers in the HTML shell.
/// Idempotent; HTML without markers passes through unchanged.
pub fn inject_json_ld(html: &str, json_ld: &str) -> String {
    let start = match html.find(JSONLD_START_MARKER) {
        Some(i) => i,
        None => return html.to_string(),
    };
    let end = match html.find(JSONLD_END_MARKER) {
        Some(i) if i > start => i,
        _ => return html.to_string(),
    };
    format!(
        "{}{}\n  <script type=\"application/ld+json\">\n{}\n  </script>\n  {}{}",
        &html[..start],
        JSONLD_START_MARKER,
        json_ld,
        JSONLD_END_MARKER,
        &html[end + JSONLD_END_MARKER.len()..]
    )
}
