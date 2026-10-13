use crate::domain::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteConfig {
    #[serde(default)]
    pub site: SiteMeta,
    #[serde(default)]
    pub hospital: BrandConfig,
    #[serde(default)]
    pub rehab: BrandConfig,
    #[serde(default)]
    pub contact: ContactInfo,
    #[serde(default)]
    pub social: SocialLinks,
    #[serde(default)]
    pub footer: FooterConfig,
    #[serde(default)]
    pub seo: SeoConfig,
    #[serde(default)]
    pub emergency: EmergencyBanner,
    #[serde(default)]
    pub analytics: AnalyticsConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteMeta {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub tagline: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BrandConfig {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub tagline: String,
    #[serde(default)]
    pub services: Vec<Service>,
    #[serde(default)]
    pub team: Vec<TeamMember>,
    #[serde(default)]
    pub why_us: Vec<WhyUsFeature>,
    #[serde(default)]
    pub testimonials: Vec<Testimonial>,
    #[serde(default)]
    pub faqs: Vec<FaqItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SeoConfig {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub canonical: String,
    #[serde(default)]
    pub schema_type: String,
    #[serde(default)]
    pub image: String,
    #[serde(default)]
    pub price_range: String,
    #[serde(default)]
    pub sitemap_lastmod: String,
    #[serde(default)]
    pub geo_lat: f64,
    #[serde(default)]
    pub geo_lng: f64,
    #[serde(default)]
    pub area_served: Vec<String>,
    #[serde(default)]
    pub address: SchemaAddress,
    #[serde(default)]
    pub opening_hours: OpeningHours,
    #[serde(default)]
    pub emergency_hours: OpeningHours,
}

/// PostalAddress components for schema.org — kept separate from the free-form
/// visible `contact.address` so NAP consistency can be asserted by tests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SchemaAddress {
    #[serde(default)]
    pub street: String,
    #[serde(default)]
    pub locality: String,
    #[serde(default)]
    pub region: String,
    #[serde(default)]
    pub postal_code: String,
    #[serde(default)]
    pub country: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OpeningHours {
    #[serde(default)]
    pub days: Vec<String>,
    #[serde(default)]
    pub opens: String,
    #[serde(default)]
    pub closes: String,
}

/// Analytics integrations. Values come from TOML only (Rule 11).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AnalyticsConfig {
    #[serde(default)]
    pub clarity_project_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FooterConfig {
    #[serde(default)]
    pub about_text: String,
    #[serde(default)]
    pub quick_links: Vec<FooterLink>,
    #[serde(default)]
    pub legal_links: Vec<FooterLink>,
}
