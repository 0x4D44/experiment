/**
 * Rhythm scorer that manages energy and bonuses based on timing accuracy
 */

import { HitAccuracy, BeatEvent } from '../timing/TimingEngine.js';

export interface EnergyState {
  current: number;
  maximum: number;
  rechargeRate: number;
}

export class RhythmScorer {
  private energy: number;
  private maxEnergy: number;
  private baseRechargeRate: number;
  private score: number = 0;
  private combo: number = 0;
  private maxCombo: number = 0;

  constructor(maxEnergy: number = 100, rechargeRate: number = 5) {
    this.energy = maxEnergy;
    this.maxEnergy = maxEnergy;
    this.baseRechargeRate = rechargeRate;
  }

  /**
   * Process a beat event and update energy/score
   */
  processBeat(beatEvent: BeatEvent): {
    energyChange: number;
    scoreChange: number;
    comboChange: number;
    sonarRange: number;
  } {
    let energyChange = 0;
    let scoreChange = 0;
    let comboChange = 0;

    switch (beatEvent.accuracy) {
      case HitAccuracy.PERFECT:
        energyChange = 10; // Gain energy
        scoreChange = 100 * (1 + this.combo * 0.1); // Combo multiplier
        comboChange = 1;
        break;

      case HitAccuracy.GOOD:
        energyChange = 5; // Small energy gain
        scoreChange = 75 * (1 + this.combo * 0.1);
        comboChange = 1;
        break;

      case HitAccuracy.OK:
        energyChange = 0; // Neutral
        scoreChange = 50;
        comboChange = 0; // Doesn't break combo but doesn't increase it
        break;

      case HitAccuracy.MISS:
        energyChange = -20; // Lose energy
        scoreChange = 0;
        comboChange = -this.combo; // Break combo
        break;
    }

    // Apply energy change
    this.energy = Math.max(0, Math.min(this.maxEnergy, this.energy + energyChange));

    // Apply score change
    this.score += Math.floor(scoreChange);

    // Apply combo change
    if (comboChange > 0) {
      this.combo += comboChange;
      this.maxCombo = Math.max(this.maxCombo, this.combo);
    } else if (comboChange < 0) {
      this.combo = 0;
    }

    // Calculate sonar range based on energy
    const sonarRange = this.calculateSonarRange();

    return {
      energyChange,
      scoreChange: Math.floor(scoreChange),
      comboChange,
      sonarRange,
    };
  }

  /**
   * Calculate sonar range based on current energy
   */
  private calculateSonarRange(): number {
    // Range scales with energy: 10-30 units
    const minRange = 10;
    const maxRange = 30;
    const energyRatio = this.energy / this.maxEnergy;
    return minRange + (maxRange - minRange) * energyRatio;
  }

  /**
   * Get current sonar range
   */
  getSonarRange(): number {
    return this.calculateSonarRange();
  }

  /**
   * Recharge energy over time (call this regularly)
   */
  rechargeEnergy(deltaTime: number): number {
    const recharge = this.baseRechargeRate * deltaTime;
    const oldEnergy = this.energy;
    this.energy = Math.min(this.maxEnergy, this.energy + recharge);
    return this.energy - oldEnergy;
  }

  /**
   * Consume energy for movement
   */
  consumeEnergyForMovement(amount: number): boolean {
    if (this.energy >= amount) {
      this.energy -= amount;
      return true;
    }
    return false;
  }

  /**
   * Get current energy state
   */
  getEnergyState(): EnergyState {
    return {
      current: this.energy,
      maximum: this.maxEnergy,
      rechargeRate: this.baseRechargeRate,
    };
  }

  /**
   * Get current score
   */
  getScore(): number {
    return this.score;
  }

  /**
   * Get current combo
   */
  getCombo(): number {
    return this.combo;
  }

  /**
   * Get max combo achieved
   */
  getMaxCombo(): number {
    return this.maxCombo;
  }

  /**
   * Get energy percentage (0-1)
   */
  getEnergyPercentage(): number {
    return this.energy / this.maxEnergy;
  }

  /**
   * Check if player has enough energy
   */
  hasEnergy(amount: number): boolean {
    return this.energy >= amount;
  }

  /**
   * Reset scorer state
   */
  reset(): void {
    this.energy = this.maxEnergy;
    this.score = 0;
    this.combo = 0;
    this.maxCombo = 0;
  }

  /**
   * Get performance grade based on score and combo
   */
  getGrade(): string {
    const avgCombo = this.maxCombo;

    if (avgCombo >= 50 && this.score >= 5000) {
      return 'S';
    } else if (avgCombo >= 30 && this.score >= 3000) {
      return 'A';
    } else if (avgCombo >= 20 && this.score >= 2000) {
      return 'B';
    } else if (avgCombo >= 10 && this.score >= 1000) {
      return 'C';
    } else {
      return 'D';
    }
  }
}
