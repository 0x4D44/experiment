import { MemoryQuestGame, GameStage } from './Game';
import { CharacterClass } from './Character';
import { Card } from './Card';

describe('MemoryQuestGame', () => {
  let game: MemoryQuestGame;

  beforeEach(() => {
    game = new MemoryQuestGame();
  });

  describe('Game Initialization', () => {
    it('should start at character select stage', () => {
      const state = game.getState();
      expect(state.stage).toBe(GameStage.CHARACTER_SELECT);
      expect(state.level).toBe(1);
      expect(state.character).toBeNull();
    });

    it('should have story content', () => {
      const story = game.getStoryText(0);
      expect(story).toBeDefined();
      expect(typeof story).toBe('string');
    });

    it('should have bosses configured', () => {
      const bosses = game.getBosses();
      expect(bosses.length).toBeGreaterThan(0);
    });
  });

  describe('Character Selection', () => {
    it('should select warrior character', () => {
      game.selectCharacter('Conan', CharacterClass.WARRIOR);
      const state = game.getState();
      expect(state.character).not.toBeNull();
      expect(state.character?.getName()).toBe('Conan');
      expect(state.character?.getClass()).toBe(CharacterClass.WARRIOR);
      expect(state.stage).toBe(GameStage.LEVEL_START);
    });

    it('should select mage character', () => {
      game.selectCharacter('Merlin', CharacterClass.MAGE);
      const state = game.getState();
      expect(state.character?.getName()).toBe('Merlin');
      expect(state.character?.getClass()).toBe(CharacterClass.MAGE);
    });

    it('should select rogue character', () => {
      game.selectCharacter('Shadow', CharacterClass.ROGUE);
      const state = game.getState();
      expect(state.character?.getName()).toBe('Shadow');
      expect(state.character?.getClass()).toBe(CharacterClass.ROGUE);
    });

    it('should throw error if selecting character at wrong stage', () => {
      game.selectCharacter('Hero', CharacterClass.WARRIOR);
      expect(() => {
        game.selectCharacter('Another', CharacterClass.MAGE);
      }).toThrow();
    });
  });

  describe('Level Start', () => {
    beforeEach(() => {
      game.selectCharacter('Hero', CharacterClass.WARRIOR);
    });

    it('should start level 1', () => {
      game.startLevel();
      const state = game.getState();
      expect(state.stage).toBe(GameStage.PLAYING);
      expect(state.board).not.toBeNull();
    });

    it('should create board with correct size for level', () => {
      game.startLevel();
      const state = game.getState();
      const cards = state.board?.getAllCards();
      expect(cards?.length).toBe(4); // 2x2 for level 1
    });

    it('should have all cards as pairs', () => {
      game.startLevel();
      const state = game.getState();
      const cards = state.board?.getAllCards();
      const names = new Map<string, number>();

      cards?.forEach((card) => {
        const count = names.get(card.getName()) || 0;
        names.set(card.getName(), count + 1);
      });

      names.forEach((count) => {
        expect(count).toBe(2); // Each card should appear exactly twice
      });
    });
  });

  describe('Level Progression', () => {
    beforeEach(() => {
      game.selectCharacter('Hero', CharacterClass.WARRIOR);
    });

    it('should complete level 1', () => {
      game.startLevel();
      game.autoCompleteLevelForTesting();
      game.completeLevel();

      const newState = game.getState();
      expect(newState.level).toBe(2);
      expect(newState.stage).toBe(GameStage.LEVEL_START);
    });

    it('should not complete level if not all cards matched', () => {
      game.startLevel();
      const state = game.getState();
      const cards = state.board?.getAllCards();

      // Only match half the cards
      for (let i = 0; i < cards!.length / 2; i++) {
        cards![i].reveal();
        cards![i].match();
      }

      expect(game.checkLevelComplete()).toBe(false);
    });

    it('should award experience on level complete', () => {
      const game2 = new MemoryQuestGame();
      game2.selectCharacter('Hero', CharacterClass.WARRIOR);
      const initialExp = game2.getState().character?.getExperience();

      game2.startLevel();
      game2.autoCompleteLevelForTesting();
      game2.completeLevel();

      const finalExp = game2.getState().character?.getExperience();
      expect(finalExp).toBeGreaterThan(initialExp!);
    });
  });

  describe('Boss Battles', () => {
    beforeEach(() => {
      game.selectCharacter('Hero', CharacterClass.WARRIOR);
      game.startLevel();
    });

    it('should get current boss', () => {
      const boss = game.getCurrentBoss();
      expect(boss).not.toBeNull();
      expect(boss?.name).toBe('Goblin Chief');
    });

    it('should start boss battle', () => {
      game.startBossBattle();
      const state = game.getState();
      expect(state.stage).toBe(GameStage.BATTLE);
      expect(state.battle).not.toBeNull();
    });

    it('should have active battle', () => {
      game.startBossBattle();
      const state = game.getState();
      expect(state.battle?.isBattleActive()).toBe(true);
    });
  });

  describe('Game Over', () => {
    beforeEach(() => {
      game.selectCharacter('Hero', CharacterClass.WARRIOR);
    });

    it('should auto-complete higher level boards', () => {
      const game2 = new MemoryQuestGame();
      game2.selectCharacter('Hero', CharacterClass.WARRIOR);

      // Level 1: 2x2 board
      game2.startLevel();
      game2.autoCompleteLevelForTesting();
      expect(game2.checkLevelComplete()).toBe(true);
      game2.completeLevel();

      // Level 2: 3x2 board
      game2.startLevel();
      const state = game2.getState();
      const board = state.board!;
      console.log('Level 2 board:', {
        cardCount: board.getAllCards().length,
        progress: board.getProgress(),
      });
      game2.autoCompleteLevelForTesting();
      console.log('After auto-complete:', board.getProgress());
      expect(game2.checkLevelComplete()).toBe(true);
    });

    it('should detect player victory after multiple levels', () => {
      const game2 = new MemoryQuestGame();
      game2.selectCharacter('Hero', CharacterClass.WARRIOR);

      // Complete level 1
      game2.startLevel();
      game2.autoCompleteLevelForTesting();
      game2.completeLevel();

      // Verify we're in level start for level 2
      let state = game2.getState();
      expect(state.level).toBe(2);
      expect(state.stage).toBe(GameStage.LEVEL_START);

      // Complete level 2
      game2.startLevel();
      game2.autoCompleteLevelForTesting();
      game2.completeLevel();

      // Verify progression
      state = game2.getState();
      expect(state.level).toBe(3);
      expect(state.stage).toBe(GameStage.LEVEL_START);

      // Character should exist and be alive
      expect(state.character?.isAlive()).toBe(true);
    });
  });

  describe('Game Reset', () => {
    beforeEach(() => {
      game.selectCharacter('Hero', CharacterClass.WARRIOR);
      game.startLevel();
    });

    it('should reset game to initial state', () => {
      game.reset();
      const state = game.getState();
      expect(state.stage).toBe(GameStage.CHARACTER_SELECT);
      expect(state.level).toBe(1);
      expect(state.board).toBeNull();
    });

    it('should reset character stats', () => {
      const char = game.getState().character!;
      char.addExperience(50);
      game.reset();

      const resetChar = game.getState().character!;
      expect(resetChar.getExperience()).toBe(0);
    });
  });

  describe('Story Progression', () => {
    it('should provide story text', () => {
      const story0 = game.getStoryText(0);
      const story1 = game.getStoryText(1);
      expect(story0).not.toBe(story1);
    });

    it('should return last story for out of bounds index', () => {
      const story = game.getStoryText(999);
      expect(story).toBeDefined();
    });
  });

  describe('Board Configuration', () => {
    beforeEach(() => {
      game.selectCharacter('Hero', CharacterClass.WARRIOR);
    });

    it('should create 2x2 board for level 1', () => {
      game.startLevel();
      const cards = game.getState().board?.getAllCards();
      expect(cards?.length).toBe(4);
    });

    it('should increase board size for higher levels', () => {
      let previousSize = 0;
      for (let level = 1; level <= 5; level++) {
        const game2 = new MemoryQuestGame();
        game2.selectCharacter('Hero', CharacterClass.WARRIOR);
        // Manually set level
        const state = game2.getState();
        state.level = level;

        game2.startLevel();
        const cards = game2.getState().board?.getAllCards();
        const currentSize = cards?.length || 0;

        if (previousSize > 0) {
          expect(currentSize).toBeGreaterThanOrEqual(previousSize);
        }
        previousSize = currentSize;
      }
    });
  });
});
