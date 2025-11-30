use display_config::get_displays;

fn main() {
    #[cfg(target_os = "windows")]
    display_config::windows::set_process_per_monitor_dpi_aware()
        .expect("Failed to set process as DPI aware");

    let displays = get_displays().expect("Failed to get displays");

    for display in displays {
        println!("Display ID: {:?}", display.id);
        println!("  Origin: {:?}", display.origin);
        println!("  Size: {:?}", display.size);
        println!("  Scale factor: {:?}", display.scale_factor);
        println!("  Is primary: {:?}", display.is_primary);
        println!("  Is mirrored: {:?}", display.is_mirrored);
        println!()
    }
}
