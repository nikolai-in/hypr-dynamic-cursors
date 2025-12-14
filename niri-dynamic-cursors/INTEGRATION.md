# Integration Guide for niri

This document explains how to integrate niri-dynamic-cursors into the niri window manager.

## Background

Unlike Hyprland which has a plugin system, niri is written in Rust and doesn't currently support plugins. Therefore, this library must be integrated directly into niri's source code.

## Integration Approaches

### Option 1: Fork niri (Recommended for Now)

1. Fork the niri repository
2. Add niri-dynamic-cursors as a dependency
3. Integrate the cursor transformation into niri's rendering pipeline
4. Maintain your fork with upstream niri updates

### Option 2: Patch File

Create a patch that niri users can apply to add dynamic cursor support.

### Option 3: Upstream Integration

Propose adding this feature to niri upstream (would require discussion with niri maintainers).

## Technical Integration Steps

### 1. Add Dependency

In niri's `Cargo.toml`:

```toml
[dependencies]
niri-dynamic-cursors = { path = "../niri-dynamic-cursors" }
```

Or if published to crates.io:

```toml
[dependencies]
niri-dynamic-cursors = "0.1"
```

### 2. Initialize in niri's State

In niri's main compositor state structure (likely in `src/niri.rs` or similar):

```rust
use niri_dynamic_cursors::{DynamicCursors, Config};

pub struct State {
    // ... existing fields ...
    dynamic_cursors: Option<DynamicCursors>,
}

impl State {
    pub fn new() -> Self {
        // Load config from niri's config file
        let dc_config = load_dynamic_cursors_config();
        let dynamic_cursors = if dc_config.enabled {
            Some(DynamicCursors::new(dc_config))
        } else {
            None
        };

        Self {
            // ... existing fields ...
            dynamic_cursors,
        }
    }
}
```

### 3. Hook Cursor Movement

Find niri's cursor movement handler (likely in pointer input handling):

```rust
// In the pointer motion handler
fn handle_pointer_motion(&mut self, event: PointerMotionEvent) {
    let (x, y) = self.pointer.position();
    let timestamp_ms = event.time_msec();
    
    // Update dynamic cursors
    if let Some(ref mut dc) = self.dynamic_cursors {
        dc.on_cursor_moved(x, y, timestamp_ms);
    }
    
    // ... rest of motion handling ...
}
```

### 4. Integrate Cursor Rendering

This is the most complex part. Find niri's cursor rendering code (likely in `src/render.rs` or similar):

```rust
use niri_dynamic_cursors::renderer::CursorBuffer;

fn render_cursor(&mut self, cursor_image: &CursorImage) {
    // Get the transform if dynamic cursors are enabled
    let transform = if let Some(ref dc) = self.dynamic_cursors {
        dc.get_cursor_transform()
    } else {
        CursorTransform::default()
    };
    
    // If transform is non-identity, transform the cursor buffer
    if transform.rotation.abs() > 0.1 
        || (transform.scale_x - 1.0).abs() > 0.01 
        || (transform.scale_y - 1.0).abs() > 0.01 {
        
        // Convert cursor image to CursorBuffer
        let buffer = CursorBuffer {
            width: cursor_image.width,
            height: cursor_image.height,
            hotspot_x: cursor_image.hotspot_x,
            hotspot_y: cursor_image.hotspot_y,
            data: cursor_image.data.clone(),
        };
        
        // Transform it
        let use_nearest = self.dynamic_cursors
            .as_ref()
            .map(|dc| dc.is_shaking())
            .unwrap_or(false);
        
        let transformed = buffer.transform(&transform, use_nearest);
        
        // Render the transformed buffer
        self.render_cursor_buffer(&transformed);
    } else {
        // Render cursor normally
        self.render_cursor_buffer(cursor_image);
    }
}
```

### 5. Configuration Integration

Add dynamic cursors configuration to niri's config file format.

In niri's config parser (likely using KDL format):

```kdl
// In niri's config.kdl
dynamic-cursors {
    enabled true
    mode "tilt"
    threshold 2.0
    
    rotate {
        length 20.0
        offset 0.0
    }
    
    tilt {
        limit 5000.0
        function "negative_quadratic"
        window 100
    }
    
    shake {
        enabled true
        threshold 6.0
        base 4.0
        speed 4.0
    }
}
```

Parse it in niri's config code:

```rust
use niri_dynamic_cursors::config::Config as DCConfig;

// In config parsing
fn parse_dynamic_cursors(node: &KdlNode) -> DCConfig {
    // Parse the KDL node into DCConfig
    // This would need to be implemented based on niri's config format
    todo!()
}
```

## Testing

1. Build niri with the integration:
```bash
cargo build --release
```

2. Run niri in a nested session for testing:
```bash
./target/release/niri
```

3. Move the cursor and verify the effects are working

## Performance Considerations

- The cursor transformation happens on every frame when the cursor is moving
- Use nearest-neighbor scaling for shake magnification to improve performance
- Consider adding a config option to disable effects for performance-critical use cases

## Potential Issues

1. **Coordinate Systems**: Ensure cursor coordinates match between niri and the library
2. **Timing**: Make sure timestamps are in milliseconds
3. **Buffer Formats**: The library expects ARGB8888; convert if niri uses a different format
4. **Hotspot Handling**: Ensure cursor hotspot is correctly transformed

## Future Improvements

- Add IPC events for shake detection (similar to Hyprland version)
- Support for per-shape configuration
- Hardware cursor optimization
- Integration with niri's animation system

## Getting Help

- Open an issue on the hypr-dynamic-cursors repository
- Check niri's documentation for compositor architecture
- Join niri's community channels for integration questions
