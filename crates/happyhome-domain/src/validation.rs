use crate::types::{ContactForm, ContactFormInput};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ValidationError {
    #[error("Name cannot be empty")]
    EmptyName,
    #[error("Name exceeds maximum length of 100 characters")]
    NameTooLong,
    #[error("Phone cannot be empty")]
    EmptyPhone,
    #[error("Invalid phone format")]
    InvalidPhone,
    #[error("Email cannot be empty")]
    EmptyEmail,
    #[error("Invalid email format")]
    InvalidEmail,
    #[error("Message exceeds maximum length of 1000 characters")]
    MessageTooLong,
    #[error("Input contains malicious script or disallowed characters")]
    DisallowedContent,
}

pub fn sanitize_text(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

pub fn validate_contact_form(form: &ContactFormInput) -> Result<ContactForm, ValidationError> {
    let name = form.name.trim();
    let phone = form.phone.trim();
    let email = form.email.trim();
    let message = form.message.trim();

    if name.is_empty() {
        return Err(ValidationError::EmptyName);
    }
    if name.len() > 100 {
        return Err(ValidationError::NameTooLong);
    }
    if contains_xss_payload(name) {
        return Err(ValidationError::DisallowedContent);
    }

    if phone.is_empty() {
        return Err(ValidationError::EmptyPhone);
    }
    if !is_valid_phone(phone) {
        return Err(ValidationError::InvalidPhone);
    }

    if email.is_empty() {
        return Err(ValidationError::EmptyEmail);
    }
    if !is_valid_email(email) {
        return Err(ValidationError::InvalidEmail);
    }

    if message.len() > 1000 {
        return Err(ValidationError::MessageTooLong);
    }
    if contains_xss_payload(message) {
        return Err(ValidationError::DisallowedContent);
    }

    Ok(ContactForm {
        name: sanitize_text(name),
        phone: phone.to_string(),
        email: email.to_string(),
        service: sanitize_text(&form.service),
        message: sanitize_text(message),
    })
}

pub fn is_valid_email(email: &str) -> bool {
    if email.len() > 120 {
        return false;
    }
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    let (local, domain) = (parts[0], parts[1]);
    if local.is_empty() || domain.is_empty() {
        return false;
    }
    if !domain.contains('.') {
        return false;
    }
    let domain_parts: Vec<&str> = domain.split('.').collect();
    if domain_parts.iter().any(|p| p.is_empty()) {
        return false;
    }
    let tld = domain_parts.last().unwrap();
    if tld.len() < 2 {
        return false;
    }
    true
}

pub fn is_valid_phone(phone: &str) -> bool {
    let cleaned = phone.replace(|c: char| c.is_whitespace() || c == '+' || c == '-' || c == '(' || c == ')', "");
    if cleaned.is_empty() || !cleaned.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    // E.164 phone length range: 7 to 15 digits
    let digits_count = cleaned.len();
    (7..=15).contains(&digits_count)
}

pub fn contains_xss_payload(text: &str) -> bool {
    let lower = text.to_lowercase();
    lower.contains("<script")
        || lower.contains("javascript:")
        || lower.contains("onload=")
        || lower.contains("onerror=")
        || lower.contains("<iframe")
        || lower.contains("alert(")
        || lower.contains("document.cookie")
}

