/**
 * High-precision timing engine for rhythm detection and scoring
 */

export interface TimingWindow {
  perfect: number;  // ±ms for perfect hit
  good: number;     // ±ms for good hit
  ok: number;       // ±ms for ok hit
}

export enum HitAccuracy {
  PERFECT = 'PERFECT',
  GOOD = 'GOOD',
  OK = 'OK',
  MISS = 'MISS',
}

export interface BeatEvent {
  timestamp: number;
  accuracy: HitAccuracy;
  deviation: number; // ms from expected beat
}

export class TimingEngine {
  private startTime: number;
  private beatHistory: BeatEvent[] = [];
  private timingWindow: TimingWindow;
  private lastBeatTime: number | null = null;
  private expectedTempo: number | null = null; // BPM
  private tempoHistory: number[] = [];

  constructor(timingWindow: TimingWindow = { perfect: 50, good: 100, ok: 150 }) {
    this.startTime = this.getHighPrecisionTime();
    this.timingWindow = timingWindow;
  }

  /**
   * Get high-precision timestamp in milliseconds
   */
  getHighPrecisionTime(): number {
    // Use performance.now() for sub-millisecond precision
    return performance.now();
  }

  /**
   * Get elapsed time since engine started
   */
  getElapsedTime(): number {
    return this.getHighPrecisionTime() - this.startTime;
  }

  /**
   * Register a beat input and calculate accuracy
   */
  registerBeat(): BeatEvent {
    const currentTime = this.getHighPrecisionTime();

    if (this.lastBeatTime === null) {
      // First beat - always perfect
      this.lastBeatTime = currentTime;
      const event: BeatEvent = {
        timestamp: currentTime,
        accuracy: HitAccuracy.PERFECT,
        deviation: 0,
      };
      this.beatHistory.push(event);
      return event;
    }

    const timeSinceLastBeat = currentTime - this.lastBeatTime;

    // Calculate expected beat time based on established tempo
    let deviation = 0;
    let accuracy = HitAccuracy.PERFECT;

    if (this.expectedTempo !== null) {
      const expectedInterval = (60000 / this.expectedTempo); // ms per beat
      const expectedBeatTime = this.lastBeatTime + expectedInterval;
      deviation = currentTime - expectedBeatTime;

      // Determine accuracy based on deviation
      const absDeviation = Math.abs(deviation);
      if (absDeviation <= this.timingWindow.perfect) {
        accuracy = HitAccuracy.PERFECT;
      } else if (absDeviation <= this.timingWindow.good) {
        accuracy = HitAccuracy.GOOD;
      } else if (absDeviation <= this.timingWindow.ok) {
        accuracy = HitAccuracy.OK;
      } else {
        accuracy = HitAccuracy.MISS;
      }
    } else {
      // Second beat - establish tempo
      const bpm = 60000 / timeSinceLastBeat;
      this.tempoHistory.push(bpm);
      this.expectedTempo = bpm;
      accuracy = HitAccuracy.PERFECT; // Second beat is always perfect
    }

    // Update tempo estimation with moving average
    if (accuracy !== HitAccuracy.MISS && this.tempoHistory.length > 0) {
      const bpm = 60000 / timeSinceLastBeat;
      this.tempoHistory.push(bpm);

      // Keep only last 8 beats for tempo calculation
      if (this.tempoHistory.length > 8) {
        this.tempoHistory.shift();
      }

      // Update expected tempo with weighted average
      this.expectedTempo = this.calculateWeightedTempo();
    }

    this.lastBeatTime = currentTime;

    const event: BeatEvent = {
      timestamp: currentTime,
      accuracy,
      deviation,
    };

    this.beatHistory.push(event);
    return event;
  }

  /**
   * Calculate weighted tempo average (more recent beats have higher weight)
   */
  private calculateWeightedTempo(): number {
    if (this.tempoHistory.length === 0) {
      return 120; // Default
    }

    let weightedSum = 0;
    let totalWeight = 0;

    for (let i = 0; i < this.tempoHistory.length; i++) {
      const weight = i + 1; // More recent = higher weight
      weightedSum += this.tempoHistory[i] * weight;
      totalWeight += weight;
    }

    return weightedSum / totalWeight;
  }

  /**
   * Get current tempo in BPM
   */
  getCurrentTempo(): number | null {
    return this.expectedTempo;
  }

  /**
   * Get rhythm consistency score (0-1)
   */
  getConsistencyScore(): number {
    if (this.beatHistory.length < 3) {
      return 1.0; // Not enough data
    }

    // Calculate based on recent beats (last 16)
    const recentBeats = this.beatHistory.slice(-16);
    let totalScore = 0;

    for (const beat of recentBeats) {
      switch (beat.accuracy) {
        case HitAccuracy.PERFECT:
          totalScore += 1.0;
          break;
        case HitAccuracy.GOOD:
          totalScore += 0.75;
          break;
        case HitAccuracy.OK:
          totalScore += 0.5;
          break;
        case HitAccuracy.MISS:
          totalScore += 0.0;
          break;
      }
    }

    return totalScore / recentBeats.length;
  }

  /**
   * Get average deviation in milliseconds
   */
  getAverageDeviation(): number {
    if (this.beatHistory.length < 2) {
      return 0;
    }

    const recentBeats = this.beatHistory.slice(-16);
    const deviations = recentBeats.map(b => Math.abs(b.deviation));
    return deviations.reduce((a, b) => a + b, 0) / deviations.length;
  }

  /**
   * Get beat history
   */
  getBeatHistory(): BeatEvent[] {
    return [...this.beatHistory];
  }

  /**
   * Get timing statistics
   */
  getStatistics(): {
    totalBeats: number;
    perfectBeats: number;
    goodBeats: number;
    okBeats: number;
    missedBeats: number;
    consistency: number;
    averageDeviation: number;
    currentTempo: number | null;
  } {
    const stats = {
      totalBeats: this.beatHistory.length,
      perfectBeats: 0,
      goodBeats: 0,
      okBeats: 0,
      missedBeats: 0,
      consistency: this.getConsistencyScore(),
      averageDeviation: this.getAverageDeviation(),
      currentTempo: this.getCurrentTempo(),
    };

    for (const beat of this.beatHistory) {
      switch (beat.accuracy) {
        case HitAccuracy.PERFECT:
          stats.perfectBeats++;
          break;
        case HitAccuracy.GOOD:
          stats.goodBeats++;
          break;
        case HitAccuracy.OK:
          stats.okBeats++;
          break;
        case HitAccuracy.MISS:
          stats.missedBeats++;
          break;
      }
    }

    return stats;
  }

  /**
   * Reset the timing engine
   */
  reset(): void {
    this.startTime = this.getHighPrecisionTime();
    this.beatHistory = [];
    this.lastBeatTime = null;
    this.expectedTempo = null;
    this.tempoHistory = [];
  }
}
