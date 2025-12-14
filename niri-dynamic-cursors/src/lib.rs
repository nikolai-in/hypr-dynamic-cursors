pub mod config;
pub mod cursor_state;
pub mod modes;
pub mod shake;
pub mod renderer;

use cursor_state::CursorState;
pub use config::Config;

/// Main dynamic cursors handler
pub struct DynamicCursors {
    config: Config,
    state: CursorState,
}

impl DynamicCursors {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            state: CursorState::new(),
        }
    }

    pub fn on_cursor_moved(&mut self, x: f64, y: f64, timestamp_ms: u64) {
        self.state.update_position(x, y, timestamp_ms);
        self.state.update_shake(&self.config.shake);
    }

    pub fn get_cursor_transform(&self) -> CursorTransform {
        let mode_transform = match self.config.mode {
            config::CursorMode::None => CursorTransform::default(),
            config::CursorMode::Rotate => self.state.get_rotate_transform(&self.config.rotate),
            config::CursorMode::Tilt => self.state.get_tilt_transform(&self.config.tilt),
            config::CursorMode::Stretch => self.state.get_stretch_transform(&self.config.stretch),
        };

        let shake_scale = self.state.get_shake_magnification(&self.config.shake);

        CursorTransform {
            rotation: mode_transform.rotation,
            scale_x: mode_transform.scale_x * shake_scale,
            scale_y: mode_transform.scale_y * shake_scale,
            offset_x: mode_transform.offset_x,
            offset_y: mode_transform.offset_y,
        }
    }

    pub fn is_shaking(&self) -> bool {
        self.state.is_shaking()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CursorTransform {
    pub rotation: f64,    // in degrees
    pub scale_x: f64,
    pub scale_y: f64,
    pub offset_x: f64,
    pub offset_y: f64,
}

impl Default for DynamicCursors {
    fn default() -> Self {
        Self::new(Config::default())
    }
}
