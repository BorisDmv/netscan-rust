use std::io::{self, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;
use colored::Colorize; // Import the colored crate
use std::process::Stdio;
use tokio::process::Command;

// Function to scan ports (remains largely the same, but added IpAddr type)
fn scan_ports(target_ip: IpAddr, min_port: u16, max_port: u16) -> Vec<u16> {
    let mut open_ports = Vec::new();
    for port in min_port..=max_port {
        let address = SocketAddr::new(target_ip, port);
        match TcpStream::connect_timeout(&address, Duration::from_millis(200)) {
            Ok(_) => {
                open_ports.push(port);
            }
            Err(_) => {}
        }
    }
    open_ports
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

    loop {
        println!("Choose an option:");
        println!("1. Check if a host is reachable (Ping)");
        println!("2. Scan ports on a range of IPs");
        println!("3. Exit");
        print!("Enter your choice: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();

        match choice {
             "1" => {
                let target_ip = read_ip_input("Enter the target IP address to ping: ")?;
                // The result is already printed inside is_host_reachable
                let _ = is_host_reachable(target_ip).await;
            }
            "2" => {
                println!("Enter the IP range to scan:");
                let min_ip = read_ip_input("Enter the minimum IP address: ")?;
                let max_ip = read_ip_input("Enter the maximum IP address: ")?;
                print!("Enter ports to scan (comma separated, e.g. 22,80,443): ");
                io::stdout().flush()?;
                let mut ports_input = String::new();
                io::stdin().read_line(&mut ports_input)?;
                let ports_to_check: Vec<u16> = ports_input.trim().split(',')
                    .filter_map(|s| s.trim().parse::<u16>().ok())
                    .collect();
                if ports_to_check.is_empty() {
                    eprintln!("{}", "No valid ports entered. Aborting scan.".red());
                    continue;
                }
                // Only support IPv4 for range scan
                let (min_v4, max_v4) = match (min_ip, max_ip) {
                    (IpAddr::V4(min), IpAddr::V4(max)) => (min, max),
                    _ => {
                        eprintln!("{}", "Only IPv4 ranges are supported for range scan.".red());
                        continue;
                    }
                };
                let mut current = u32::from(min_v4);
                let end = u32::from(max_v4);
                while current <= end {
                    let ip = std::net::Ipv4Addr::from(current);
                    println!("Checking IP: {}", ip);
                    let mut open_ports = Vec::new();
                    for &port in &ports_to_check {
                        let result = scan_ports(IpAddr::V4(ip), port, port);
                        if !result.is_empty() {
                            open_ports.push(port);
                        }
                    }
                    if !open_ports.is_empty() {
                        let ports_str = format!("{:?}", open_ports).green();
                        println!("{} - {} port open", ip, ports_str);
                    } else {
                        println!("{} - no ports open", ip.to_string().red());
                    }
                    current += 1;
                }
            }
            "3" => {
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