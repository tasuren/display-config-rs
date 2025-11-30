use display_config::DisplayObserver;

fn main() {
    #[cfg(target_os = "windows")]
    display_config::windows::set_process_per_monitor_dpi_aware()
        .expect("Failed to set process as DPI aware");

    let monitor = DisplayObserver::new().expect("Failed to create the instance");
    monitor.set_callback(|event| println!("{event:#?}"));
    monitor.run().expect("Failed to run the application");
}
