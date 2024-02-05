use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};
use std::process::Command;

fn main() {
    // Get CPU information
    let cpu_info = Command::new("wmic")
        .args(&["cpu", "get", "name"])
        .output()
        .expect("Failed to execute command")
        .stdout;

    let cpu_str = String::from_utf8_lossy(&cpu_info);

    // Extract CPU details
    let cpu_details = cpu_str
        .lines()
        .skip(1) // Skip the header line
        .next(); // Take the first line

    let cpu_formatted = match cpu_details {
        Some(details) => format!("CPU: {}", details.trim()),
        None => "CPU: Unknown".to_string(),
    };

    // Get GPU information
    let gpu_info = Command::new("wmic")
        .args(&["path", "win32_videocontroller", "get", "caption"])
        .output()
        .expect("Failed to execute command")
        .stdout;

    let gpu_str = String::from_utf8_lossy(&gpu_info)
        .lines()
        .skip(1) // Skip the header line
        .next() // Take the first line
        .map(|line| format!("GPU: {}", line.trim()))
        .unwrap_or_else(|| "GPU: Unknown".to_string());

    // Get screen resolution
    let screen_resolution_info = Command::new("wmic")
        .args(&[
            "path",
            "Win32_VideoController",
            "get",
            "CurrentHorizontalResolution,CurrentVerticalResolution",
            "/format:value",
        ])
        .output()
        .expect("Failed to execute command")
        .stdout;

    let screen_resolution_str = String::from_utf8_lossy(&screen_resolution_info);

    // Extract numerical values from screen resolution
    let resolution_str = screen_resolution_str
        .lines()
        .filter_map(|line| {
            if line.contains('=') {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() == 2 {
                    Some(parts[1].trim())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<&str>>()
        .join("x");

    // Get RAM information
    let ram_info = Command::new("wmic")
        .args(&["memorychip", "get", "capacity"])
        .output()
        .expect("Failed to execute command")
        .stdout;

    let ram_capacity_vec: Vec<u64> = io::Cursor::new(ram_info)
        .lines()
        .skip(1) // Skip the header line
        .filter_map(|line| line.unwrap().trim().parse().ok())
        .collect();

    let total_ram_capacity_gb = ram_capacity_vec.iter().sum::<u64>() / (1024 * 1024 * 1024);

    let ram_str = format!("RAM: {}GB", total_ram_capacity_gb);

    // Write information to a text file
    let mut file = File::create("system_info.txt").expect("Failed to create file");

    writeln!(file, "{}", cpu_formatted).expect("Failed to write to file");
    writeln!(file, "{}", gpu_str).expect("Failed to write to file");
    writeln!(file, "Display: {}", resolution_str).expect("Failed to write to file");
    writeln!(file, "{}", ram_str).expect("Failed to write to file");

    println!("System information has been written to system_info.txt");
}
