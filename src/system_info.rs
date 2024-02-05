use std::io::{self, BufRead};
use std::process::Command;

// Function to get CPU information
pub fn get_cpu_info() -> String {
    let cpu_info = Command::new("wmic")
        .args(&["cpu", "get", "name"])
        .output()
        .expect("Failed to execute command")
        .stdout;

    let cpu_str = String::from_utf8_lossy(&cpu_info);
    let cpu_details = cpu_str.lines().skip(1).next();

    match cpu_details {
        Some(details) => format!("CPU: {}", details.trim()),
        None => "CPU: Unknown".to_string(),
    }
}

// Function to get GPU information
pub fn get_gpu_info() -> String {
    let gpu_info = Command::new("wmic")
        .args(&["path", "win32_videocontroller", "get", "caption"])
        .output()
        .expect("Failed to execute command")
        .stdout;

    String::from_utf8_lossy(&gpu_info)
        .lines()
        .skip(1)
        .next()
        .map(|line| format!("GPU: {}", line.trim()))
        .unwrap_or_else(|| "GPU: Unknown".to_string())
}

// Function to get display information
pub fn get_display_info() -> String {
    let display_info = Command::new("wmic")
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

    let display_str = String::from_utf8_lossy(&display_info);

    let display_values: Vec<&str> = display_str
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
        .collect();

    format!("Display: {}", display_values.join("x"))
}

// Function to get RAM information
pub fn get_ram_info() -> String {
    let ram_info = Command::new("wmic")
        .args(&["memorychip", "get", "capacity"])
        .output()
        .expect("Failed to execute command")
        .stdout;

    let ram_capacity_vec: Vec<u64> = io::Cursor::new(ram_info)
        .lines()
        .skip(1)
        .filter_map(|line| line.unwrap().trim().parse().ok())
        .collect();

    let total_ram_capacity_gb = ram_capacity_vec.iter().sum::<u64>() / (1024 * 1024 * 1024);
    format!("RAM: {}GB", total_ram_capacity_gb)
}
