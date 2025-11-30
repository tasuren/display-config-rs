# display-config-rs

A Rust crate for retrieving display configurations and observing changes
(monitor plug/unplug, resolution changes, etc.) on Windows and macOS.

## Features

- **Get Display Info**: Retrieve a list of connected displays with details like:
    - ID (Platform specific)
    - Logical Size
    - Logical Position - Supports negative coordinates for multi-monitor setups.
    - Scale Factor
    - Primary status
    - Mirroring status
- **Observe Changes**: Listen for display configuration events in real-time:
    - `Added`: A new display was connected.
    - `Removed`: A display was disconnected.
    - `SizeChanged`: Display size changed.
    - `OriginChanged`: Display position changed (e.g., rearranged in settings).
    - `Mirrored` / `UnMirrored`: Mirroring settings changed.
- **Cross-Platform**: Unified API for Windows and macOS.

## Examples

### List Displays

```rust
use display_config::get_displays;

fn main() {
    #[cfg(target_os = "windows")]
    display_config::windows::set_process_per_monitor_dpi_aware()
        .expect("Failed to set process as DPI aware");

    let displays = get_displays().expect("Failed to get displays");

    for display in displays {
        println!("Display ID: {:?}", display.id);
        println!("  Origin: {:?}", display.origin); // e.g., LogicalPosition { x: 0, y: 0 }
        println!("  Size: {:?}", display.size);     // e.g., LogicalSize { width: 1920, height: 1080 }
        println!("  Scale Factor: {}", display.scale_factor);
        println!("  Is Primary: {}", display.is_primary);
    }
}
```

### Observe Changes

```rust
use display_config::{DisplayObserver, Event};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    display_config::windows::set_process_per_monitor_dpi_aware()
        .expect("Failed to set process as DPI aware");

    let observer = DisplayObserver::new()?;

    // Set a callback to handle events
    observer.set_callback(|event| {
        match event {
            Event::Added(display) => println!("Display added: {:?}", display),
            Event::Removed(id) => println!("Display removed: {:?}", id),
            Event::SizeChanged { display, .. } => println!("Size changed: {:?}", display),
            Event::OriginChanged { display, .. } => println!("Position changed: {:?}", display),
            _ => {}
        }
    });

    // Start the event listener (blocking on macOS)
    observer.run()?;

    Ok(())
}
```

## Platform Support

| Platform    | Status           | Notes                                                                                                   |
| ----------- | ---------------- | ------------------------------------------------------------------------------------------------------- |
| **Windows** | ✅ Supported     | Uses `EnumDisplayMonitors` and `WM_DISPLAYCHANGE`. Requires Windows 8.1+ for per-monitor DPI awareness. |
| **macOS**   | ✅ Supported     | Uses CoreGraphics for display info and callbacks.                                                       |
| **Linux**   | ❌ Not Supported | Planned for future release.                                                                             |

## License

This project is licensed under the [MIT License](https://github.com/tasuren/display-config-rs/blob/main/LICENSE).
