/**
 * Tests for TimingEngine
 */

import { TimingEngine, HitAccuracy, TimingWindow } from './TimingEngine';

describe('TimingEngine', () => {
  let engine: TimingEngine;

  beforeEach(() => {
    engine = new TimingEngine();
  });

  describe('getHighPrecisionTime', () => {
    it('should return a timestamp', () => {
      const time = engine.getHighPrecisionTime();
      expect(typeof time).toBe('number');
      expect(time).toBeGreaterThan(0);
    });

    it('should return increasing timestamps', () => {
      const time1 = engine.getHighPrecisionTime();
      const time2 = engine.getHighPrecisionTime();
      expect(time2).toBeGreaterThanOrEqual(time1);
    });
  });

  describe('registerBeat', () => {
    it('should register first beat as PERFECT', () => {
      const beat = engine.registerBeat();
      expect(beat.accuracy).toBe(HitAccuracy.PERFECT);
      expect(beat.deviation).toBe(0);
    });

    it('should register second beat as PERFECT', () => {
      engine.registerBeat();
      const beat = engine.registerBeat();
      expect(beat.accuracy).toBe(HitAccuracy.PERFECT);
    });

    it('should establish tempo after two beats', () => {
      engine.registerBeat();

      // Wait 500ms (120 BPM)
      const waitTime = 500;
      const startTime = performance.now();
      while (performance.now() - startTime < waitTime) {
        // Wait
      }

      engine.registerBeat();
      const tempo = engine.getCurrentTempo();

      expect(tempo).not.toBeNull();
      expect(tempo).toBeGreaterThan(0);
    });

    it('should detect PERFECT timing within window', () => {
      const window: TimingWindow = { perfect: 50, good: 100, ok: 150 };
      engine = new TimingEngine(window);

      engine.registerBeat();

      // Wait exactly 500ms (120 BPM)
      const waitTime = 500;
      let startTime = performance.now();
      while (performance.now() - startTime < waitTime) {
        // Wait
      }

      engine.registerBeat(); // Establish tempo

      // Wait 500ms again (within perfect window)
      startTime = performance.now();
      while (performance.now() - startTime < waitTime) {
        // Wait
      }

      const beat = engine.registerBeat();
      expect([HitAccuracy.PERFECT, HitAccuracy.GOOD]).toContain(beat.accuracy);
    });

    it('should calculate deviation correctly', () => {
      engine.registerBeat();

      const waitTime = 500;
      let startTime = performance.now();
      while (performance.now() - startTime < waitTime) {
        // Wait
      }

      engine.registerBeat(); // Establish tempo

      // Wait longer than expected
      startTime = performance.now();
      while (performance.now() - startTime < waitTime + 200) {
        // Wait
      }

      const beat = engine.registerBeat();
      expect(Math.abs(beat.deviation)).toBeGreaterThan(0);
    });
  });

  describe('getCurrentTempo', () => {
    it('should return null before tempo is established', () => {
      expect(engine.getCurrentTempo()).toBeNull();
    });

    it('should return tempo after two beats', () => {
      engine.registerBeat();

      const waitTime = 500; // 120 BPM
      const startTime = performance.now();
      while (performance.now() - startTime < waitTime) {
        // Wait
      }

      engine.registerBeat();

      const tempo = engine.getCurrentTempo();
      expect(tempo).not.toBeNull();
      expect(tempo).toBeCloseTo(120, 0); // Within 1 BPM
    });
  });

  describe('getConsistencyScore', () => {
    it('should return 1.0 with less than 3 beats', () => {
      expect(engine.getConsistencyScore()).toBe(1.0);

      engine.registerBeat();
      expect(engine.getConsistencyScore()).toBe(1.0);

      engine.registerBeat();
      expect(engine.getConsistencyScore()).toBe(1.0);
    });

    it('should calculate consistency based on accuracy', () => {
      // Register beats with consistent timing
      for (let i = 0; i < 5; i++) {
        engine.registerBeat();

        const waitTime = 500;
        const startTime = performance.now();
        while (performance.now() - startTime < waitTime) {
          // Wait
        }
      }

      const consistency = engine.getConsistencyScore();
      expect(consistency).toBeGreaterThan(0);
      expect(consistency).toBeLessThanOrEqual(1);
    });
  });

  describe('getAverageDeviation', () => {
    it('should return 0 with less than 2 beats', () => {
      expect(engine.getAverageDeviation()).toBe(0);

      engine.registerBeat();
      expect(engine.getAverageDeviation()).toBe(0);
    });

    it('should calculate average deviation', () => {
      engine.registerBeat();

      // Register several beats
      for (let i = 0; i < 5; i++) {
        const waitTime = 500;
        const startTime = performance.now();
        while (performance.now() - startTime < waitTime) {
          // Wait
        }
        engine.registerBeat();
      }

      const avgDev = engine.getAverageDeviation();
      expect(avgDev).toBeGreaterThanOrEqual(0);
    });
  });

  describe('getStatistics', () => {
    it('should return correct statistics', () => {
      engine.registerBeat();

      const waitTime = 500;
      const startTime = performance.now();
      while (performance.now() - startTime < waitTime) {
        // Wait
      }

      engine.registerBeat();

      const stats = engine.getStatistics();

      expect(stats.totalBeats).toBe(2);
      expect(stats.perfectBeats).toBeGreaterThan(0);
      expect(stats.consistency).toBeGreaterThanOrEqual(0);
      expect(stats.consistency).toBeLessThanOrEqual(1);
    });

    it('should count beat accuracies correctly', () => {
      const window: TimingWindow = { perfect: 1, good: 5, ok: 10 };
      engine = new TimingEngine(window);

      // First beat
      engine.registerBeat();

      // Second beat at consistent interval
      let waitTime = 500;
      let startTime = performance.now();
      while (performance.now() - startTime < waitTime) {
        // Wait
      }
      engine.registerBeat();

      // Third beat - slightly late (should be GOOD or OK)
      startTime = performance.now();
      while (performance.now() - startTime < waitTime + 50) {
        // Wait
      }
      engine.registerBeat();

      const stats = engine.getStatistics();
      expect(stats.totalBeats).toBe(3);
      expect(stats.perfectBeats + stats.goodBeats + stats.okBeats + stats.missedBeats).toBe(3);
    });
  });

  describe('reset', () => {
    it('should reset all state', () => {
      engine.registerBeat();

      const waitTime = 500;
      const startTime = performance.now();
      while (performance.now() - startTime < waitTime) {
        // Wait
      }

      engine.registerBeat();

      expect(engine.getCurrentTempo()).not.toBeNull();
      expect(engine.getStatistics().totalBeats).toBe(2);

      engine.reset();

      expect(engine.getCurrentTempo()).toBeNull();
      expect(engine.getStatistics().totalBeats).toBe(0);
      expect(engine.getBeatHistory()).toHaveLength(0);
    });
  });

  describe('timing window accuracy', () => {
    it('should respect custom timing windows', () => {
      const strictWindow: TimingWindow = { perfect: 10, good: 30, ok: 50 };
      engine = new TimingEngine(strictWindow);

      engine.registerBeat();

      const waitTime = 500;
      let startTime = performance.now();
      while (performance.now() - startTime < waitTime) {
        // Wait
      }

      engine.registerBeat(); // Establish tempo

      // Wait with larger deviation
      startTime = performance.now();
      while (performance.now() - startTime < waitTime + 100) {
        // Wait
      }

      const beat = engine.registerBeat();
      // Should be MISS with strict window and 100ms deviation
      expect([HitAccuracy.OK, HitAccuracy.MISS]).toContain(beat.accuracy);
    });
  });

  describe('tempo tracking', () => {
    it('should adapt to changing tempo', () => {
      engine.registerBeat();

      // Establish initial tempo at 120 BPM (500ms)
      let waitTime = 500;
      let startTime = performance.now();
      while (performance.now() - startTime < waitTime) {
        // Wait
      }
      engine.registerBeat();

      const tempo1 = engine.getCurrentTempo();

      // Continue at faster tempo (150 BPM = 400ms)
      for (let i = 0; i < 5; i++) {
        waitTime = 400;
        startTime = performance.now();
        while (performance.now() - startTime < waitTime) {
          // Wait
        }
        engine.registerBeat();
      }

      const tempo2 = engine.getCurrentTempo();

      expect(tempo1).not.toBeNull();
      expect(tempo2).not.toBeNull();
      expect(tempo2).not.toBe(tempo1);
    });
  });
});
