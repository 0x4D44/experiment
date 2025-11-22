//! Weather system for F1GP
//!
//! Implements dynamic weather conditions that affect grip levels and visibility.
//! Integrates with existing surface physics system.

use serde::{Deserialize, Serialize};

/// Weather conditions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeatherCondition {
    /// Clear, dry conditions (100% grip)
    Dry,

    /// Light rain (80% grip)
    LightRain,

    /// Heavy rain (60% grip)
    HeavyRain,
}

impl WeatherCondition {
    /// Get grip multiplier for this weather condition
    ///
    /// This multiplier is applied on top of surface-specific grip
    pub fn grip_multiplier(self) -> f32 {
        match self {
            WeatherCondition::Dry => 1.0,
            WeatherCondition::LightRain => 0.8,
            WeatherCondition::HeavyRain => 0.6,
        }
    }

    /// Get visibility range in meters
    pub fn visibility_range(self) -> f32 {
        match self {
            WeatherCondition::Dry => 1000.0,
            WeatherCondition::LightRain => 500.0,
            WeatherCondition::HeavyRain => 200.0,
        }
    }

    /// Get display name
    pub fn name(self) -> &'static str {
        match self {
            WeatherCondition::Dry => "Dry",
            WeatherCondition::LightRain => "Light Rain",
            WeatherCondition::HeavyRain => "Heavy Rain",
        }
    }

    /// Get weather icon/symbol for HUD
    pub fn symbol(self) -> &'static str {
        match self {
            WeatherCondition::Dry => "SUN",
            WeatherCondition::LightRain => "RAIN",
            WeatherCondition::HeavyRain => "STORM",
        }
    }
}

impl Default for WeatherCondition {
    fn default() -> Self {
        WeatherCondition::Dry
    }
}

/// Weather system state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherSystem {
    /// Current weather condition
    pub condition: WeatherCondition,

    /// Track wetness level (0.0 = completely dry, 1.0 = fully wet)
    ///
    /// This value changes gradually as rain starts/stops, creating
    /// a drying line effect
    pub wetness: f32,

    /// Time until next weather change (seconds, None = static weather)
    pub time_to_change: Option<f32>,

    /// Enable dynamic weather changes during race
    pub dynamic_weather: bool,
}

impl WeatherSystem {
    /// Create new weather system with given condition
    pub fn new(condition: WeatherCondition) -> Self {
        let wetness = match condition {
            WeatherCondition::Dry => 0.0,
            WeatherCondition::LightRain => 0.5,
            WeatherCondition::HeavyRain => 1.0,
        };

        Self {
            condition,
            wetness,
            time_to_change: None,
            dynamic_weather: false,
        }
    }

    /// Create weather system with dynamic changes enabled
    pub fn new_dynamic(initial_condition: WeatherCondition, change_interval: f32) -> Self {
        let mut weather = Self::new(initial_condition);
        weather.dynamic_weather = true;
        weather.time_to_change = Some(change_interval);
        weather
    }

    /// Update weather system (call once per frame)
    pub fn update(&mut self, delta_time: f32) {
        // Update track wetness gradually
        let target_wetness = match self.condition {
            WeatherCondition::Dry => 0.0,
            WeatherCondition::LightRain => 0.5,
            WeatherCondition::HeavyRain => 1.0,
        };

        // Track dries/wets at different rates
        let change_rate = if self.wetness < target_wetness {
            // Gets wet quickly (0.1 per second)
            0.1
        } else {
            // Dries slowly (0.02 per second - 50 seconds to fully dry)
            0.02
        };

        // Smoothly interpolate wetness
        if self.wetness < target_wetness {
            self.wetness = (self.wetness + change_rate * delta_time).min(target_wetness);
        } else if self.wetness > target_wetness {
            self.wetness = (self.wetness - change_rate * delta_time).max(target_wetness);
        }

        // Handle dynamic weather changes
        if self.dynamic_weather {
            let should_change = if let Some(ref mut time) = self.time_to_change {
                *time -= delta_time;
                *time <= 0.0
            } else {
                false
            };

            if should_change {
                // Change weather randomly
                self.change_weather_random();
                // Reset timer (2-5 minutes between changes)
                self.time_to_change = Some(120.0 + fastrand::f32() * 180.0);
            }
        }
    }

    /// Change to random weather condition
    fn change_weather_random(&mut self) {
        let conditions = [
            WeatherCondition::Dry,
            WeatherCondition::LightRain,
            WeatherCondition::HeavyRain,
        ];

        // Pick random condition different from current
        loop {
            let new_condition = conditions[fastrand::usize(0..3)];
            if new_condition != self.condition {
                log::info!(
                    "Weather changing: {} -> {}",
                    self.condition.name(),
                    new_condition.name()
                );
                self.condition = new_condition;
                break;
            }
        }
    }

    /// Get effective grip multiplier at current wetness level
    ///
    /// This interpolates between dry and wet grip based on track wetness
    pub fn effective_grip_multiplier(&self) -> f32 {
        let dry_grip = 1.0;
        let wet_grip = self.condition.grip_multiplier();

        // Linear interpolation between dry and wet grip
        dry_grip * (1.0 - self.wetness) + wet_grip * self.wetness
    }

    /// Check if track is wet enough to matter (> 10% wet)
    pub fn is_wet(&self) -> bool {
        self.wetness > 0.1
    }

    /// Get rain intensity (0.0 = no rain, 1.0 = heavy rain)
    ///
    /// Used for visual/audio effects
    pub fn rain_intensity(&self) -> f32 {
        match self.condition {
            WeatherCondition::Dry => 0.0,
            WeatherCondition::LightRain => 0.4,
            WeatherCondition::HeavyRain => 1.0,
        }
    }
}

impl Default for WeatherSystem {
    fn default() -> Self {
        Self::new(WeatherCondition::Dry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_grip_multipliers() {
        assert_eq!(WeatherCondition::Dry.grip_multiplier(), 1.0);
        assert_eq!(WeatherCondition::LightRain.grip_multiplier(), 0.8);
        assert_eq!(WeatherCondition::HeavyRain.grip_multiplier(), 0.6);
    }

    #[test]
    fn test_weather_system_creation() {
        let weather = WeatherSystem::new(WeatherCondition::Dry);
        assert_eq!(weather.condition, WeatherCondition::Dry);
        assert_eq!(weather.wetness, 0.0);
        assert!(!weather.is_wet());
    }

    #[test]
    fn test_weather_wetness_change() {
        let mut weather = WeatherSystem::new(WeatherCondition::Dry);
        weather.condition = WeatherCondition::HeavyRain;

        // Should gradually get wet
        weather.update(1.0);
        assert!(weather.wetness > 0.0);
        assert!(weather.wetness < 1.0);

        // After enough time, should be fully wet
        weather.update(10.0);
        assert_eq!(weather.wetness, 1.0);
        assert!(weather.is_wet());
    }

    #[test]
    fn test_weather_drying() {
        let mut weather = WeatherSystem::new(WeatherCondition::HeavyRain);
        weather.wetness = 1.0;
        weather.condition = WeatherCondition::Dry;

        // Should gradually dry
        weather.update(1.0);
        assert!(weather.wetness < 1.0);
        assert!(weather.wetness > 0.0);
    }

    #[test]
    fn test_effective_grip() {
        let mut weather = WeatherSystem::new(WeatherCondition::Dry);
        assert_eq!(weather.effective_grip_multiplier(), 1.0);

        weather.wetness = 0.5;
        weather.condition = WeatherCondition::HeavyRain;
        // Should be halfway between 1.0 (dry) and 0.6 (heavy rain)
        assert!((weather.effective_grip_multiplier() - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_dynamic_weather() {
        let weather = WeatherSystem::new_dynamic(WeatherCondition::Dry, 60.0);
        assert!(weather.dynamic_weather);
        assert_eq!(weather.time_to_change, Some(60.0));
    }

    #[test]
    fn test_rain_intensity() {
        assert_eq!(WeatherCondition::Dry.grip_multiplier(), 1.0);

        let weather = WeatherSystem::new(WeatherCondition::LightRain);
        assert!(weather.rain_intensity() > 0.0);
        assert!(weather.rain_intensity() < 1.0);
    }
}
