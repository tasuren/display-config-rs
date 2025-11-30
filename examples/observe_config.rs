use display_config::DisplayObserver;

fn main() {
    let monitor = DisplayObserver::new().expect("Failed to create the instance");
    monitor.set_callback(|event| println!("{event:#?}"));
    monitor.run().expect("Failed to run the application");
}
