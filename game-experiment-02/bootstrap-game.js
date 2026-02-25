const fs = require('fs');
const path = require('path');

const files = {
  'src/Item.ts': `export interface ItemProperties {
  isUsable?: boolean;
  effect?: string;
  [key: string]: string | boolean | undefined;
}

export class Item {
  id: string;
  name: string;
  description: string;
  isUsable: boolean;
  effect?: string;

  constructor(name: string, description: string, properties?: ItemProperties) {
    this.id = \`item_\${Date.now()}_\${Math.random().toString(36).substr(2, 9)}\`;
    this.name = name;
    this.description = description;
    this.isUsable = properties?.isUsable ?? false;
    this.effect = properties?.effect;
  }
}`,

  'src/Inventory.ts': `import { Item } from './Item';

export class Inventory {
  private items: Item[] = [];
  private capacity: number;

  constructor(capacity: number) {
    this.capacity = capacity;
  }

  addItem(item: Item): void {
    if (this.items.length >= this.capacity) {
      throw new Error('Inventory is full');
    }
    this.items.push(item);
  }

  removeItem(itemId: string): void {
    this.items = this.items.filter((item) => item.id !== itemId);
  }

  getItems(): Item[] {
    return [...this.items];
  }

  getCapacity(): number {
    return this.capacity;
  }

  getAvailableSlots(): number {
    return this.capacity - this.items.length;
  }

  findByName(name: string): Item | null {
    const item = this.items.find((i) => i.name === name);
    return item ?? null;
  }

  findById(id: string): Item | null {
    const item = this.items.find((i) => i.id === id);
    return item ?? null;
  }

  hasItem(name: string): boolean {
    return this.items.some((i) => i.name === name);
  }

  listItems(): string[] {
    return this.items.map((item) => \`\${item.name} - \${item.description}\`);
  }
}`,

  'src/Location.ts': `import { Item } from './Item';
import { Player } from './Player';

export interface Exit {
  direction: string;
  locationId: string;
  description: string;
}

export class Location {
  id: string;
  title: string;
  description: string;
  items: Item[] = [];
  exits: Exit[] = [];
  visited: boolean = false;

  constructor(id: string, title: string, description: string) {
    this.id = id;
    this.title = title;
    this.description = description;
  }

  addExit(direction: string, locationId: string, description: string): void {
    this.exits.push({ direction, locationId, description });
  }

  addItem(item: Item): void {
    this.items.push(item);
  }

  removeItem(itemId: string): Item | null {
    const index = this.items.findIndex((i) => i.id === itemId);
    if (index !== -1) {
      const item = this.items[index];
      this.items.splice(index, 1);
      return item;
    }
    return null;
  }

  hasItem(name: string): boolean {
    return this.items.some((i) => i.name === name);
  }

  getFullDescription(player: Player): string {
    let desc = \`\n=== \${this.title} ===\n\n\${this.description}\n\`;
    
    if (this.items.length > 0) {
      desc += '\nYou see:\n';
      this.items.forEach((item) => {
        desc += \`  - \${item.name}: \${item.description}\n\`;
      });
    }

    if (this.exits.length > 0) {
      desc += '\nExits:\n';
      this.exits.forEach((exit) => {
        desc += \`  - \${exit.direction}: \${exit.description}\n\`;
      });
    }

    return desc;
  }
}`,

  'src/Player.ts': `import { Inventory } from './Inventory';
import { Item } from './Item';

export class Player {
  name: string;
  health: number;
  maxHealth: number;
  currentLocationId: string;
  inventory: Inventory;
  hasWon: boolean = false;
  puzzlesSolved: Set<string> = new Set();

  constructor(name: string, startingLocationId: string) {
    this.name = name;
    this.health = 100;
    this.maxHealth = 100;
    this.currentLocationId = startingLocationId;
    this.inventory = new Inventory(15);
  }

  takeDamage(amount: number): void {
    this.health = Math.max(0, this.health - amount);
  }

  heal(amount: number): void {
    this.health = Math.min(this.maxHealth, this.health + amount);
  }

  isAlive(): boolean {
    return this.health > 0;
  }

  solvePuzzle(puzzleId: string): void {
    this.puzzlesSolved.add(puzzleId);
  }

  hasSolvedPuzzle(puzzleId: string): boolean {
    return this.puzzlesSolved.has(puzzleId);
  }

  getStatus(): string {
    return \`[\${this.name}] Health: \${this.health}/\${this.maxHealth} | Inventory: \${this.inventory.getItems().length}/\${this.inventory.getCapacity()}\`;
  }
}`,

  'src/__tests__/Item.test.ts': `import { Item } from '../Item';

describe('Item', () => {
  it('should create an item with name and description', () => {
    const item = new Item('Golden Key', 'A shimmering golden key');
    expect(item.name).toBe('Golden Key');
    expect(item.description).toBe('A shimmering golden key');
  });

  it('should support optional properties', () => {
    const item = new Item('Health Potion', 'Restores 25 HP', { isUsable: true, effect: 'heal' });
    expect(item.isUsable).toBe(true);
    expect(item.effect).toBe('heal');
  });

  it('should have a unique ID', () => {
    const item1 = new Item('Item 1', 'desc');
    const item2 = new Item('Item 2', 'desc');
    expect(item1.id).not.toBe(item2.id);
  });
});`,

  'src/__tests__/Inventory.test.ts': `import { Inventory } from '../Inventory';
import { Item } from '../Item';

describe('Inventory', () => {
  let inventory: Inventory;

  beforeEach(() => {
    inventory = new Inventory(10);
  });

  it('should add items to inventory', () => {
    const item = new Item('Test', 'desc');
    inventory.addItem(item);
    expect(inventory.getItems()).toContain(item);
  });

  it('should not add items beyond capacity', () => {
    const inventory2 = new Inventory(1);
    inventory2.addItem(new Item('Item 1', 'desc'));
    expect(() => inventory2.addItem(new Item('Item 2', 'desc'))).toThrow('Inventory is full');
  });

  it('should remove items', () => {
    const item = new Item('Test', 'desc');
    inventory.addItem(item);
    inventory.removeItem(item.id);
    expect(inventory.getItems()).not.toContain(item);
  });

  it('should find items by name', () => {
    const item = new Item('Golden Key', 'desc');
    inventory.addItem(item);
    expect(inventory.findByName('Golden Key')).toBe(item);
  });
});`
};

// Create directories
['src/__tests__'].forEach(dir => {
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
});

// Create files
Object.entries(files).forEach(([filePath, content]) => {
  const dir = path.dirname(filePath);
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
  fs.writeFileSync(filePath, content);
  console.log(`Created: ${filePath}`);
});

console.log('\nBootstrap complete!');
