/// Standalone example demonstrating the niri-dynamic-cursors library
/// This shows how the library can be used independently

use niri_dynamic_cursors::{DynamicCursors, Config, CursorTransform};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    env_logger::init();

    println!("niri-dynamic-cursors standalone example");
    println!("========================================\n");

    // Load config from file or use defaults
    let config = if let Ok(config_str) = std::fs::read_to_string("config.toml") {
        match Config::from_toml(&config_str) {
            Ok(cfg) => {
                println!("Loaded configuration from config.toml");
                cfg
            }
            Err(e) => {
                eprintln!("Error parsing config: {}, using defaults", e);
                Config::default()
            }
        }
    } else {
        println!("Using default configuration");
        Config::default()
    };

    println!("Configuration:");
    println!("  Mode: {:?}", config.mode);
    println!("  Shake enabled: {}", config.shake.enabled);
    println!();

    // Create the dynamic cursors handler
    let mut dynamic_cursors = DynamicCursors::new(config);

    // Simulate cursor movements
    println!("Simulating cursor movements:\n");

    let movements = vec![
        (100.0, 100.0, "Initial position"),
        (150.0, 100.0, "Moving right slowly"),
        (250.0, 100.0, "Moving right faster"),
        (350.0, 100.0, "Moving right very fast"),
        (350.0, 200.0, "Moving down"),
        (250.0, 200.0, "Moving left"),
        (150.0, 200.0, "Still moving left"),
        // Simulate shaking
        (160.0, 200.0, "Shake: right"),
        (140.0, 200.0, "Shake: left"),
        (160.0, 200.0, "Shake: right"),
        (140.0, 200.0, "Shake: left"),
        (160.0, 200.0, "Shake: right"),
        (140.0, 200.0, "Shake: left"),
        (150.0, 200.0, "Shake: center"),
    ];

    let start_time = get_timestamp_ms();

    for (i, (x, y, description)) in movements.iter().enumerate() {
        let timestamp = start_time + (i as u64 * 50); // 50ms intervals
        
        dynamic_cursors.on_cursor_moved(*x, *y, timestamp);
        let transform = dynamic_cursors.get_cursor_transform();
        
        println!("{}: ({}, {})", description, x, y);
        print_transform(&transform);
        
        if dynamic_cursors.is_shaking() {
            println!("  🔍 SHAKING DETECTED!");
        }
        println!();

        // Simulate delay
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    println!("Example completed!");
}

fn print_transform(transform: &CursorTransform) {
    println!("  Transform:");
    println!("    Rotation: {:.2}°", transform.rotation);
    println!("    Scale: ({:.2}, {:.2})", transform.scale_x, transform.scale_y);
    if transform.scale_x > 1.1 || transform.scale_y > 1.1 {
        println!("    ⚡ Cursor is magnified!");
    }
}

fn get_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}
