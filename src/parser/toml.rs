use std::fs;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub common: Common,
    pub header: Header
}

#[derive(Debug, Deserialize)]
pub struct Common {
    pub page_width: u32,
    pub page_height: u32,
    pub top_margin: u32,
    pub bottom_margin: u32,
    pub left_margin: u32,
    pub right_margin: u32,
    pub leading: u32,
    pub font_type: String,
    pub font_size: u32
}

#[derive(Debug, Deserialize)]
pub struct Header {
    pub h1_size: u32,
    pub h2_size: u32,
    pub h3_size: u32,
    pub h4_size: u32,
    pub h5_size: u32,
    pub h6_size: u32
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read configuration file")]
    FileReadError(#[from] std::io::Error),
    #[error("failed to parse TOML")]
    TomlParseError(#[from] toml::de::Error),
}

impl Config {
    pub fn read_file(file_path: &str) -> Result<Self, ConfigError> {
        let config_content = fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_read_file_success() {
        // Example TOML content
        let toml_content = r#"
            [common]
            page_width = 210
            page_height = 297
            top_margin = 10
            bottom_margin = 10
            left_margin = 10
            right_margin = 10
            leading = 5
            font_type = "Arial"
            font_size = 12

            [header]
            h1_size = 24
            h2_size = 20
            h3_size = 18
            h4_size = 16
            h5_size = 14
            h6_size = 12
        "#;

        // Write the TOML content to a temporary file
        let file_path = "test_config.toml";
        let mut file = File::create(file_path).expect("Failed to create test file");
        file.write_all(toml_content.as_bytes()).expect("Failed to write to test file");

        // Call the read_file function and verify the result
        let config = Config::read_file(file_path).expect("Failed to read config");
        assert_eq!(config.common.page_width, 210);
        assert_eq!(config.common.page_height, 297);
        assert_eq!(config.common.top_margin, 10);
        assert_eq!(config.common.bottom_margin, 10);
        assert_eq!(config.common.left_margin, 10);
        assert_eq!(config.common.right_margin, 10);
        assert_eq!(config.common.leading, 5);
        assert_eq!(config.common.font_type, "Arial");
        assert_eq!(config.common.font_size, 12);

        assert_eq!(config.header.h1_size, 24);
        assert_eq!(config.header.h2_size, 20);
        assert_eq!(config.header.h3_size, 18);
        assert_eq!(config.header.h4_size, 16);
        assert_eq!(config.header.h5_size, 14);
        assert_eq!(config.header.h6_size, 12);

        // Clean up the temporary file
        fs::remove_file(file_path).expect("Failed to delete test file");
    }

    #[test]
    fn test_read_file_missing_file() {
        // Call the read_file function with a non-existent file path
        let result = Config::read_file("non_existent_file.toml");

        // Verify that an error is returned
        assert!(result.is_err());
        if let Err(e) = result {
            match e {
                ConfigError::FileReadError(_) => (),
                _ => panic!("Expected FileReadError, got {:?}", e),
            }
        }
    }

    #[test]
    fn test_read_file_invalid_toml() {
        // Example invalid TOML content
        let toml_content = r#"
            [common]
            page_width = "invalid"
        "#;

        // Write the invalid TOML content to a temporary file
        let file_path = "invalid_test_config.toml";
        let mut file = File::create(file_path).expect("Failed to create test file");
        file.write_all(toml_content.as_bytes()).expect("Failed to write to test file");

        // Call the read_file function and verify that an error is returned
        let result = Config::read_file(file_path);
        assert!(result.is_err());
        if let Err(e) = result {
            match e {
                ConfigError::TomlParseError(_) => (),
                _ => panic!("Expected TomlParseError, got {:?}", e),
            }
        }

        // Clean up the temporary file
        fs::remove_file(file_path).expect("Failed to delete test file");
    }
}