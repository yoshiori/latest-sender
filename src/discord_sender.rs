use anyhow::{Context, Result};
use reqwest::blocking::multipart;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct DiscordSender;

impl DiscordSender {
    pub fn send_file<P: AsRef<Path>>(
        webhook_url: &str,
        file_path: P,
        message: Option<&str>,
    ) -> Result<()> {
        let path = file_path.as_ref();
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .context("Failed to get file name")?;

        let mut file =
            File::open(path).with_context(|| format!("Failed to open file: {path:?}"))?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .with_context(|| format!("Failed to read file: {path:?}"))?;

        let mut form = multipart::Form::new().part(
            "file",
            multipart::Part::bytes(buffer).file_name(file_name.to_string()),
        );

        if let Some(msg) = message {
            form = form.text("content", msg.to_string());
        }

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(webhook_url)
            .multipart(form)
            .send()
            .context("Failed to send request to Discord")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .unwrap_or_else(|_| "No error message".to_string());
            anyhow::bail!("Discord API returned error: {} - {}", status, error_text);
        }

        Ok(())
    }

    pub async fn send_file_async<P: AsRef<Path>>(
        webhook_url: &str,
        file_path: P,
        message: Option<&str>,
    ) -> Result<()> {
        let path = file_path.as_ref();
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .context("Failed to get file name")?;

        let buffer = tokio::fs::read(path)
            .await
            .with_context(|| format!("Failed to read file: {path:?}"))?;

        let mut form = reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::bytes(buffer).file_name(file_name.to_string()),
        );

        if let Some(msg) = message {
            form = form.text("content", msg.to_string());
        }

        let client = reqwest::Client::new();
        let response = client
            .post(webhook_url)
            .multipart(form)
            .send()
            .await
            .context("Failed to send request to Discord")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "No error message".to_string());
            anyhow::bail!("Discord API returned error: {} - {}", status, error_text);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_send_file_success() -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "Test content")?;

        let mut server = Server::new();
        let _m = server
            .mock("POST", "/api/webhooks/test")
            .with_status(204)
            .create();

        let webhook_url = format!("{}/api/webhooks/test", server.url());

        DiscordSender::send_file(&webhook_url, temp_file.path(), Some("Test message"))?;

        Ok(())
    }

    #[test]
    fn test_send_file_discord_error() -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "Test content")?;

        let mut server = Server::new();
        let _m = server
            .mock("POST", "/api/webhooks/test")
            .with_status(400)
            .with_body(r#"{"message": "Invalid webhook token"}"#)
            .create();

        let webhook_url = format!("{}/api/webhooks/test", server.url());

        let result = DiscordSender::send_file(&webhook_url, temp_file.path(), None);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Discord API returned error"));

        Ok(())
    }

    #[tokio::test]
    async fn test_send_file_async_success() -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "Test content")?;

        let mut server = Server::new_async().await;
        let _m = server
            .mock("POST", "/api/webhooks/test")
            .with_status(204)
            .create_async()
            .await;

        let webhook_url = format!("{}/api/webhooks/test", server.url());

        DiscordSender::send_file_async(&webhook_url, temp_file.path(), Some("Test message"))
            .await?;

        Ok(())
    }
}
