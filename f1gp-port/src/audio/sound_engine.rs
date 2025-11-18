//! Sound Engine - SDL2-based audio system for F1GP
//!
//! Provides real-time audio synthesis for:
//! - Engine sounds (RPM-based)
//! - Gear shifts
//! - Menu sounds
//! - Effects

use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use std::sync::{Arc, Mutex};
use std::f32::consts::PI;

/// Audio engine state (shared between game thread and audio callback)
#[derive(Debug, Clone)]
pub struct AudioState {
    /// Current engine RPM
    pub rpm: f32,
    /// Master volume (0.0 to 1.0)
    pub volume: f32,
    /// Engine volume (0.0 to 1.0)
    pub engine_volume: f32,
    /// Whether to play gear shift sound
    pub gear_shift_trigger: bool,
    /// Gear shift time remaining (in samples)
    pub gear_shift_samples: u32,
    /// Menu beep trigger
    pub menu_beep_trigger: bool,
    /// Menu beep samples remaining
    pub menu_beep_samples: u32,
    /// Tire squeal intensity (0.0 = no squeal, 1.0 = max squeal)
    pub tire_squeal_intensity: f32,
    /// Collision trigger
    pub collision_trigger: bool,
    /// Collision samples remaining
    pub collision_samples: u32,
    /// Muted flag
    pub muted: bool,
}

impl Default for AudioState {
    fn default() -> Self {
        Self {
            rpm: 1000.0, // Idle RPM
            volume: 0.3, // 30% master volume
            engine_volume: 0.7,
            gear_shift_trigger: false,
            gear_shift_samples: 0,
            menu_beep_trigger: false,
            menu_beep_samples: 0,
            tire_squeal_intensity: 0.0,
            collision_trigger: false,
            collision_samples: 0,
            muted: false,
        }
    }
}

/// Audio callback that generates engine sounds in real-time
struct EngineAudioCallback {
    state: Arc<Mutex<AudioState>>,
    phase: f32,
    sample_rate: f32,
    gear_shift_phase: f32,
    menu_beep_phase: f32,
}

impl AudioCallback for EngineAudioCallback {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Copy state values to avoid holding lock during generation
        let (rpm, volume, engine_volume, _muted, mut gear_shift_samples, mut menu_beep_samples, tire_squeal_intensity, mut collision_samples) = {
            let mut state = self.state.lock().unwrap();

            // If muted, output silence
            if state.muted {
                for x in out.iter_mut() {
                    *x = 0.0;
                }
                return;
            }

            // Check for new gear shift trigger
            if state.gear_shift_trigger {
                state.gear_shift_trigger = false;
                state.gear_shift_samples = (self.sample_rate * 0.1) as u32; // 100ms
                self.gear_shift_phase = 0.0;
            }

            // Check for new menu beep trigger
            if state.menu_beep_trigger {
                state.menu_beep_trigger = false;
                state.menu_beep_samples = (self.sample_rate * 0.05) as u32; // 50ms
                self.menu_beep_phase = 0.0;
            }

            // Check for new collision trigger
            if state.collision_trigger {
                state.collision_trigger = false;
                state.collision_samples = (self.sample_rate * 0.15) as u32; // 150ms
            }

            // Copy values we need
            (
                state.rpm,
                state.volume,
                state.engine_volume,
                state.muted,
                state.gear_shift_samples,
                state.menu_beep_samples,
                state.tire_squeal_intensity,
                state.collision_samples,
            )
        }; // Lock is released here

        // Generate audio samples
        for x in out.iter_mut() {
            let mut sample = 0.0;

            // 1. Engine sound (always playing)
            let engine_sample = self.generate_engine_sound(rpm);
            sample += engine_sample * engine_volume;

            // 2. Gear shift sound (short burst)
            if gear_shift_samples > 0 {
                let shift_sample = self.generate_gear_shift();
                sample += shift_sample * 0.5;
                gear_shift_samples = gear_shift_samples.saturating_sub(1);
            }

            // 3. Menu beep sound
            if menu_beep_samples > 0 {
                let beep_sample = self.generate_menu_beep();
                sample += beep_sample * 0.3;
                menu_beep_samples = menu_beep_samples.saturating_sub(1);
            }

            // 4. Tire squeal sound (continuous based on intensity)
            if tire_squeal_intensity > 0.01 {
                let squeal_sample = self.generate_tire_squeal();
                sample += squeal_sample * tire_squeal_intensity * 0.4;
            }

            // 5. Collision sound (short burst)
            if collision_samples > 0 {
                let collision_sample = self.generate_collision();
                sample += collision_sample * 0.6;
                collision_samples = collision_samples.saturating_sub(1);
            }

            // Apply master volume and output
            *x = (sample * volume).clamp(-1.0, 1.0);
        }

        // Update state with new sample counts
        {
            let mut state = self.state.lock().unwrap();
            state.gear_shift_samples = gear_shift_samples;
            state.menu_beep_samples = menu_beep_samples;
            state.collision_samples = collision_samples;
        }
    }
}

impl EngineAudioCallback {
    /// Generate engine sound based on current RPM
    /// Uses sawtooth wave with harmonics for realistic engine tone
    fn generate_engine_sound(&mut self, rpm: f32) -> f32 {
        // Convert RPM to frequency
        // Formula: freq = base_freq + (rpm / max_rpm) * freq_range
        // Idle (1000 RPM) = ~40 Hz
        // Redline (13000 RPM) = ~300 Hz
        let rpm_normalized = ((rpm - 1000.0) / 12000.0).clamp(0.0, 1.0);
        let base_freq = 40.0;
        let freq_range = 260.0;
        let frequency = base_freq + rpm_normalized * freq_range;

        // Generate sawtooth wave with harmonics
        let t = self.phase / self.sample_rate;
        let fundamental = self.sawtooth(t * frequency);
        let harmonic_2 = self.sawtooth(t * frequency * 2.0) * 0.5;
        let harmonic_3 = self.sawtooth(t * frequency * 3.0) * 0.25;

        // Advance phase
        self.phase += 1.0;
        if self.phase >= self.sample_rate {
            self.phase -= self.sample_rate;
        }

        // Mix harmonics
        let engine_wave = (fundamental + harmonic_2 + harmonic_3) / 1.75;

        // Add subtle noise for roughness (10% noise)
        let noise = (fastrand::f32() * 2.0 - 1.0) * 0.1;

        engine_wave * 0.9 + noise * 0.1
    }

    /// Generate gear shift sound
    /// White noise burst with envelope
    fn generate_gear_shift(&mut self) -> f32 {
        // Generate white noise
        let noise = fastrand::f32() * 2.0 - 1.0;

        // Apply envelope (attack-decay)
        let progress = self.gear_shift_phase / 4800.0; // ~100ms at 48kHz
        let envelope = if progress < 0.1 {
            progress / 0.1 // Attack (10ms)
        } else {
            (1.0 - progress) / 0.9 // Decay (90ms)
        };

        self.gear_shift_phase += 1.0;

        noise * envelope
    }

    /// Generate menu beep sound
    /// Simple sine wave at 440 Hz (A note)
    fn generate_menu_beep(&mut self) -> f32 {
        let frequency = 440.0;
        let t = self.menu_beep_phase / self.sample_rate;
        let wave = (2.0 * PI * frequency * t).sin();

        // Apply envelope
        let progress = self.menu_beep_phase / 2400.0; // ~50ms at 48kHz
        let envelope = if progress < 0.2 {
            progress / 0.2 // Attack
        } else {
            (1.0 - progress) / 0.8 // Decay
        };

        self.menu_beep_phase += 1.0;

        wave * envelope
    }

    /// Generate tire squeal sound
    /// High-frequency filtered noise for realistic tire squeal
    fn generate_tire_squeal(&mut self) -> f32 {
        // Generate white noise
        let noise = fastrand::f32() * 2.0 - 1.0;

        // Apply high-pass filter (tire squeal is high frequency)
        // Using a simple one-pole high-pass filter
        // Tire squeal is typically 1-4 kHz range
        let filtered = noise * 0.7; // Simplified filtering

        filtered
    }

    /// Generate collision sound
    /// Sharp impact sound with quick decay
    fn generate_collision(&mut self) -> f32 {
        // Generate noise with some low frequency component for "thud"
        let noise = fastrand::f32() * 2.0 - 1.0;

        // Mix with low frequency sine for impact "thump"
        let thump_freq = 80.0;
        let t = self.phase / self.sample_rate;
        let thump = (2.0 * PI * thump_freq * t).sin() * 0.5;

        noise * 0.7 + thump * 0.3
    }

    /// Generate sawtooth wave
    fn sawtooth(&self, t: f32) -> f32 {
        2.0 * (t - (t + 0.5).floor())
    }
}

/// Main audio engine
pub struct SoundEngine {
    _device: AudioDevice<EngineAudioCallback>,
    state: Arc<Mutex<AudioState>>,
}

impl SoundEngine {
    /// Initialize the sound engine with SDL2 audio
    pub fn new(sdl_audio: &sdl2::AudioSubsystem) -> Result<Self, String> {
        let desired_spec = AudioSpecDesired {
            freq: Some(48000),
            channels: Some(1), // Mono
            samples: Some(1024),
        };

        let state = Arc::new(Mutex::new(AudioState::default()));
        let callback_state = state.clone();

        let device = sdl_audio.open_playback(None, &desired_spec, |spec| {
            log::info!("Audio initialized: {} Hz, {} channels", spec.freq, spec.channels);

            EngineAudioCallback {
                state: callback_state,
                phase: 0.0,
                sample_rate: spec.freq as f32,
                gear_shift_phase: 0.0,
                menu_beep_phase: 0.0,
            }
        })?;

        // Start playback
        device.resume();

        log::info!("Sound engine initialized successfully");

        Ok(Self { _device: device, state })
    }

    /// Update engine RPM
    pub fn set_rpm(&self, rpm: f32) {
        if let Ok(mut state) = self.state.lock() {
            state.rpm = rpm.clamp(0.0, 15000.0);
        }
    }

    /// Trigger gear shift sound
    pub fn play_gear_shift(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.gear_shift_trigger = true;
        }
    }

    /// Trigger menu beep sound
    pub fn play_menu_beep(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.menu_beep_trigger = true;
        }
    }

    /// Set tire squeal intensity (0.0 = no squeal, 1.0 = max squeal)
    pub fn set_tire_squeal(&self, intensity: f32) {
        if let Ok(mut state) = self.state.lock() {
            state.tire_squeal_intensity = intensity.clamp(0.0, 1.0);
        }
    }

    /// Trigger collision sound
    pub fn play_collision(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.collision_trigger = true;
        }
    }

    /// Set master volume (0.0 to 1.0)
    pub fn set_volume(&self, volume: f32) {
        if let Ok(mut state) = self.state.lock() {
            state.volume = volume.clamp(0.0, 1.0);
        }
    }

    /// Set engine volume (0.0 to 1.0)
    pub fn set_engine_volume(&self, volume: f32) {
        if let Ok(mut state) = self.state.lock() {
            state.engine_volume = volume.clamp(0.0, 1.0);
        }
    }

    /// Toggle mute
    pub fn toggle_mute(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.muted = !state.muted;
            log::info!("Audio {}", if state.muted { "muted" } else { "unmuted" });
        }
    }

    /// Check if muted
    pub fn is_muted(&self) -> bool {
        self.state.lock().map(|s| s.muted).unwrap_or(true)
    }

    /// Get current RPM
    pub fn get_rpm(&self) -> f32 {
        self.state.lock().map(|s| s.rpm).unwrap_or(0.0)
    }
}
