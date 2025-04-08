# NetScan CLI - Rust Port Scanner & Pinger

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
A simple command-line interface (CLI) tool written in Rust for basic network scanning tasks, including port scanning and host reachability checks (ping).

## Features

* **Port Scanning:** Scan a range of TCP ports on a target IP address to identify open ports.
* **Customizable Port Range:** Set the minimum and maximum ports for scanning via the interactive menu. Defaults to common ports (1-1024).
* **Host Reachability (Ping):** Check if a target host is reachable using ICMP echo requests (ping).
* **Interactive Menu:** Easy-to-use menu system for selecting actions.
* **Colorized Output:** Uses terminal colors for better readability of open/closed ports and status messages.

## Requirements

* **Rust Toolchain:** You need `rustc` and `cargo` installed. The recommended way to install is via [rustup](https://rustup.rs/).
* **Permissions (for Ping):** The ping functionality (sending/receiving ICMP packets) often requires elevated privileges on most operating systems. You might need to run the application using `sudo` (on Linux/macOS) or as an Administrator (on Windows) for the ping option (Option 2) to work correctly. Port scanning usually doesn't require special permissions.

## Installation & Building

1.  **Clone the repository:**
    ```bash
    # Replace with your actual repository URL if you host it on Git
    git clone https://your-repository-url/netscan-cli.git
    cd netscan-cli
    ```
    *(If you don't have a Git repository, just navigate to the project directory containing `Cargo.toml`)*

2.  **Build the project:**
    For a release build (optimized):
    ```bash
    cargo build --release
    ```
    For a debug build:
    ```bash
    cargo build
    ```

3.  **Locate the executable:**
    * The release executable will be at `target/release/netscan-cli` (or your project's name).
    * The debug executable will be at `target/debug/netscan-cli`.

## Usage

Run the compiled executable from your terminal:

```bash
# If built in release mode
./target/release/netscan-cli

# Or if built in debug mode
./target/debug/netscan-cli

# On Windows (adjust path slashes if needed)
.\target\release\netscan-cli.exe
