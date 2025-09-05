use anyhow::{Context, Result};
use clap::Parser;
use latest_sender::{config::Config, discord_sender::DiscordSender, file_finder::FileFinder};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(
    name = "latest-sender",
    version = "0.1.0",
    about = "Find and send the latest file to Discord webhook"
)]
struct Args {
    #[clap(
        short,
        long,
        value_name = "FILE",
        help = "Path to the configuration file",
        default_value = "config.toml"
    )]
    config: PathBuf,

    #[clap(short, long, help = "Run in dry-run mode (don't actually send files)")]
    dry_run: bool,

    #[clap(short, long, help = "Enable verbose output")]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("Loading configuration from: {:?}", args.config);
    }

    let config = Config::from_file(&args.config)
        .with_context(|| format!("Failed to load config from {:?}", args.config))?;

    if config.backups.is_empty() {
        println!("No backup configurations found in the config file");
        return Ok(());
    }

    let mut total_sent = 0;
    let mut total_skipped = 0;

    for backup in &config.backups {
        println!("\nProcessing backup: {}", backup.name);

        match FileFinder::find_latest_file(&backup.source_directory, &backup.file_pattern) {
            Ok(Some(file_path)) => {
                println!("  Found latest file: {file_path:?}");

                if args.dry_run {
                    println!("  [DRY RUN] Would send file to webhook");
                    total_skipped += 1;
                } else {
                    print!("  Sending file to Discord webhook...");
                    match DiscordSender::send_file(
                        &backup.webhook_url,
                        &file_path,
                        Some(&format!("Latest backup from: {}", backup.name)),
                    ) {
                        Ok(_) => {
                            println!(" ✓ Success!");
                            total_sent += 1;
                        }
                        Err(e) => {
                            println!(" ✗ Failed!");
                            eprintln!("  Error: {e}");
                            if args.verbose {
                                eprintln!("  Debug: {e:?}");
                            }
                        }
                    }
                }
            }
            Ok(None) => {
                println!("  No files found matching pattern: {}", backup.file_pattern);
                total_skipped += 1;
            }
            Err(e) => {
                eprintln!("  Error searching for files: {e}");
                if args.verbose {
                    eprintln!("  Debug: {e:?}");
                }
            }
        }
    }

    println!("\n{}", "=".repeat(50));
    println!("Summary:");
    println!("  Total backups processed: {}", config.backups.len());
    println!("  Files sent: {total_sent}");
    println!("  Files skipped: {total_skipped}");

    if args.dry_run {
        println!("\n[DRY RUN MODE] No files were actually sent");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parsing() {
        let args =
            Args::parse_from(&["latest-sender", "-c", "test.toml", "--dry-run", "--verbose"]);
        assert_eq!(args.config, PathBuf::from("test.toml"));
        assert!(args.dry_run);
        assert!(args.verbose);
    }

    #[test]
    fn test_default_args() {
        let args = Args::parse_from(&["latest-sender"]);
        assert_eq!(args.config, PathBuf::from("config.toml"));
        assert!(!args.dry_run);
        assert!(!args.verbose);
    }
}
