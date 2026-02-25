/**
 * Word Warrior - Core Game Engine
 * A typing game with combat elements
 */

// ============================================================================
// WORD DATA STRUCTURE
// ============================================================================

interface WordData {
  text: string;
  difficulty: number; // 1-5 stars
  category: 'fire' | 'ice' | 'earth' | 'lightning' | 'heal';
  basePoints: number;
}

class Word {
  text: string;
  difficulty: number;
  category: 'fire' | 'ice' | 'earth' | 'lightning' | 'heal';
  basePoints: number;

  constructor(data: WordData) {
    if (!data.text || data.text.trim().length === 0) {
      throw new Error('Word text cannot be empty');
    }
    if (data.difficulty < 1 || data.difficulty > 5) {
      throw new Error('Difficulty must be between 1 and 5');
    }
    this.text = data.text.toLowerCase();
    this.difficulty = data.difficulty;
    this.category = data.category;
    this.basePoints = data.basePoints;
  }

  getDamage(speedBonus: number = 1): number {
    return Math.floor(this.basePoints * this.difficulty * speedBonus);
  }

  getSpeedBonus(timeMs: number): number {
    // Typing faster = higher bonus (max 2x at 100ms or less)
    if (timeMs <= 100) return 2.0;
    if (timeMs <= 200) return 1.8;
    if (timeMs <= 300) return 1.5;
    if (timeMs <= 500) return 1.2;
    if (timeMs <= 750) return 1.0;
    return 0.8; // Slow typing penalty
  }
}

// ============================================================================
// ENEMY DATA STRUCTURE
// ============================================================================

class Enemy {
  name: string;
  maxHealth: number;
  currentHealth: number;
  attack: number;
  level: number;
  experience: number;
  isBoss: boolean;

  constructor(
    name: string,
    health: number,
    attack: number,
    level: number = 1,
    isBoss: boolean = false
  ) {
    if (health <= 0) throw new Error('Health must be positive');
    if (attack < 0) throw new Error('Attack cannot be negative');
    this.name = name;
    this.maxHealth = health;
    this.currentHealth = health;
    this.attack = attack;
    this.level = level;
    this.experience = level * 10;
    this.isBoss = isBoss;
  }

  takeDamage(amount: number): void {
    if (amount < 0) throw new Error('Damage cannot be negative');
    this.currentHealth = Math.max(0, this.currentHealth - amount);
  }

  heal(amount: number): void {
    if (amount < 0) throw new Error('Healing cannot be negative');
    this.currentHealth = Math.min(this.maxHealth, this.currentHealth + amount);
  }

  isAlive(): boolean {
    return this.currentHealth > 0;
  }

  getHealthPercent(): number {
    return (this.currentHealth / this.maxHealth) * 100;
  }

  getAttackDamage(): number {
    // Boss attacks deal more damage
    const multiplier = this.isBoss ? 1.5 : 1.0;
    return Math.floor(this.attack * multiplier);
  }

  reset(): void {
    this.currentHealth = this.maxHealth;
  }
}

// ============================================================================
// PLAYER CHARACTER
// ============================================================================

class Player {
  name: string;
  maxHealth: number;
  currentHealth: number;
  maxMana: number;
  currentMana: number;
  level: number;
  totalExperience: number;
  nextLevelExperience: number;
  accuracy: number; // 0-100
  wordCount: number; // Total words typed
  abilities: Set<string>;
  manaRegenPerWord: number;

  constructor(name: string = 'Warrior', maxHealth: number = 100) {
    if (maxHealth <= 0) throw new Error('Health must be positive');
    this.name = name;
    this.maxHealth = maxHealth;
    this.currentHealth = maxHealth;
    this.maxMana = 50;
    this.currentMana = this.maxMana;
    this.level = 1;
    this.totalExperience = 0;
    this.nextLevelExperience = 100;
    this.accuracy = 100;
    this.wordCount = 0;
    this.abilities = new Set();
    this.manaRegenPerWord = 5;
  }

  takeDamage(amount: number): void {
    if (amount < 0) throw new Error('Damage cannot be negative');
    this.currentHealth = Math.max(0, this.currentHealth - amount);
  }

  heal(amount: number): void {
    if (amount < 0) throw new Error('Healing cannot be negative');
    this.currentHealth = Math.min(this.maxHealth, this.currentHealth + amount);
  }

  receiveMana(amount: number): void {
    if (amount < 0) throw new Error('Mana cannot be negative');
    this.currentMana = Math.min(this.maxMana, this.currentMana + amount);
  }

  spendMana(amount: number): boolean {
    if (amount < 0) throw new Error('Mana cannot be negative');
    if (this.currentMana >= amount) {
      this.currentMana -= amount;
      return true;
    }
    return false;
  }

  gainExperience(amount: number): void {
    if (amount < 0) throw new Error('Experience cannot be negative');
    this.totalExperience += amount;

    while (this.totalExperience >= this.nextLevelExperience) {
      this.levelUp();
    }
  }

  private levelUp(): void {
    this.level++;
    this.totalExperience -= this.nextLevelExperience;
    this.nextLevelExperience = Math.floor(this.nextLevelExperience * 1.5);
    this.maxHealth = Math.floor(this.maxHealth * 1.2);
    this.currentHealth = this.maxHealth;
    this.maxMana = Math.floor(this.maxMana * 1.2);
    this.currentMana = this.maxMana;
    this.abilities.add(`ability_level_${this.level}`);
  }

  isAlive(): boolean {
    return this.currentHealth > 0;
  }

  getHealthPercent(): number {
    return (this.currentHealth / this.maxHealth) * 100;
  }

  getManaPercent(): number {
    return (this.currentMana / this.maxMana) * 100;
  }

  recordWord(correct: boolean): void {
    this.wordCount++;
    this.receiveMana(this.manaRegenPerWord);
    if (!correct) {
      this.accuracy = Math.max(0, this.accuracy - 2);
    } else {
      this.accuracy = Math.min(100, this.accuracy + 1);
    }
  }
}

// ============================================================================
// COMBO SYSTEM
// ============================================================================

class ComboSystem {
  currentCombo: number;
  maxCombo: number;
  multiplier: number;

  constructor() {
    this.currentCombo = 0;
    this.maxCombo = 0;
    this.multiplier = 1.0;
  }

  addHit(): void {
    this.currentCombo++;
    if (this.currentCombo > this.maxCombo) {
      this.maxCombo = this.currentCombo;
    }
    this.updateMultiplier();
  }

  addMiss(): void {
    this.currentCombo = 0;
    this.multiplier = 1.0;
  }

  private updateMultiplier(): void {
    // Every 5 hits = +0.5x multiplier
    this.multiplier = 1.0 + Math.floor(this.currentCombo / 5) * 0.5;
  }

  getComboMultiplier(): number {
    return this.multiplier;
  }

  reset(): void {
    this.currentCombo = 0;
    this.multiplier = 1.0;
  }

  getComboBonus(): number {
    // Base bonus: 10 points per completed 5-hit combo (at 5 hits bonus is 10)
    return Math.floor(Math.max(0, (this.currentCombo - 1) / 5) * 10);
  }
}

// ============================================================================
// WORD DATABASE - 500+ WORDS WITH CATEGORIES
// ============================================================================

class WordDatabase {
  private words: Word[] = [];

  constructor() {
    this.initializeWords();
  }

  private initializeWords(): void {
    const allWordData: WordData[] = [];

    // Fire words - expanded list (110 words)
    const fireWords = [
      { text: 'ignite', difficulty: 2, basePoints: 15 },
      { text: 'blaze', difficulty: 2, basePoints: 14 },
      { text: 'inferno', difficulty: 3, basePoints: 25 },
      { text: 'combustion', difficulty: 4, basePoints: 35 },
      { text: 'flame', difficulty: 1, basePoints: 8 },
      { text: 'scorch', difficulty: 2, basePoints: 16 },
      { text: 'torch', difficulty: 2, basePoints: 12 },
      { text: 'burn', difficulty: 1, basePoints: 6 },
      { text: 'fiery', difficulty: 2, basePoints: 14 },
      { text: 'searing', difficulty: 3, basePoints: 22 },
      { text: 'pyrite', difficulty: 2, basePoints: 13 },
      { text: 'ember', difficulty: 2, basePoints: 11 },
      { text: 'smoldering', difficulty: 4, basePoints: 32 },
      { text: 'flammable', difficulty: 3, basePoints: 24 },
      { text: 'pyre', difficulty: 2, basePoints: 10 },
      { text: 'cinder', difficulty: 2, basePoints: 13 },
      { text: 'radiant', difficulty: 3, basePoints: 20 },
      { text: 'incandescent', difficulty: 5, basePoints: 48 },
      { text: 'forge', difficulty: 2, basePoints: 14 },
      { text: 'molten', difficulty: 2, basePoints: 15 },
      { text: 'raze', difficulty: 2, basePoints: 13 },
      { text: 'combust', difficulty: 3, basePoints: 21 },
      { text: 'char', difficulty: 1, basePoints: 7 },
      { text: 'luminous', difficulty: 3, basePoints: 23 },
      { text: 'blister', difficulty: 3, basePoints: 19 },
      { text: 'flare', difficulty: 2, basePoints: 12 },
      { text: 'incinerate', difficulty: 4, basePoints: 33 },
      { text: 'kindle', difficulty: 2, basePoints: 13 },
      { text: 'scathing', difficulty: 3, basePoints: 21 },
      { text: 'scorching', difficulty: 3, basePoints: 22 },
      { text: 'conflagration', difficulty: 5, basePoints: 47 },
      { text: 'blazon', difficulty: 3, basePoints: 20 },
      { text: 'firewall', difficulty: 3, basePoints: 21 },
      { text: 'hotly', difficulty: 2, basePoints: 11 },
      { text: 'igneous', difficulty: 3, basePoints: 19 },
      { text: 'scorpion', difficulty: 2, basePoints: 14 },
      { text: 'sizzle', difficulty: 2, basePoints: 12 },
      { text: 'sputter', difficulty: 2, basePoints: 13 },
      { text: 'sulfur', difficulty: 2, basePoints: 12 },
      { text: 'swelter', difficulty: 3, basePoints: 20 },
      { text: 'torrid', difficulty: 2, basePoints: 13 },
      { text: 'volcanic', difficulty: 3, basePoints: 22 },
      { text: 'warmth', difficulty: 1, basePoints: 8 },
      { text: 'wildfire', difficulty: 3, basePoints: 23 },
      { text: 'zealous', difficulty: 2, basePoints: 13 },
      { text: 'heat', difficulty: 1, basePoints: 7 },
      { text: 'lava', difficulty: 2, basePoints: 11 },
      { text: 'sunset', difficulty: 2, basePoints: 12 },
      { text: 'phoenix', difficulty: 3, basePoints: 21 },
      { text: 'plasma', difficulty: 3, basePoints: 20 },
      { text: 'flash', difficulty: 1, basePoints: 7 },
      { text: 'light', difficulty: 1, basePoints: 6 },
      { text: 'radiance', difficulty: 3, basePoints: 21 },
      { text: 'sunburst', difficulty: 3, basePoints: 22 },
      { text: 'flashpoint', difficulty: 3, basePoints: 23 },
      { text: 'flashing', difficulty: 2, basePoints: 14 },
      { text: 'firelight', difficulty: 3, basePoints: 21 },
      { text: 'flickering', difficulty: 3, basePoints: 20 },
      { text: 'incandescently', difficulty: 5, basePoints: 49 },
      { text: 'blazingly', difficulty: 4, basePoints: 32 },
      { text: 'scorchingly', difficulty: 4, basePoints: 33 },
      { text: 'sizzlingly', difficulty: 3, basePoints: 21 },
      { text: 'ignite', difficulty: 2, basePoints: 15 },
      { text: 'burning', difficulty: 2, basePoints: 14 },
      { text: 'bursting', difficulty: 2, basePoints: 14 },
      { text: 'exploding', difficulty: 3, basePoints: 21 },
      { text: 'explode', difficulty: 3, basePoints: 20 },
      { text: 'spurt', difficulty: 1, basePoints: 7 },
      { text: 'spurting', difficulty: 2, basePoints: 13 },
      { text: 'blazing', difficulty: 2, basePoints: 14 },
      { text: 'roaring', difficulty: 2, basePoints: 13 },
      { text: 'crackling', difficulty: 2, basePoints: 14 },
      { text: 'crackle', difficulty: 2, basePoints: 12 },
      { text: 'spit', difficulty: 1, basePoints: 6 },
      { text: 'spitting', difficulty: 2, basePoints: 13 },
      { text: 'streak', difficulty: 2, basePoints: 12 },
      { text: 'streaking', difficulty: 2, basePoints: 14 },
      { text: 'boom', difficulty: 1, basePoints: 6 },
      { text: 'booming', difficulty: 2, basePoints: 12 },
      { text: 'thunder', difficulty: 2, basePoints: 15 },
      { text: 'thundering', difficulty: 3, basePoints: 21 },
      { text: 'brilliant', difficulty: 3, basePoints: 20 },
      { text: 'brilliance', difficulty: 3, basePoints: 21 },
      { text: 'glow', difficulty: 1, basePoints: 6 },
      { text: 'glowing', difficulty: 2, basePoints: 12 },
      { text: 'gleam', difficulty: 1, basePoints: 6 },
      { text: 'gleaming', difficulty: 2, basePoints: 13 },
      { text: 'shine', difficulty: 1, basePoints: 6 },
      { text: 'shining', difficulty: 2, basePoints: 12 },
      { text: 'shiny', difficulty: 1, basePoints: 6 },
      { text: 'shimmer', difficulty: 2, basePoints: 13 },
      { text: 'glimmer', difficulty: 2, basePoints: 12 },
      { text: 'flicker', difficulty: 2, basePoints: 12 },
      { text: 'spark', difficulty: 1, basePoints: 7 },
      { text: 'sparking', difficulty: 2, basePoints: 13 },
      { text: 'sparks', difficulty: 1, basePoints: 7 },
      { text: 'bright', difficulty: 1, basePoints: 7 },
      { text: 'brightly', difficulty: 2, basePoints: 13 },
      { text: 'brightness', difficulty: 3, basePoints: 20 },
      { text: 'dazzle', difficulty: 2, basePoints: 13 },
      { text: 'dazzling', difficulty: 3, basePoints: 20 },
    ];

    // Ice words - expanded list (110 words)
    const iceWords = [
      { text: 'freeze', difficulty: 2, basePoints: 14 },
      { text: 'frost', difficulty: 2, basePoints: 12 },
      { text: 'chill', difficulty: 2, basePoints: 11 },
      { text: 'blizzard', difficulty: 3, basePoints: 23 },
      { text: 'icy', difficulty: 1, basePoints: 5 },
      { text: 'snow', difficulty: 1, basePoints: 6 },
      { text: 'crystal', difficulty: 2, basePoints: 15 },
      { text: 'glacial', difficulty: 3, basePoints: 21 },
      { text: 'shiver', difficulty: 2, basePoints: 13 },
      { text: 'sleet', difficulty: 2, basePoints: 11 },
      { text: 'hail', difficulty: 2, basePoints: 12 },
      { text: 'frozen', difficulty: 2, basePoints: 14 },
      { text: 'thaw', difficulty: 1, basePoints: 8 },
      { text: 'permafrost', difficulty: 4, basePoints: 31 },
      { text: 'iceberg', difficulty: 3, basePoints: 20 },
      { text: 'arctic', difficulty: 2, basePoints: 16 },
      { text: 'frostbite', difficulty: 3, basePoints: 26 },
      { text: 'crystalline', difficulty: 4, basePoints: 29 },
      { text: 'frigid', difficulty: 2, basePoints: 13 },
      { text: 'nippy', difficulty: 2, basePoints: 10 },
      { text: 'snowdrift', difficulty: 3, basePoints: 22 },
      { text: 'icicle', difficulty: 2, basePoints: 14 },
      { text: 'gelid', difficulty: 3, basePoints: 19 },
      { text: 'congelation', difficulty: 5, basePoints: 44 },
      { text: 'subfreezing', difficulty: 4, basePoints: 33 },
      { text: 'bleak', difficulty: 2, basePoints: 11 },
      { text: 'chilly', difficulty: 2, basePoints: 10 },
      { text: 'cold', difficulty: 1, basePoints: 6 },
      { text: 'cryo', difficulty: 2, basePoints: 12 },
      { text: 'frosty', difficulty: 2, basePoints: 13 },
      { text: 'glacier', difficulty: 2, basePoints: 14 },
      { text: 'ice', difficulty: 1, basePoints: 5 },
      { text: 'numb', difficulty: 1, basePoints: 7 },
      { text: 'polar', difficulty: 2, basePoints: 12 },
      { text: 'slippery', difficulty: 2, basePoints: 13 },
      { text: 'snowfall', difficulty: 2, basePoints: 14 },
      { text: 'snowflake', difficulty: 2, basePoints: 15 },
      { text: 'snowstorm', difficulty: 3, basePoints: 21 },
      { text: 'stiffness', difficulty: 3, basePoints: 19 },
      { text: 'tundra', difficulty: 2, basePoints: 13 },
      { text: 'winter', difficulty: 2, basePoints: 12 },
      { text: 'wintry', difficulty: 2, basePoints: 12 },
      { text: 'zero', difficulty: 1, basePoints: 6 },
      { text: 'cryogenic', difficulty: 4, basePoints: 30 },
      { text: 'frostwork', difficulty: 3, basePoints: 20 },
      { text: 'snowbank', difficulty: 2, basePoints: 13 },
      { text: 'snowslide', difficulty: 3, basePoints: 21 },
      { text: 'freezing', difficulty: 2, basePoints: 14 },
      { text: 'chilling', difficulty: 2, basePoints: 13 },
      { text: 'cold', difficulty: 1, basePoints: 6 },
      { text: 'coldness', difficulty: 2, basePoints: 13 },
      { text: 'coldly', difficulty: 2, basePoints: 12 },
      { text: 'cool', difficulty: 1, basePoints: 6 },
      { text: 'cooling', difficulty: 2, basePoints: 12 },
      { text: 'cool', difficulty: 1, basePoints: 6 },
      { text: 'coolness', difficulty: 2, basePoints: 13 },
      { text: 'coolly', difficulty: 2, basePoints: 12 },
      { text: 'bitter', difficulty: 2, basePoints: 12 },
      { text: 'bitterly', difficulty: 3, basePoints: 19 },
      { text: 'bitterness', difficulty: 3, basePoints: 20 },
      { text: 'crisp', difficulty: 1, basePoints: 7 },
      { text: 'crisply', difficulty: 2, basePoints: 12 },
      { text: 'crispness', difficulty: 3, basePoints: 19 },
      { text: 'brisk', difficulty: 1, basePoints: 7 },
      { text: 'briskly', difficulty: 2, basePoints: 12 },
      { text: 'briskness', difficulty: 3, basePoints: 19 },
      { text: 'chill', difficulty: 2, basePoints: 11 },
      { text: 'chiller', difficulty: 2, basePoints: 13 },
      { text: 'chilliness', difficulty: 3, basePoints: 20 },
      { text: 'frosty', difficulty: 2, basePoints: 13 },
      { text: 'frostily', difficulty: 3, basePoints: 19 },
      { text: 'frostiness', difficulty: 3, basePoints: 20 },
      { text: 'icy', difficulty: 1, basePoints: 5 },
      { text: 'icily', difficulty: 2, basePoints: 11 },
      { text: 'iciness', difficulty: 2, basePoints: 12 },
      { text: 'shivering', difficulty: 3, basePoints: 20 },
      { text: 'shiver', difficulty: 2, basePoints: 13 },
      { text: 'shivery', difficulty: 2, basePoints: 13 },
      { text: 'trembling', difficulty: 3, basePoints: 20 },
      { text: 'tremble', difficulty: 2, basePoints: 12 },
      { text: 'snow', difficulty: 1, basePoints: 6 },
      { text: 'snowing', difficulty: 2, basePoints: 13 },
      { text: 'snowy', difficulty: 1, basePoints: 6 },
      { text: 'snowiness', difficulty: 3, basePoints: 20 },
      { text: 'sleet', difficulty: 2, basePoints: 11 },
      { text: 'sleety', difficulty: 2, basePoints: 12 },
      { text: 'slettiness', difficulty: 3, basePoints: 20 },
      { text: 'hail', difficulty: 2, basePoints: 12 },
      { text: 'hailing', difficulty: 2, basePoints: 13 },
      { text: 'hailstone', difficulty: 3, basePoints: 20 },
      { text: 'frosted', difficulty: 2, basePoints: 13 },
      { text: 'ice', difficulty: 1, basePoints: 5 },
    ];

    // Earth words - expanded list (110 words)
    const earthWords = [
      { text: 'stone', difficulty: 1, basePoints: 8 },
      { text: 'earth', difficulty: 1, basePoints: 7 },
      { text: 'boulder', difficulty: 2, basePoints: 14 },
      { text: 'quake', difficulty: 2, basePoints: 13 },
      { text: 'tremor', difficulty: 2, basePoints: 12 },
      { text: 'bedrock', difficulty: 2, basePoints: 15 },
      { text: 'granite', difficulty: 2, basePoints: 14 },
      { text: 'mineral', difficulty: 2, basePoints: 13 },
      { text: 'rocky', difficulty: 1, basePoints: 6 },
      { text: 'terrain', difficulty: 2, basePoints: 12 },
      { text: 'geological', difficulty: 4, basePoints: 27 },
      { text: 'cavern', difficulty: 2, basePoints: 11 },
      { text: 'canyon', difficulty: 2, basePoints: 12 },
      { text: 'sediment', difficulty: 3, basePoints: 18 },
      { text: 'stalactite', difficulty: 4, basePoints: 28 },
      { text: 'seismic', difficulty: 3, basePoints: 20 },
      { text: 'upheaval', difficulty: 3, basePoints: 21 },
      { text: 'monolith', difficulty: 3, basePoints: 22 },
      { text: 'subterranean', difficulty: 4, basePoints: 31 },
      { text: 'abyss', difficulty: 2, basePoints: 13 },
      { text: 'limestone', difficulty: 3, basePoints: 19 },
      { text: 'sandstone', difficulty: 3, basePoints: 20 },
      { text: 'basalt', difficulty: 2, basePoints: 12 },
      { text: 'fault', difficulty: 2, basePoints: 11 },
      { text: 'scarp', difficulty: 2, basePoints: 10 },
      { text: 'cave', difficulty: 1, basePoints: 7 },
      { text: 'chalk', difficulty: 1, basePoints: 6 },
      { text: 'clay', difficulty: 1, basePoints: 6 },
      { text: 'coal', difficulty: 1, basePoints: 6 },
      { text: 'concrete', difficulty: 2, basePoints: 14 },
      { text: 'core', difficulty: 1, basePoints: 6 },
      { text: 'crust', difficulty: 2, basePoints: 12 },
      { text: 'crystaline', difficulty: 3, basePoints: 19 },
      { text: 'dig', difficulty: 1, basePoints: 5 },
      { text: 'dirt', difficulty: 1, basePoints: 6 },
      { text: 'erosion', difficulty: 3, basePoints: 19 },
      { text: 'foundation', difficulty: 3, basePoints: 20 },
      { text: 'gravel', difficulty: 2, basePoints: 11 },
      { text: 'ground', difficulty: 1, basePoints: 7 },
      { text: 'iron', difficulty: 1, basePoints: 7 },
      { text: 'marble', difficulty: 2, basePoints: 13 },
      { text: 'mountain', difficulty: 2, basePoints: 14 },
      { text: 'mud', difficulty: 1, basePoints: 5 },
      { text: 'opal', difficulty: 1, basePoints: 6 },
      { text: 'ore', difficulty: 1, basePoints: 6 },
      { text: 'pebble', difficulty: 2, basePoints: 11 },
      { text: 'quartz', difficulty: 2, basePoints: 12 },
      { text: 'quarry', difficulty: 2, basePoints: 13 },
      { text: 'quarrying', difficulty: 3, basePoints: 20 },
      { text: 'quarried', difficulty: 2, basePoints: 13 },
      { text: 'rock', difficulty: 1, basePoints: 6 },
      { text: 'rocky', difficulty: 1, basePoints: 6 },
      { text: 'rockiness', difficulty: 3, basePoints: 20 },
      { text: 'rockily', difficulty: 2, basePoints: 12 },
      { text: 'rocks', difficulty: 1, basePoints: 6 },
      { text: 'rubble', difficulty: 2, basePoints: 12 },
      { text: 'rubbly', difficulty: 2, basePoints: 12 },
      { text: 'sand', difficulty: 1, basePoints: 6 },
      { text: 'sandy', difficulty: 1, basePoints: 6 },
      { text: 'sandiness', difficulty: 3, basePoints: 20 },
      { text: 'sandily', difficulty: 2, basePoints: 12 },
      { text: 'sedimentary', difficulty: 4, basePoints: 29 },
      { text: 'sedimentary', difficulty: 4, basePoints: 29 },
      { text: 'soil', difficulty: 1, basePoints: 6 },
      { text: 'soiled', difficulty: 2, basePoints: 12 },
      { text: 'soiling', difficulty: 2, basePoints: 13 },
      { text: 'soiliness', difficulty: 3, basePoints: 20 },
      { text: 'solid', difficulty: 1, basePoints: 7 },
      { text: 'solidly', difficulty: 2, basePoints: 12 },
      { text: 'solidness', difficulty: 3, basePoints: 20 },
      { text: 'stability', difficulty: 3, basePoints: 20 },
      { text: 'stable', difficulty: 2, basePoints: 12 },
      { text: 'stably', difficulty: 2, basePoints: 11 },
      { text: 'stableness', difficulty: 3, basePoints: 20 },
      { text: 'structure', difficulty: 3, basePoints: 20 },
      { text: 'structural', difficulty: 4, basePoints: 28 },
      { text: 'structurally', difficulty: 4, basePoints: 29 },
      { text: 'substantial', difficulty: 4, basePoints: 29 },
      { text: 'substantially', difficulty: 5, basePoints: 39 },
      { text: 'substance', difficulty: 3, basePoints: 20 },
      { text: 'substanceless', difficulty: 4, basePoints: 29 },
      { text: 'substratum', difficulty: 4, basePoints: 28 },
      { text: 'substrate', difficulty: 3, basePoints: 20 },
      { text: 'subsoil', difficulty: 3, basePoints: 20 },
      { text: 'subsurface', difficulty: 4, basePoints: 28 },
      { text: 'surface', difficulty: 2, basePoints: 13 },
      { text: 'surfaced', difficulty: 2, basePoints: 13 },
      { text: 'surfacing', difficulty: 3, basePoints: 20 },
      { text: 'terra', difficulty: 2, basePoints: 12 },
      { text: 'terrain', difficulty: 2, basePoints: 12 },
      { text: 'territorial', difficulty: 4, basePoints: 29 },
      { text: 'territory', difficulty: 3, basePoints: 20 },
      { text: 'terrestrial', difficulty: 4, basePoints: 29 },
      { text: 'texture', difficulty: 2, basePoints: 13 },
      { text: 'textured', difficulty: 2, basePoints: 13 },
      { text: 'textural', difficulty: 3, basePoints: 20 },
      { text: 'textually', difficulty: 3, basePoints: 20 },
      { text: 'vibration', difficulty: 3, basePoints: 20 },
    ];

    // Lightning words - expanded list (110 words)
    const lightningWords = [
      { text: 'spark', difficulty: 1, basePoints: 9 },
      { text: 'bolt', difficulty: 1, basePoints: 8 },
      { text: 'thunder', difficulty: 2, basePoints: 16 },
      { text: 'lightning', difficulty: 3, basePoints: 24 },
      { text: 'electric', difficulty: 2, basePoints: 14 },
      { text: 'voltage', difficulty: 2, basePoints: 13 },
      { text: 'arc', difficulty: 1, basePoints: 5 },
      { text: 'charge', difficulty: 2, basePoints: 12 },
      { text: 'current', difficulty: 2, basePoints: 11 },
      { text: 'static', difficulty: 2, basePoints: 10 },
      { text: 'zap', difficulty: 1, basePoints: 6 },
      { text: 'electrocute', difficulty: 4, basePoints: 30 },
      { text: 'thunderstorm', difficulty: 3, basePoints: 26 },
      { text: 'tempest', difficulty: 2, basePoints: 15 },
      { text: 'conductor', difficulty: 3, basePoints: 19 },
      { text: 'electromagnetic', difficulty: 5, basePoints: 46 },
      { text: 'corona', difficulty: 2, basePoints: 12 },
      { text: 'luminescence', difficulty: 4, basePoints: 32 },
      { text: 'ionize', difficulty: 3, basePoints: 21 },
      { text: 'surge', difficulty: 2, basePoints: 13 },
      { text: 'discharge', difficulty: 3, basePoints: 23 },
      { text: 'pulsar', difficulty: 2, basePoints: 14 },
      { text: 'flicker', difficulty: 2, basePoints: 11 },
      { text: 'jolt', difficulty: 1, basePoints: 7 },
      { text: 'energy', difficulty: 2, basePoints: 13 },
      { text: 'ampere', difficulty: 2, basePoints: 12 },
      { text: 'battery', difficulty: 2, basePoints: 13 },
      { text: 'blinding', difficulty: 2, basePoints: 13 },
      { text: 'brightness', difficulty: 3, basePoints: 21 },
      { text: 'buzz', difficulty: 1, basePoints: 6 },
      { text: 'cable', difficulty: 1, basePoints: 6 },
      { text: 'circuit', difficulty: 2, basePoints: 12 },
      { text: 'crack', difficulty: 1, basePoints: 6 },
      { text: 'electricity', difficulty: 4, basePoints: 29 },
      { text: 'electron', difficulty: 3, basePoints: 20 },
      { text: 'flash', difficulty: 1, basePoints: 7 },
      { text: 'glow', difficulty: 1, basePoints: 6 },
      { text: 'light', difficulty: 1, basePoints: 7 },
      { text: 'neon', difficulty: 1, basePoints: 6 },
      { text: 'photon', difficulty: 2, basePoints: 12 },
      { text: 'power', difficulty: 2, basePoints: 12 },
      { text: 'radiant', difficulty: 2, basePoints: 13 },
      { text: 'radiation', difficulty: 3, basePoints: 20 },
      { text: 'shine', difficulty: 1, basePoints: 6 },
      { text: 'shock', difficulty: 1, basePoints: 7 },
      { text: 'signal', difficulty: 2, basePoints: 12 },
      { text: 'wire', difficulty: 1, basePoints: 6 },
      { text: 'wired', difficulty: 1, basePoints: 6 },
      { text: 'wiring', difficulty: 2, basePoints: 12 },
      { text: 'wireless', difficulty: 3, basePoints: 20 },
      { text: 'wireless', difficulty: 3, basePoints: 20 },
      { text: 'sparkle', difficulty: 2, basePoints: 13 },
      { text: 'sparkling', difficulty: 3, basePoints: 20 },
      { text: 'sparkly', difficulty: 2, basePoints: 13 },
      { text: 'sparkler', difficulty: 2, basePoints: 14 },
      { text: 'sparks', difficulty: 1, basePoints: 7 },
      { text: 'sparky', difficulty: 2, basePoints: 12 },
      { text: 'snap', difficulty: 1, basePoints: 6 },
      { text: 'snapping', difficulty: 2, basePoints: 13 },
      { text: 'snappy', difficulty: 2, basePoints: 12 },
      { text: 'snaps', difficulty: 1, basePoints: 6 },
      { text: 'snapped', difficulty: 2, basePoints: 12 },
      { text: 'snap', difficulty: 1, basePoints: 6 },
      { text: 'crack', difficulty: 1, basePoints: 6 },
      { text: 'crackle', difficulty: 2, basePoints: 12 },
      { text: 'crackling', difficulty: 2, basePoints: 14 },
      { text: 'crackly', difficulty: 2, basePoints: 12 },
      { text: 'crackling', difficulty: 2, basePoints: 14 },
      { text: 'crackup', difficulty: 2, basePoints: 13 },
      { text: 'pop', difficulty: 1, basePoints: 6 },
      { text: 'popping', difficulty: 2, basePoints: 12 },
      { text: 'poppy', difficulty: 2, basePoints: 12 },
      { text: 'pops', difficulty: 1, basePoints: 6 },
      { text: 'popped', difficulty: 2, basePoints: 12 },
      { text: 'boom', difficulty: 1, basePoints: 6 },
      { text: 'booming', difficulty: 2, basePoints: 12 },
      { text: 'boom', difficulty: 1, basePoints: 6 },
      { text: 'booms', difficulty: 1, basePoints: 6 },
      { text: 'boomed', difficulty: 2, basePoints: 12 },
      { text: 'bang', difficulty: 1, basePoints: 6 },
      { text: 'banging', difficulty: 2, basePoints: 12 },
      { text: 'bangs', difficulty: 1, basePoints: 6 },
      { text: 'banged', difficulty: 2, basePoints: 12 },
      { text: 'collision', difficulty: 3, basePoints: 20 },
      { text: 'collide', difficulty: 2, basePoints: 13 },
      { text: 'colliding', difficulty: 3, basePoints: 20 },
      { text: 'collision', difficulty: 3, basePoints: 20 },
      { text: 'explosive', difficulty: 3, basePoints: 20 },
      { text: 'explosion', difficulty: 3, basePoints: 20 },
      { text: 'explode', difficulty: 3, basePoints: 20 },
      { text: 'exploding', difficulty: 3, basePoints: 21 },
      { text: 'blast', difficulty: 2, basePoints: 12 },
      { text: 'blasting', difficulty: 2, basePoints: 13 },
      { text: 'blasts', difficulty: 2, basePoints: 12 },
      { text: 'blasted', difficulty: 2, basePoints: 12 },
      { text: 'burst', difficulty: 2, basePoints: 12 },
      { text: 'bursting', difficulty: 2, basePoints: 14 },
      { text: 'bursts', difficulty: 2, basePoints: 12 },
      { text: 'bursted', difficulty: 2, basePoints: 12 },
    ];

    // Healing words - expanded list (110 words)
    const healWords = [
      { text: 'heal', difficulty: 1, basePoints: 10 },
      { text: 'medicine', difficulty: 2, basePoints: 18 },
      { text: 'cure', difficulty: 1, basePoints: 9 },
      { text: 'potion', difficulty: 2, basePoints: 14 },
      { text: 'remedy', difficulty: 2, basePoints: 13 },
      { text: 'restore', difficulty: 2, basePoints: 16 },
      { text: 'rejuvenate', difficulty: 3, basePoints: 24 },
      { text: 'vitality', difficulty: 2, basePoints: 15 },
      { text: 'mend', difficulty: 1, basePoints: 8 },
      { text: 'revival', difficulty: 2, basePoints: 15 },
      { text: 'sanctuary', difficulty: 3, basePoints: 21 },
      { text: 'blessing', difficulty: 2, basePoints: 14 },
      { text: 'regenerate', difficulty: 3, basePoints: 23 },
      { text: 'resuscitate', difficulty: 4, basePoints: 34 },
      { text: 'wholeness', difficulty: 3, basePoints: 20 },
      { text: 'recovery', difficulty: 2, basePoints: 17 },
      { text: 'recuperate', difficulty: 3, basePoints: 22 },
      { text: 'revive', difficulty: 2, basePoints: 13 },
      { text: 'aid', difficulty: 1, basePoints: 6 },
      { text: 'balm', difficulty: 1, basePoints: 8 },
      { text: 'elixir', difficulty: 2, basePoints: 12 },
      { text: 'salve', difficulty: 2, basePoints: 11 },
      { text: 'therapy', difficulty: 2, basePoints: 14 },
      { text: 'health', difficulty: 1, basePoints: 10 },
      { text: 'vigor', difficulty: 2, basePoints: 12 },
      { text: 'antidote', difficulty: 2, basePoints: 15 },
      { text: 'bandage', difficulty: 2, basePoints: 13 },
      { text: 'benefit', difficulty: 2, basePoints: 14 },
      { text: 'boost', difficulty: 1, basePoints: 8 },
      { text: 'comfort', difficulty: 2, basePoints: 13 },
      { text: 'convalescence', difficulty: 4, basePoints: 31 },
      { text: 'doctor', difficulty: 2, basePoints: 13 },
      { text: 'enlightenment', difficulty: 4, basePoints: 30 },
      { text: 'essence', difficulty: 2, basePoints: 13 },
      { text: 'flourish', difficulty: 2, basePoints: 14 },
      { text: 'fortify', difficulty: 2, basePoints: 14 },
      { text: 'gratify', difficulty: 2, basePoints: 13 },
      { text: 'help', difficulty: 1, basePoints: 7 },
      { text: 'hope', difficulty: 1, basePoints: 7 },
      { text: 'improve', difficulty: 2, basePoints: 14 },
      { text: 'invigorate', difficulty: 3, basePoints: 22 },
      { text: 'life', difficulty: 1, basePoints: 8 },
      { text: 'lift', difficulty: 1, basePoints: 7 },
      { text: 'medic', difficulty: 2, basePoints: 12 },
      { text: 'nurse', difficulty: 2, basePoints: 12 },
      { text: 'physician', difficulty: 3, basePoints: 20 },
      { text: 'purity', difficulty: 2, basePoints: 13 },
      { text: 'refresh', difficulty: 2, basePoints: 14 },
      { text: 'relief', difficulty: 2, basePoints: 13 },
      { text: 'renew', difficulty: 2, basePoints: 12 },
      { text: 'repair', difficulty: 2, basePoints: 13 },
      { text: 'replenish', difficulty: 3, basePoints: 21 },
      { text: 'restorative', difficulty: 4, basePoints: 29 },
      { text: 'rest', difficulty: 1, basePoints: 7 },
      { text: 'resting', difficulty: 2, basePoints: 13 },
      { text: 'rested', difficulty: 2, basePoints: 12 },
      { text: 'serenity', difficulty: 3, basePoints: 20 },
      { text: 'sedate', difficulty: 2, basePoints: 12 },
      { text: 'sedative', difficulty: 3, basePoints: 20 },
      { text: 'slumber', difficulty: 2, basePoints: 13 },
      { text: 'slumbering', difficulty: 3, basePoints: 21 },
      { text: 'sleepy', difficulty: 2, basePoints: 12 },
      { text: 'sleep', difficulty: 1, basePoints: 7 },
      { text: 'sleeping', difficulty: 2, basePoints: 13 },
      { text: 'slept', difficulty: 1, basePoints: 7 },
      { text: 'soothe', difficulty: 2, basePoints: 12 },
      { text: 'soothing', difficulty: 3, basePoints: 20 },
      { text: 'soothingly', difficulty: 3, basePoints: 21 },
      { text: 'soothed', difficulty: 2, basePoints: 13 },
      { text: 'spirit', difficulty: 2, basePoints: 12 },
      { text: 'spiritual', difficulty: 3, basePoints: 20 },
      { text: 'spirited', difficulty: 2, basePoints: 13 },
      { text: 'spiritually', difficulty: 4, basePoints: 29 },
      { text: 'strength', difficulty: 3, basePoints: 19 },
      { text: 'strengthen', difficulty: 3, basePoints: 21 },
      { text: 'strengthening', difficulty: 4, basePoints: 29 },
      { text: 'strong', difficulty: 1, basePoints: 7 },
      { text: 'strongly', difficulty: 2, basePoints: 13 },
      { text: 'stronger', difficulty: 2, basePoints: 13 },
      { text: 'strongest', difficulty: 3, basePoints: 20 },
      { text: 'support', difficulty: 2, basePoints: 13 },
      { text: 'supporting', difficulty: 3, basePoints: 20 },
      { text: 'supported', difficulty: 2, basePoints: 13 },
      { text: 'supportive', difficulty: 3, basePoints: 20 },
      { text: 'supportively', difficulty: 4, basePoints: 29 },
      { text: 'sustain', difficulty: 2, basePoints: 13 },
      { text: 'sustaining', difficulty: 3, basePoints: 20 },
      { text: 'sustained', difficulty: 2, basePoints: 13 },
      { text: 'sustenance', difficulty: 3, basePoints: 21 },
      { text: 'therapeutic', difficulty: 4, basePoints: 29 },
      { text: 'touch', difficulty: 1, basePoints: 7 },
      { text: 'touching', difficulty: 2, basePoints: 13 },
      { text: 'touchingly', difficulty: 3, basePoints: 20 },
      { text: 'touched', difficulty: 2, basePoints: 12 },
      { text: 'tranquil', difficulty: 3, basePoints: 20 },
      { text: 'tranquility', difficulty: 4, basePoints: 29 },
      { text: 'tranquilly', difficulty: 3, basePoints: 20 },
      { text: 'ultimate', difficulty: 3, basePoints: 20 },
      { text: 'ultimately', difficulty: 4, basePoints: 29 },
      { text: 'vitally', difficulty: 2, basePoints: 13 },
      { text: 'vitamins', difficulty: 3, basePoints: 20 },
      { text: 'vivid', difficulty: 2, basePoints: 12 },
      { text: 'vividly', difficulty: 3, basePoints: 20 },
      { text: 'vivacity', difficulty: 3, basePoints: 20 },
      { text: 'vivaciously', difficulty: 4, basePoints: 29 },
      { text: 'vital', difficulty: 2, basePoints: 12 },
      { text: 'vitalize', difficulty: 3, basePoints: 20 },
      { text: 'vitalizing', difficulty: 4, basePoints: 29 },
      { text: 'wellness', difficulty: 3, basePoints: 20 },
      { text: 'well', difficulty: 1, basePoints: 7 },
      { text: 'welldoing', difficulty: 3, basePoints: 20 },
      { text: 'welfare', difficulty: 2, basePoints: 13 },
      { text: 'wellness', difficulty: 3, basePoints: 20 },
      { text: 'wonderful', difficulty: 3, basePoints: 20 },
      { text: 'wonderfully', difficulty: 4, basePoints: 29 },
      { text: 'young', difficulty: 1, basePoints: 7 },
      { text: 'youngly', difficulty: 2, basePoints: 12 },
      { text: 'younger', difficulty: 2, basePoints: 12 },
      { text: 'youngest', difficulty: 2, basePoints: 13 },
      { text: 'youthful', difficulty: 2, basePoints: 13 },
      { text: 'youthfully', difficulty: 3, basePoints: 20 },
      { text: 'youthfulness', difficulty: 4, basePoints: 29 },
      { text: 'zest', difficulty: 1, basePoints: 7 },
      { text: 'zesty', difficulty: 2, basePoints: 12 },
      { text: 'zestily', difficulty: 2, basePoints: 12 },
      { text: 'zestfulness', difficulty: 3, basePoints: 20 },
      { text: 'zone', difficulty: 1, basePoints: 7 },
      { text: 'zoning', difficulty: 2, basePoints: 12 },
      { text: 'zoned', difficulty: 1, basePoints: 6 },
      { text: 'zones', difficulty: 1, basePoints: 7 },
    ];

    // Combine all word arrays with categories
    fireWords.forEach(w => allWordData.push({ ...w, category: 'fire' }));
    iceWords.forEach(w => allWordData.push({ ...w, category: 'ice' }));
    earthWords.forEach(w => allWordData.push({ ...w, category: 'earth' }));
    lightningWords.forEach(w => allWordData.push({ ...w, category: 'lightning' }));
    healWords.forEach(w => allWordData.push({ ...w, category: 'heal' }));

    // Create Word objects and sort by difficulty
    this.words = allWordData
      .map(data => new Word(data))
      .sort((a, b) => a.difficulty - b.difficulty);
  }

  getWordsByDifficulty(difficulty: number): Word[] {
    return this.words.filter(w => w.difficulty === difficulty);
  }

  getRandomWord(maxDifficulty: number = 5): Word {
    const available = this.words.filter(w => w.difficulty <= maxDifficulty);
    return available[Math.floor(Math.random() * available.length)];
  }

  getRandomWordsByCategory(
    category: 'fire' | 'ice' | 'earth' | 'lightning' | 'heal'
  ): Word {
    const available = this.words.filter(w => w.category === category);
    return available[Math.floor(Math.random() * available.length)];
  }

  getAllWords(): Word[] {
    return [...this.words];
  }

  getWordCount(): number {
    return this.words.length;
  }

  validateWord(text: string): Word | null {
    const word = this.words.find(w => w.text === text.toLowerCase());
    return word || null;
  }
}

// ============================================================================
// BATTLE SYSTEM
// ============================================================================

class Battle {
  player: Player;
  enemy: Enemy;
  combo: ComboSystem;
  wordDatabase: WordDatabase;
  round: number;
  isActive: boolean;
  playerScore: number;
  timeStarted: number;

  constructor(
    player: Player,
    enemy: Enemy,
    wordDatabase: WordDatabase
  ) {
    this.player = player;
    this.enemy = enemy;
    this.combo = new ComboSystem();
    this.wordDatabase = wordDatabase;
    this.round = 1;
    this.isActive = true;
    this.playerScore = 0;
    this.timeStarted = Date.now();
  }

  processWord(typedWord: string): {
    valid: boolean;
    damage: number;
    healing: number;
    manaRestored: number;
    points: number;
    message: string;
  } {
    const timeMs = Date.now() - this.timeStarted;
    const word = this.wordDatabase.validateWord(typedWord);

    if (!word) {
      this.combo.addMiss();
      this.player.recordWord(false);
      return {
        valid: false,
        damage: 0,
        healing: 0,
        manaRestored: 0,
        points: 0,
        message: 'Word not found!',
      };
    }

    // Word is valid
    this.combo.addHit();
    this.player.recordWord(true);

    const speedBonus = word.getSpeedBonus(timeMs);
    let damage = word.getDamage(speedBonus);
    let healing = 0;
    let manaRestored = 0;

    // Apply combo multiplier
    damage = Math.floor(damage * this.combo.getComboMultiplier());

    // Handle word category effects
    if (word.category === 'heal') {
      healing = damage; // Healing words heal instead of damage
      damage = 0;
      this.player.heal(healing);
    } else if (word.category === 'ice') {
      // Ice words reduce enemy attack on their next turn
      this.enemy.attack = Math.floor(this.enemy.attack * 0.8);
    }

    // Mana restoration
    manaRestored = this.player.manaRegenPerWord;
    this.player.receiveMana(manaRestored);

    // Apply damage to enemy
    if (damage > 0) {
      this.enemy.takeDamage(damage);
    }

    // Gain experience
    this.player.gainExperience(Math.floor(damage / 2 + 5));

    // Calculate points
    const points =
      damage + this.combo.getComboBonus() + Math.floor(speedBonus * 10);
    this.playerScore += points;

    // Check battle end
    if (!this.enemy.isAlive()) {
      this.isActive = false;
    }

    return {
      valid: true,
      damage,
      healing,
      manaRestored,
      points,
      message: `${word.text} (${word.category}) - ${damage} damage! Combo x${Math.floor(this.combo.currentCombo / 5) + 1}`,
    };
  }

  enemyTurn(): number {
    const damage = this.enemy.getAttackDamage();
    this.player.takeDamage(damage);
    return damage;
  }

  isBattleActive(): boolean {
    return this.isActive && this.player.isAlive() && this.enemy.isAlive();
  }

  getBattleStats(): {
    playerHealth: number;
    playerHealthMax: number;
    enemyHealth: number;
    enemyHealthMax: number;
    combo: number;
    score: number;
    round: number;
  } {
    return {
      playerHealth: this.player.currentHealth,
      playerHealthMax: this.player.maxHealth,
      enemyHealth: this.enemy.currentHealth,
      enemyHealthMax: this.enemy.maxHealth,
      combo: this.combo.currentCombo,
      score: this.playerScore,
      round: this.round,
    };
  }
}

// ============================================================================
// GAME MANAGER
// ============================================================================

class Game {
  player: Player;
  wordDatabase: WordDatabase;
  currentBattle: Battle | null;
  currentLevel: number;
  totalScore: number;
  bossDefeated: boolean;
  gameOver: boolean;
  enemies: Enemy[] = [];

  constructor() {
    this.player = new Player('Warrior');
    this.wordDatabase = new WordDatabase();
    this.currentBattle = null;
    this.currentLevel = 1;
    this.totalScore = 0;
    this.bossDefeated = false;
    this.gameOver = false;
    this.initializeEnemies();
  }

  private initializeEnemies(): void {
    // Level 1-3: Regular enemies
    this.enemies.push(new Enemy('Goblin', 20, 5, 1, false));
    this.enemies.push(new Enemy('Orc', 35, 8, 1, false));
    this.enemies.push(new Enemy('Troll', 50, 12, 2, false));

    // Level 4-5: Stronger enemies
    this.enemies.push(new Enemy('Drake', 75, 15, 3, false));
    this.enemies.push(new Enemy('Demon', 100, 20, 4, false));

    // Boss enemies
    this.enemies.push(new Enemy('Shadow Lord', 150, 25, 5, true));
    this.enemies.push(new Enemy('Fire Dragon', 180, 28, 6, true));
    this.enemies.push(new Enemy('Ice Lich', 160, 26, 6, true));
  }

  startBattle(enemyIndex: number = 0): void {
    if (enemyIndex >= this.enemies.length) {
      this.gameOver = true;
      this.bossDefeated = true;
      return;
    }

    const enemy = this.enemies[enemyIndex];
    enemy.reset();
    this.currentBattle = new Battle(this.player, enemy, this.wordDatabase);
    this.currentLevel = enemyIndex + 1;
  }

  processWord(typedWord: string): {
    valid: boolean;
    damage: number;
    healing: number;
    manaRestored: number;
    points: number;
    message: string;
    battleActive: boolean;
  } {
    if (!this.currentBattle) {
      return {
        valid: false,
        damage: 0,
        healing: 0,
        manaRestored: 0,
        points: 0,
        message: 'No battle in progress',
        battleActive: false,
      };
    }

    const result = this.currentBattle.processWord(typedWord);

    if (result.valid && this.currentBattle.isBattleActive()) {
      // Enemy counter attack
      const enemyDamage = this.currentBattle.enemyTurn();
      result.message += ` | Enemy deals ${enemyDamage} damage!`;
    }

    if (!this.currentBattle.isBattleActive()) {
      this.totalScore += this.currentBattle.playerScore;
      if (this.currentBattle.enemy.isBoss) {
        this.bossDefeated = true;
        this.gameOver = true;
      } else {
        this.currentLevel++;
      }
    }

    return {
      ...result,
      battleActive: this.currentBattle.isBattleActive(),
    };
  }

  getGameStatus(): {
    level: number;
    playerHealth: number;
    playerHealthMax: number;
    playerMana: number;
    playerManaMax: number;
    playerLevel: number;
    combo: number;
    totalScore: number;
    gameOver: boolean;
    bossDefeated: boolean;
  } {
    if (!this.currentBattle) {
      return {
        level: this.currentLevel,
        playerHealth: this.player.currentHealth,
        playerHealthMax: this.player.maxHealth,
        playerMana: this.player.currentMana,
        playerManaMax: this.player.maxMana,
        playerLevel: this.player.level,
        combo: 0,
        totalScore: this.totalScore,
        gameOver: this.gameOver,
        bossDefeated: this.bossDefeated,
      };
    }

    return {
      level: this.currentLevel,
      playerHealth: this.currentBattle.player.currentHealth,
      playerHealthMax: this.currentBattle.player.maxHealth,
      playerMana: this.currentBattle.player.currentMana,
      playerManaMax: this.currentBattle.player.maxMana,
      playerLevel: this.currentBattle.player.level,
      combo: this.currentBattle.combo.currentCombo,
      totalScore: this.totalScore + this.currentBattle.playerScore,
      gameOver: this.gameOver,
      bossDefeated: this.bossDefeated,
    };
  }
}

// ============================================================================
// EXPORTS
// ============================================================================

export {
  Word,
  WordData,
  Enemy,
  Player,
  ComboSystem,
  WordDatabase,
  Battle,
  Game,
};
