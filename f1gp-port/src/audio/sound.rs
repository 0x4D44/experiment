// Sound Types and Sources - Stage 6.6
// Defines different types of sounds and how to generate them

use std::time::Duration;

/// Types of sounds in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SoundType {
    /// Engine sound (RPM-based)
    Engine,
    /// Tire screech (when sliding)
    TireScreech,
    /// Gear shift
    GearShift,
    /// Collision/impact
    Collision,
    /// Wind/aerodynamic noise
    Wind,
    /// Kerb rumble
    Kerb,
}

/// Sound source for audio playback
#[derive(Debug, Clone)]
pub struct SoundSource {
    /// Type of sound
    pub sound_type: SoundType,
    /// Volume (0.0 to 1.0)
    pub volume: f32,
    /// Pitch/frequency multiplier (1.0 = normal)
    pub pitch: f32,
    /// Whether the sound should loop
    pub looping: bool,
    /// Fade in duration
    pub fade_in: Option<Duration>,
    /// Fade out duration
    pub fade_out: Option<Duration>,
}

impl SoundSource {
    /// Create a new sound source
    pub fn new(sound_type: SoundType) -> Self {
        Self {
            sound_type,
            volume: 1.0,
            pitch: 1.0,
            looping: false,
            fade_in: None,
            fade_out: None,
        }
    }

    /// Set volume (0.0 to 1.0)
    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume.clamp(0.0, 1.0);
        self
    }

    /// Set pitch multiplier
    pub fn with_pitch(mut self, pitch: f32) -> Self {
        self.pitch = pitch.max(0.1); // Prevent zero/negative pitch
        self
    }

    /// Enable looping
    pub fn looping(mut self) -> Self {
        self.looping = true;
        self
    }

    /// Set fade in duration
    pub fn with_fade_in(mut self, duration: Duration) -> Self {
        self.fade_in = Some(duration);
        self
    }

    /// Set fade out duration
    pub fn with_fade_out(mut self, duration: Duration) -> Self {
        self.fade_out = Some(duration);
        self
    }

    /// Create engine sound with RPM
    pub fn engine(rpm: f32) -> Self {
        // Map RPM to pitch (idle ~1000 RPM, max ~15000 RPM)
        let pitch = (rpm / 10000.0).clamp(0.5, 2.0);
        Self::new(SoundType::Engine)
            .with_pitch(pitch)
            .looping()
    }

    /// Create tire screech sound with slip ratio
    pub fn tire_screech(slip_ratio: f32) -> Self {
        let volume = slip_ratio.clamp(0.0, 1.0);
        Self::new(SoundType::TireScreech)
            .with_volume(volume)
    }

    /// Create gear shift sound
    pub fn gear_shift() -> Self {
        Self::new(SoundType::GearShift)
            .with_volume(0.8)
    }

    /// Create collision sound with impact strength
    pub fn collision(impact: f32) -> Self {
        let volume = (impact / 50.0).clamp(0.0, 1.0);
        Self::new(SoundType::Collision)
            .with_volume(volume)
    }

    /// Create wind sound based on speed
    pub fn wind(speed: f32) -> Self {
        // Wind sound starts at 50 km/h
        let volume = ((speed - 50.0) / 200.0).clamp(0.0, 0.5);
        let pitch = (speed / 200.0).clamp(0.8, 1.5);
        Self::new(SoundType::Wind)
            .with_volume(volume)
            .with_pitch(pitch)
            .looping()
    }

    /// Create kerb rumble sound
    pub fn kerb() -> Self {
        Self::new(SoundType::Kerb)
            .with_volume(0.7)
            .looping()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sound_source_creation() {
        let sound = SoundSource::new(SoundType::Engine);
        assert_eq!(sound.sound_type, SoundType::Engine);
        assert_eq!(sound.volume, 1.0);
        assert_eq!(sound.pitch, 1.0);
        assert!(!sound.looping);
    }

    #[test]
    fn test_sound_source_builder() {
        let sound = SoundSource::new(SoundType::Engine)
            .with_volume(0.5)
            .with_pitch(1.5)
            .looping();

        assert_eq!(sound.volume, 0.5);
        assert_eq!(sound.pitch, 1.5);
        assert!(sound.looping);
    }

    #[test]
    fn test_engine_sound() {
        let sound = SoundSource::engine(10000.0);
        assert_eq!(sound.sound_type, SoundType::Engine);
        assert_eq!(sound.pitch, 1.0);
        assert!(sound.looping);
    }

    #[test]
    fn test_tire_screech() {
        let sound = SoundSource::tire_screech(0.7);
        assert_eq!(sound.sound_type, SoundType::TireScreech);
        assert_eq!(sound.volume, 0.7);
    }

    #[test]
    fn test_volume_clamping() {
        let sound = SoundSource::new(SoundType::Engine).with_volume(1.5);
        assert_eq!(sound.volume, 1.0);

        let sound = SoundSource::new(SoundType::Engine).with_volume(-0.5);
        assert_eq!(sound.volume, 0.0);
    }
}
