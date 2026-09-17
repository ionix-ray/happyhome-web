use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum BrandMode {
    Hospital,
    #[default]
    Rehab,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ContactFormInput {
    #[serde(default)] pub name: String,
    #[serde(default)] pub phone: String,
    #[serde(default)] pub email: String,
    #[serde(default)] pub service: String,
    #[serde(default)] pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContactForm {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub service: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamMember {
    #[serde(default)] pub name: String,
    #[serde(default)] pub role: String,
    #[serde(default)] pub title: String,
    #[serde(default)] pub specialty: String,
    #[serde(default)] pub image_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Service {
    #[serde(default)] pub name: String,
    #[serde(default)] pub slug: String,
    #[serde(default)] pub description: String,
    #[serde(default)] pub icon_key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WhyUsFeature {
    #[serde(default)] pub title: String,
    #[serde(default)] pub description: String,
    #[serde(default)] pub icon_key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Testimonial {
    #[serde(default)] pub name: String,
    #[serde(default)] pub role: String,
    #[serde(default)] pub text: String,
    #[serde(default)] pub stars: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FaqItem {
    #[serde(default)] pub question: String,
    #[serde(default)] pub answer: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SocialLinks {
    #[serde(default)] pub facebook: Option<String>,
    #[serde(default)] pub twitter: Option<String>,
    #[serde(default)] pub linkedin: Option<String>,
    #[serde(default)] pub youtube: Option<String>,
    #[serde(default)] pub instagram: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ContactInfo {
    #[serde(default)] pub address: String,
    #[serde(default)] pub phone_primary: String,
    #[serde(default)] pub phone_whatsapp: String,
    #[serde(default)] pub phone_secondary: String,
    #[serde(default)] pub email: String,
    #[serde(default)] pub hours: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FooterLink {
    #[serde(default)] pub label: String,
    #[serde(default)] pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct NavItem {
    #[serde(default)] pub label: String,
    #[serde(default)] pub anchor: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EmergencyBanner {
    #[serde(default)] pub message: String,
    #[serde(default)] pub phone: String,
    #[serde(default)] pub whatsapp_phone: String,
}
