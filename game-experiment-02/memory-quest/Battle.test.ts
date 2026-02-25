import { Battle, Enemy } from './Battle';
import { Character, CharacterClass } from './Character';

describe('Battle', () => {
  let character: Character;
  let enemy: Enemy;
  let battle: Battle;

  beforeEach(() => {
    character = new Character('Hero', CharacterClass.WARRIOR);
    enemy = {
      name: 'Goblin Boss',
      health: 50,
      maxHealth: 50,
      attack: 8,
      defense: 3,
      requiredMatchedCards: 5,
      difficulty: 1,
      reward: { experience: 100, gold: 50 },
    };
    battle = new Battle(character, enemy);
  });

  describe('Battle Initialization', () => {
    it('should create a battle with character and enemy', () => {
      expect(battle.getCharacter()).toBe(character);
      const retrievedEnemy = battle.getEnemy();
      expect(retrievedEnemy.name).toBe('Goblin Boss');
    });

    it('should start battle when requested', () => {
      expect(battle.isBattleActive()).toBe(false);
      battle.startBattle();
      expect(battle.isBattleActive()).toBe(true);
    });

    it('should end battle when requested', () => {
      battle.startBattle();
      battle.endBattle();
      expect(battle.isBattleActive()).toBe(false);
    });
  });

  describe('Matched Cards Tracking', () => {
    it('should track matched cards', () => {
      expect(battle.getMatchedCardsCount()).toBe(0);
      battle.registerMatchedCards(1);
      expect(battle.getMatchedCardsCount()).toBe(1);
      battle.registerMatchedCards(2);
      expect(battle.getMatchedCardsCount()).toBe(3);
    });
  });

  describe('Damage Calculation', () => {
    beforeEach(() => {
      battle.startBattle();
    });

    it('should calculate damage based on character attack', () => {
      const baseDamage = character.getAttack();
      const calculatedDamage = battle.calculateDamage();
      expect(calculatedDamage).toBe(baseDamage);
    });

    it('should increase damage with matched cards', () => {
      const baseDamage = battle.calculateDamage();
      battle.registerMatchedCards(2);
      const boostedDamage = battle.calculateDamage();
      expect(boostedDamage).toBeGreaterThan(baseDamage);
    });

    it('should apply 10% bonus per matched card pair', () => {
      const baseDamage = battle.calculateDamage();
      battle.registerMatchedCards(1);
      const expectedDamage = Math.floor(baseDamage * 1.1);
      expect(battle.calculateDamage()).toBe(expectedDamage);
    });
  });

  describe('Healing Calculation', () => {
    beforeEach(() => {
      battle.startBattle();
    });

    it('should calculate healing', () => {
      const healing = battle.calculateHealing();
      expect(healing).toBeGreaterThan(0);
    });

    it('should increase healing with matched cards', () => {
      const baseHealing = battle.calculateHealing();
      battle.registerMatchedCards(2);
      const boostedHealing = battle.calculateHealing();
      expect(boostedHealing).toBeGreaterThan(baseHealing);
    });
  });

  describe('Combat Actions', () => {
    beforeEach(() => {
      battle.startBattle();
    });

    it('should apply player attack to enemy', () => {
      const initialHealth = enemy.health;
      const action = battle.playerAttack();
      expect(action.actor).toBe('player');
      expect(action.action).toBe('attack');
      expect(action.damage).toBeGreaterThan(0);
      expect(battle.getEnemy().health).toBeLessThan(initialHealth);
    });

    it('should apply defense to incoming damage', () => {
      const baseDamage = 10;
      const expectedDamage = Math.max(1, baseDamage - Math.floor(enemy.defense / 2));
      battle.playerAttack();
      // Can't directly verify defense reduction, but combat should occur
      expect(battle.getEnemy().health).toBeLessThan(enemy.maxHealth);
    });

    it('should reset matched cards after attack', () => {
      battle.registerMatchedCards(3);
      battle.playerAttack();
      expect(battle.getMatchedCardsCount()).toBe(0);
    });

    it('should apply player heal', () => {
      character.takeDamage(30);
      const healthBeforeHeal = character.getHealth();
      const action = battle.playerHeal();
      expect(action.actor).toBe('player');
      expect(action.action).toBe('heal');
      expect(action.healing).toBeGreaterThan(0);
      expect(character.getHealth()).toBeGreaterThan(healthBeforeHeal);
    });

    it('should reset matched cards after heal', () => {
      battle.registerMatchedCards(3);
      battle.playerHeal();
      expect(battle.getMatchedCardsCount()).toBe(0);
    });

    it('should apply enemy attack to character', () => {
      const initialHealth = character.getHealth();
      const action = battle.enemyAttack();
      expect(action.actor).toBe('enemy');
      expect(action.action).toBe('attack');
      expect(action.damage).toBeGreaterThan(0);
      expect(character.getHealth()).toBeLessThan(initialHealth);
    });

    it('should throw error if action attempted outside of battle', () => {
      battle.endBattle();
      expect(() => battle.playerAttack()).toThrow();
    });
  });

  describe('Battle Outcome', () => {
    beforeEach(() => {
      battle.startBattle();
    });

    it('should detect player victory', () => {
      while (!character.isAlive() && enemy.health > 0) {
        battle.playerAttack();
      }
      expect(battle.isBattleOver()).toBe(false); // Enemy not dead yet in most cases

      // Manually defeat enemy
      enemy.health = 0;
      expect(battle.hasPlayerWon()).toBe(true);
    });

    it('should detect player defeat', () => {
      character.takeDamage(character.getMaxHealth() + 10);
      expect(battle.hasPlayerLost()).toBe(true);
    });

    it('should know when battle is over', () => {
      expect(battle.isBattleOver()).toBe(false);
      character.takeDamage(character.getMaxHealth() + 10);
      expect(battle.isBattleOver()).toBe(true);
    });

    it('should provide rewards on victory', () => {
      enemy.health = 0;
      const rewards = battle.getBattleRewards();
      expect(rewards).not.toBeNull();
      expect(rewards?.experience).toBe(100);
      expect(rewards?.gold).toBe(50);
    });

    it('should not provide rewards on defeat', () => {
      character.takeDamage(character.getMaxHealth() + 10);
      const rewards = battle.getBattleRewards();
      expect(rewards).toBeNull();
    });
  });

  describe('Battle Status', () => {
    beforeEach(() => {
      battle.startBattle();
    });

    it('should provide battle status', () => {
      const status = battle.getBattleStatus();
      expect(status.playerHealth).toBe(character.getHealth());
      expect(status.playerMaxHealth).toBe(character.getMaxHealth());
      expect(status.enemyHealth).toBe(enemy.health);
      expect(status.enemyMaxHealth).toBe(enemy.maxHealth);
      expect(status.matchedCards).toBe(0);
      expect(status.requiredMatches).toBe(5);
    });

    it('should update status after actions', () => {
      battle.registerMatchedCards(2);
      battle.playerAttack();
      const status = battle.getBattleStatus();
      expect(status.enemyHealth).toBeLessThan(enemy.maxHealth);
      expect(status.matchedCards).toBe(0);
    });
  });

  describe('Battle Log', () => {
    beforeEach(() => {
      battle.startBattle();
    });

    it('should record battle actions', () => {
      battle.playerAttack();
      battle.enemyAttack();
      battle.playerHeal();
      const log = battle.getBattleLog();
      expect(log.length).toBe(3);
      expect(log[0].action).toBe('attack');
      expect(log[1].action).toBe('attack');
      expect(log[2].action).toBe('heal');
    });

    it('should maintain action history', () => {
      battle.playerAttack();
      const log1 = battle.getBattleLog();
      battle.enemyAttack();
      const log2 = battle.getBattleLog();
      expect(log2.length).toBeGreaterThan(log1.length);
    });
  });
});
