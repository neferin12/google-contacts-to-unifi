# Google Contacts to Unifi Talk Converter (gctu)

A simple command-line tool written in Rust to convert Google Contacts exports (Google CSV format) into the CSV format expected by Unifi Talk.

## Requirements

- Rust and Cargo

## Installation

You can install the CLI globally using Cargo:

```bash
cargo install --path .
```

This will compile the tool and place the `gctu` binary in your `~/.cargo/bin` directory, making it available globally.

Alternatively, you can just build it from source without installing globally:

```bash
cargo build --release
```

The executable will be located in `target/release/gctu`.

## Usage

1. Export your contacts from Google Contacts using the **Google CSV** format.
2. Run the `gctu` executable to convert the file:

```bash
gctu -i /path/to/google_contacts.csv -o unifi_talk_contacts.csv
```

### Options

- `-i, --input <FILE>`: Path to the input Google Contacts CSV file (Required).
- `-o, --output <FILE>`: Path to the output Unifi Talk CSV file (Default: `unifi_talk_contacts.csv`).
- `-h, --help`: Print help information.
- `-V, --version`: Print version information.

## Testing

Run tests using cargo:

```bash
cargo test
```
