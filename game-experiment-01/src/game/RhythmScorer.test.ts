/**
 * Tests for RhythmScorer
 */

import { RhythmScorer } from './RhythmScorer';
import { HitAccuracy, BeatEvent } from '../timing/TimingEngine';

describe('RhythmScorer', () => {
  let scorer: RhythmScorer;

  beforeEach(() => {
    scorer = new RhythmScorer();
  });

  describe('processBeat', () => {
    it('should gain energy on PERFECT beat', () => {
      // Deplete some energy first so we're not at max
      const missBeat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.MISS,
        deviation: 200,
      };
      scorer.processBeat(missBeat);

      const beat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.PERFECT,
        deviation: 0,
      };

      const initialEnergy = scorer.getEnergyState().current;
      const result = scorer.processBeat(beat);

      expect(result.energyChange).toBe(10);
      expect(scorer.getEnergyState().current).toBe(initialEnergy + 10);
    });

    it('should gain less energy on GOOD beat', () => {
      // Deplete some energy first so we're not at max
      const missBeat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.MISS,
        deviation: 200,
      };
      scorer.processBeat(missBeat);

      const beat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.GOOD,
        deviation: 50,
      };

      const initialEnergy = scorer.getEnergyState().current;
      const result = scorer.processBeat(beat);

      expect(result.energyChange).toBe(5);
      expect(scorer.getEnergyState().current).toBe(initialEnergy + 5);
    });

    it('should not change energy on OK beat', () => {
      const beat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.OK,
        deviation: 100,
      };

      const initialEnergy = scorer.getEnergyState().current;
      const result = scorer.processBeat(beat);

      expect(result.energyChange).toBe(0);
      expect(scorer.getEnergyState().current).toBe(initialEnergy);
    });

    it('should lose energy on MISS', () => {
      const beat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.MISS,
        deviation: 200,
      };

      const initialEnergy = scorer.getEnergyState().current;
      const result = scorer.processBeat(beat);

      expect(result.energyChange).toBe(-20);
      expect(scorer.getEnergyState().current).toBe(initialEnergy - 20);
    });

    it('should not exceed maximum energy', () => {
      const beat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.PERFECT,
        deviation: 0,
      };

      // Process many perfect beats
      for (let i = 0; i < 20; i++) {
        scorer.processBeat(beat);
      }

      expect(scorer.getEnergyState().current).toBeLessThanOrEqual(scorer.getEnergyState().maximum);
    });

    it('should not go below zero energy', () => {
      const beat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.MISS,
        deviation: 200,
      };

      // Process many misses
      for (let i = 0; i < 20; i++) {
        scorer.processBeat(beat);
      }

      expect(scorer.getEnergyState().current).toBeGreaterThanOrEqual(0);
    });

    it('should increase score on any hit', () => {
      const beat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.PERFECT,
        deviation: 0,
      };

      const initialScore = scorer.getScore();
      scorer.processBeat(beat);

      expect(scorer.getScore()).toBeGreaterThan(initialScore);
    });

    it('should not increase score on MISS', () => {
      const beat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.MISS,
        deviation: 200,
      };

      const initialScore = scorer.getScore();
      scorer.processBeat(beat);

      expect(scorer.getScore()).toBe(initialScore);
    });
  });

  describe('combo system', () => {
    it('should increase combo on successful hits', () => {
      const beat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.PERFECT,
        deviation: 0,
      };

      expect(scorer.getCombo()).toBe(0);

      scorer.processBeat(beat);
      expect(scorer.getCombo()).toBe(1);

      scorer.processBeat(beat);
      expect(scorer.getCombo()).toBe(2);
    });

    it('should not increase combo on OK hit', () => {
      const perfectBeat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.PERFECT,
        deviation: 0,
      };

      const okBeat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.OK,
        deviation: 120,
      };

      scorer.processBeat(perfectBeat);
      expect(scorer.getCombo()).toBe(1);

      scorer.processBeat(okBeat);
      expect(scorer.getCombo()).toBe(1); // Combo maintained but not increased
    });

    it('should reset combo on MISS', () => {
      const perfectBeat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.PERFECT,
        deviation: 0,
      };

      const missBeat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.MISS,
        deviation: 200,
      };

      scorer.processBeat(perfectBeat);
      scorer.processBeat(perfectBeat);
      expect(scorer.getCombo()).toBe(2);

      scorer.processBeat(missBeat);
      expect(scorer.getCombo()).toBe(0);
    });

    it('should track max combo', () => {
      const perfectBeat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.PERFECT,
        deviation: 0,
      };

      const missBeat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.MISS,
        deviation: 200,
      };

      // Build combo
      for (let i = 0; i < 5; i++) {
        scorer.processBeat(perfectBeat);
      }

      expect(scorer.getMaxCombo()).toBe(5);

      // Break combo
      scorer.processBeat(missBeat);
      expect(scorer.getCombo()).toBe(0);

      // Max combo should remain
      expect(scorer.getMaxCombo()).toBe(5);

      // Build smaller combo
      for (let i = 0; i < 3; i++) {
        scorer.processBeat(perfectBeat);
      }

      expect(scorer.getMaxCombo()).toBe(5); // Still the old max
    });

    it('should apply combo multiplier to score', () => {
      const beat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.PERFECT,
        deviation: 0,
      };

      scorer.processBeat(beat);
      const scoreAfterFirst = scorer.getScore();

      scorer.processBeat(beat);
      const scoreAfterSecond = scorer.getScore();

      const firstBeatScore = scoreAfterFirst;
      const secondBeatScore = scoreAfterSecond - scoreAfterFirst;

      // Second beat should score more due to combo multiplier
      expect(secondBeatScore).toBeGreaterThan(firstBeatScore);
    });
  });

  describe('sonar range', () => {
    it('should calculate sonar range based on energy', () => {
      const range = scorer.getSonarRange();
      expect(range).toBeGreaterThan(0);
      expect(range).toBeLessThanOrEqual(30);
    });

    it('should decrease range with lower energy', () => {
      const initialRange = scorer.getSonarRange();

      // Deplete energy
      const missBeat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.MISS,
        deviation: 200,
      };

      for (let i = 0; i < 5; i++) {
        scorer.processBeat(missBeat);
      }

      const depletedRange = scorer.getSonarRange();
      expect(depletedRange).toBeLessThan(initialRange);
    });

    it('should increase range with higher energy', () => {
      // Start with depleted energy
      const missBeat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.MISS,
        deviation: 200,
      };

      for (let i = 0; i < 5; i++) {
        scorer.processBeat(missBeat);
      }

      const depletedRange = scorer.getSonarRange();

      // Recover energy
      const perfectBeat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.PERFECT,
        deviation: 0,
      };

      for (let i = 0; i < 5; i++) {
        scorer.processBeat(perfectBeat);
      }

      const recoveredRange = scorer.getSonarRange();
      expect(recoveredRange).toBeGreaterThan(depletedRange);
    });
  });

  describe('energy management', () => {
    it('should recharge energy over time', () => {
      const initialEnergy = scorer.getEnergyState().current;

      // Deplete some energy
      const missBeat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.MISS,
        deviation: 200,
      };
      scorer.processBeat(missBeat);

      const depletedEnergy = scorer.getEnergyState().current;
      expect(depletedEnergy).toBeLessThan(initialEnergy);

      // Recharge over 1 second
      const recharge = scorer.rechargeEnergy(1.0);
      expect(recharge).toBeGreaterThan(0);
      expect(scorer.getEnergyState().current).toBeGreaterThan(depletedEnergy);
    });

    it('should consume energy for movement', () => {
      const initialEnergy = scorer.getEnergyState().current;
      const consumed = scorer.consumeEnergyForMovement(10);

      expect(consumed).toBe(true);
      expect(scorer.getEnergyState().current).toBe(initialEnergy - 10);
    });

    it('should fail to consume energy when insufficient', () => {
      // Deplete energy
      const missBeat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.MISS,
        deviation: 200,
      };

      for (let i = 0; i < 10; i++) {
        scorer.processBeat(missBeat);
      }

      const consumed = scorer.consumeEnergyForMovement(1000);
      expect(consumed).toBe(false);
    });

    it('should check if player has enough energy', () => {
      expect(scorer.hasEnergy(10)).toBe(true);
      expect(scorer.hasEnergy(1000)).toBe(false);
    });
  });

  describe('grading system', () => {
    it('should return grade based on performance', () => {
      const grade = scorer.getGrade();
      expect(['S', 'A', 'B', 'C', 'D']).toContain(grade);
    });

    it('should give higher grade for better performance', () => {
      const perfectBeat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.PERFECT,
        deviation: 0,
      };

      // Build high score and combo
      for (let i = 0; i < 60; i++) {
        scorer.processBeat(perfectBeat);
      }

      const grade = scorer.getGrade();
      expect(['S', 'A']).toContain(grade);
    });
  });

  describe('reset', () => {
    it('should reset all state', () => {
      const beat: BeatEvent = {
        timestamp: 0,
        accuracy: HitAccuracy.PERFECT,
        deviation: 0,
      };

      // Build up state
      for (let i = 0; i < 5; i++) {
        scorer.processBeat(beat);
      }

      expect(scorer.getScore()).toBeGreaterThan(0);
      expect(scorer.getCombo()).toBeGreaterThan(0);

      // Reset
      scorer.reset();

      expect(scorer.getScore()).toBe(0);
      expect(scorer.getCombo()).toBe(0);
      expect(scorer.getMaxCombo()).toBe(0);
      expect(scorer.getEnergyState().current).toBe(scorer.getEnergyState().maximum);
    });
  });
});
