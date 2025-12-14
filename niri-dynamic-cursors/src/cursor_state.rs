use std::collections::VecDeque;
use crate::config::{RotateConfig, TiltConfig, StretchConfig, ShakeConfig};
use crate::modes::{calculate_rotation, calculate_tilt, calculate_stretch};
use crate::shake::ShakeDetector;
use crate::CursorTransform;

const MAX_HISTORY: usize = 100;

#[derive(Debug)]
pub struct CursorState {
    position: (f64, f64),
    history: VecDeque<PositionSample>,
    shake_detector: ShakeDetector,
}

#[derive(Debug, Clone, Copy)]
pub struct PositionSample {
    pub x: f64,
    pub y: f64,
    pub timestamp_ms: u64,
}

impl CursorState {
    pub fn new() -> Self {
        Self {
            position: (0.0, 0.0),
            history: VecDeque::with_capacity(MAX_HISTORY),
            shake_detector: ShakeDetector::new(),
        }
    }

    pub fn update_position(&mut self, x: f64, y: f64, timestamp_ms: u64) {
        self.position = (x, y);
        
        let sample = PositionSample { x, y, timestamp_ms };
        self.history.push_back(sample);

        // Keep history limited
        while self.history.len() > MAX_HISTORY {
            self.history.pop_front();
        }
    }

    pub fn update_shake(&mut self, config: &ShakeConfig) {
        if !config.enabled {
            return;
        }
        
        self.shake_detector.update(&self.history, config);
    }

    pub fn get_velocity(&self, window_ms: u64) -> (f64, f64) {
        if self.history.len() < 2 {
            return (0.0, 0.0);
        }

        let current = self.history.back().unwrap();
        let current_time = current.timestamp_ms;

        // Find samples within the time window
        let mut oldest = current;
        for sample in self.history.iter().rev() {
            if current_time - sample.timestamp_ms <= window_ms {
                oldest = sample;
            } else {
                break;
            }
        }

        let dt = (current.timestamp_ms - oldest.timestamp_ms) as f64;
        if dt < 1.0 {
            return (0.0, 0.0);
        }

        let dx = current.x - oldest.x;
        let dy = current.y - oldest.y;

        // Convert to px/s
        (dx / dt * 1000.0, dy / dt * 1000.0)
    }

    pub fn get_rotate_transform(&self, config: &RotateConfig) -> CursorTransform {
        let (vx, vy) = self.get_velocity(100);
        let rotation = calculate_rotation(vx, vy, config);

        CursorTransform {
            rotation,
            scale_x: 1.0,
            scale_y: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }

    pub fn get_tilt_transform(&self, config: &TiltConfig) -> CursorTransform {
        let (vx, vy) = self.get_velocity(config.window);
        let (rotation, scale_x, scale_y) = calculate_tilt(vx, vy, config);

        CursorTransform {
            rotation,
            scale_x,
            scale_y,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }

    pub fn get_stretch_transform(&self, config: &StretchConfig) -> CursorTransform {
        let (vx, vy) = self.get_velocity(config.window);
        let (rotation, scale_x, scale_y) = calculate_stretch(vx, vy, config);

        CursorTransform {
            rotation,
            scale_x,
            scale_y,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }

    pub fn get_shake_magnification(&self, config: &ShakeConfig) -> f64 {
        if !config.enabled {
            return 1.0;
        }
        
        self.shake_detector.get_magnification()
    }

    pub fn is_shaking(&self) -> bool {
        self.shake_detector.is_active()
    }
}
