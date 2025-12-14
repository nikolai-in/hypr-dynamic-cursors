use std::collections::VecDeque;
use crate::config::ShakeConfig;
use crate::cursor_state::PositionSample;

#[derive(Debug)]
pub struct ShakeDetector {
    is_active: bool,
    shake_start: Option<u64>,
    shake_intensity: f64,
    current_magnification: f64,
    last_update: u64,
}

impl ShakeDetector {
    pub fn new() -> Self {
        Self {
            is_active: false,
            shake_start: None,
            shake_intensity: 0.0,
            current_magnification: 1.0,
            last_update: 0,
        }
    }

    pub fn update(&mut self, history: &VecDeque<PositionSample>, config: &ShakeConfig) {
        if history.len() < 3 {
            return;
        }

        let current_time = history.back().unwrap().timestamp_ms;
        
        // Calculate shake intensity from recent movement
        let intensity = self.calculate_shake_intensity(history);
        
        // Detect shake start
        if !self.is_active && intensity > config.threshold {
            self.is_active = true;
            self.shake_start = Some(current_time);
            self.shake_intensity = intensity;
            self.current_magnification = config.base;
        }

        // Update shake magnification if active
        if self.is_active {
            let shake_duration = if let Some(start) = self.shake_start {
                (current_time - start) as f64 / 1000.0 // Convert to seconds
            } else {
                0.0
            };

            // Increase magnification over time
            let speed_contribution = config.speed * shake_duration;
            let influence_contribution = config.influence * intensity;
            
            self.current_magnification = config.base + speed_contribution + influence_contribution;

            // Apply limit if set
            if config.limit > 0.0 {
                self.current_magnification = self.current_magnification.min(config.limit);
            }

            // Check if shake has ended
            if intensity < config.threshold / 2.0 {
                // Start timeout
                if current_time - self.last_update > config.timeout {
                    self.is_active = false;
                    self.current_magnification = 1.0;
                    self.shake_start = None;
                }
            } else {
                self.last_update = current_time;
            }
        }

        self.shake_intensity = intensity;
    }

    fn calculate_shake_intensity(&self, history: &VecDeque<PositionSample>) -> f64 {
        if history.len() < 3 {
            return 0.0;
        }

        // Look at movement in last 200ms
        let current = history.back().unwrap();
        let window_ms = 200;

        let mut trail_distance = 0.0;
        let mut prev = current;

        for sample in history.iter().rev().skip(1) {
            if current.timestamp_ms - sample.timestamp_ms > window_ms {
                break;
            }

            let dx = prev.x - sample.x;
            let dy = prev.y - sample.y;
            trail_distance += (dx * dx + dy * dy).sqrt();
            prev = sample;
        }

        // Calculate diagonal distance
        if let Some(oldest) = history.iter().rev().find(|s| {
            current.timestamp_ms - s.timestamp_ms <= window_ms
        }) {
            let dx = current.x - oldest.x;
            let dy = current.y - oldest.y;
            let diagonal = (dx * dx + dy * dy).sqrt();

            if diagonal < 1.0 {
                return 0.0;
            }

            // Intensity is the ratio of trail to diagonal
            // Higher ratio means more back-and-forth movement (shaking)
            trail_distance / diagonal
        } else {
            0.0
        }
    }

    pub fn get_magnification(&self) -> f64 {
        self.current_magnification
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shake_detector_initialization() {
        let detector = ShakeDetector::new();
        assert!(!detector.is_active());
        assert_eq!(detector.get_magnification(), 1.0);
    }

    #[test]
    fn test_no_shake_on_linear_movement() {
        let mut detector = ShakeDetector::new();
        let mut history = VecDeque::new();

        // Simulate linear movement
        for i in 0..10 {
            history.push_back(PositionSample {
                x: i as f64 * 10.0,
                y: i as f64 * 10.0,
                timestamp_ms: i * 10,
            });
        }

        let config = ShakeConfig::default();
        detector.update(&history, &config);

        // Should not detect shake on linear movement
        assert!(!detector.is_active());
    }
}
