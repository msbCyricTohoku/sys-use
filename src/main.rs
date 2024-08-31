use sysinfo::{System, SystemExt, CpuExt};
use std::{thread, time::Duration};

fn main() {
    //create a new sys object
    let mut sys = System::new_all();

    loop {
        //refresh system information
        sys.refresh_all();

        //here we get global CPU usage
        let cpu_usage = sys.global_cpu_info().cpu_usage();

        //here we get total memory and used memory
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let memory_usage = (used_memory as f64 / total_memory as f64) * 100.0;

        //clear the console using escape key codes
        print!("\x1B[2J\x1B[1;1H");

        //here we print CPU usage using a simple bar
        println!("CPU Usage: {:.2}%", cpu_usage);
        print_progress_bar(cpu_usage as usize, 100, 30);

        //here print memory usage with a bar
        println!("\nMemory Usage: {:.2}%", memory_usage);
        print_progress_bar(memory_usage as usize, 100, 30);

        //refresh for 1 sec, using sleep
        thread::sleep(Duration::from_secs(1));
    }
}

fn print_progress_bar(current: usize, total: usize, width: usize) {
    let filled_width = (current * width) / total;
    let empty_width = width - filled_width;

    let filled_bar = "█".repeat(filled_width);
    let empty_bar = " ".repeat(empty_width);

    println!("[{}{}]", filled_bar, empty_bar);
}

