/**
 * Battle system for boss encounters and combat
 */

import { Character } from './Character';
import { Card } from './Card';

export interface Enemy {
  name: string;
  health: number;
  maxHealth: number;
  attack: number;
  defense: number;
  requiredMatchedCards: number;
  difficulty: number;
  reward: { experience: number; gold: number };
}

export interface BattleAction {
  actor: 'player' | 'enemy';
  action: 'attack' | 'heal' | 'special';
  damage?: number;
  healing?: number;
  success: boolean;
}

export class Battle {
  private character: Character;
  private enemy: Enemy;
  private battleLog: BattleAction[] = [];
  private isActive: boolean = false;
  private matchedCardsCount: number = 0;

  constructor(character: Character, enemy: Enemy) {
    this.character = character;
    this.enemy = enemy;
  }

  /**
   * Start a new battle
   */
  startBattle(): void {
    this.isActive = true;
    this.battleLog = [];
    this.matchedCardsCount = 0;
  }

  /**
   * End the battle
   */
  endBattle(): void {
    this.isActive = false;
  }

  /**
   * Check if battle is active
   */
  isBattleActive(): boolean {
    return this.isActive;
  }

  /**
   * Register matched cards (increases damage/healing multiplier)
   */
  registerMatchedCards(count: number): void {
    this.matchedCardsCount += count;
  }

  /**
   * Get current matched cards for battle
   */
  getMatchedCardsCount(): number {
    return this.matchedCardsCount;
  }

  /**
   * Calculate damage based on matched cards
   * Each pair increases damage by 10%
   */
  calculateDamage(): number {
    const baseDamage = this.character.getAttack();
    const matchBonus = this.matchedCardsCount * 0.1;
    return Math.floor(baseDamage * (1 + matchBonus));
  }

  /**
   * Calculate healing based on matched cards
   */
  calculateHealing(): number {
    const baseHealing = Math.floor(this.character.getMaxHealth() * 0.15);
    const matchBonus = this.matchedCardsCount * 0.1;
    return Math.floor(baseHealing * (1 + matchBonus));
  }

  /**
   * Player attacks enemy
   */
  playerAttack(): BattleAction {
    if (!this.isActive) {
      throw new Error('Battle is not active');
    }

    const damage = this.calculateDamage();
    const actualDamage = Math.max(1, damage - Math.floor(this.enemy.defense / 2));

    this.enemy.health = Math.max(0, this.enemy.health - actualDamage);

    const action: BattleAction = {
      actor: 'player',
      action: 'attack',
      damage: actualDamage,
      success: true,
    };

    this.battleLog.push(action);
    this.matchedCardsCount = 0; // Reset matched cards after action

    return action;
  }

  /**
   * Player heals
   */
  playerHeal(): BattleAction {
    if (!this.isActive) {
      throw new Error('Battle is not active');
    }

    const healing = this.calculateHealing();
    this.character.heal(healing);

    const action: BattleAction = {
      actor: 'player',
      action: 'heal',
      healing: healing,
      success: true,
    };

    this.battleLog.push(action);
    this.matchedCardsCount = 0; // Reset matched cards after action

    return action;
  }

  /**
   * Enemy attacks player
   */
  enemyAttack(): BattleAction {
    if (!this.isActive) {
      throw new Error('Battle is not active');
    }

    const baseDamage = this.enemy.attack;
    const variance = Math.floor(baseDamage * 0.2);
    const damage = baseDamage + Math.floor((Math.random() - 0.5) * variance);

    this.character.takeDamage(Math.max(1, damage));

    const action: BattleAction = {
      actor: 'enemy',
      action: 'attack',
      damage: Math.max(1, damage),
      success: true,
    };

    this.battleLog.push(action);

    return action;
  }

  /**
   * Check if player has won
   */
  hasPlayerWon(): boolean {
    return this.enemy.health <= 0 && this.character.isAlive();
  }

  /**
   * Check if player has lost
   */
  hasPlayerLost(): boolean {
    return !this.character.isAlive();
  }

  /**
   * Check if battle is over
   */
  isBattleOver(): boolean {
    return this.hasPlayerWon() || this.hasPlayerLost();
  }

  /**
   * Get battle status
   */
  getBattleStatus(): {
    playerHealth: number;
    playerMaxHealth: number;
    enemyHealth: number;
    enemyMaxHealth: number;
    matchedCards: number;
    requiredMatches: number;
  } {
    return {
      playerHealth: this.character.getHealth(),
      playerMaxHealth: this.character.getMaxHealth(),
      enemyHealth: Math.max(0, this.enemy.health),
      enemyMaxHealth: this.enemy.maxHealth,
      matchedCards: this.matchedCardsCount,
      requiredMatches: this.enemy.requiredMatchedCards,
    };
  }

  /**
   * Get battle rewards if won
   */
  getBattleRewards(): { experience: number; gold: number } | null {
    if (this.hasPlayerWon()) {
      return this.enemy.reward;
    }
    return null;
  }

  /**
   * Get battle log
   */
  getBattleLog(): BattleAction[] {
    return [...this.battleLog];
  }

  /**
   * Get enemy info
   */
  getEnemy(): Enemy {
    return { ...this.enemy };
  }

  /**
   * Get character info
   */
  getCharacter(): Character {
    return this.character;
  }
}
