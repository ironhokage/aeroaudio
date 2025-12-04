use cpal::traits::{DeviceTrait, HostTrait};
use std::error::Error;

pub fn get_name_of_audio_device() {
    // Get the default audio host
    let host = cpal::default_host();
    let device_name = host.default_output_device(); // '?' works here

    // Try to get the device's name and return it wrapped in Some()
    match device_name {
        Some(audio_device) => match audio_device.name() {
            Ok(name_of_audio_device) => println!("Output device name is: {}", name_of_audio_device),
            Err(e) => eprintln!("An error occurred getting device name: {}", e),
        },
        None => println!("No default output device found."),
    }
}

/// Lists all available audio input and output devices.
pub fn get_list_audio_devices() {
    let result: anyhow::Result<(), Box<dyn Error>> = (|| {
        let start_index_from_one: bool = false;
        let host = cpal::default_host();
        println!("\nAudio host: {:?}", host.id());

        let devices = host.devices()?;
        println!("\n--- Connected Audio Devices ---");

        // Iterate through the devices, keeping track of the index.
        for (i, device) in devices.enumerate() {
            let name = device.name()?;
            let display_index = if start_index_from_one { i + 1 } else { i };
            println!("\t{}: {}", display_index, name);

            if device.default_input_config().is_ok() { println!("  - Type: Input"); }
            if device.default_output_config().is_ok() { println!("  - Type: Output"); }
        }
        // Print the name of the system's default input device, if one exists.
        if let Some(default_input) = host.default_input_device() {
            println!("\nDefault Input Device: {}",
                     default_input.name()?);
        }

        // Print the name of the system's default output device, if one exists.
        if let Some(default_output) = host.default_output_device() {
            println!("\nDefault Output Device: {}",
                     default_output.name()?);
        }
        // Return Ok(()) on success within the IIFE.
        Ok(())
    })();
    // Centralized error handling: if the IIFE returned an Err, print the error message to stderr.
    if let Err(e) = result {
        eprintln!("Error listing audio devices: {}", e);
    }
}