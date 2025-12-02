use rodio::cpal::traits::{DeviceTrait, HostTrait};
use rodio::cpal;

fn main() {
    match get_name_of_audio_device() {
        Some(name) => println!("Using audio device: {}", name),
        None => println!("No default output device found, or name could not be retrieved."),
    }
}
fn get_name_of_audio_device() -> Option<String> {
    // Get the default audio host
    let host = cpal::default_host();
    let device_name = host.default_output_device()?; // The '?' operator propagates the None/Error

    // Try to get the device's name and return it wrapped in Some()
    // We use ? here as well to handle potential errors from device.name()
    match device_name.name() {
        Ok(name) => Some(name),
        Err(e) => {
            eprintln!("Error retrieving device name: {}", e);
            None // Return None if we can't get the name string
        }
    }
}
