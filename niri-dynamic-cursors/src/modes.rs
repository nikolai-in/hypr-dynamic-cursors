use crate::config::{RotateConfig, TiltConfig, StretchConfig, ActivationFunction};

/// Calculate rotation angle for rotate mode
pub fn calculate_rotation(vx: f64, vy: f64, config: &RotateConfig) -> f64 {
    if vx.abs() < 0.1 && vy.abs() < 0.1 {
        return 0.0;
    }

    // Calculate the angle of movement
    let angle = vy.atan2(vx).to_degrees();
    
    // Apply offset
    angle + config.offset
}

/// Calculate tilt transformation
pub fn calculate_tilt(vx: f64, vy: f64, config: &TiltConfig) -> (f64, f64, f64) {
    let speed = (vx * vx + vy * vy).sqrt();
    
    if speed < 0.1 {
        return (0.0, 1.0, 1.0);
    }

    // Calculate tilt based on x velocity (horizontal movement)
    let tilt_factor = (vx / config.limit).abs().min(1.0);
    let activated = apply_activation(tilt_factor, config.function);
    
    // Maximum tilt is 60 degrees
    let max_tilt = 60.0;
    let tilt = activated * max_tilt * vx.signum();

    (tilt, 1.0, 1.0)
}

/// Calculate stretch transformation
pub fn calculate_stretch(vx: f64, vy: f64, config: &StretchConfig) -> (f64, f64, f64) {
    let speed = (vx * vx + vy * vy).sqrt();
    
    if speed < 0.1 {
        return (0.0, 1.0, 1.0);
    }

    let stretch_factor = (speed / config.limit).min(1.0);
    let activated = apply_activation(stretch_factor, config.function);
    
    // Calculate angle of movement
    let angle = vy.atan2(vx);
    
    // Stretch in direction of movement (max 2x)
    let max_stretch = 2.0;
    let stretch = 1.0 + activated * (max_stretch - 1.0);
    
    // Apply stretching along movement axis
    let cos_a = angle.cos().abs();
    let sin_a = angle.sin().abs();
    
    let scale_x = 1.0 + (stretch - 1.0) * cos_a;
    let scale_y = 1.0 + (stretch - 1.0) * sin_a;

    (angle.to_degrees(), scale_x, scale_y)
}

/// Apply activation function to a normalized value (0.0 to 1.0)
fn apply_activation(value: f64, function: ActivationFunction) -> f64 {
    match function {
        ActivationFunction::Linear => value,
        ActivationFunction::Quadratic => value * value,
        ActivationFunction::NegativeQuadratic => {
            // This creates a more aggressive curve
            1.0 - (1.0 - value).powi(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_activation() {
        assert_eq!(apply_activation(0.5, ActivationFunction::Linear), 0.5);
        assert_eq!(apply_activation(0.5, ActivationFunction::Quadratic), 0.25);
        assert!(apply_activation(0.5, ActivationFunction::NegativeQuadratic) > 0.5);
    }

    #[test]
    fn test_calculate_rotation() {
        let config = RotateConfig {
            length: 20.0,
            offset: 0.0,
        };
        
        // Moving right should give 0 degrees
        let angle = calculate_rotation(100.0, 0.0, &config);
        assert!((angle - 0.0).abs() < 1.0);
        
        // Moving down should give 90 degrees
        let angle = calculate_rotation(0.0, 100.0, &config);
        assert!((angle - 90.0).abs() < 1.0);
    }

    #[test]
    fn test_calculate_tilt() {
        let config = TiltConfig {
            limit: 5000.0,
            function: ActivationFunction::Linear,
            window: 100,
        };
        
        // No movement should give no tilt
        let (tilt, sx, sy) = calculate_tilt(0.0, 0.0, &config);
        assert_eq!(tilt, 0.0);
        assert_eq!(sx, 1.0);
        assert_eq!(sy, 1.0);
        
        // Moving right should give positive tilt
        let (tilt, _, _) = calculate_tilt(2500.0, 0.0, &config);
        assert!(tilt > 0.0 && tilt <= 60.0);
    }
}
