/**
 * Audio engine for generating sonar pulses and echoes
 */

export interface Echo {
  distance: number;
  angle: number; // radians
  type: 'obstacle' | 'objective' | 'wall';
}

// Define types for web-audio-api
interface WebAudioContext {
  createOscillator(): WebAudioOscillatorNode;
  createGain(): WebAudioGainNode;
  createStereoPanner?(): WebAudioStereoPannerNode;
  createPanner(): WebAudioPannerNode;
  readonly currentTime: number;
  readonly destination: WebAudioDestinationNode;
  close(): Promise<void>;
}

interface WebAudioOscillatorNode {
  connect(destination: WebAudioGainNode): void;
  type: 'sine' | 'square' | 'sawtooth' | 'triangle';
  readonly frequency: WebAudioParam;
  start(when?: number): void;
  stop(when?: number): void;
}

interface WebAudioGainNode {
  connect(destination: WebAudioGainNode | WebAudioStereoPannerNode | WebAudioPannerNode | WebAudioDestinationNode): void;
  readonly gain: WebAudioParam;
}

interface WebAudioStereoPannerNode {
  connect(destination: WebAudioGainNode): void;
  readonly pan: WebAudioParam;
}

interface WebAudioPannerNode {
  connect(destination: WebAudioGainNode): void;
  setPosition(x: number, y: number, z: number): void;
}

interface WebAudioDestinationNode {
  // Destination node
}

interface WebAudioParam {
  value: number;
  setValueAtTime(value: number, time: number): void;
  linearRampToValueAtTime(value: number, time: number): void;
  exponentialRampToValueAtTime(value: number, time: number): void;
}

export class AudioEngine {
  private audioContext: WebAudioContext | null = null;
  private masterGain: WebAudioGainNode | null = null;
  private isInitialized = false;

  /**
   * Initialize audio context (must be called after user interaction)
   */
  async initialize(): Promise<void> {
    if (this.isInitialized) {
      return;
    }

    try {
      // Use web-audio-api for Node.js environment
      // eslint-disable-next-line @typescript-eslint/no-explicit-any, @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-unsafe-member-access
      const { AudioContext } = await import('web-audio-api') as any;
      // eslint-disable-next-line @typescript-eslint/no-unsafe-call
      this.audioContext = new AudioContext() as WebAudioContext;
      if (!this.audioContext) {
        throw new Error('Failed to create audio context');
      }
      this.masterGain = this.audioContext.createGain();
      this.masterGain.connect(this.audioContext.destination);
      this.masterGain.gain.value = 0.3; // Master volume
      this.isInitialized = true;
    } catch (error) {
      console.error('Failed to initialize audio context:', error);
      throw new Error('Audio initialization failed');
    }
  }

  /**
   * Play a sonar pulse sound
   */
  playSonarPulse(intensity: number = 1.0): void {
    if (!this.audioContext || !this.masterGain) {
      console.warn('Audio context not initialized');
      return;
    }

    const now = this.audioContext.currentTime;

    // Create oscillator for the pulse
    const oscillator = this.audioContext.createOscillator();
    const gainNode = this.audioContext.createGain();

    oscillator.connect(gainNode);
    gainNode.connect(this.masterGain);

    // Sonar pulse: frequency sweep from high to low
    oscillator.type = 'sine';
    oscillator.frequency.setValueAtTime(1200, now);
    oscillator.frequency.exponentialRampToValueAtTime(300, now + 0.15);

    // Envelope: quick attack, moderate decay
    gainNode.gain.setValueAtTime(0, now);
    gainNode.gain.linearRampToValueAtTime(0.4 * intensity, now + 0.01);
    gainNode.gain.exponentialRampToValueAtTime(0.01, now + 0.15);

    oscillator.start(now);
    oscillator.stop(now + 0.15);
  }

  /**
   * Play echo sounds based on detected objects
   */
  playEchoes(echoes: Echo[]): void {
    if (!this.audioContext || !this.masterGain) {
      console.warn('Audio context not initialized');
      return;
    }

    for (const echo of echoes) {
      this.playEcho(echo);
    }
  }

  /**
   * Play a single echo
   */
  private playEcho(echo: Echo): void {
    if (!this.audioContext || !this.masterGain) {
      return;
    }

    const now = this.audioContext.currentTime;

    // Calculate delay based on distance (speed of sound approximation)
    // Assuming 1 unit = 10 meters, speed of sound = 343 m/s
    const delay = (echo.distance * 10) / 343;

    // Calculate volume based on distance (inverse square law)
    const baseVolume = 0.3 / Math.max(1, echo.distance * echo.distance);

    // Different frequencies for different object types
    let frequency = 400;
    switch (echo.type) {
      case 'obstacle':
        frequency = 400;
        break;
      case 'objective':
        frequency = 600; // Higher pitch for objectives
        break;
      case 'wall':
        frequency = 300; // Lower pitch for walls
        break;
    }

    // Create oscillator for the echo
    const oscillator = this.audioContext.createOscillator();
    const gainNode = this.audioContext.createGain();
    const panNode = this.audioContext.createStereoPanner?.() || this.audioContext.createPanner();

    oscillator.connect(gainNode);
    gainNode.connect(panNode);
    panNode.connect(this.masterGain);

    // Set panning based on angle
    if ('pan' in panNode) {
      // StereoPannerNode
      (panNode as WebAudioStereoPannerNode).pan.value = Math.sin(echo.angle);
    } else {
      // PannerNode fallback
      const x = Math.cos(echo.angle);
      const z = Math.sin(echo.angle);
      (panNode as WebAudioPannerNode).setPosition(x, 0, z);
    }

    // Echo characteristics
    oscillator.type = 'sine';
    oscillator.frequency.setValueAtTime(frequency, now + delay);

    // Echo envelope: softer and shorter than pulse
    gainNode.gain.setValueAtTime(0, now + delay);
    gainNode.gain.linearRampToValueAtTime(baseVolume, now + delay + 0.005);
    gainNode.gain.exponentialRampToValueAtTime(0.01, now + delay + 0.1);

    oscillator.start(now + delay);
    oscillator.stop(now + delay + 0.1);
  }

  /**
   * Play a positive feedback sound (good rhythm)
   */
  playPositiveFeedback(): void {
    if (!this.audioContext || !this.masterGain) {
      return;
    }

    const now = this.audioContext.currentTime;

    // Ascending arpeggio
    const frequencies = [523.25, 659.25, 783.99]; // C5, E5, G5
    frequencies.forEach((freq, index) => {
      const oscillator = this.audioContext!.createOscillator();
      const gainNode = this.audioContext!.createGain();

      oscillator.connect(gainNode);
      gainNode.connect(this.masterGain!);

      oscillator.type = 'sine';
      oscillator.frequency.value = freq;

      const startTime = now + index * 0.1;
      gainNode.gain.setValueAtTime(0, startTime);
      gainNode.gain.linearRampToValueAtTime(0.15, startTime + 0.01);
      gainNode.gain.exponentialRampToValueAtTime(0.01, startTime + 0.15);

      oscillator.start(startTime);
      oscillator.stop(startTime + 0.15);
    });
  }

  /**
   * Play a negative feedback sound (poor rhythm)
   */
  playNegativeFeedback(): void {
    if (!this.audioContext || !this.masterGain) {
      return;
    }

    const now = this.audioContext.currentTime;

    // Descending tone
    const oscillator = this.audioContext.createOscillator();
    const gainNode = this.audioContext.createGain();

    oscillator.connect(gainNode);
    gainNode.connect(this.masterGain);

    oscillator.type = 'triangle';
    oscillator.frequency.setValueAtTime(400, now);
    oscillator.frequency.exponentialRampToValueAtTime(200, now + 0.2);

    gainNode.gain.setValueAtTime(0, now);
    gainNode.gain.linearRampToValueAtTime(0.2, now + 0.01);
    gainNode.gain.exponentialRampToValueAtTime(0.01, now + 0.2);

    oscillator.start(now);
    oscillator.stop(now + 0.2);
  }

  /**
   * Play objective reached sound
   */
  playObjectiveReached(): void {
    if (!this.audioContext || !this.masterGain) {
      return;
    }

    const now = this.audioContext.currentTime;

    // Victory chime
    const frequencies = [523.25, 659.25, 783.99, 1046.5]; // C5, E5, G5, C6
    frequencies.forEach((freq, index) => {
      const oscillator = this.audioContext!.createOscillator();
      const gainNode = this.audioContext!.createGain();

      oscillator.connect(gainNode);
      gainNode.connect(this.masterGain!);

      oscillator.type = 'sine';
      oscillator.frequency.value = freq;

      const startTime = now + index * 0.08;
      gainNode.gain.setValueAtTime(0, startTime);
      gainNode.gain.linearRampToValueAtTime(0.2, startTime + 0.01);
      gainNode.gain.exponentialRampToValueAtTime(0.01, startTime + 0.3);

      oscillator.start(startTime);
      oscillator.stop(startTime + 0.3);
    });
  }

  /**
   * Play collision warning sound
   */
  playCollisionWarning(): void {
    if (!this.audioContext || !this.masterGain) {
      return;
    }

    const now = this.audioContext.currentTime;

    // Harsh warning tone
    const oscillator = this.audioContext.createOscillator();
    const gainNode = this.audioContext.createGain();

    oscillator.connect(gainNode);
    gainNode.connect(this.masterGain);

    oscillator.type = 'square';
    oscillator.frequency.setValueAtTime(200, now);

    gainNode.gain.setValueAtTime(0, now);
    gainNode.gain.linearRampToValueAtTime(0.3, now + 0.01);
    gainNode.gain.exponentialRampToValueAtTime(0.01, now + 0.15);

    oscillator.start(now);
    oscillator.stop(now + 0.15);
  }

  /**
   * Cleanup audio resources
   */
  async cleanup(): Promise<void> {
    if (this.audioContext) {
      await this.audioContext.close();
      this.audioContext = null;
      this.masterGain = null;
      this.isInitialized = false;
    }
  }

  /**
   * Check if audio engine is initialized
   */
  getIsInitialized(): boolean {
    return this.isInitialized;
  }
}
