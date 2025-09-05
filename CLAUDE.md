# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`latest-sender` is a Rust CLI application that finds the latest file in specified directories and sends them to Discord webhooks. It supports multiple backup configurations through TOML configuration files.

## Architecture

The application is structured with the following modules:

- **config.rs**: Handles TOML configuration file parsing with the `BackupConfig` structure for each backup task
- **file_finder.rs**: Implements file search logic using glob patterns and identifies the latest file by modification timestamp  
- **discord_sender.rs**: Manages Discord webhook API integration for file uploads (both sync and async)
- **main.rs**: CLI entry point with argument parsing and orchestrates the backup workflow

## Build and Test Commands

```bash
# Build the project
cargo build

# Build release binary
cargo build --release

# Run tests
cargo test

# Run with verbose output
cargo run -- --verbose

# Run in dry-run mode (no actual file uploads)
cargo run -- --dry-run

# Run with custom config file
cargo run -- -c /path/to/config.toml

# Check code formatting
cargo fmt -- --check

# Run clippy linter
cargo clippy -- -D warnings
```

## Configuration Format

The application expects a TOML configuration file with the following structure:

```toml
[[backups]]
name = "backup_name"
source_directory = "/path/to/source"
file_pattern = "*.txt"
webhook_url = "https://discord.com/api/webhooks/..."
```

## Key Dependencies

- **clap**: Command-line argument parsing
- **tokio**: Async runtime for Discord API calls
- **reqwest**: HTTP client for webhook requests
- **glob**: File pattern matching
- **chrono**: Timestamp handling
- **toml/serde**: Configuration file parsing
- **anyhow**: Error handling