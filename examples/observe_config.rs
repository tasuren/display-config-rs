use display_config::{DisplayObserver, Event};

fn on_event(event: Event) {
    match event {
        Event::Added(display) => {
            println!(
                "Added ... id = {:?}, origin = {:?}, size = {:?}",
                display.id, display.origin, display.size
            );
        }
        Event::Removed(id) => {
            println!("Removed ... id = {:?}", id);
        }
        Event::SizeChanged {
            display,
            before,
            after,
        } => {
            println!(
                "SizeChanged ... id = {:?}, before = {:?}, after = {:?}",
                display.id, before, after
            );
        }
        Event::OriginChanged {
            display,
            before,
            after,
        } => {
            println!(
                "OriginChanged ... id = {:?}, before = {:?}, after = {:?}",
                display.id, before, after
            );
        }
        Event::Mirrored(display) => {
            println!("Mirrored ... id = {:?}", display.id);
        }
        Event::UnMirrored(display) => {
            println!("UnMirrored ... id = {:?}", display.id);
        }
    }
}

fn main() {
    let monitor = DisplayObserver::new().expect("Failed to create the instance");
    monitor.set_callback(on_event);
    monitor.run().expect("Failed to run the application");
}
