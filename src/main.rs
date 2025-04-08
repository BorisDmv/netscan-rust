use std::io::{self, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;
use colored::Colorize; // Import the colored crate
use std::process::Stdio;
use tokio::process::Command;

// Function to scan ports (remains largely the same, but added IpAddr type)
fn scan_ports(target_ip: IpAddr, min_port: u16, max_port: u16) {
    println!("Scanning ports {} through {} on {}...", min_port, max_port, target_ip);
    let mut open_ports = 0;
    for port in min_port..=max_port {
        let address = SocketAddr::new(target_ip, port);
        // Increased timeout slightly for potentially slower networks
        match TcpStream::connect_timeout(&address, Duration::from_millis(200)) {
            Ok(_) => {
                println!("[{}] Port {} is {}", "open".green(), port, "open".green());
                open_ports += 1;
            }
            // Optionally hide closed ports for less verbose output
            // Err(_) => {} // Uncomment this line to hide closed ports
            Err(_) => println!("[{}] Port {} is {}", "closed".red(), port, "closed".red()),
        }
    }
    println!("Port scan complete. Found {} open ports.", open_ports);
}

// Async function to check host reachability using tokio-ping
pub async fn is_host_reachable(target_ip: IpAddr) -> bool {
    let ip_str = target_ip.to_string();

    let output = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg(&ip_str)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .await;

    match output {
        Ok(out) if out.status.success() => {
            println!("Host {} is reachable", ip_str.green());
            true
        }
        Ok(_) => {
            println!("Host {} is not reachable", ip_str.yellow());
            false
        }
        Err(e) => {
            eprintln!("Failed to ping {}: {}", ip_str.red(), e);
            false
        }
    }
}


// Helper function to read and parse u16 input
fn read_u16_input(prompt: &str) -> io::Result<u16> {
    loop {
        print!("{}", prompt);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().parse::<u16>() {
            Ok(num) => return Ok(num),
            Err(_) => eprintln!("{}", "Invalid input. Please enter a number between 0 and 65535.".red()),
        }
    }
}

// Helper function to read and parse IP address input
fn read_ip_input(prompt: &str) -> io::Result<IpAddr> {
    loop {
        print!("{}", prompt);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match IpAddr::from_str(input.trim()) {
            Ok(ip) => return Ok(ip),
            Err(e) => eprintln!("{} '{}': {}", "Invalid IP address format".red(), input.trim(), e),
        }
    }
}


#[tokio::main]
async fn main() -> io::Result<()> {
    let mut min_port: u16 = 1;
    let mut max_port: u16 = 1024; // Common well-known ports

    loop {
        println!("\nCurrent Port Range: {}-{}", min_port, max_port);
        println!("Choose an option:");
        println!("1. Scan ports on a target IP");
        println!("2. Check if a host is reachable (Ping)");
        println!("3. Change port scan range");
        println!("4. Exit");
        print!("Enter your choice: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();

        match choice {
            "1" => {
                let target_ip = read_ip_input("Enter the target IP address to scan: ")?;

                // Optional: Check reachability before scanning
                if !is_host_reachable(target_ip).await {
                     println!("{}", "Host seems unreachable, port scan might fail or be inaccurate.".yellow());
                     // Optionally ask user if they want to continue
                     // print!("Continue scan anyway? (y/N): ");
                     // io::stdout().flush()?;
                     // let mut confirm = String::new();
                     // io::stdin().read_line(&mut confirm)?;
                     // if !confirm.trim().eq_ignore_ascii_case("y") {
                     //     continue; // Skip scan if host unreachable and user doesn't confirm
                     // }
                     continue; 
                }

                scan_ports(target_ip, min_port, max_port);
            }
             "2" => {
                let target_ip = read_ip_input("Enter the target IP address to ping: ")?;
                // The result is already printed inside is_host_reachable
                let _ = is_host_reachable(target_ip).await;
            }
            "3" => {
                println!("\nEnter new port range (0-65535)");
                let new_min = read_u16_input("Enter the minimum port number: ")?;
                let new_max = read_u16_input("Enter the maximum port number: ")?;

                if new_min > new_max {
                    eprintln!("{}", "Error: Minimum port cannot be greater than maximum port.".red());
                } else {
                    min_port = new_min;
                    max_port = new_max;
                    println!("Port range updated to {}-{}", min_port, max_port);
                }
            }
            "4" => {
                println!("Exiting application.");
                break;
            }
            _ => {
                println!("{}", "Invalid choice. Please enter a number from the options.".red());
            }
        }
    }

    Ok(())
}