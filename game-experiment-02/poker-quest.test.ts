// Poker Quest - Comprehensive Test Suite

import {
  PokerQuestGame,
  evaluateHand,
  getHandDamage,
  createDeck,
  drawCards,
  rankValue,
  HandType,
  GameState,
  Card,
} from "./poker-quest";

// ============= UTILITY FUNCTION TESTS =============

describe("rankValue", () => {
  test("should return correct numeric values for ranks", () => {
    expect(rankValue("A" as any)).toBe(14);
    expect(rankValue("K" as any)).toBe(13);
    expect(rankValue("Q" as any)).toBe(12);
    expect(rankValue("J" as any)).toBe(11);
    expect(rankValue("10" as any)).toBe(10);
    expect(rankValue("2" as any)).toBe(2);
  });
});

describe("createDeck", () => {
  test("should create a valid 52-card deck", () => {
    const deck = createDeck();
    expect(deck.length).toBe(52);
  });

  test("all cards should be unique", () => {
    const deck = createDeck();
    const ids = new Set(deck.map((c) => c.id));
    expect(ids.size).toBe(52);
  });

  test("should contain all suits", () => {
    const deck = createDeck();
    const suits = new Set(deck.map((c) => c.suit));
    expect(suits.size).toBe(4);
  });

  test("should contain all ranks", () => {
    const deck = createDeck();
    const ranks = new Set(deck.map((c) => c.rank));
    expect(ranks.size).toBe(13);
  });
});

describe("drawCards", () => {
  test("should draw correct number of cards", () => {
    const deck = createDeck();
    const drawn = drawCards(deck, 5);
    expect(drawn.length).toBe(5);
    expect(deck.length).toBe(47);
  });

  test("should remove cards from deck", () => {
    const deck = createDeck();
    const initialLength = deck.length;
    drawCards(deck, 3);
    expect(deck.length).toBe(initialLength - 3);
  });

  test("should handle drawing more cards than available", () => {
    const deck = createDeck();
    const drawn = drawCards(deck, 100);
    expect(drawn.length).toBe(52);
  });
});

// ============= POKER HAND EVALUATION TESTS =============

describe("evaluateHand - Straight Flush", () => {
  test("should recognize a straight flush", () => {
    const cards: Card[] = [
      { suit: "♥", rank: "5", id: "5♥" },
      { suit: "♥", rank: "6", id: "6♥" },
      { suit: "♥", rank: "7", id: "7♥" },
      { suit: "♥", rank: "8", id: "8♥" },
      { suit: "♥", rank: "9", id: "9♥" },
    ];
    const hand = evaluateHand(cards);
    expect(hand.type).toBe(HandType.STRAIGHT_FLUSH);
  });

  test("should recognize a royal flush", () => {
    const cards: Card[] = [
      { suit: "♠", rank: "10", id: "10♠" },
      { suit: "♠", rank: "J", id: "J♠" },
      { suit: "♠", rank: "Q", id: "Q♠" },
      { suit: "♠", rank: "K", id: "K♠" },
      { suit: "♠", rank: "A", id: "A♠" },
    ];
    const hand = evaluateHand(cards);
    expect(hand.type).toBe(HandType.ROYAL_FLUSH);
  });
});

describe("evaluateHand - Four of a Kind", () => {
  test("should recognize four of a kind", () => {
    const cards: Card[] = [
      { suit: "♥", rank: "K", id: "K♥" },
      { suit: "♦", rank: "K", id: "K♦" },
      { suit: "♣", rank: "K", id: "K♣" },
      { suit: "♠", rank: "K", id: "K♠" },
      { suit: "♥", rank: "5", id: "5♥" },
    ];
    const hand = evaluateHand(cards);
    expect(hand.type).toBe(HandType.FOUR_OF_A_KIND);
  });
});

describe("evaluateHand - Full House", () => {
  test("should recognize a full house", () => {
    const cards: Card[] = [
      { suit: "♥", rank: "K", id: "K♥" },
      { suit: "♦", rank: "K", id: "K♦" },
      { suit: "♣", rank: "K", id: "K♣" },
      { suit: "♠", rank: "5", id: "5♠" },
      { suit: "♥", rank: "5", id: "5♥" },
    ];
    const hand = evaluateHand(cards);
    expect(hand.type).toBe(HandType.FULL_HOUSE);
  });
});

describe("evaluateHand - Flush", () => {
  test("should recognize a flush", () => {
    const cards: Card[] = [
      { suit: "♦", rank: "2", id: "2♦" },
      { suit: "♦", rank: "5", id: "5♦" },
      { suit: "♦", rank: "9", id: "9♦" },
      { suit: "♦", rank: "J", id: "J♦" },
      { suit: "♦", rank: "K", id: "K♦" },
    ];
    const hand = evaluateHand(cards);
    expect(hand.type).toBe(HandType.FLUSH);
  });
});

describe("evaluateHand - Straight", () => {
  test("should recognize a straight", () => {
    const cards: Card[] = [
      { suit: "♥", rank: "5", id: "5♥" },
      { suit: "♦", rank: "6", id: "6♦" },
      { suit: "♣", rank: "7", id: "7♣" },
      { suit: "♠", rank: "8", id: "8♠" },
      { suit: "♥", rank: "9", id: "9♥" },
    ];
    const hand = evaluateHand(cards);
    expect(hand.type).toBe(HandType.STRAIGHT);
  });

  test("should recognize ace-low straight", () => {
    const cards: Card[] = [
      { suit: "♥", rank: "A", id: "A♥" },
      { suit: "♦", rank: "2", id: "2♦" },
      { suit: "♣", rank: "3", id: "3♣" },
      { suit: "♠", rank: "4", id: "4♠" },
      { suit: "♥", rank: "5", id: "5♥" },
    ];
    const hand = evaluateHand(cards);
    expect(hand.type).toBe(HandType.STRAIGHT);
  });
});

describe("evaluateHand - Three of a Kind", () => {
  test("should recognize three of a kind", () => {
    const cards: Card[] = [
      { suit: "♥", rank: "Q", id: "Q♥" },
      { suit: "♦", rank: "Q", id: "Q♦" },
      { suit: "♣", rank: "Q", id: "Q♣" },
      { suit: "♠", rank: "3", id: "3♠" },
      { suit: "♥", rank: "7", id: "7♥" },
    ];
    const hand = evaluateHand(cards);
    expect(hand.type).toBe(HandType.THREE_OF_A_KIND);
  });
});

describe("evaluateHand - Two Pair", () => {
  test("should recognize two pair", () => {
    const cards: Card[] = [
      { suit: "♥", rank: "10", id: "10♥" },
      { suit: "♦", rank: "10", id: "10♦" },
      { suit: "♣", rank: "6", id: "6♣" },
      { suit: "♠", rank: "6", id: "6♠" },
      { suit: "♥", rank: "K", id: "K♥" },
    ];
    const hand = evaluateHand(cards);
    expect(hand.type).toBe(HandType.TWO_PAIR);
  });
});

describe("evaluateHand - Pair", () => {
  test("should recognize a pair", () => {
    const cards: Card[] = [
      { suit: "♥", rank: "9", id: "9♥" },
      { suit: "♦", rank: "9", id: "9♦" },
      { suit: "♣", rank: "2", id: "2♣" },
      { suit: "♠", rank: "5", id: "5♠" },
      { suit: "♥", rank: "K", id: "K♥" },
    ];
    const hand = evaluateHand(cards);
    expect(hand.type).toBe(HandType.PAIR);
  });
});

describe("evaluateHand - High Card", () => {
  test("should recognize high card", () => {
    const cards: Card[] = [
      { suit: "♥", rank: "2", id: "2♥" },
      { suit: "♦", rank: "5", id: "5♦" },
      { suit: "♣", rank: "7", id: "7♣" },
      { suit: "♠", rank: "9", id: "9♠" },
      { suit: "♥", rank: "K", id: "K♥" },
    ];
    const hand = evaluateHand(cards);
    expect(hand.type).toBe(HandType.HIGH_CARD);
  });
});

// ============= HAND DAMAGE TESTS =============

describe("getHandDamage", () => {
  test("should return correct damage for each hand type", () => {
    expect(
      getHandDamage({
        type: HandType.ROYAL_FLUSH,
        power: 1000000,
        cards: [],
      })
    ).toBe(100);
    expect(
      getHandDamage({
        type: HandType.STRAIGHT_FLUSH,
        power: 900000,
        cards: [],
      })
    ).toBe(80);
    expect(
      getHandDamage({
        type: HandType.FOUR_OF_A_KIND,
        power: 800000,
        cards: [],
      })
    ).toBe(60);
    expect(
      getHandDamage({
        type: HandType.FULL_HOUSE,
        power: 700000,
        cards: [],
      })
    ).toBe(50);
    expect(
      getHandDamage({
        type: HandType.FLUSH,
        power: 600000,
        cards: [],
      })
    ).toBe(35);
    expect(
      getHandDamage({ type: HandType.STRAIGHT, power: 500000, cards: [] })
    ).toBe(30);
    expect(
      getHandDamage({
        type: HandType.THREE_OF_A_KIND,
        power: 400000,
        cards: [],
      })
    ).toBe(25);
    expect(
      getHandDamage({ type: HandType.TWO_PAIR, power: 300000, cards: [] })
    ).toBe(15);
    expect(
      getHandDamage({ type: HandType.PAIR, power: 200000, cards: [] })
    ).toBe(8);
    expect(
      getHandDamage({
        type: HandType.HIGH_CARD,
        power: 100000,
        cards: [],
      })
    ).toBe(2);
  });

  test("should return 0 damage for invalid hand", () => {
    expect(
      getHandDamage({ type: HandType.INVALID, power: 0, cards: [] })
    ).toBe(0);
  });
});

// ============= GAME LOGIC TESTS =============

describe("PokerQuestGame - Initialization", () => {
  test("should initialize with default values", () => {
    const game = new PokerQuestGame();
    expect(game.player.health).toBe(100);
    expect(game.player.maxHealth).toBe(100);
    expect(game.player.gold).toBe(0);
    expect(game.player.wins).toBe(0);
    expect(game.gameState).toBe(GameState.TITLE);
    expect(game.player.deck.length).toBe(52);
  });

  test("should initialize shop with upgrades", () => {
    const game = new PokerQuestGame();
    expect(game.shop.length).toBeGreaterThan(0);
    expect(game.shop[0].name).toBeTruthy();
    expect(game.shop[0].cost).toBeGreaterThan(0);
  });
});

describe("PokerQuestGame - Game Flow", () => {
  test("should start new run correctly", () => {
    const game = new PokerQuestGame();
    game.startNewRun();

    expect(game.gameState).toBe(GameState.ENCOUNTER);
    expect(game.encounters).toBe(1);
    expect(game.player.hand.length).toBe(5);
    expect(game.enemy).toBeTruthy();
  });

  test("should generate encounters with increasing difficulty", () => {
    const game = new PokerQuestGame();
    game.startNewRun();

    const firstEnemyHealth = game.enemy?.maxHealth || 0;
    game.encounters++;
    game.generateEncounter();
    const secondEnemyHealth = game.enemy?.maxHealth || 0;

    expect(secondEnemyHealth).toBeGreaterThanOrEqual(firstEnemyHealth);
  });

  test("should track enemy encounters", () => {
    const game = new PokerQuestGame();
    game.startNewRun();

    for (let i = 1; i < 5; i++) {
      expect(game.encounters).toBe(i);
      game.enemy!.health = -1; // Force defeat
      game.defeatEnemy();

      if (i < 4) {
        game.gameState = GameState.ENCOUNTER;
        game.generateEncounter();
      }
    }
  });
});

describe("PokerQuestGame - Card Selection", () => {
  test("should select cards correctly", () => {
    const game = new PokerQuestGame();
    game.startNewRun();

    game.selectCard(0);
    expect(game.selectedCards).toContain(0);

    game.selectCard(1);
    expect(game.selectedCards).toContain(1);
  });

  test("should deselect cards", () => {
    const game = new PokerQuestGame();
    game.startNewRun();

    game.selectCard(0);
    expect(game.selectedCards).toContain(0);

    game.selectCard(0); // Deselect
    expect(game.selectedCards).not.toContain(0);
  });

  test("should limit to 5 card selection", () => {
    const game = new PokerQuestGame();
    game.startNewRun();

    for (let i = 0; i < 6; i++) {
      game.selectCard(i);
    }

    expect(game.selectedCards.length).toBe(5);
  });
});

describe("PokerQuestGame - Combat", () => {
  test("should execute attack and deal damage", () => {
    const game = new PokerQuestGame();
    game.startNewRun();

    if (game.enemy) {
      const initialEnemyHealth = game.enemy.health;
      game.selectedCards = [0, 1, 2, 3, 4]; // Select 5 cards

      game.executeAttack();

      expect(game.selectedCards.length).toBe(0); // Should clear selection
      expect(game.enemy.health).not.toBe(initialEnemyHealth);
    }
  });

  test("should defeat enemy when health reaches 0", () => {
    const game = new PokerQuestGame();
    game.startNewRun();

    if (game.enemy) {
      game.enemy.health = 10; // Set low health
      game.selectedCards = [0, 1, 2, 3, 4];
      game.executeAttack();

      if (game.enemy.health <= 0) {
        expect(game.gameState).not.toBe(GameState.ENCOUNTER);
      }
    }
  });

  test("should lose game when player health reaches 0", () => {
    const game = new PokerQuestGame();
    game.startNewRun();

    game.player.health = 10;
    game.selectedCards = [0, 1, 2, 3, 4];
    game.executeAttack();

    if (game.player.health <= 0) {
      expect(game.gameState).toBe(GameState.LOST);
    }
  });
});

describe("PokerQuestGame - Shop", () => {
  test("should allow purchase with sufficient gold", () => {
    const game = new PokerQuestGame();
    game.startNewRun();
    game.player.gold = 100;

    const upgrade = game.shop[0];
    const initialMaxHealth = game.player.maxHealth;

    if (upgrade.name === "Health Boost") {
      game.buyUpgrade(0);
      expect(game.player.maxHealth).toBeGreaterThan(initialMaxHealth);
      expect(game.player.gold).toBeLessThan(100);
    }
  });

  test("should reject purchase with insufficient gold", () => {
    const game = new PokerQuestGame();
    game.startNewRun();
    game.player.gold = 10;

    const upgrade = game.shop[0];
    const initialGold = game.player.gold;

    if (upgrade.cost > initialGold) {
      game.buyUpgrade(0);
      expect(game.player.gold).toBe(initialGold);
    }
  });
});

describe("PokerQuestGame - Status", () => {
  test("should provide current game status", () => {
    const game = new PokerQuestGame();
    game.startNewRun();

    const status = game.getGameStatus();
    expect(status).toContain("Health:");
    expect(status).toContain("Encounter:");
    expect(status).toContain("Gold:");
  });
});
