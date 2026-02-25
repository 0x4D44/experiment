/**
 * Character class representing a player character with class-specific abilities
 */

export enum CharacterClass {
  WARRIOR = 'WARRIOR',
  MAGE = 'MAGE',
  ROGUE = 'ROGUE',
}

export interface CharacterStats {
  health: number;
  maxHealth: number;
  mana: number;
  maxMana: number;
  attack: number;
  defense: number;
  speed: number;
}

export class Character {
  private name: string;
  private characterClass: CharacterClass;
  private level: number = 1;
  private experience: number = 0;
  private experienceForNextLevel: number = 100;
  private stats: CharacterStats;
  private inventory: Map<string, number> = new Map(); // cardId -> count

  constructor(name: string, characterClass: CharacterClass) {
    this.name = name;
    this.characterClass = characterClass;
    this.stats = this.initializeStats(characterClass);
  }

  /**
   * Initialize stats based on character class
   */
  private initializeStats(charClass: CharacterClass): CharacterStats {
    switch (charClass) {
      case CharacterClass.WARRIOR:
        return {
          health: 100,
          maxHealth: 100,
          mana: 30,
          maxMana: 30,
          attack: 15,
          defense: 12,
          speed: 8,
        };
      case CharacterClass.MAGE:
        return {
          health: 60,
          maxHealth: 60,
          mana: 80,
          maxMana: 80,
          attack: 10,
          defense: 6,
          speed: 12,
        };
      case CharacterClass.ROGUE:
        return {
          health: 70,
          maxHealth: 70,
          mana: 40,
          maxMana: 40,
          attack: 12,
          defense: 8,
          speed: 16,
        };
    }
  }

  /**
   * Get character info
   */
  getName(): string {
    return this.name;
  }

  getClass(): CharacterClass {
    return this.characterClass;
  }

  getLevel(): number {
    return this.level;
  }

  getExperience(): number {
    return this.experience;
  }

  getExperienceForNextLevel(): number {
    return this.experienceForNextLevel;
  }

  /**
   * Get all stats
   */
  getStats(): CharacterStats {
    return { ...this.stats };
  }

  /**
   * Get individual stat
   */
  getHealth(): number {
    return this.stats.health;
  }

  getMaxHealth(): number {
    return this.stats.maxHealth;
  }

  getMana(): number {
    return this.stats.mana;
  }

  getMaxMana(): number {
    return this.stats.maxMana;
  }

  getAttack(): number {
    return this.stats.attack;
  }

  getDefense(): number {
    return this.stats.defense;
  }

  getSpeed(): number {
    return this.stats.speed;
  }

  /**
   * Restore health
   */
  heal(amount: number): void {
    this.stats.health = Math.min(this.stats.health + amount, this.stats.maxHealth);
  }

  /**
   * Take damage
   */
  takeDamage(damage: number): void {
    const actualDamage = Math.max(1, damage - Math.floor(this.stats.defense / 2));
    this.stats.health = Math.max(0, this.stats.health - actualDamage);
  }

  /**
   * Restore mana
   */
  restoreMana(amount: number): void {
    this.stats.mana = Math.min(this.stats.mana + amount, this.stats.maxMana);
  }

  /**
   * Use mana for spell
   */
  useMana(amount: number): boolean {
    if (this.stats.mana >= amount) {
      this.stats.mana -= amount;
      return true;
    }
    return false;
  }

  /**
   * Check if character is alive
   */
  isAlive(): boolean {
    return this.stats.health > 0;
  }

  /**
   * Add experience and level up if needed
   */
  addExperience(amount: number): boolean {
    this.experience += amount;
    if (this.experience >= this.experienceForNextLevel) {
      this.levelUp();
      return true;
    }
    return false;
  }

  /**
   * Level up the character
   */
  private levelUp(): void {
    this.level += 1;
    this.experience = 0;
    this.experienceForNextLevel = Math.floor(this.experienceForNextLevel * 1.5);

    // Increase stats based on class
    switch (this.characterClass) {
      case CharacterClass.WARRIOR:
        this.stats.maxHealth += 20;
        this.stats.health = this.stats.maxHealth;
        this.stats.attack += 3;
        this.stats.defense += 2;
        break;
      case CharacterClass.MAGE:
        this.stats.maxHealth += 10;
        this.stats.health = this.stats.maxHealth;
        this.stats.maxMana += 20;
        this.stats.mana = this.stats.maxMana;
        this.stats.attack += 2;
        break;
      case CharacterClass.ROGUE:
        this.stats.maxHealth += 15;
        this.stats.health = this.stats.maxHealth;
        this.stats.attack += 4;
        this.stats.speed += 2;
        break;
    }
  }

  /**
   * Inventory management
   */
  addToInventory(cardId: string, quantity: number = 1): void {
    const current = this.inventory.get(cardId) || 0;
    this.inventory.set(cardId, current + quantity);
  }

  removeFromInventory(cardId: string, quantity: number = 1): boolean {
    const current = this.inventory.get(cardId) || 0;
    if (current >= quantity) {
      this.inventory.set(cardId, current - quantity);
      if (this.inventory.get(cardId) === 0) {
        this.inventory.delete(cardId);
      }
      return true;
    }
    return false;
  }

  getInventory(): Map<string, number> {
    return new Map(this.inventory);
  }

  getInventoryCount(): number {
    return this.inventory.size;
  }

  clearInventory(): void {
    this.inventory.clear();
  }

  /**
   * Reset character to initial state (for new game)
   */
  reset(): void {
    this.level = 1;
    this.experience = 0;
    this.experienceForNextLevel = 100;
    this.stats = this.initializeStats(this.characterClass);
    this.inventory.clear();
  }
}
