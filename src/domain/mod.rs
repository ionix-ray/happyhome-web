pub mod config;
pub mod types;
pub mod validation;

#[cfg(test)]
mod tests {
    use super::config::*;
    use super::types::*;
    use super::validation::*;

    #[test]
    fn test_validate_contact_form_valid() {
        let input = ContactFormInput {
            name: "John Doe".into(),
            phone: "+91 8249914673".into(),
            email: "john@example.com".into(),
            service: "Rehab".into(),
            message: "Help".into(),
        };
        assert!(validate_contact_form(&input).is_ok());
    }

    #[test]
    fn test_validate_contact_form_empty_name() {
        let input = ContactFormInput {
            name: "".into(),
            phone: "1234567890".into(),
            email: "a@b.com".into(),
            ..Default::default()
        };
        assert_eq!(
            validate_contact_form(&input).unwrap_err(),
            ValidationError::EmptyName
        );
    }

    #[test]
    fn test_validate_contact_form_invalid_email() {
        let input = ContactFormInput {
            name: "John".into(),
            phone: "1234567890".into(),
            email: "notanemail".into(),
            ..Default::default()
        };
        assert_eq!(
            validate_contact_form(&input).unwrap_err(),
            ValidationError::InvalidEmail
        );
    }

    #[test]
    fn test_validate_contact_form_invalid_phone() {
        let input = ContactFormInput {
            name: "John".into(),
            phone: "abc".into(),
            email: "a@b.com".into(),
            ..Default::default()
        };
        assert_eq!(
            validate_contact_form(&input).unwrap_err(),
            ValidationError::InvalidPhone
        );
    }

    #[test]
    fn test_site_config_deserializes() {
        let toml_str = r#"
        [site]
        name = "Test Site"
        tagline = "Test Tagline"
        logo_url = ""
        favicon_url = ""
        "#;
        let config: SiteConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.site.name, "Test Site");
    }

    #[test]
    fn test_brand_config_has_services() {
        let brand = BrandConfig {
            services: vec![Service {
                name: "A".into(),
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_eq!(brand.services.len(), 1);
    }

    #[test]
    fn test_all_types_implement_clone_partial_eq() {
        let service = Service::default();
        let service_clone = service.clone();
        assert_eq!(service, service_clone);
    }

    #[test]
    fn test_validate_contact_form_name_too_long() {
        let long_name = "A".repeat(101);
        let input = ContactFormInput {
            name: long_name,
            phone: "+91 8249914673".into(),
            email: "test@example.com".into(),
            service: "Neuro".into(),
            message: "Hello".into(),
        };
        assert_eq!(
            validate_contact_form(&input).unwrap_err(),
            ValidationError::NameTooLong
        );
    }

    #[test]
    fn test_validate_contact_form_message_too_long() {
        let long_message = "M".repeat(1001);
        let input = ContactFormInput {
            name: "Rahul Verma".into(),
            phone: "+91 8249914673".into(),
            email: "rahul@example.com".into(),
            service: "Stroke Rehab".into(),
            message: long_message,
        };
        assert_eq!(
            validate_contact_form(&input).unwrap_err(),
            ValidationError::MessageTooLong
        );
    }

    #[test]
    fn test_validate_contact_form_blocks_xss_in_name() {
        let input = ContactFormInput {
            name: "<script>alert('pwned')</script>".into(),
            phone: "+91 8249914673".into(),
            email: "test@example.com".into(),
            service: "General".into(),
            message: "Routine checkup".into(),
        };
        assert_eq!(
            validate_contact_form(&input).unwrap_err(),
            ValidationError::DisallowedContent
        );
    }

    #[test]
    fn test_validate_contact_form_blocks_xss_in_message() {
        let input = ContactFormInput {
            name: "Anjali Patnaik".into(),
            phone: "+91 8249914673".into(),
            email: "anjali@example.com".into(),
            service: "Physiotherapy".into(),
            message: "<iframe src='javascript:document.cookie'></iframe>".into(),
        };
        assert_eq!(
            validate_contact_form(&input).unwrap_err(),
            ValidationError::DisallowedContent
        );
    }

    #[test]
    fn test_sanitizes_html_characters_on_success() {
        let input = ContactFormInput {
            name: "Ramesh & Sons".into(),
            phone: "9876543210".into(),
            email: "ramesh@example.com".into(),
            service: "Spine Care".into(),
            message: "Need quote for 'inpatient' beds & nursing".into(),
        };
        let valid = validate_contact_form(&input).expect("Should sanitize and succeed");
        assert_eq!(valid.name, "Ramesh &amp; Sons");
        assert!(valid.message.contains("&amp;"));
        assert!(valid.message.contains("&#x27;inpatient&#x27;"));
    }

    #[test]
    fn test_phone_validation_varieties() {
        assert!(is_valid_phone("+91 8249914673"));
        assert!(is_valid_phone("0674-2345678"));
        assert!(is_valid_phone("(0674) 2345678"));
        assert!(is_valid_phone("9876543210"));
        // Too short (< 7 digits)
        assert!(!is_valid_phone("12345"));
        // Too long (> 15 digits)
        assert!(!is_valid_phone("12345678901234567"));
        // Alphabetic / invalid characters
        assert!(!is_valid_phone("+91-INVALID"));
    }

    #[test]
    fn test_email_validation_varieties() {
        assert!(is_valid_email("doctor@happyhomehospital.com"));
        assert!(is_valid_email("patient.help@domain.co.in"));
        assert!(!is_valid_email("plainaddress"));
        assert!(!is_valid_email("@missingusername.com"));
        assert!(!is_valid_email("username@.com"));
        assert!(!is_valid_email("username@domain"));
        assert!(!is_valid_email("user@domain.c"));
        let too_long = format!("{}@example.com", "a".repeat(125));
        assert!(!is_valid_email(&too_long));
    }

    #[test]
    fn test_full_production_site_toml_loads_and_verifies_integrity() {
        const PROD_TOML: &str = include_str!("../../config/site.toml");
        let config: SiteConfig =
            toml::from_str(PROD_TOML).expect("Production site.toml must parse");

        // Brand invariant checks
        assert_eq!(
            config.hospital.services.len(),
            6,
            "Hospital must have exactly 6 core divisions"
        );
        assert_eq!(
            config.rehab.services.len(),
            9,
            "Rehab must have exactly 9 specialized recovery tracks"
        );

        // Doctor team invariant checks
        assert!(
            !config.hospital.team.is_empty(),
            "Hospital team must not be empty"
        );
        assert!(
            !config.rehab.team.is_empty(),
            "Rehab team must not be empty"
        );
        assert!(
            config
                .rehab
                .team
                .iter()
                .any(|d| d.name.contains("Dr. Vijay Kumar Sahu")),
            "Dr. Sahu must be present in rehab team"
        );

        // Emergency and contact validation
        assert_eq!(config.emergency.phone, "+91 8249914673");
        assert!(config.contact.address.contains("Bhubaneswar"));
        assert!(config.contact.address.contains("Ghatikia"));
    }
}
