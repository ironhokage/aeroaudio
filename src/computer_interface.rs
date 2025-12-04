use std::collections::VecDeque;
use std::io::stdout;
use std::io::Write;
use std::thread;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

// --- Helper Functions (for UI later) ---

/// Returns the CPU brand name as a String.
pub fn get_cpu_brand_name_string(sys: &System) -> String {
    if let Some(cpu) = sys.cpus().first() {
        cpu.brand().to_string()
    } else {
        "Unknown CPU Brand".to_string()
    }
}

/// Returns the current CPU frequency as a formatted String (e.g., "3500 Mhz").
pub fn get_cpu_frequency_string(sys: &mut System) -> String {
    // Must refresh before reading the value
    sys.refresh_cpu_frequency();

    if let Some(cpu) = sys.cpus().first() {
        format!("{} Mhz", cpu.frequency())
    } else {
        "N/A".to_string()
    }
}

/// Returns the global CPU usage as a formatted String (e.g., "25.5%").
/// Requires history management for smoothing.
pub fn get_cpu_usage_string(sys: &mut System, history: &mut VecDeque<f32>) -> String {
    // Must refresh before calculating the usage
    sys.refresh_cpu_usage();

    // Compute global CPU usage
    let sum: f32 = sys.cpus().iter().map(|c| c.cpu_usage()).sum();
    let global_usage = sum / sys.cpus().len() as f32;

    // Keep a short history to smooth the display
    history.push_back(global_usage);
    if history.len() > 5 {
        history.pop_front();
    }
    let smoothed: f32 = history.iter().copied().sum::<f32>() / history.len() as f32;

    // Task Manager-like display: round and ignore tiny values
    let display = if smoothed < 1.0 { 0.0 } else { smoothed };
    format!("{:>3.0}%", display)
}

pub fn run_cpu_monitor() {
    let mut stdout = stdout();
    let mut usage_history: VecDeque<f32> = VecDeque::new();

    // Initialize the System struct once in the main scope
    let refresh_kind = RefreshKind::nothing().with_cpu(CpuRefreshKind::everything());
    let mut sys = System::new_with_specifics(refresh_kind);

    println!("Cpu brand: {}", get_cpu_brand_name_string(&mut sys));
    loop {
        // Use the helper functions to get formatted strings
        let usage = get_cpu_usage_string(&mut sys, &mut usage_history);
        let frequency = get_cpu_frequency_string(&mut sys);

        let output = format!(
            "Global CPU Usage: {} | Frequency: {}",
            usage, frequency
        );

        // Print to console using carriage return \r to update the single line
        print!("\r{}", output);
        stdout.flush().unwrap();

        // Wait for next cycle
        thread::sleep(Duration::from_millis(500));
    }
}

