use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub enabled: bool,
    pub mode: CursorMode,
    pub threshold: f64,
    pub rotate: RotateConfig,
    pub tilt: TiltConfig,
    pub stretch: StretchConfig,
    pub shake: ShakeConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CursorMode {
    None,
    Rotate,
    Tilt,
    Stretch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotateConfig {
    /// Length in pixels of the simulated stick used to rotate the cursor
    pub length: f64,
    /// Clockwise offset applied to the angle in degrees
    pub offset: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TiltConfig {
    /// Controls how powerful the tilt is (speed in px/s for full tilt)
    pub limit: f64,
    /// Activation function: linear, quadratic, or negative_quadratic
    pub function: ActivationFunction,
    /// Time window in ms over which speed is calculated
    pub window: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StretchConfig {
    /// Controls how much the cursor is stretched (speed in px/s for full stretch)
    pub limit: f64,
    /// Activation function: linear, quadratic, or negative_quadratic
    pub function: ActivationFunction,
    /// Time window in ms over which speed is calculated
    pub window: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivationFunction {
    Linear,
    Quadratic,
    NegativeQuadratic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShakeConfig {
    pub enabled: bool,
    pub nearest: bool,
    pub threshold: f64,
    pub base: f64,
    pub speed: f64,
    pub influence: f64,
    pub limit: f64,
    pub timeout: u64,
    pub effects: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: CursorMode::Tilt,
            threshold: 2.0,
            rotate: RotateConfig::default(),
            tilt: TiltConfig::default(),
            stretch: StretchConfig::default(),
            shake: ShakeConfig::default(),
        }
    }
}

impl Default for RotateConfig {
    fn default() -> Self {
        Self {
            length: 20.0,
            offset: 0.0,
        }
    }
}

impl Default for TiltConfig {
    fn default() -> Self {
        Self {
            limit: 5000.0,
            function: ActivationFunction::NegativeQuadratic,
            window: 100,
        }
    }
}

impl Default for StretchConfig {
    fn default() -> Self {
        Self {
            limit: 3000.0,
            function: ActivationFunction::Quadratic,
            window: 100,
        }
    }
}

impl Default for ShakeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            nearest: true,
            threshold: 6.0,
            base: 4.0,
            speed: 4.0,
            influence: 0.0,
            limit: 0.0,
            timeout: 2000,
            effects: false,
        }
    }
}

impl Config {
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_str)
    }

    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }
}
