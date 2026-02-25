import { Card, CardType } from './Card';

describe('Card', () => {
  let card: Card;

  beforeEach(() => {
    card = new Card({
      id: 'spell-01',
      type: CardType.SPELL,
      name: 'Fireball',
      power: 25,
      description: 'A powerful fire spell',
      imageSymbol: '🔥',
    });
  });

  describe('Card Creation', () => {
    it('should create a card with correct properties', () => {
      expect(card.getId()).toBe('spell-01');
      expect(card.getType()).toBe(CardType.SPELL);
      expect(card.getName()).toBe('Fireball');
      expect(card.getPower()).toBe(25);
      expect(card.getDescription()).toBe('A powerful fire spell');
      expect(card.getImageSymbol()).toBe('🔥');
    });

    it('should create a card that is initially hidden and not matched', () => {
      expect(card.isCardRevealed()).toBe(false);
      expect(card.isCardMatched()).toBe(false);
    });
  });

  describe('Card Reveal/Hide', () => {
    it('should reveal a card', () => {
      card.reveal();
      expect(card.isCardRevealed()).toBe(true);
    });

    it('should hide a card', () => {
      card.reveal();
      expect(card.isCardRevealed()).toBe(true);
      card.hide();
      expect(card.isCardRevealed()).toBe(false);
    });

    it('should reset a card to hidden state if not matched', () => {
      card.reveal();
      card.reset();
      expect(card.isCardRevealed()).toBe(false);
    });

    it('should stay revealed after reset if matched', () => {
      card.reveal();
      card.match();
      expect(card.isCardMatched()).toBe(true);
      card.reset();
      expect(card.isCardRevealed()).toBe(true);
    });
  });

  describe('Card Matching', () => {
    it('should match a card', () => {
      expect(card.isCardMatched()).toBe(false);
      card.match();
      expect(card.isCardMatched()).toBe(true);
      expect(card.isCardRevealed()).toBe(true);
    });

    it('should stay matched after reset', () => {
      card.match();
      card.reset();
      expect(card.isCardMatched()).toBe(true);
    });
  });

  describe('Card Export', () => {
    it('should export card data', () => {
      const data = card.toJSON();
      expect(data.id).toBe('spell-01');
      expect(data.type).toBe(CardType.SPELL);
      expect(data.name).toBe('Fireball');
      expect(data.power).toBe(25);
    });
  });
});
