use std::fs::File;
use std::io::Write;

mod system_info;

fn main() {
    let cpu_info = system_info::get_cpu_info();
    let gpu_info = system_info::get_gpu_info();
    let display_info = system_info::get_display_info();
    let ram_info = system_info::get_ram_info();

    // Write information to a text file
    let mut file = File::create("system_info.txt").expect("Failed to create file");

    writeln!(file, "{}", cpu_info).expect("Failed to write to file");
    writeln!(file, "{}", gpu_info).expect("Failed to write to file");
    writeln!(file, "{}", display_info).expect("Failed to write to file");
    writeln!(file, "{}", ram_info).expect("Failed to write to file");

    println!("System information has been written to system_info.txt");
}
