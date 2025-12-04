mod audio_machine_interface;
mod computer_interface;

fn main() {

    //Gets the name of the current used audio device
    audio_machine_interface::get_name_of_audio_device();
    // Call the function and handle the error it might return
    audio_machine_interface::get_list_audio_devices();
    // Monitors the cpus workload
    computer_interface::run_cpu_monitor();

}

