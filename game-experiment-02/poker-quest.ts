// Poker Quest - Roguelike with Poker Hands Combat
// A game where you draw poker hands to defeat enemies

// ============= ENUMS & TYPES =============

type CardSuit = "♥" | "♦" | "♣" | "♠";
type CardRank = "A" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "J" | "Q" | "K";

enum HandType {
  ROYAL_FLUSH = "Royal Flush",
  STRAIGHT_FLUSH = "Straight Flush",
  FOUR_OF_A_KIND = "Four of a Kind",
  FULL_HOUSE = "Full House",
  FLUSH = "Flush",
  STRAIGHT = "Straight",
  THREE_OF_A_KIND = "Three of a Kind",
  TWO_PAIR = "Two Pair",
  PAIR = "Pair",
  HIGH_CARD = "High Card",
  INVALID = "Invalid",
}

enum GameState {
  TITLE = "TITLE",
  PLAYING = "PLAYING",
  ENCOUNTER = "ENCOUNTER",
  SHOP = "SHOP",
  BOSS = "BOSS",
  WON = "WON",
  LOST = "LOST",
}

interface Card {
  suit: CardSuit;
  rank: CardRank;
  id: string;
}

interface Hand {
  type: HandType;
  power: number;
  cards: Card[];
}

interface Player {
  health: number;
  maxHealth: number;
  deck: Card[];
  hand: Card[];
  gold: number;
  wins: number;
  level: number;
}

interface Enemy {
  id: string;
  name: string;
  health: number;
  maxHealth: number;
  deck: Card[];
  hand: Card[];
  hand_eval: Hand;
  ai_type: "aggressive" | "defensive" | "balanced" | "boss";
}

interface Upgrade {
  name: string;
  description: string;
  cost: number;
  effect: (player: Player) => void;
}

// ============= POKER HAND EVALUATION =============

function rankValue(rank: CardRank): number {
  const values: Record<string, number> = {
    A: 14,
    K: 13,
    Q: 12,
    J: 11,
    10: 10,
    9: 9,
    8: 8,
    7: 7,
    6: 6,
    5: 5,
    4: 4,
    3: 3,
    2: 2,
  };
  return values[rank];
}

function evaluateHand(cards: Card[]): Hand {
  if (cards.length < 5) {
    return { type: HandType.INVALID, power: 0, cards };
  }

  const hand = cards.slice(0, 5);
  const ranks = hand.map((c) => rankValue(c.rank)).sort((a, b) => b - a);
  const suits = hand.map((c) => c.suit);
  const rankCounts = new Map<number, number>();

  ranks.forEach((r) => {
    rankCounts.set(r, (rankCounts.get(r) || 0) + 1);
  });

  const counts = Array.from(rankCounts.values()).sort((a, b) => b - a);
  const isFlush = suits.every((s) => s === suits[0]);

  // Check for straight
  const uniqueRanks = Array.from(new Set(ranks)).sort((a, b) => b - a);
  let isStraight = false;
  let straightHigh = 0;

  if (uniqueRanks.length === 5) {
    if (uniqueRanks[0] - uniqueRanks[4] === 4) {
      isStraight = true;
      straightHigh = uniqueRanks[0];
    } else if (
      uniqueRanks[0] === 14 &&
      uniqueRanks[4] === 2 &&
      uniqueRanks[1] === 5
    ) {
      // Ace-low straight
      isStraight = true;
      straightHigh = 5;
    }
  }

  let handType: HandType = HandType.HIGH_CARD;
  let power = ranks[0];

  // Determine hand type
  if (isStraight && isFlush && straightHigh === 14) {
    handType = HandType.ROYAL_FLUSH;
    power = 1000000;
  } else if (isStraight && isFlush) {
    handType = HandType.STRAIGHT_FLUSH;
    power = 900000 + straightHigh;
  } else if (counts[0] === 4) {
    handType = HandType.FOUR_OF_A_KIND;
    const firstKey = Array.from(rankCounts.keys())[0];
    power = 800000 + (firstKey || 0);
  } else if (counts[0] === 3 && counts[1] === 2) {
    handType = HandType.FULL_HOUSE;
    power = 700000 + ranks[0];
  } else if (isFlush) {
    handType = HandType.FLUSH;
    power = 600000 + ranks[0];
  } else if (isStraight) {
    handType = HandType.STRAIGHT;
    power = 500000 + straightHigh;
  } else if (counts[0] === 3) {
    handType = HandType.THREE_OF_A_KIND;
    power = 400000 + ranks[0];
  } else if (counts[0] === 2 && counts[1] === 2) {
    handType = HandType.TWO_PAIR;
    power = 300000 + Math.max(...Array.from(rankCounts.keys()));
  } else if (counts[0] === 2) {
    handType = HandType.PAIR;
    power = 200000 + ranks[0];
  } else {
    handType = HandType.HIGH_CARD;
    power = 100000 + ranks[0];
  }

  return { type: handType, power, cards: hand };
}

function getHandDamage(hand: Hand): number {
  const damageMap: Record<HandType, number> = {
    [HandType.ROYAL_FLUSH]: 100,
    [HandType.STRAIGHT_FLUSH]: 80,
    [HandType.FOUR_OF_A_KIND]: 60,
    [HandType.FULL_HOUSE]: 50,
    [HandType.FLUSH]: 35,
    [HandType.STRAIGHT]: 30,
    [HandType.THREE_OF_A_KIND]: 25,
    [HandType.TWO_PAIR]: 15,
    [HandType.PAIR]: 8,
    [HandType.HIGH_CARD]: 2,
    [HandType.INVALID]: 0,
  };

  return damageMap[hand.type];
}

// ============= CARD MANAGEMENT =============

function createDeck(): Card[] {
  const deck: Card[] = [];
  const ranks: CardRank[] = ["A", "K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2"];
  const suits: CardSuit[] = ["♥", "♦", "♣", "♠"];

  for (const rank of ranks) {
    for (const suit of suits) {
      deck.push({
        suit,
        rank,
        id: `${rank}${suit}`,
      });
    }
  }

  return deck.sort(() => Math.random() - 0.5);
}

function drawCards(deck: Card[], count: number): Card[] {
  const drawn: Card[] = [];
  for (let i = 0; i < count && deck.length > 0; i++) {
    const card = deck.pop();
    if (card) drawn.push(card);
  }
  return drawn;
}

// ============= GAME LOGIC =============

class PokerQuestGame {
  player: Player;
  enemy: Enemy | null = null;
  gameState: GameState = GameState.TITLE;
  encounters: number = 0;
  maxEncounters: number = 10;
  shop: Upgrade[] = [];
  selectedCards: number[] = [];
  log: string[] = [];

  constructor() {
    this.player = {
      health: 100,
      maxHealth: 100,
      deck: createDeck(),
      hand: [],
      gold: 0,
      wins: 0,
      level: 1,
    };

    this.initializeShop();
  }

  initializeShop(): void {
    this.shop = [
      {
        name: "Health Boost",
        description: "+20 Max Health",
        cost: 50,
        effect: (p) => {
          p.maxHealth += 20;
          p.health = p.maxHealth;
        },
      },
      {
        name: "Extra Card",
        description: "Draw 6 cards instead of 5",
        cost: 75,
        effect: (p) => {
          // This would need special handling in combat
        },
      },
      {
        name: "Gold Multiplier",
        description: "Earn 50% more gold",
        cost: 100,
        effect: (p) => {
          // Applied during reward phase
        },
      },
      {
        name: "Refresh Deck",
        description: "Refill your deck with fresh cards",
        cost: 30,
        effect: (p) => {
          p.deck = createDeck();
        },
      },
    ];
  }

  startNewRun(): void {
    this.player = {
      health: 100,
      maxHealth: 100,
      deck: createDeck(),
      hand: [],
      gold: 0,
      wins: 0,
      level: 1,
    };
    this.encounters = 0;
    this.selectedCards = [];
    this.log = ["New run started!"];
    this.gameState = GameState.ENCOUNTER;
    this.generateEncounter();
  }

  generateEncounter(): void {
    this.encounters++;
    const isBoss = this.encounters === this.maxEncounters;
    const enemyTypes = [
      { name: "Goblin", ai: "aggressive" as const },
      { name: "Orc", ai: "balanced" as const },
      { name: "Troll", ai: "defensive" as const },
      { name: "Wizard", ai: "balanced" as const },
      { name: "Dark Knight", ai: "aggressive" as const },
    ];

    const type = isBoss
      ? { name: "Dragon Lord", ai: "boss" as const }
      : enemyTypes[Math.floor(Math.random() * enemyTypes.length)];

    const health = isBoss ? 150 : 50 + this.encounters * 10;

    this.enemy = {
      id: `enemy_${this.encounters}`,
      name: type.name,
      health,
      maxHealth: health,
      deck: createDeck(),
      hand: [],
      hand_eval: { type: HandType.INVALID, power: 0, cards: [] },
      ai_type: type.ai,
    };

    this.player.hand = drawCards(this.player.deck, 5);
    this.selectedCards = [];
    this.log = [
      `Encountered: ${this.enemy.name} (Encounter ${this.encounters}/${this.maxEncounters})`,
      `Enemy HP: ${this.enemy.maxHealth}`,
    ];

    if (this.encounters === 1) {
      this.log.push("Click 5 cards to play them against the enemy!");
    }

    this.gameState = GameState.ENCOUNTER;
  }

  selectCard(index: number): void {
    if (
      this.gameState !== GameState.ENCOUNTER ||
      !this.enemy ||
      index < 0 ||
      index >= this.player.hand.length
    ) {
      return;
    }

    if (this.selectedCards.includes(index)) {
      this.selectedCards = this.selectedCards.filter((i) => i !== index);
    } else if (this.selectedCards.length < 5) {
      this.selectedCards.push(index);
    }
  }

  executeAttack(): void {
    if (!this.enemy || this.selectedCards.length === 0) {
      this.log.push("Error: Invalid attack");
      return;
    }

    // Evaluate player's hand
    const playerCards = this.selectedCards.map((i) => this.player.hand[i]);
    const playerHand = evaluateHand(playerCards);
    const playerDamage = getHandDamage(playerHand);

    // Enemy AI chooses their hand
    const enemyCards = drawCards(this.enemy.deck, 5);
    const enemyHand = evaluateHand(enemyCards);
    const enemyDamage = getHandDamage(enemyHand);

    this.log = [
      `Player: ${playerHand.type} - ${playerDamage} damage`,
      `Enemy: ${enemyHand.type} - ${enemyDamage} damage`,
    ];

    // Resolve combat
    this.enemy.health -= playerDamage;
    this.player.health -= enemyDamage;

    if (this.enemy.health <= 0) {
      this.defeatEnemy();
    } else if (this.player.health <= 0) {
      this.loseGame();
    } else {
      // Continue combat
      this.selectedCards = [];
      this.player.hand = drawCards(this.player.deck, 5);
      this.log.push(
        `Your health: ${this.player.health}/${this.player.maxHealth}`
      );
      this.log.push(`Enemy health: ${this.enemy.health}/${this.enemy.maxHealth}`
      );
    }
  }

  defeatEnemy(): void {
    if (!this.enemy) return;

    const goldReward = 10 + this.encounters * 5;
    this.player.gold += goldReward;
    this.player.wins++;

    this.log.push(`Victory! Earned ${goldReward} gold`);

    if (this.encounters >= this.maxEncounters) {
      this.gameState = GameState.WON;
      this.log.push("You defeated all enemies!");
    } else {
      this.gameState = GameState.SHOP;
      this.log.push("Visit the shop to buy upgrades!");
    }
  }

  loseGame(): void {
    this.gameState = GameState.LOST;
    this.log.push(`Game Over! You won ${this.player.wins} encounters`);
  }

  buyUpgrade(index: number): void {
    const upgrade = this.shop[index];
    if (this.player.gold >= upgrade.cost) {
      this.player.gold -= upgrade.cost;
      upgrade.effect(this.player);
      this.log = [
        `Purchased: ${upgrade.name}`,
        `Gold remaining: ${this.player.gold}`,
      ];
      setTimeout(() => {
        this.generateEncounter();
      }, 1000);
    } else {
      this.log = [`Not enough gold for ${upgrade.name}`];
    }
  }

  getGameStatus(): string {
    return `Health: ${this.player.health}/${this.player.maxHealth} | Encounter: ${this.encounters}/${this.maxEncounters} | Gold: ${this.player.gold}`;
  }
}

// Export for testing
export {
  PokerQuestGame,
  evaluateHand,
  getHandDamage,
  createDeck,
  drawCards,
  rankValue,
  HandType,
  GameState,
  Card,
  Hand,
};
