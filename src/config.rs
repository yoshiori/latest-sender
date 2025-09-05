use anyhow::Result;
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
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
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
}
