use anyhow::{anyhow, Result};
use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub backups: Vec<BackupConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    pub name: String,
    pub source_directory: String,
    pub file_pattern: String,
    pub webhook_url: String,
    pub check_period: Option<String>,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
}

impl BackupConfig {
    pub fn parse_check_period(&self) -> Result<Option<Duration>> {
        match &self.check_period {
            None => Ok(None),
            Some(period_str) => {
                let period = parse_duration_string(period_str)?;
                Ok(Some(period))
            }
        }
    }
}

fn parse_duration_string(duration_str: &str) -> Result<Duration> {
    let std_duration = humantime::parse_duration(duration_str)
        .map_err(|e| anyhow!("Invalid duration format '{duration_str}': {e}"))?;

    // Convert std::time::Duration to chrono::Duration
    let chrono_duration =
        Duration::from_std(std_duration).map_err(|e| anyhow!("Duration too large: {e}"))?;

    Ok(chrono_duration)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_from_file() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(
            file,
            r#"
[[backups]]
name = "test_backup"
source_directory = "/path/to/source"
file_pattern = "*.txt"
webhook_url = "https://discord.com/api/webhooks/test"

[[backups]]
name = "another_backup"
source_directory = "/another/path"
file_pattern = "*.log"
webhook_url = "https://discord.com/api/webhooks/another"
"#
        )?;

        let config = Config::from_file(file.path())?;
        assert_eq!(config.backups.len(), 2);
        assert_eq!(config.backups[0].name, "test_backup");
        assert_eq!(config.backups[0].source_directory, "/path/to/source");
        assert_eq!(config.backups[0].file_pattern, "*.txt");
        assert_eq!(
            config.backups[0].webhook_url,
            "https://discord.com/api/webhooks/test"
        );

        assert_eq!(config.backups[1].name, "another_backup");
        assert_eq!(config.backups[1].source_directory, "/another/path");
        assert_eq!(config.backups[1].file_pattern, "*.log");
        assert_eq!(
            config.backups[1].webhook_url,
            "https://discord.com/api/webhooks/another"
        );

        Ok(())
    }

    #[test]
    fn test_parse_duration_string() -> Result<()> {
        // Test valid formats
        assert_eq!(parse_duration_string("1d")?, Duration::days(1));
        assert_eq!(parse_duration_string("24h")?, Duration::hours(24));
        assert_eq!(parse_duration_string("30m")?, Duration::minutes(30));
        assert_eq!(parse_duration_string("1w")?, Duration::weeks(1));
        assert_eq!(
            parse_duration_string("2d 3h")?,
            Duration::days(2) + Duration::hours(3)
        );

        // Test invalid formats
        assert!(parse_duration_string("invalid").is_err());
        assert!(parse_duration_string("").is_err());

        Ok(())
    }

    #[test]
    fn test_backup_config_parse_check_period() -> Result<()> {
        let config = BackupConfig {
            name: "test".to_string(),
            source_directory: "/tmp".to_string(),
            file_pattern: "*.txt".to_string(),
            webhook_url: "http://example.com".to_string(),
            check_period: Some("24h".to_string()),
        };

        let period = config.parse_check_period()?;
        assert_eq!(period, Some(Duration::hours(24)));

        let config_none = BackupConfig {
            name: "test".to_string(),
            source_directory: "/tmp".to_string(),
            file_pattern: "*.txt".to_string(),
            webhook_url: "http://example.com".to_string(),
            check_period: None,
        };

        let period_none = config_none.parse_check_period()?;
        assert_eq!(period_none, None);

        Ok(())
    }
}
