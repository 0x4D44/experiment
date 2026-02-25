/**
 * Game manager - orchestrates all game systems
 */

import { Board, BoardConfig } from './Board';
import { Card, CardData, CardType } from './Card';
import { Character, CharacterClass } from './Character';
import { Battle, Enemy } from './Battle';

export enum GameStage {
  CHARACTER_SELECT = 'CHARACTER_SELECT',
  LEVEL_START = 'LEVEL_START',
  PLAYING = 'PLAYING',
  BATTLE = 'BATTLE',
  LEVEL_COMPLETE = 'LEVEL_COMPLETE',
  GAME_OVER = 'GAME_OVER',
}

export interface GameState {
  stage: GameStage;
  level: number;
  character: Character | null;
  board: Board | null;
  battle: Battle | null;
  storyIndex: number;
}

export class MemoryQuestGame {
  private state: GameState;
  private story: string[] = [];
  private cardLibrary: Map<string, CardData> = new Map();
  private bosses: Enemy[] = [];

  constructor() {
    this.state = {
      stage: GameStage.CHARACTER_SELECT,
      level: 1,
      character: null,
      board: null,
      battle: null,
      storyIndex: 0,
    };

    this.initializeCardLibrary();
    this.initializeBosses();
    this.initializeStory();
  }

  /**
   * Initialize all cards in the game
   */
  private initializeCardLibrary(): void {
    // Spells
    const spells: CardData[] = [
      {
        id: 'spell-01',
        type: CardType.SPELL,
        name: 'Fireball',
        power: 25,
        description: 'A powerful fire spell',
        imageSymbol: '🔥',
      },
      {
        id: 'spell-02',
        type: CardType.SPELL,
        name: 'Ice Storm',
        power: 30,
        description: 'Freeze your enemies',
        imageSymbol: '❄️',
      },
      {
        id: 'spell-03',
        type: CardType.SPELL,
        name: 'Lightning Bolt',
        power: 35,
        description: 'Strike with electricity',
        imageSymbol: '⚡',
      },
      {
        id: 'spell-04',
        type: CardType.SPELL,
        name: 'Healing Light',
        power: 20,
        description: 'Restore health to allies',
        imageSymbol: '✨',
      },
      {
        id: 'spell-05',
        type: CardType.SPELL,
        name: 'Time Warp',
        power: 40,
        description: 'Manipulate time itself',
        imageSymbol: '⏰',
      },
      {
        id: 'spell-06',
        type: CardType.SPELL,
        name: 'Meteor Strike',
        power: 45,
        description: 'Call meteors from the sky',
        imageSymbol: '☄️',
      },
    ];

    // Items
    const items: CardData[] = [
      {
        id: 'item-01',
        type: CardType.ITEM,
        name: 'Health Potion',
        power: 15,
        description: 'Restore 50 health',
        imageSymbol: '🧪',
      },
      {
        id: 'item-02',
        type: CardType.ITEM,
        name: 'Mana Elixir',
        power: 20,
        description: 'Restore 30 mana',
        imageSymbol: '💙',
      },
      {
        id: 'item-03',
        type: CardType.ITEM,
        name: 'Iron Sword',
        power: 25,
        description: 'A sturdy melee weapon',
        imageSymbol: '🗡️',
      },
      {
        id: 'item-04',
        type: CardType.ITEM,
        name: 'Dragon Scale',
        power: 30,
        description: 'Legendary armor piece',
        imageSymbol: '🐉',
      },
      {
        id: 'item-05',
        type: CardType.ITEM,
        name: 'Ancient Amulet',
        power: 35,
        description: 'Grants mystical power',
        imageSymbol: '🔮',
      },
      {
        id: 'item-06',
        type: CardType.ITEM,
        name: 'Treasure Chest',
        power: 50,
        description: 'Contains vast riches',
        imageSymbol: '💎',
      },
    ];

    // Monsters
    const monsters: CardData[] = [
      {
        id: 'monster-01',
        type: CardType.MONSTER,
        name: 'Goblin',
        power: 10,
        description: 'A small green creature',
        imageSymbol: '👹',
      },
      {
        id: 'monster-02',
        type: CardType.MONSTER,
        name: 'Orc',
        power: 15,
        description: 'A fierce green warrior',
        imageSymbol: '🗡️',
      },
      {
        id: 'monster-03',
        type: CardType.MONSTER,
        name: 'Troll',
        power: 20,
        description: 'A massive cave dweller',
        imageSymbol: '👹',
      },
      {
        id: 'monster-04',
        type: CardType.MONSTER,
        name: 'Vampire',
        power: 25,
        description: 'An undead immortal',
        imageSymbol: '🧛',
      },
      {
        id: 'monster-05',
        type: CardType.MONSTER,
        name: 'Dragon',
        power: 35,
        description: 'A mighty flying beast',
        imageSymbol: '🐉',
      },
      {
        id: 'monster-06',
        type: CardType.MONSTER,
        name: 'Ancient Evil',
        power: 50,
        description: 'An evil force from the past',
        imageSymbol: '💀',
      },
    ];

    [...spells, ...items, ...monsters].forEach((card) => {
      this.cardLibrary.set(card.id, card);
    });
  }

  /**
   * Initialize boss encounters
   */
  private initializeBosses(): void {
    this.bosses = [
      {
        name: 'Goblin Chief',
        health: 60,
        maxHealth: 60,
        attack: 10,
        defense: 3,
        requiredMatchedCards: 3,
        difficulty: 1,
        reward: { experience: 150, gold: 75 },
      },
      {
        name: 'Wyvern Lord',
        health: 100,
        maxHealth: 100,
        attack: 15,
        defense: 5,
        requiredMatchedCards: 5,
        difficulty: 2,
        reward: { experience: 250, gold: 125 },
      },
      {
        name: 'Shadow Wizard',
        health: 80,
        maxHealth: 80,
        attack: 18,
        defense: 4,
        requiredMatchedCards: 6,
        difficulty: 2,
        reward: { experience: 300, gold: 150 },
      },
      {
        name: 'Dark Overlord',
        health: 150,
        maxHealth: 150,
        attack: 20,
        defense: 8,
        requiredMatchedCards: 8,
        difficulty: 3,
        reward: { experience: 500, gold: 250 },
      },
      {
        name: 'Ancient Dragon',
        health: 200,
        maxHealth: 200,
        attack: 25,
        defense: 10,
        requiredMatchedCards: 10,
        difficulty: 4,
        reward: { experience: 1000, gold: 500 },
      },
    ];
  }

  /**
   * Initialize story progression
   */
  private initializeStory(): void {
    this.story = [
      'You are awakening in the mystical land of Aethoria...',
      'Choose your class wisely. Warriors are strong, Mages are mystical, Rogues are swift.',
      'Level 1: The Goblin Caves - Face the Goblin Chief and recover the lost artifacts.',
      'Level 2: The Wyvern Peaks - Defeat the Wyvern Lord and claim its treasure.',
      'Level 3: The Shadow Tower - Battle the Shadow Wizard and dispel the darkness.',
      'Level 4: The Dark Kingdom - Confront the Dark Overlord and save the realm.',
      'Level 5: The Ancient Temple - Face the Ancient Dragon and restore peace to the world!',
      'Congratulations! You have saved Aethoria and become a legend!',
    ];
  }

  /**
   * Get game state
   */
  getState(): GameState {
    return {
      ...this.state,
      character: this.state.character ? this.state.character : null,
      board: this.state.board ? this.state.board : null,
      battle: this.state.battle ? this.state.battle : null,
    };
  }

  /**
   * Select character class
   */
  selectCharacter(name: string, characterClass: CharacterClass): void {
    if (this.state.stage !== GameStage.CHARACTER_SELECT) {
      throw new Error('Cannot select character at this game stage');
    }

    this.state.character = new Character(name, characterClass);
    this.state.stage = GameStage.LEVEL_START;
  }

  /**
   * Start a level
   */
  startLevel(): void {
    if (this.state.stage !== GameStage.LEVEL_START || !this.state.character) {
      throw new Error('Cannot start level at this stage');
    }

    // Create board based on level difficulty
    const boardConfig = this.getBoardConfigForLevel(this.state.level);
    this.state.board = new Board(boardConfig);

    // Get cards for this level
    const cards = this.getCardsForLevel(this.state.level, boardConfig);
    this.state.board.initializeBoard(cards);

    this.state.stage = GameStage.PLAYING;
  }

  /**
   * Get board configuration for level
   */
  private getBoardConfigForLevel(level: number): BoardConfig {
    switch (level) {
      case 1:
        return { rows: 2, cols: 2 };
      case 2:
        return { rows: 3, cols: 2 };
      case 3:
        return { rows: 3, cols: 3 };
      case 4:
        return { rows: 4, cols: 3 };
      case 5:
        return { rows: 4, cols: 4 };
      default:
        return { rows: 2, cols: 2 };
    }
  }

  /**
   * Get cards for current level
   */
  private getCardsForLevel(level: number, config: BoardConfig): CardData[] {
    const cards: CardData[] = [];
    const totalCards = config.rows * config.cols;
    const cardIndices = new Map<string, number>();

    // Create pairs
    const selectedIds = Array.from(this.cardLibrary.keys());
    for (let i = 0; i < totalCards / 2; i++) {
      const randomId = selectedIds[Math.floor(Math.random() * selectedIds.length)];
      const cardData = this.cardLibrary.get(randomId);
      if (cardData) {
        cards.push(cardData);
        cards.push({ ...cardData, id: `${cardData.id}-pair-${i}` });
      }
    }

    return cards.slice(0, totalCards);
  }

  /**
   * Check if level is complete
   */
  checkLevelComplete(): boolean {
    if (!this.state.board) {
      return false;
    }
    return this.state.board.isBoardComplete();
  }

  /**
   * Complete current level
   */
  completeLevel(): void {
    if (!this.checkLevelComplete() || !this.state.character) {
      throw new Error('Cannot complete level');
    }

    // Award experience for completing level
    this.state.character.addExperience(50 * this.state.level);

    if (this.state.level >= 5) {
      this.state.stage = GameStage.GAME_OVER;
    } else {
      this.state.level += 1;
      this.state.stage = GameStage.LEVEL_START;
    }
  }

  /**
   * Start boss battle
   */
  startBossBattle(): void {
    if (!this.state.character) {
      throw new Error('No character selected');
    }

    const boss = this.bosses[this.state.level - 1];
    if (!boss) {
      throw new Error(`No boss for level ${this.state.level}`);
    }

    this.state.battle = new Battle(this.state.character, boss);
    this.state.battle.startBattle();
    this.state.stage = GameStage.BATTLE;
  }

  /**
   * Check if player won the game
   */
  hasPlayerWon(): boolean {
    return (
      this.state.stage === GameStage.GAME_OVER &&
      this.state.character !== null &&
      this.state.character.isAlive()
    );
  }

  /**
   * Get game story text
   */
  getStoryText(index: number): string {
    return this.story[Math.min(index, this.story.length - 1)];
  }

  /**
   * Get all bosses
   */
  getBosses(): Enemy[] {
    return [...this.bosses];
  }

  /**
   * Get current boss
   */
  getCurrentBoss(): Enemy | null {
    if (this.state.level - 1 < this.bosses.length) {
      return this.bosses[this.state.level - 1];
    }
    return null;
  }

  /**
   * Helper to auto-complete current level (for testing)
   */
  autoCompleteLevelForTesting(): void {
    if (!this.state.board) {
      throw new Error('No board available');
    }

    const board = this.state.board;
    const boardConfig = this.getBoardConfigForLevel(this.state.level);
    const rows = boardConfig.rows;
    const cols = boardConfig.cols;
    const matched = new Set<string>();

    // Iterate through board positions and match pairs
    for (let i = 0; i < rows; i++) {
      for (let j = 0; j < cols; j++) {
        const card1 = board.getCardAt(i, j);
        if (!card1 || matched.has(card1.getId())) continue;

        // Find matching card
        for (let ii = 0; ii < rows; ii++) {
          for (let jj = 0; jj < cols; jj++) {
            if (i === ii && j === jj) continue;

            const card2 = board.getCardAt(ii, jj);
            if (!card2 || matched.has(card2.getId())) continue;

            if (card2.getName() === card1.getName()) {
              // Match this pair
              board.flipCard(i, j);
              board.flipCard(ii, jj);
              matched.add(card1.getId());
              matched.add(card2.getId());
              break;
            }
          }
          if (matched.has(card1.getId())) break;
        }
      }
    }
  }

  /**
   * Reset game
   */
  reset(): void {
    if (this.state.character) {
      this.state.character.reset();
    }
    this.state.stage = GameStage.CHARACTER_SELECT;
    this.state.level = 1;
    this.state.board = null;
    this.state.battle = null;
    this.state.storyIndex = 0;
  }
}
