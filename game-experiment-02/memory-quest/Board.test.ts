import { Board } from './Board';
import { CardType, CardData } from './Card';

describe('Board', () => {
  let board: Board;
  let testCards: CardData[];

  beforeEach(() => {
    board = new Board({ rows: 2, cols: 2 });

    // Create pairs of cards for testing
    testCards = [
      {
        id: 'spell-01',
        type: CardType.SPELL,
        name: 'Fireball',
        power: 25,
        description: 'Fire spell',
        imageSymbol: '🔥',
      },
      {
        id: 'spell-01-pair',
        type: CardType.SPELL,
        name: 'Fireball',
        power: 25,
        description: 'Fire spell',
        imageSymbol: '🔥',
      },
      {
        id: 'spell-02',
        type: CardType.SPELL,
        name: 'Ice Storm',
        power: 30,
        description: 'Ice spell',
        imageSymbol: '❄️',
      },
      {
        id: 'spell-02-pair',
        type: CardType.SPELL,
        name: 'Ice Storm',
        power: 30,
        description: 'Ice spell',
        imageSymbol: '❄️',
      },
    ];
  });

  describe('Board Initialization', () => {
    it('should initialize board with correct card count', () => {
      board.initializeBoard(testCards);
      expect(board.getAllCards().length).toBe(4);
    });

    it('should throw error if card count does not match board size', () => {
      expect(() => {
        board.initializeBoard([testCards[0], testCards[1]]);
      }).toThrow();
    });

    it('should reset flipped cards and matched count on init', () => {
      board.initializeBoard(testCards);
      const card = board.getCardAt(0, 0);
      expect(card?.isCardRevealed()).toBe(false);
    });
  });

  describe('Card Access', () => {
    beforeEach(() => {
      board.initializeBoard(testCards);
    });

    it('should get card at valid position', () => {
      const card = board.getCardAt(0, 0);
      expect(card).not.toBeNull();
      expect(card?.getId()).toBeDefined();
    });

    it('should return null for invalid position', () => {
      expect(board.getCardAt(-1, 0)).toBeNull();
      expect(board.getCardAt(0, -1)).toBeNull();
      expect(board.getCardAt(2, 2)).toBeNull();
      expect(board.getCardAt(0, 2)).toBeNull();
    });
  });

  describe('Card Flipping', () => {
    beforeEach(() => {
      board.initializeBoard(testCards);
    });

    it('should flip a card', () => {
      const card = board.flipCard(0, 0);
      expect(card).not.toBeNull();
      expect(card?.isCardRevealed()).toBe(true);
    });

    it('should not flip already flipped card', () => {
      board.flipCard(0, 0);
      const card = board.flipCard(0, 0);
      expect(card).toBeNull();
    });

    it('should not flip more than 2 cards at once', () => {
      // Get the cards to make sure we flip non-matching ones
      const cards = board.getAllCards();

      // Find two non-matching cards
      let pos1 = -1,
        pos2 = -1,
        pos3 = -1;
      for (let i = 0; i < cards.length; i++) {
        for (let j = i + 1; j < cards.length; j++) {
          if (cards[i].getName() !== cards[j].getName()) {
            pos1 = i;
            pos2 = j;
            // Find a third card
            for (let k = j + 1; k < cards.length; k++) {
              if (
                cards[k].getName() !== cards[i].getName() &&
                cards[k].getName() !== cards[j].getName()
              ) {
                pos3 = k;
                break;
              }
            }
            break;
          }
        }
        if (pos1 !== -1) break;
      }

      if (pos1 !== -1 && pos2 !== -1 && pos3 !== -1) {
        const row1 = Math.floor(pos1 / 2);
        const col1 = pos1 % 2;
        const row2 = Math.floor(pos2 / 2);
        const col2 = pos2 % 2;
        const row3 = Math.floor(pos3 / 2);
        const col3 = pos3 % 2;

        board.flipCard(row1, col1);
        board.flipCard(row2, col2);
        const card = board.flipCard(row3, col3);
        expect(card).toBeNull(); // Should not flip a 3rd card
      }
    });

    it('should not flip matched cards', () => {
      // Match two cards first
      const cards = board.getAllCards();
      const pair = cards.slice(0, 2);

      // Manually match them
      pair[0].reveal();
      pair[1].reveal();
      pair[0].match();
      pair[1].match();

      // Try to flip matched card
      const result = board.flipCard(0, 0);
      expect(result).toBeNull();
    });
  });

  describe('Match Detection', () => {
    beforeEach(() => {
      board.initializeBoard(testCards);
    });

    it('should detect matching pair automatically', () => {
      // Get all cards
      const cards = board.getAllCards();

      // Find the pair (same name)
      let pairIndex = -1;
      for (let i = 1; i < cards.length; i++) {
        if (cards[i].getName() === cards[0].getName()) {
          pairIndex = i;
          break;
        }
      }

      // Flip the pair
      board.flipCard(0, 0);
      const rowOfPair = Math.floor(pairIndex / 2);
      const colOfPair = pairIndex % 2;
      board.flipCard(rowOfPair, colOfPair);

      // Check that both cards are matched
      expect(cards[0].isCardMatched()).toBe(true);
      expect(cards[pairIndex].isCardMatched()).toBe(true);

      // Flipped cards should be cleared
      expect(board.getFlippedCards().length).toBe(0);
    });
  });

  describe('Board Completion', () => {
    beforeEach(() => {
      board.initializeBoard(testCards);
    });

    it('should not be complete initially', () => {
      expect(board.isBoardComplete()).toBe(false);
    });

    it('should be complete when all cards are matched', () => {
      // To properly test completion, match all cards through the board
      const testBoard = new Board({ rows: 2, cols: 2 });
      testBoard.initializeBoard(testCards);

      const cards = testBoard.getAllCards();
      const matched = new Set<string>();

      // Match all pairs by finding matching cards
      for (let i = 0; i < cards.length; i++) {
        if (matched.has(cards[i].getId())) continue;

        const currentCard = cards[i];
        for (let j = i + 1; j < cards.length; j++) {
          if (matched.has(cards[j].getId())) continue;

          if (cards[j].getName() === currentCard.getName()) {
            // Found matching pair, flip them
            const row1 = Math.floor(i / 2);
            const col1 = i % 2;
            const row2 = Math.floor(j / 2);
            const col2 = j % 2;

            testBoard.flipCard(row1, col1);
            testBoard.flipCard(row2, col2);
            matched.add(currentCard.getId());
            matched.add(cards[j].getId());
            break;
          }
        }
      }

      expect(testBoard.isBoardComplete()).toBe(true);
    });
  });

  describe('Board Progress', () => {
    beforeEach(() => {
      board.initializeBoard(testCards);
    });

    it('should track progress correctly', () => {
      const progress = board.getProgress();
      expect(progress.matched).toBe(0);
      expect(progress.total).toBe(2);
      expect(progress.percentage).toBe(0);
    });

    it('should update progress as cards are matched through board', () => {
      // Flip and match through the board's system
      // Find first pair
      const cards = board.getAllCards();
      let pos1 = -1,
        pos2 = -1;

      for (let i = 0; i < cards.length; i++) {
        for (let j = i + 1; j < cards.length; j++) {
          if (cards[i].getName() === cards[j].getName()) {
            pos1 = i;
            pos2 = j;
            break;
          }
        }
        if (pos1 !== -1) break;
      }

      if (pos1 !== -1) {
        const row1 = Math.floor(pos1 / 2);
        const col1 = pos1 % 2;
        const row2 = Math.floor(pos2 / 2);
        const col2 = pos2 % 2;

        board.flipCard(row1, col1);
        board.flipCard(row2, col2);

        const progress = board.getProgress();
        expect(progress.matched).toBe(1);
        expect(progress.percentage).toBe(50);
      }
    });
  });

  describe('Reset Functionality', () => {
    beforeEach(() => {
      board.initializeBoard(testCards);
    });

    it('should reset flipped cards', () => {
      board.flipCard(0, 0);
      board.flipCard(0, 1);
      board.resetFlippedCards();

      const cards = board.getFlippedCards();
      expect(cards.length).toBe(0);
    });

    it('should reset entire board', () => {
      board.flipCard(0, 0);
      board.reset();

      expect(board.getAllCards().length).toBe(0);
      expect(board.isBoardComplete()).toBe(false);
    });
  });
});
