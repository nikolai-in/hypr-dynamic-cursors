# niri-dynamic-cursors

Port of hypr-dynamic-cursors to the [niri](https://github.com/YaLTeR/niri) window manager.

This implementation provides dynamic cursor effects for niri, making your cursor more realistic by simulating how it would behave if it was an actual object being dragged across your screen. It also includes the popular "shake to find" feature.

## Features

- **Cursor Modes:**
  - `rotate` - Rotates the cursor based on movement direction
  - `tilt` - Tilts the cursor based on x-velocity and speed
  - `stretch` - Stretches the cursor shape based on direction and velocity
  - `none` - No cursor transformation
  
- **Shake to Find:** Magnifies the cursor when shaken, making it easier to locate

## Status

⚠️ **Experimental** - This is an initial port of hypr-dynamic-cursors to niri. 

### Current Limitations

Since niri doesn't have a plugin system like Hyprland, this implementation takes a different approach:

1. **Integration Method:** This is currently a library that needs to be integrated into niri itself, either as:
   - A fork of niri with this functionality built-in
   - A patch that can be applied to niri
   - Future: If niri adds plugin support, this could become a standalone plugin

2. **Implementation Approach:** 
   - Written in Rust (matching niri's language)
   - Uses smithay compositor primitives where possible
   - Provides a clean API that can be integrated into niri's cursor rendering pipeline

## Architecture

Unlike the Hyprland version which uses function hooks, this port:
- Is written entirely in Rust
- Provides a clean API for cursor transformation
- Tracks cursor position and velocity
- Calculates appropriate transformations based on configuration
- Renders transformed cursor buffers

## Integration Guide

To integrate this into niri, you would need to:

1. Add this as a dependency to niri's `Cargo.toml`:
```toml
[dependencies]
niri-dynamic-cursors = { path = "../niri-dynamic-cursors" }
```

2. In niri's cursor rendering code, add hooks to:
   - Track cursor movement: `dynamic_cursors.on_cursor_moved(x, y, timestamp)`
   - Get cursor transform: `let transform = dynamic_cursors.get_cursor_transform()`
   - Apply the transform when rendering the cursor

3. Example integration (pseudo-code):
```rust
use niri_dynamic_cursors::{DynamicCursors, Config};

// During initialization
let config = Config::default(); // or load from config file
let mut dynamic_cursors = DynamicCursors::new(config);

// In cursor movement handler
dynamic_cursors.on_cursor_moved(x, y, timestamp_ms);

// In cursor rendering code
let transform = dynamic_cursors.get_cursor_transform();
// Apply transform.rotation, transform.scale_x, transform.scale_y
// to the cursor rendering
```

## Configuration

Create a configuration file (e.g., `~/.config/niri/dynamic-cursors.toml`):

```toml
enabled = true
mode = "tilt"  # or "rotate", "stretch", "none"
threshold = 2.0

[rotate]
length = 20.0
offset = 0.0

[tilt]
limit = 5000.0
function = "negative_quadratic"
window = 100

[stretch]
limit = 3000.0
function = "quadratic"
window = 100

[shake]
enabled = true
nearest = true
threshold = 6.0
base = 4.0
speed = 4.0
influence = 0.0
limit = 0.0  # 0 means no limit
timeout = 2000
effects = false
```

Load it in your code:
```rust
let config_str = std::fs::read_to_string("~/.config/niri/dynamic-cursors.toml")?;
let config = Config::from_toml(&config_str)?;
```

## Building

```bash
cd niri-dynamic-cursors
cargo build --release
```

## Testing

Run the tests:
```bash
cargo test
```

## Next Steps

To fully integrate this into niri, one of these approaches would be needed:

1. **Fork niri** and integrate this library directly
2. **Create a patch** that niri users can apply
3. **Propose to niri upstream** to add plugin support or native dynamic cursor support
4. **Wait for niri plugin system** if one is planned

## Differences from Hyprland Version

- Written in Rust instead of C++
- No function hooking (not needed/possible with Rust)
- Clean API-based integration instead of plugin-based
- Same algorithms and features as the original
- More modular architecture

## Credits

Original hypr-dynamic-cursors by [VirtCode](https://github.com/VirtCode/hypr-dynamic-cursors)

This port maintains the same MIT license.

## Contributing

Contributions welcome! This is an experimental port and there's much room for improvement, especially around:
- Actual niri integration
- Performance optimization
- Additional features
- Better documentation

## License

MIT License - See LICENSE.md
