// Audio Engine - Stage 6.6
// Core audio system using rodio

use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

use super::sound::{SoundSource, SoundType};

/// Audio engine errors
#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Failed to initialize audio output")]
    InitializationError,

    #[error("Failed to play sound: {0}")]
    PlaybackError(String),

    #[error("Sound file not found: {0:?}")]
    SoundNotFound(SoundType),
}

pub type Result<T> = std::result::Result<T, AudioError>;

/// Audio engine for managing sound playback
pub struct AudioEngine {
    /// Audio output stream (must be kept alive)
    _stream: OutputStream,
    /// Audio output stream handle
    stream_handle: OutputStreamHandle,
    /// Active sound sinks (for looping sounds)
    sinks: HashMap<SoundType, Sink>,
    /// Master volume (0.0 to 1.0)
    master_volume: f32,
    /// Whether audio is enabled
    enabled: bool,
}

impl AudioEngine {
    /// Create a new audio engine
    pub fn new() -> Result<Self> {
        // Initialize audio output
        let (stream, stream_handle) = OutputStream::try_default()
            .map_err(|_| AudioError::InitializationError)?;

        Ok(Self {
            _stream: stream,
            stream_handle,
            sinks: HashMap::new(),
            master_volume: 1.0,
            enabled: true,
        })
    }

    /// Set master volume (0.0 to 1.0)
    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);

        // Update volume on all active sinks
        for sink in self.sinks.values() {
            sink.set_volume(self.master_volume);
        }
    }

    /// Get master volume
    pub fn master_volume(&self) -> f32 {
        self.master_volume
    }

    /// Enable or disable audio
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;

        if !enabled {
            // Pause all active sinks
            for sink in self.sinks.values() {
                sink.pause();
            }
        } else {
            // Resume all active sinks
            for sink in self.sinks.values() {
                sink.play();
            }
        }
    }

    /// Check if audio is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Play a sound (one-shot)
    pub fn play_sound(&self, source: &SoundSource) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        // For now, we'll generate simple tones (will be replaced with actual samples)
        let sample_rate = 44100;
        let duration = 0.1; // 100ms for non-looping sounds

        // Generate a simple sine wave as a placeholder
        let frequency = match source.sound_type {
            SoundType::Engine => 200.0 * source.pitch,
            SoundType::TireScreech => 800.0,
            SoundType::GearShift => 150.0,
            SoundType::Collision => 100.0,
            SoundType::Wind => 300.0 * source.pitch,
            SoundType::Kerb => 120.0,
        };

        let samples = generate_sine_wave(frequency, source.volume * self.master_volume, duration, sample_rate);

        // Play the sound
        self.stream_handle.play_raw(samples.convert_samples())
            .map_err(|e| AudioError::PlaybackError(e.to_string()))?;

        Ok(())
    }

    /// Start a looping sound
    pub fn start_looping(&mut self, source: &SoundSource) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        // Stop existing sound of this type
        self.stop_looping(source.sound_type);

        // For now, we'll generate simple tones
        let sample_rate = 44100;
        let duration = 1.0; // 1 second loop

        let frequency = match source.sound_type {
            SoundType::Engine => 200.0 * source.pitch,
            SoundType::TireScreech => 800.0,
            SoundType::GearShift => 150.0,
            SoundType::Collision => 100.0,
            SoundType::Wind => 300.0 * source.pitch,
            SoundType::Kerb => 120.0,
        };

        let samples = generate_sine_wave(frequency, source.volume * self.master_volume, duration, sample_rate);

        // Create a sink for this sound
        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| AudioError::PlaybackError(e.to_string()))?;

        // Append the samples with repeat
        sink.append(samples.repeat_infinite());
        sink.set_volume(source.volume * self.master_volume);

        // Store the sink
        self.sinks.insert(source.sound_type, sink);

        Ok(())
    }

    /// Stop a looping sound
    pub fn stop_looping(&mut self, sound_type: SoundType) {
        if let Some(sink) = self.sinks.remove(&sound_type) {
            sink.stop();
        }
    }

    /// Update a looping sound's pitch (for engine RPM changes)
    pub fn update_pitch(&self, sound_type: SoundType, pitch: f32) {
        if let Some(sink) = self.sinks.get(&sound_type) {
            sink.set_speed(pitch);
        }
    }

    /// Update a looping sound's volume
    pub fn update_volume(&self, sound_type: SoundType, volume: f32) {
        if let Some(sink) = self.sinks.get(&sound_type) {
            sink.set_volume(volume * self.master_volume);
        }
    }

    /// Stop all sounds
    pub fn stop_all(&mut self) {
        for sink in self.sinks.values() {
            sink.stop();
        }
        self.sinks.clear();
    }

    /// Update the audio engine (call each frame)
    pub fn update(&mut self, _delta_time: f32) {
        // Remove finished sinks
        self.sinks.retain(|_, sink| !sink.empty());
    }
}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        self.stop_all();
    }
}

/// Generate a simple sine wave for testing
fn generate_sine_wave(frequency: f32, amplitude: f32, duration: f32, sample_rate: u32) -> SineWave {
    SineWave {
        frequency,
        amplitude,
        sample_rate,
        num_samples: (duration * sample_rate as f32) as usize,
        current_sample: 0,
    }
}

/// Simple sine wave generator (placeholder for actual audio samples)
struct SineWave {
    frequency: f32,
    amplitude: f32,
    sample_rate: u32,
    num_samples: usize,
    current_sample: usize,
}

impl Iterator for SineWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_sample >= self.num_samples {
            return None;
        }

        let t = self.current_sample as f32 / self.sample_rate as f32;
        let sample = (2.0 * std::f32::consts::PI * self.frequency * t).sin() * self.amplitude;

        self.current_sample += 1;

        Some(sample)
    }
}

impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.num_samples - self.current_sample)
    }

    fn channels(&self) -> u16 {
        1 // Mono
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        Some(std::time::Duration::from_secs_f32(
            self.num_samples as f32 / self.sample_rate as f32
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_engine_creation() {
        // Audio engine creation may fail in CI without audio devices
        if let Ok(engine) = AudioEngine::new() {
            assert!(engine.is_enabled());
            assert_eq!(engine.master_volume(), 1.0);
        }
    }

    #[test]
    fn test_master_volume() {
        if let Ok(mut engine) = AudioEngine::new() {
            engine.set_master_volume(0.5);
            assert_eq!(engine.master_volume(), 0.5);

            engine.set_master_volume(1.5); // Should clamp to 1.0
            assert_eq!(engine.master_volume(), 1.0);

            engine.set_master_volume(-0.5); // Should clamp to 0.0
            assert_eq!(engine.master_volume(), 0.0);
        }
    }

    #[test]
    fn test_enable_disable() {
        if let Ok(mut engine) = AudioEngine::new() {
            assert!(engine.is_enabled());

            engine.set_enabled(false);
            assert!(!engine.is_enabled());

            engine.set_enabled(true);
            assert!(engine.is_enabled());
        }
    }

    #[test]
    fn test_sine_wave_generation() {
        let mut wave = generate_sine_wave(440.0, 1.0, 0.01, 44100);

        // Should generate some samples
        assert!(wave.next().is_some());

        // Should have correct properties
        assert_eq!(wave.channels(), 1);
        assert_eq!(wave.sample_rate(), 44100);
    }
}
