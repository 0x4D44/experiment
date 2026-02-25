/**
 * Card class representing a single card in the Memory Quest game
 * Cards can be Spells, Items, or Monsters with associated power levels
 */

export enum CardType {
  SPELL = 'SPELL',
  ITEM = 'ITEM',
  MONSTER = 'MONSTER',
}

export interface CardData {
  id: string;
  type: CardType;
  name: string;
  power: number;
  description: string;
  imageSymbol: string;
}

export class Card {
  private data: CardData;
  private isRevealed: boolean = false;
  private isMatched: boolean = false;

  constructor(data: CardData) {
    this.data = data;
  }

  reveal(): void {
    this.isRevealed = true;
  }

  hide(): void {
    this.isRevealed = false;
  }

  match(): void {
    this.isMatched = true;
    this.isRevealed = true;
  }

  getId(): string {
    return this.data.id;
  }

  getType(): CardType {
    return this.data.type;
  }

  getName(): string {
    return this.data.name;
  }

  getPower(): number {
    return this.data.power;
  }

  getDescription(): string {
    return this.data.description;
  }

  getImageSymbol(): string {
    return this.data.imageSymbol;
  }

  isCardRevealed(): boolean {
    return this.isRevealed;
  }

  isCardMatched(): boolean {
    return this.isMatched;
  }

  reset(): void {
    if (!this.isMatched) {
      this.isRevealed = false;
    }
  }

  toJSON(): CardData {
    return { ...this.data };
  }
}
