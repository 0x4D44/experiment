/**
 * Sound Effects Manager
 * Handles all audio feedback using Web Audio API
 */

class SoundManager {
    constructor() {
        this.enabled = true;
        this.audioContext = null;
        this.initAudioContext();
    }

    /**
     * Initialize Web Audio API context
     */
    initAudioContext() {
        try {
            window.AudioContext = window.AudioContext || window.webkitAudioContext;
            this.audioContext = new AudioContext();
        } catch (e) {
            console.warn('Web Audio API not supported');
            this.enabled = false;
        }
    }

    /**
     * Create oscillator for generating tones
     */
    createOscillator(frequency, type = 'sine') {
        if (!this.enabled || !this.audioContext) return null;

        const oscillator = this.audioContext.createOscillator();
        const gainNode = this.audioContext.createGain();

        oscillator.connect(gainNode);
        gainNode.connect(this.audioContext.destination);

        oscillator.type = type;
        oscillator.frequency.value = frequency;

        return { oscillator, gainNode };
    }

    /**
     * Play card flip sound
     */
    playFlip() {
        if (!this.enabled) return;

        const { oscillator, gainNode } = this.createOscillator(800, 'sine');
        if (!oscillator) return;

        const now = this.audioContext.currentTime;

        gainNode.gain.setValueAtTime(0.1, now);
        gainNode.gain.exponentialRampToValueAtTime(0.01, now + 0.1);

        oscillator.start(now);
        oscillator.stop(now + 0.1);
    }

    /**
     * Play match success sound
     */
    playMatch() {
        if (!this.enabled) return;

        // Play ascending chord
        const frequencies = [523.25, 659.25, 783.99]; // C5, E5, G5
        const startTime = this.audioContext.currentTime;

        frequencies.forEach((freq, index) => {
            const { oscillator, gainNode } = this.createOscillator(freq, 'sine');
            if (!oscillator) return;

            const time = startTime + (index * 0.1);

            gainNode.gain.setValueAtTime(0.15, time);
            gainNode.gain.exponentialRampToValueAtTime(0.01, time + 0.3);

            oscillator.start(time);
            oscillator.stop(time + 0.3);
        });
    }

    /**
     * Play mismatch sound
     */
    playMismatch() {
        if (!this.enabled) return;

        const { oscillator, gainNode } = this.createOscillator(200, 'sawtooth');
        if (!oscillator) return;

        const now = this.audioContext.currentTime;

        gainNode.gain.setValueAtTime(0.1, now);
        gainNode.gain.exponentialRampToValueAtTime(0.01, now + 0.15);

        oscillator.start(now);
        oscillator.stop(now + 0.15);
    }

    /**
     * Play win celebration sound
     */
    playWin() {
        if (!this.enabled) return;

        // Play victory fanfare
        const melody = [
            { freq: 523.25, time: 0 },      // C5
            { freq: 659.25, time: 0.15 },   // E5
            { freq: 783.99, time: 0.3 },    // G5
            { freq: 1046.50, time: 0.45 }   // C6
        ];

        const startTime = this.audioContext.currentTime;

        melody.forEach(note => {
            const { oscillator, gainNode } = this.createOscillator(note.freq, 'sine');
            if (!oscillator) return;

            const time = startTime + note.time;

            gainNode.gain.setValueAtTime(0.2, time);
            gainNode.gain.exponentialRampToValueAtTime(0.01, time + 0.4);

            oscillator.start(time);
            oscillator.stop(time + 0.4);
        });
    }

    /**
     * Play button click sound
     */
    playClick() {
        if (!this.enabled) return;

        const { oscillator, gainNode } = this.createOscillator(1000, 'square');
        if (!oscillator) return;

        const now = this.audioContext.currentTime;

        gainNode.gain.setValueAtTime(0.05, now);
        gainNode.gain.exponentialRampToValueAtTime(0.01, now + 0.05);

        oscillator.start(now);
        oscillator.stop(now + 0.05);
    }

    /**
     * Play new game sound
     */
    playNewGame() {
        if (!this.enabled) return;

        const { oscillator, gainNode } = this.createOscillator(440, 'triangle');
        if (!oscillator) return;

        const now = this.audioContext.currentTime;

        oscillator.frequency.setValueAtTime(440, now);
        oscillator.frequency.exponentialRampToValueAtTime(880, now + 0.2);

        gainNode.gain.setValueAtTime(0.1, now);
        gainNode.gain.exponentialRampToValueAtTime(0.01, now + 0.2);

        oscillator.start(now);
        oscillator.stop(now + 0.2);
    }

    /**
     * Toggle sound on/off
     */
    toggle() {
        this.enabled = !this.enabled;

        // Resume audio context if it was suspended
        if (this.enabled && this.audioContext && this.audioContext.state === 'suspended') {
            this.audioContext.resume();
        }

        return this.enabled;
    }

    /**
     * Set volume (0-1)
     */
    setVolume(volume) {
        this.volume = Math.max(0, Math.min(1, volume));
    }
}

// Create global sound manager instance
const soundManager = new SoundManager();
