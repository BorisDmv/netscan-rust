# NetScan CLI - Rust Network Scanner & Pinger

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
A simple command-line interface (CLI) tool written in Rust for basic network scanning tasks, including port scanning across IP ranges and host reachability checks (ping).

## Features

* **Port Scanning:** Scan a custom list of TCP ports on a single IP or across an IPv4 range to identify open ports.
* **Customizable Port List:** Enter a comma-separated list of ports to scan (e.g. `22,80,443`).
* **IPv4 Range Scanning:** Scan multiple IPv4 addresses by specifying a start and end IP.
* **Host Reachability (Ping):** Check if a target host is reachable using ICMP echo requests (ping).
* **Interactive Menu:** Easy-to-use menu system for selecting actions:
    1. Check if a host is reachable (Ping)
    2. Scan ports on a range of IPs
    3. Exit
* **Colorized Output:** Uses terminal colors for better readability of open/closed ports and status messages.

## Requirements

* **Rust Toolchain:** You need `rustc` and `cargo` installed. The recommended way to install is via [rustup](https://rustup.rs/).
* **Permissions (for Ping):** The ping functionality (sending/receiving ICMP packets) often requires elevated privileges on most operating systems. You might need to run the application using `sudo` (on Linux/macOS) or as an Administrator (on Windows) for the ping option (Option 1) to work correctly. Port scanning usually doesn't require special permissions.

## Installation & Building

1.  **Clone the repository:**
    ```bash
    git clone git@github.com:BorisDmv/netscan-rust.git
    cd netscan-rust
    ```

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
    * The release executable will be at `target/release/netscan-rust`.
    * The debug executable will be at `target/debug/netscan-rust`.

## Usage

Run the compiled executable from your terminal:

```bash
# If built in release mode
./target/release/netscan-rust

# Or if built in debug mode
./target/debug/netscan-rust

# On Windows (adjust path slashes if needed)
.\target\release\netscan-rust.exe
