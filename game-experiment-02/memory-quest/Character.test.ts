import { Character, CharacterClass } from './Character';

describe('Character', () => {
  let warrior: Character;
  let mage: Character;
  let rogue: Character;

  beforeEach(() => {
    warrior = new Character('Conan', CharacterClass.WARRIOR);
    mage = new Character('Merlin', CharacterClass.MAGE);
    rogue = new Character('Shadow', CharacterClass.ROGUE);
  });

  describe('Character Creation', () => {
    it('should create a warrior with correct stats', () => {
      expect(warrior.getName()).toBe('Conan');
      expect(warrior.getClass()).toBe(CharacterClass.WARRIOR);
      expect(warrior.getLevel()).toBe(1);
      expect(warrior.getHealth()).toBe(100);
      expect(warrior.getMaxHealth()).toBe(100);
      expect(warrior.getAttack()).toBe(15);
      expect(warrior.getDefense()).toBe(12);
    });

    it('should create a mage with correct stats', () => {
      expect(mage.getName()).toBe('Merlin');
      expect(mage.getClass()).toBe(CharacterClass.MAGE);
      expect(mage.getMaxMana()).toBe(80);
      expect(mage.getAttack()).toBe(10);
      expect(mage.getDefense()).toBe(6);
    });

    it('should create a rogue with correct stats', () => {
      expect(rogue.getName()).toBe('Shadow');
      expect(rogue.getClass()).toBe(CharacterClass.ROGUE);
      expect(rogue.getSpeed()).toBe(16);
      expect(rogue.getAttack()).toBe(12);
    });
  });

  describe('Health Management', () => {
    it('should start with full health', () => {
      expect(warrior.getHealth()).toBe(warrior.getMaxHealth());
    });

    it('should take damage', () => {
      const initialHealth = warrior.getHealth();
      warrior.takeDamage(20);
      // Defense of 12 reduces damage by 6, so actual damage is 20 - 6 = 14
      expect(warrior.getHealth()).toBe(initialHealth - 14);
    });

    it('should apply defense to damage', () => {
      const initialHealth = warrior.getHealth();
      warrior.takeDamage(10);
      // With defense of 12, reduction is 6, so damage is 10 - 6 = 4
      expect(warrior.getHealth()).toBe(initialHealth - 4);
    });

    it('should not go below 0 health', () => {
      warrior.takeDamage(1000);
      expect(warrior.getHealth()).toBe(0);
    });

    it('should heal character', () => {
      warrior.takeDamage(50);
      const healthAfterDamage = warrior.getHealth();
      warrior.heal(20);
      expect(warrior.getHealth()).toBe(healthAfterDamage + 20);
    });

    it('should not exceed max health', () => {
      warrior.heal(50);
      expect(warrior.getHealth()).toBe(warrior.getMaxHealth());
    });

    it('should detect if character is alive', () => {
      expect(warrior.isAlive()).toBe(true);
      warrior.takeDamage(1000);
      expect(warrior.isAlive()).toBe(false);
    });
  });

  describe('Mana Management', () => {
    it('should start with full mana', () => {
      expect(mage.getMana()).toBe(mage.getMaxMana());
    });

    it('should use mana for spells', () => {
      expect(mage.useMana(20)).toBe(true);
      expect(mage.getMana()).toBe(80 - 20);
    });

    it('should not use mana if not enough', () => {
      expect(mage.useMana(100)).toBe(false);
      expect(mage.getMana()).toBe(80);
    });

    it('should restore mana', () => {
      mage.useMana(40);
      mage.restoreMana(20);
      expect(mage.getMana()).toBe(60);
    });

    it('should not exceed max mana', () => {
      mage.restoreMana(50);
      expect(mage.getMana()).toBe(mage.getMaxMana());
    });
  });

  describe('Experience and Leveling', () => {
    it('should start at level 1', () => {
      expect(warrior.getLevel()).toBe(1);
      expect(warrior.getExperience()).toBe(0);
    });

    it('should gain experience', () => {
      warrior.addExperience(50);
      expect(warrior.getExperience()).toBe(50);
    });

    it('should level up when experience threshold is reached', () => {
      const didLevelUp = warrior.addExperience(100);
      expect(didLevelUp).toBe(true);
      expect(warrior.getLevel()).toBe(2);
      expect(warrior.getExperience()).toBe(0);
    });

    it('should increase stats on level up', () => {
      const initialAttack = warrior.getAttack();
      warrior.addExperience(100);
      expect(warrior.getAttack()).toBeGreaterThan(initialAttack);
    });

    it('should restore health on level up', () => {
      warrior.takeDamage(50);
      warrior.addExperience(100);
      expect(warrior.getHealth()).toBe(warrior.getMaxHealth());
    });

    it('should increase experience requirement for next level', () => {
      const exp1 = warrior.getExperienceForNextLevel();
      warrior.addExperience(100);
      const exp2 = warrior.getExperienceForNextLevel();
      expect(exp2).toBeGreaterThan(exp1);
    });
  });

  describe('Inventory Management', () => {
    it('should have empty inventory initially', () => {
      expect(warrior.getInventoryCount()).toBe(0);
    });

    it('should add items to inventory', () => {
      warrior.addToInventory('spell-01', 1);
      const inventory = warrior.getInventory();
      expect(inventory.get('spell-01')).toBe(1);
    });

    it('should stack items in inventory', () => {
      warrior.addToInventory('spell-01', 2);
      warrior.addToInventory('spell-01', 3);
      const inventory = warrior.getInventory();
      expect(inventory.get('spell-01')).toBe(5);
    });

    it('should remove items from inventory', () => {
      warrior.addToInventory('spell-01', 5);
      const removed = warrior.removeFromInventory('spell-01', 2);
      expect(removed).toBe(true);
      expect(warrior.getInventory().get('spell-01')).toBe(3);
    });

    it('should not remove more items than available', () => {
      warrior.addToInventory('spell-01', 2);
      const removed = warrior.removeFromInventory('spell-01', 5);
      expect(removed).toBe(false);
      expect(warrior.getInventory().get('spell-01')).toBe(2);
    });

    it('should remove item key when count reaches 0', () => {
      warrior.addToInventory('spell-01', 2);
      warrior.removeFromInventory('spell-01', 2);
      const inventory = warrior.getInventory();
      expect(inventory.has('spell-01')).toBe(false);
    });

    it('should clear entire inventory', () => {
      warrior.addToInventory('spell-01', 3);
      warrior.addToInventory('item-01', 2);
      warrior.clearInventory();
      expect(warrior.getInventoryCount()).toBe(0);
    });
  });

  describe('Character Reset', () => {
    it('should reset character to initial state', () => {
      warrior.addExperience(100);
      warrior.takeDamage(50);
      warrior.addToInventory('spell-01', 3);

      warrior.reset();

      expect(warrior.getLevel()).toBe(1);
      expect(warrior.getExperience()).toBe(0);
      expect(warrior.getHealth()).toBe(warrior.getMaxHealth());
      expect(warrior.getInventoryCount()).toBe(0);
    });
  });
});
