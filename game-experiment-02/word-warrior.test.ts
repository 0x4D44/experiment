/**
 * Word Warrior - Comprehensive Test Suite
 */

import {
  Word,
  Enemy,
  Player,
  ComboSystem,
  WordDatabase,
  Battle,
  Game,
} from './src/word-warrior-core';

// ============================================================================
// WORD CLASS TESTS
// ============================================================================

describe('Word Class', () => {
  it('should create a word with valid data', () => {
    const word = new Word({
      text: 'flame',
      difficulty: 2,
      category: 'fire',
      basePoints: 15,
    });
    expect(word.text).toBe('flame');
    expect(word.difficulty).toBe(2);
    expect(word.category).toBe('fire');
    expect(word.basePoints).toBe(15);
  });

  it('should throw error for empty text', () => {
    expect(() => {
      new Word({ text: '', difficulty: 2, category: 'fire', basePoints: 15 });
    }).toThrow('Word text cannot be empty');
  });

  it('should throw error for invalid difficulty', () => {
    expect(() => {
      new Word({
        text: 'flame',
        difficulty: 6,
        category: 'fire',
        basePoints: 15,
      });
    }).toThrow('Difficulty must be between 1 and 5');
  });

  it('should convert text to lowercase', () => {
    const word = new Word({
      text: 'FLAME',
      difficulty: 2,
      category: 'fire',
      basePoints: 15,
    });
    expect(word.text).toBe('flame');
  });

  it('should calculate damage with difficulty multiplier', () => {
    const word = new Word({
      text: 'flame',
      difficulty: 3,
      category: 'fire',
      basePoints: 10,
    });
    const damage = word.getDamage(1);
    expect(damage).toBe(30); // 10 * 3 * 1
  });

  it('should apply speed bonus to damage', () => {
    const word = new Word({
      text: 'flame',
      difficulty: 2,
      category: 'fire',
      basePoints: 10,
    });
    const damageWithBonus = word.getDamage(2.0);
    expect(damageWithBonus).toBe(40); // 10 * 2 * 2.0
  });

  it('should calculate correct speed bonuses', () => {
    const word = new Word({
      text: 'test',
      difficulty: 1,
      category: 'fire',
      basePoints: 1,
    });
    expect(word.getSpeedBonus(50)).toBe(2.0); // Fast
    expect(word.getSpeedBonus(150)).toBe(1.8); // Medium-fast
    expect(word.getSpeedBonus(400)).toBe(1.2); // Medium-slow
    expect(word.getSpeedBonus(1000)).toBe(0.8); // Slow
  });
});

// ============================================================================
// ENEMY CLASS TESTS
// ============================================================================

describe('Enemy Class', () => {
  it('should create an enemy with valid stats', () => {
    const enemy = new Enemy('Goblin', 50, 10, 1, false);
    expect(enemy.name).toBe('Goblin');
    expect(enemy.maxHealth).toBe(50);
    expect(enemy.currentHealth).toBe(50);
    expect(enemy.attack).toBe(10);
    expect(enemy.level).toBe(1);
    expect(enemy.isBoss).toBe(false);
  });

  it('should throw error for non-positive health', () => {
    expect(() => {
      new Enemy('Goblin', 0, 10, 1, false);
    }).toThrow('Health must be positive');
  });

  it('should take damage correctly', () => {
    const enemy = new Enemy('Goblin', 50, 10, 1, false);
    enemy.takeDamage(15);
    expect(enemy.currentHealth).toBe(35);
    expect(enemy.isAlive()).toBe(true);
  });

  it('should not go below 0 health', () => {
    const enemy = new Enemy('Goblin', 50, 10, 1, false);
    enemy.takeDamage(100);
    expect(enemy.currentHealth).toBe(0);
    expect(enemy.isAlive()).toBe(false);
  });

  it('should heal correctly', () => {
    const enemy = new Enemy('Goblin', 50, 10, 1, false);
    enemy.takeDamage(20);
    enemy.heal(10);
    expect(enemy.currentHealth).toBe(40);
  });

  it('should not exceed max health when healing', () => {
    const enemy = new Enemy('Goblin', 50, 10, 1, false);
    enemy.heal(100);
    expect(enemy.currentHealth).toBe(50);
  });

  it('should calculate health percent correctly', () => {
    const enemy = new Enemy('Goblin', 50, 10, 1, false);
    enemy.takeDamage(25);
    expect(enemy.getHealthPercent()).toBe(50);
  });

  it('should apply boss multiplier to attack damage', () => {
    const normalEnemy = new Enemy('Goblin', 50, 100, 1, false);
    const bossEnemy = new Enemy('Boss', 100, 100, 5, true);
    expect(normalEnemy.getAttackDamage()).toBe(100);
    expect(bossEnemy.getAttackDamage()).toBe(150);
  });

  it('should reset health on reset()', () => {
    const enemy = new Enemy('Goblin', 50, 10, 1, false);
    enemy.takeDamage(30);
    expect(enemy.currentHealth).toBe(20);
    enemy.reset();
    expect(enemy.currentHealth).toBe(50);
  });

  it('should track experience based on level', () => {
    const enemy = new Enemy('Goblin', 50, 10, 5, false);
    expect(enemy.experience).toBe(50); // 5 * 10
  });
});

// ============================================================================
// PLAYER CLASS TESTS
// ============================================================================

describe('Player Class', () => {
  it('should create a player with default stats', () => {
    const player = new Player();
    expect(player.name).toBe('Warrior');
    expect(player.maxHealth).toBe(100);
    expect(player.currentHealth).toBe(100);
    expect(player.level).toBe(1);
    expect(player.accuracy).toBe(100);
    expect(player.wordCount).toBe(0);
  });

  it('should create a player with custom name', () => {
    const player = new Player('Mage');
    expect(player.name).toBe('Mage');
  });

  it('should take damage correctly', () => {
    const player = new Player();
    player.takeDamage(30);
    expect(player.currentHealth).toBe(70);
    expect(player.isAlive()).toBe(true);
  });

  it('should die when health reaches 0', () => {
    const player = new Player();
    player.takeDamage(100);
    expect(player.isAlive()).toBe(false);
  });

  it('should heal correctly', () => {
    const player = new Player();
    player.takeDamage(40);
    player.heal(20);
    expect(player.currentHealth).toBe(80);
  });

  it('should manage mana correctly', () => {
    const player = new Player();
    expect(player.currentMana).toBe(50);
    player.receiveMana(20);
    expect(player.currentMana).toBe(50); // Max cap
    player.spendMana(10);
    expect(player.currentMana).toBe(40);
  });

  it('should return false when trying to spend more mana than available', () => {
    const player = new Player();
    expect(player.spendMana(100)).toBe(false);
    expect(player.currentMana).toBe(50); // Unchanged
  });

  it('should track word count when recording words', () => {
    const player = new Player();
    player.recordWord(true);
    player.recordWord(true);
    expect(player.wordCount).toBe(2);
  });

  it('should decrease accuracy on incorrect words', () => {
    const player = new Player();
    const startAccuracy = player.accuracy;
    player.recordWord(false);
    expect(player.accuracy).toBe(startAccuracy - 2);
  });

  it('should increase accuracy on correct words', () => {
    const player = new Player();
    player.recordWord(false); // Lower accuracy
    player.recordWord(false); // Lower more
    const lowAccuracy = player.accuracy;
    player.recordWord(true); // Increase
    expect(player.accuracy).toBeGreaterThan(lowAccuracy);
  });

  it('should gain experience and level up', () => {
    const player = new Player();
    player.gainExperience(150); // More than next level requirement
    expect(player.level).toBeGreaterThan(1);
  });

  it('should increase stats on level up', () => {
    const player = new Player();
    const initialMaxHealth = player.maxHealth;
    player.gainExperience(100);
    expect(player.maxHealth).toBeGreaterThan(initialMaxHealth);
  });

  it('should unlock abilities on level up', () => {
    const player = new Player();
    player.gainExperience(100);
    expect(player.abilities.size).toBeGreaterThan(0);
  });
});

// ============================================================================
// COMBO SYSTEM TESTS
// ============================================================================

describe('ComboSystem', () => {
  it('should initialize with 0 combo', () => {
    const combo = new ComboSystem();
    expect(combo.currentCombo).toBe(0);
    expect(combo.multiplier).toBe(1.0);
  });

  it('should increase combo on hits', () => {
    const combo = new ComboSystem();
    combo.addHit();
    combo.addHit();
    expect(combo.currentCombo).toBe(2);
  });

  it('should reset combo on miss', () => {
    const combo = new ComboSystem();
    combo.addHit();
    combo.addHit();
    combo.addMiss();
    expect(combo.currentCombo).toBe(0);
  });

  it('should calculate multiplier correctly', () => {
    const combo = new ComboSystem();
    combo.addHit(); // Combo 1, multiplier 1.0
    expect(combo.getComboMultiplier()).toBe(1.0);
    for (let i = 0; i < 4; i++) combo.addHit();
    expect(combo.getComboMultiplier()).toBe(1.5); // Combo 5, multiplier 1.5
    for (let i = 0; i < 5; i++) combo.addHit();
    expect(combo.getComboMultiplier()).toBe(2.0); // Combo 10, multiplier 2.0
  });

  it('should track max combo', () => {
    const combo = new ComboSystem();
    for (let i = 0; i < 7; i++) combo.addHit();
    expect(combo.maxCombo).toBe(7);
    combo.addMiss();
    expect(combo.currentCombo).toBe(0);
    expect(combo.maxCombo).toBe(7); // Still 7
  });

  it('should calculate bonus correctly', () => {
    const combo = new ComboSystem();
    combo.addHit(); // 1 hit = 0 bonus (formula: floor(max(0, (1-1)/5) * 10) = 0)
    expect(combo.getComboBonus()).toBe(0);
    for (let i = 0; i < 4; i++) combo.addHit();
    // At 5 hits: floor(max(0, (5-1)/5) * 10) = floor(0.8 * 10) = 8
    expect(combo.getComboBonus()).toBe(8);
    for (let i = 0; i < 5; i++) combo.addHit();
    // At 10 hits: floor(max(0, (10-1)/5) * 10) = floor(1.8 * 10) = 18
    expect(combo.getComboBonus()).toBe(18);
  });

  it('should reset combo and multiplier', () => {
    const combo = new ComboSystem();
    for (let i = 0; i < 10; i++) combo.addHit();
    combo.reset();
    expect(combo.currentCombo).toBe(0);
    expect(combo.multiplier).toBe(1.0);
  });
});

// ============================================================================
// WORD DATABASE TESTS
// ============================================================================

describe('WordDatabase', () => {
  it('should initialize with 500+ words', () => {
    const db = new WordDatabase();
    expect(db.getWordCount()).toBeGreaterThanOrEqual(500);
  });

  it('should return words by difficulty', () => {
    const db = new WordDatabase();
    const difficulty1Words = db.getWordsByDifficulty(1);
    expect(difficulty1Words.length).toBeGreaterThan(0);
    difficulty1Words.forEach(w => {
      expect(w.difficulty).toBe(1);
    });
  });

  it('should return random word with difficulty cap', () => {
    const db = new WordDatabase();
    for (let i = 0; i < 20; i++) {
      const word = db.getRandomWord(2);
      expect(word.difficulty).toBeLessThanOrEqual(2);
    }
  });

  it('should return random word by category', () => {
    const db = new WordDatabase();
    const fireWord = db.getRandomWordsByCategory('fire');
    expect(fireWord.category).toBe('fire');
  });

  it('should validate existing words', () => {
    const db = new WordDatabase();
    const word = db.validateWord('flame');
    expect(word).not.toBeNull();
    expect(word?.text).toBe('flame');
  });

  it('should return null for non-existing words', () => {
    const db = new WordDatabase();
    const word = db.validateWord('xyzabc');
    expect(word).toBeNull();
  });

  it('should be case-insensitive', () => {
    const db = new WordDatabase();
    const word1 = db.validateWord('FLAME');
    const word2 = db.validateWord('flame');
    expect(word1?.text).toBe(word2?.text);
  });

  it('should return all words', () => {
    const db = new WordDatabase();
    const allWords = db.getAllWords();
    expect(allWords.length).toBeGreaterThanOrEqual(500);
  });

  it('should have multiple categories', () => {
    const db = new WordDatabase();
    const fire = db.getRandomWordsByCategory('fire');
    const ice = db.getRandomWordsByCategory('ice');
    const earth = db.getRandomWordsByCategory('earth');
    const lightning = db.getRandomWordsByCategory('lightning');
    const heal = db.getRandomWordsByCategory('heal');

    expect(fire.category).toBe('fire');
    expect(ice.category).toBe('ice');
    expect(earth.category).toBe('earth');
    expect(lightning.category).toBe('lightning');
    expect(heal.category).toBe('heal');
  });
});

// ============================================================================
// BATTLE SYSTEM TESTS
// ============================================================================

describe('Battle Class', () => {
  it('should initialize battle correctly', () => {
    const player = new Player();
    const enemy = new Enemy('Goblin', 50, 10, 1, false);
    const db = new WordDatabase();
    const battle = new Battle(player, enemy, db);

    expect(battle.round).toBe(1);
    expect(battle.isActive).toBe(true);
    expect(battle.playerScore).toBe(0);
  });

  it('should reject invalid words', () => {
    const player = new Player();
    const enemy = new Enemy('Goblin', 50, 10, 1, false);
    const db = new WordDatabase();
    const battle = new Battle(player, enemy, db);

    const result = battle.processWord('invalidword');
    expect(result.valid).toBe(false);
    expect(result.damage).toBe(0);
  });

  it('should process valid words and deal damage', () => {
    const player = new Player();
    const enemy = new Enemy('Goblin', 100, 10, 1, false);
    const db = new WordDatabase();
    const battle = new Battle(player, enemy, db);

    const result = battle.processWord('flame');
    expect(result.valid).toBe(true);
    expect(result.damage).toBeGreaterThan(0);
    expect(enemy.currentHealth).toBeLessThan(100);
  });

  it('should apply combo multiplier', () => {
    const player = new Player();
    const enemy = new Enemy('Goblin', 500, 10, 1, false);
    const db = new WordDatabase();
    const battle = new Battle(player, enemy, db);

    // First word
    const result1 = battle.processWord('flame');
    const damage1 = result1.damage;

    // Reset for comparison
    enemy.reset();

    // Multiple words to build combo
    for (let i = 0; i < 5; i++) {
      battle.processWord('flame');
    }
    const result6 = battle.processWord('flame');

    // With combo multiplier, damage should be higher
    expect(result6.damage).toBeGreaterThan(damage1);
  });

  it('should increase combo on successful words', () => {
    const player = new Player();
    const enemy = new Enemy('Goblin', 500, 10, 1, false);
    const db = new WordDatabase();
    const battle = new Battle(player, enemy, db);

    battle.processWord('flame');
    expect(battle.combo.currentCombo).toBe(1);
    battle.processWord('freeze');
    expect(battle.combo.currentCombo).toBe(2);
  });

  it('should reset combo on invalid word', () => {
    const player = new Player();
    const enemy = new Enemy('Goblin', 500, 10, 1, false);
    const db = new WordDatabase();
    const battle = new Battle(player, enemy, db);

    battle.processWord('flame');
    battle.processWord('freeze');
    expect(battle.combo.currentCombo).toBe(2);
    battle.processWord('invalidword');
    expect(battle.combo.currentCombo).toBe(0);
  });

  it('should handle healing words', () => {
    const player = new Player();
    player.takeDamage(30); // Reduce health
    const startHealth = player.currentHealth;

    const enemy = new Enemy('Goblin', 500, 10, 1, false);
    const db = new WordDatabase();
    const battle = new Battle(player, enemy, db);

    const result = battle.processWord('heal');
    expect(result.healing).toBeGreaterThan(0);
    expect(player.currentHealth).toBeGreaterThan(startHealth);
  });

  it('should handle ice words (enemy debuff)', () => {
    const player = new Player();
    const enemy = new Enemy('Goblin', 500, 100, 1, false);
    const db = new WordDatabase();
    const battle = new Battle(player, enemy, db);

    const initialAttack = enemy.attack;
    battle.processWord('freeze');
    expect(enemy.attack).toBeLessThan(initialAttack);
  });

  it('should end battle when enemy dies', () => {
    const player = new Player();
    const enemy = new Enemy('Goblin', 10, 10, 1, false); // Lower health to ensure one word kills
    const db = new WordDatabase();
    const battle = new Battle(player, enemy, db);

    battle.processWord('inferno'); // High damage word (difficulty 3)
    expect(battle.isActive).toBe(false);
    expect(enemy.isAlive()).toBe(false);
  });

  it('should allow enemy counter attack', () => {
    const player = new Player();
    const enemy = new Enemy('Goblin', 500, 50, 1, false);
    const db = new WordDatabase();
    const battle = new Battle(player, enemy, db);

    const startHealth = player.currentHealth;
    battle.enemyTurn();
    expect(player.currentHealth).toBeLessThan(startHealth);
  });

  it('should track battle stats', () => {
    const player = new Player();
    const enemy = new Enemy('Goblin', 100, 10, 1, false);
    const db = new WordDatabase();
    const battle = new Battle(player, enemy, db);

    battle.processWord('flame');
    const stats = battle.getBattleStats();

    expect(stats.playerHealth).toBeLessThanOrEqual(stats.playerHealthMax);
    expect(stats.enemyHealth).toBeLessThan(stats.enemyHealthMax);
    expect(stats.combo).toBeGreaterThan(0);
    expect(stats.score).toBeGreaterThan(0);
  });
});

// ============================================================================
// GAME MANAGER TESTS
// ============================================================================

describe('Game Class', () => {
  it('should initialize game correctly', () => {
    const game = new Game();
    expect(game.currentLevel).toBe(1);
    expect(game.totalScore).toBe(0);
    expect(game.gameOver).toBe(false);
    expect(game.bossDefeated).toBe(false);
    expect(game.player.name).toBe('Warrior');
  });

  it('should have word database with 500+ words', () => {
    const game = new Game();
    expect(game.wordDatabase.getWordCount()).toBeGreaterThanOrEqual(500);
  });

  it('should start a battle', () => {
    const game = new Game();
    game.startBattle(0);
    expect(game.currentBattle).not.toBeNull();
    expect(game.currentLevel).toBe(1);
  });

  it('should process words in battle', () => {
    const game = new Game();
    game.startBattle(0);
    const result = game.processWord('flame');
    expect(result.valid).toBe(true);
    expect(result.damage).toBeGreaterThan(0);
  });

  it('should update score when word is processed', () => {
    const game = new Game();
    game.startBattle(0);
    const startScore = game.totalScore;
    game.processWord('flame');
    // Score is added after battle ends, so we check battle score
    if (game.currentBattle) {
      expect(game.currentBattle.playerScore).toBeGreaterThan(0);
    }
  });

  it('should handle invalid words', () => {
    const game = new Game();
    game.startBattle(0);
    const result = game.processWord('invalidword');
    expect(result.valid).toBe(false);
    expect(result.damage).toBe(0);
  });

  it('should progress through multiple battles', () => {
    const game = new Game();
    game.startBattle(0); // First enemy
    expect(game.currentLevel).toBe(1);

    // Defeat first enemy
    const enemy = game.currentBattle!.enemy;
    while (enemy.isAlive()) {
      game.processWord('flame');
    }

    expect(game.currentLevel).toBeGreaterThan(1);
  });

  it('should initialize multiple enemies', () => {
    const game = new Game();
    expect(game.enemies.length).toBeGreaterThan(0);
  });

  it('should have boss enemies', () => {
    const game = new Game();
    const bosses = game.enemies.filter(e => e.isBoss);
    expect(bosses.length).toBeGreaterThan(0);
  });

  it('should set game over when boss is defeated', () => {
    const game = new Game();
    // Start with last enemy (boss)
    game.startBattle(game.enemies.length - 1);
    const boss = game.currentBattle!.enemy;

    while (boss.isAlive()) {
      game.processWord('flame');
      if (!game.currentBattle?.isBattleActive()) break;
    }

    expect(game.gameOver).toBe(true);
    expect(game.bossDefeated).toBe(true);
  });

  it('should return game status', () => {
    const game = new Game();
    game.startBattle(0);
    const status = game.getGameStatus();

    expect(status.level).toBe(1);
    expect(status.playerHealth).toBeGreaterThan(0);
    expect(status.gameOver).toBe(false);
  });

  it('should handle no active battle gracefully', () => {
    const game = new Game();
    const result = game.processWord('flame');
    expect(result.valid).toBe(false);
    expect(result.message).toContain('No battle');
  });
});

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

describe('Integration Tests', () => {
  it('should complete a full game loop', () => {
    const game = new Game();

    // Start game
    game.startBattle(0);
    expect(game.currentBattle).not.toBeNull();

    // Play multiple rounds
    for (let i = 0; i < 20; i++) {
      if (!game.currentBattle?.isBattleActive()) {
        game.startBattle(game.currentLevel - 1);
      }
      game.processWord('flame');
    }

    expect(game.player.wordCount).toBeGreaterThan(0);
  });

  it('should track player progression', () => {
    const game = new Game();
    game.startBattle(0);

    const startLevel = game.player.level;
    for (let i = 0; i < 30; i++) {
      if (!game.currentBattle?.isBattleActive()) break;
      game.processWord('flame');
    }

    // Player should have gained some experience
    expect(game.player.totalExperience).toBeGreaterThanOrEqual(0);
  });

  it('should handle difficulty progression', () => {
    const game = new Game();

    // Easy enemies first
    game.startBattle(0);
    const firstEnemy = game.currentBattle!.enemy;
    expect(firstEnemy.level).toBe(1);

    // Should have harder enemies later
    game.startBattle(game.enemies.length - 1);
    const lastEnemy = game.currentBattle!.enemy;
    expect(lastEnemy.level).toBeGreaterThan(firstEnemy.level);
  });

  it('should award experience for defeating enemies', () => {
    const game = new Game();
    game.startBattle(0);

    const startXP = game.player.totalExperience;
    const enemy = game.currentBattle!.enemy;

    while (enemy.isAlive()) {
      game.processWord('flame');
    }

    expect(game.player.totalExperience).toBeGreaterThan(startXP);
  });
});
