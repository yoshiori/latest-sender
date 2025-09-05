# latest-sender

A Rust CLI tool that automatically finds the latest file from specified directories and sends it to Discord webhooks.

## Features

- Manage multiple backup configurations in one place
- Flexible file search with glob patterns
- Automatically identify files with the latest timestamp
- Automatic file upload to Discord webhooks
- Safe testing with dry-run mode
- Verbose logging option

## Installation

### Build from source

```bash
git clone https://github.com/yoshiori/latest-sender.git
cd latest-sender
cargo build --release

# Binary will be generated at target/release/latest-sender
```

## Usage

### Configuration Setup

Copy `config.example.toml` to `config.toml` and edit the configuration:

```toml
[[backups]]
name = "database_backup"
source_directory = "/var/backups/database"
file_pattern = "*.sql"
webhook_url = "https://discord.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_WEBHOOK_TOKEN"

[[backups]]
name = "log_archive"  
source_directory = "/var/log/archive"
file_pattern = "*.tar.gz"
webhook_url = "https://discord.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_WEBHOOK_TOKEN"
```

### Running

Basic execution:
```bash
./latest-sender
```

Specify custom configuration file:
```bash
./latest-sender -c /path/to/config.toml
```

Dry-run mode (doesn't actually send files):
```bash
./latest-sender --dry-run
```

Verbose output:
```bash
./latest-sender --verbose
```

### Command Line Options

- `-c, --config <FILE>` - Path to configuration file (default: config.toml)
- `-d, --dry-run` - Dry-run mode (doesn't actually send files)
- `-v, --verbose` - Enable verbose output
- `-h, --help` - Display help information
- `-V, --version` - Display version information

## Development

### Setup Pre-commit Hooks

To ensure code quality, install pre-commit hooks that will automatically run `cargo fmt`, `cargo clippy`, and tests before each commit:

```bash
# Run the setup script
./setup-hooks.sh

# Or manually install pre-commit and hooks
pip install pre-commit
pre-commit install
```

### Build

```bash
cargo build
```

### Run Tests

```bash
cargo test
```

### Release Build

```bash
cargo build --release
```

### Code Formatting

```bash
cargo fmt
```

### Run Linter

```bash
cargo clippy -- -D warnings
```

## License

MIT License - See LICENSE file for details.