use cpal::traits::{DeviceTrait, HostTrait};
use anyhow::Result;
// use sysinfo::System;


const START_INDEX_FROM_ONE: bool = false; // Set this to 'false' if you want 0-based indexing
fn main() {
    match get_name_of_audio_device() {
        Some(name) => println!("Using default output device: {}", name),
        None => println!("No default output device found, or name could not be retrieved."),
    }

    // Call the function and handle the error it might return
    if let Err(e) = get_all_audio_devices() {
        eprintln!("An error occurred while listing devices: {}", e);
    }

}

// This function works because it returns an Option<String>
fn get_name_of_audio_device() -> Option<String> {
    // Get the default audio host
    let host = cpal::default_host();
    let device_name = host.default_output_device()?; // '?' works here

    // Try to get the device's name and return it wrapped in Some()
    match device_name.name() {
        Ok(name) => Some(name),
        Err(e) => {
            eprintln!("Error retrieving device name: {}", e);
            None
        }
    }
}

// We changed the return type from () to Result<()> so we can use the '?' operator.
fn get_all_audio_devices() -> Result<()> {
    // 1. Get the default audio host for the current platform.
    let host = cpal::default_host();
    println!("\nAudio host: {:?}", host.id());

    // 2. Enumerate all available audio devices (both input and output).
    let devices = host.devices()?; // '?' works now

    println!("\n--- Connected Audio Devices ---");
    for (index, device) in devices.enumerate() {
        let name = device.name()?; // Used '?' instead of .unwrap() for better error handling

        let display_index = if START_INDEX_FROM_ONE { index + 1 } else { index };
        println!("\t{}: {}", display_index, name);

        // You can also check if it's an input or output device
        if let Ok(_config) = device.default_input_config() {
            println!("  - Type: Input");
        }
        if let Ok(_config) = device.default_output_config() {
            println!("  - Type: Output");
        }
    }
    // We see what the default Input/Output devices are
    if let Some(default_input) = host.default_input_device() {
        println!("\nDefault Input Device: {}", default_input.name()?);
    }

    if let Some(default_output) = host.default_output_device() {
        println!("Default Output Device: {}", default_output.name()?);
    }

    // Return Ok(()) to signal that the function completed successfully.
    Ok(())
}
