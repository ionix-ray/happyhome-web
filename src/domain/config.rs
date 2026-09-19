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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteMeta {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub tagline: String,
    #[serde(default)]
    pub logo_url: String,
    #[serde(default)]
    pub favicon_url: String,
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
    pub og_image: String,
    #[serde(default)]
    pub canonical: String,
    #[serde(default)]
    pub schema_type: String,
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
