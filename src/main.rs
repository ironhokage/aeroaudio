use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait};
use std::collections::VecDeque;
use std::error::Error;
use std::io::{Write, stdout}; // Import Write trait and stdout
use std::thread;
use std::time::Duration;
use sysinfo::System;

fn main() {
    //Gets the name of the current used audio device
    get_name_of_audio_device();
    // Call the function and handle the error it might return
    get_list_audio_devices();
    // Monitors the cpus workload
    run_cpu_monitor();
}

// This function works because it returns an Option<String>
fn get_name_of_audio_device() {
    // Get the default audio host
    let host = cpal::default_host();
    let device_name = host.default_output_device(); // '?' works here

    // Try to get the device's name and return it wrapped in Some()
    match device_name {
        Some(device) =>match device.name() {
            Ok(name) => println!("Output device name is: {}", name),
            Err(e) => eprintln!("An error occurred getting device name: {}", e),
        },
        None => println!("No default output device found."),
    }
}

/// Lists all available audio input and output devices.
fn get_list_audio_devices() {
    // Wrap the core logic in an immediately invoked function expression (IIFE)
    // to centralize error handling at the end of the function.
    let result: Result<(), Box<dyn Error>> = (||{

        let start_index_from_one: bool = false; // Configuration to decide if device indexing starts from 0 or 1.
        let host = cpal::default_host(); // Retrieve the default audio host for the current platform (e.g., PulseAudio on Linux, WASAPI on Windows).
        println!("\nAudio host: {:?}", host.id());

        let devices = host.devices()?; // Get an iterator over all available audio devices managed by the host.
        println!("\n--- Connected Audio Devices ---");

        // Iterate through the devices, keeping track of the index.
        for (i, device) in devices.enumerate() {

            let name = device.name()?; // Retrieve the name of the device. The '?' operator handles potential errors.
            let display_index = if start_index_from_one { i + 1 } else { i }; // Determine the display index based on the configuration flag.
            println!("\t{}: {}", display_index, name);
            // Check if the device supports default input/output configuration and print the type if it does.
            if device.default_input_config().is_ok() { println!("  - Type: Input"); }
            if device.default_output_config().is_ok() { println!("  - Type: Output");}
        }
        // Print the name of the system's default input device, if one exists.
        if let Some(default_input) = host.default_input_device() {
            println!("\nDefault Input Device: {}", default_input.name()?);
        }
        // Print the name of the system's default output device, if one exists.
        if let Some(default_output) = host.default_output_device() {
            println!("\nDefault Output Device: {}", default_output.name()?);
        }
        // Return Ok(()) on success within the IIFE.
        Ok(())

    })();
    // Centralized error handling: if the IIFE returned an Err, print the error message to stderr.
    if let Err(e) = result {
        eprintln!("Error listing audio devices: {}", e);
    }
}

fn run_cpu_monitor() {
    let mut sys = System::new();
    let mut stdout = stdout();
    let mut history: VecDeque<f32> = VecDeque::new();

    // Initial refresh to get a valid delta
    sys.refresh_cpu_usage();
    thread::sleep(Duration::from_millis(100));
    sys.refresh_cpu_usage();

    loop {
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
        let output = format!("Global CPU Usage: {:>3.0}%", display);

        print!("\r{}  ", output);
        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(500));
    }
}
