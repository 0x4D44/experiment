/**
 * Board class manages the game board with cards
 * Handles card layout, flipping, and match detection
 */

import { Card, CardData, CardType } from './Card';

export interface BoardConfig {
  rows: number;
  cols: number;
}

export class Board {
  private cards: Card[] = [];
  private rows: number;
  private cols: number;
  private flippedCards: Card[] = [];
  private matchedCount: number = 0;

  constructor(config: BoardConfig) {
    this.rows = config.rows;
    this.cols = config.cols;
  }

  /**
   * Initialize board with card data
   */
  initializeBoard(cardDataArray: CardData[]): void {
    if (cardDataArray.length !== this.rows * this.cols) {
      throw new Error(
        `Card count ${cardDataArray.length} does not match board size ${this.rows * this.cols}`
      );
    }

    // Shuffle and create cards
    const shuffledData = this.shuffle([...cardDataArray]);
    this.cards = shuffledData.map((data) => new Card(data));
    this.flippedCards = [];
    this.matchedCount = 0;
  }

  /**
   * Get card at position
   */
  getCardAt(row: number, col: number): Card | null {
    if (row < 0 || row >= this.rows || col < 0 || col >= this.cols) {
      return null;
    }
    return this.cards[row * this.cols + col];
  }

  /**
   * Flip a card and return if it was successfully flipped
   */
  flipCard(row: number, col: number): Card | null {
    const card = this.getCardAt(row, col);
    if (!card) {
      return null;
    }

    // Can't flip matched cards or more than 2 cards
    if (card.isCardMatched() || this.flippedCards.length >= 2) {
      return null;
    }

    // Can't flip already flipped cards
    if (card.isCardRevealed()) {
      return null;
    }

    card.reveal();
    this.flippedCards.push(card);

    // Auto-check for match if 2 cards flipped
    if (this.flippedCards.length === 2) {
      this.checkMatch();
    }

    return card;
  }

  /**
   * Check if flipped cards match
   */
  private checkMatch(): void {
    if (this.flippedCards.length !== 2) {
      return;
    }

    const [card1, card2] = this.flippedCards;

    // Match if same name (pairs)
    if (card1.getName() === card2.getName()) {
      card1.match();
      card2.match();
      this.matchedCount += 2;
      this.flippedCards = [];
    }
  }

  /**
   * Manually reset flipped cards (if no match)
   */
  resetFlippedCards(): void {
    this.flippedCards.forEach((card) => card.reset());
    this.flippedCards = [];
  }

  /**
   * Get all flipped cards (for display purposes)
   */
  getFlippedCards(): Card[] {
    return [...this.flippedCards];
  }

  /**
   * Check if board is complete (all cards matched)
   */
  isBoardComplete(): boolean {
    const totalCards = this.rows * this.cols;
    return this.cards.length > 0 && this.matchedCount === totalCards;
  }

  /**
   * Get board progress
   */
  getProgress(): { matched: number; total: number; percentage: number } {
    const total = this.rows * this.cols;
    return {
      matched: this.matchedCount / 2,
      total: total / 2,
      percentage: (this.matchedCount / total) * 100,
    };
  }

  /**
   * Get all cards (for external access)
   */
  getAllCards(): Card[] {
    return [...this.cards];
  }

  /**
   * Reset the entire board
   */
  reset(): void {
    this.cards = [];
    this.flippedCards = [];
    this.matchedCount = 0;
  }

  /**
   * Fisher-Yates shuffle algorithm
   */
  private shuffle<T>(array: T[]): T[] {
    const result = [...array];
    for (let i = result.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [result[i], result[j]] = [result[j], result[i]];
    }
    return result;
  }
}
