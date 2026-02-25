# Game File Format Specification

## Overview

Games are defined using JSON files that describe the world, items, NPCs, and initial state. This document specifies the complete format for creating new games.

## Root Structure

```json
{
  "title": "string",
  "description": "string",
  "version": "string",
  "author": "string",
  "startingRoom": "string",
  "maxInventory": integer,
  "maxCarryWeight": number,
  "variables": { /* object */ },
  "rooms": { /* object */ },
  "items": { /* object */ },
  "npcs": { /* object */ }
}
```

### Top-Level Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | string | Yes | Display name of the game |
| `description` | string | No | Brief description of the game |
| `version` | string | No | Game version (e.g., "1.0.0") |
| `author` | string | No | Author name |
| `startingRoom` | string | Yes | ID of the first room |
| `maxInventory` | integer | No | Maximum number of items (default: 20) |
| `maxCarryWeight` | number | No | Maximum weight in pounds (default: 50.0) |
| `variables` | object | No | Initial game variables |
| `rooms` | object | Yes | Room definitions (at least one) |
| `items` | object | No | Item definitions |
| `npcs` | object | No | NPC definitions |

## Rooms

Rooms are the locations in your game world. Each room is defined with an ID and properties.

```json
"rooms": {
  "room_id": {
    "name": "string",
    "description": "string",
    "exits": { /* object */ },
    "items": [ /* array */ ],
    "npcs": [ /* array */ ],
    "locked": boolean,
    "lockItem": "string"
  }
}
```

### Room Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `name` | string | Required | Display name shown to player |
| `description` | string | Required | Room description shown when entering |
| `exits` | object | `{}` | Available exits as direction → room_id |
| `items` | array | `[]` | Array of item IDs in the room |
| `npcs` | array | `[]` | Array of NPC IDs in the room |
| `locked` | boolean | `false` | Whether the room is locked |
| `lockItem` | string | `null` | Item ID that unlocks the room |

### Example Room

```json
"treasury": {
  "name": "Royal Treasury",
  "description": "A grand chamber filled with chests of gold and jewels.",
  "exits": {
    "west": "corridor",
    "down": "basement"
  },
  "items": ["gold_coins", "crown", "chest"],
  "npcs": ["treasurer"],
  "locked": false
}
```

### Exit Directions

Valid exit directions are:
- `north`, `south`, `east`, `west` - Cardinal directions
- `up`, `down` - Vertical directions
- Custom: Any string (though standard directions are recommended)

### Locked Rooms

To make a room locked, set `locked: true` and specify the `lockItem`. Players must have that item to enter:

```json
"vault": {
  "name": "Secure Vault",
  "description": "A heavily locked vault.",
  "exits": {
    "west": "corridor"
  },
  "items": [],
  "npcs": [],
  "locked": true,
  "lockItem": "master_key"
}
```

## Items

Items are objects that exist in the world. Players can examine, take, use, and combine items.

```json
"items": {
  "item_id": {
    "name": "string",
    "description": "string",
    "takeable": boolean,
    "useable": boolean,
    "container": boolean,
    "locked": boolean,
    "lockItem": "string",
    "hidden": boolean,
    "weight": number,
    "size": number,
    "cursed": boolean,
    "stackable": boolean,
    "quantity": integer
  }
}
```

### Item Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `name` | string | Required | Display name for the item |
| `description` | string | Required | Description when examined |
| `takeable` | boolean | `true` | Can the player pick it up? |
| `useable` | boolean | `false` | Can the player use it? |
| `container` | boolean | `false` | Can it hold other items? |
| `locked` | boolean | `false` | Is container locked? |
| `lockItem` | string | `null` | Item ID that unlocks it |
| `hidden` | boolean | `false` | Hidden until examined? |
| `weight` | number | `1.0` | Weight in pounds |
| `size` | number | `1.0` | Size in cubic units |
| `cursed` | boolean | `false` | Cannot be dropped if true |
| `stackable` | boolean | `false` | Multiple can stack? |
| `quantity` | integer | `1` | For stackable items |

### Common Item Types

#### Weapons
```json
"sword": {
  "name": "Iron Sword",
  "description": "A sharp steel sword.",
  "takeable": true,
  "useable": true,
  "weight": 3.0
}
```

#### Keys
```json
"key": {
  "name": "Golden Key",
  "description": "An ornate golden key.",
  "takeable": true,
  "useable": true,
  "weight": 0.2
}
```

#### Containers
```json
"chest": {
  "name": "Wooden Chest",
  "description": "A sturdy wooden chest.",
  "takeable": false,
  "container": true,
  "locked": false,
  "weight": 10.0
}
```

#### Hidden Items
```json
"diamond": {
  "name": "Hidden Diamond",
  "description": "A brilliant diamond!",
  "takeable": true,
  "hidden": true,
  "weight": 0.1
}
```

#### Stackable Items
```json
"coins": {
  "name": "Gold Coins",
  "description": "A stack of gold coins.",
  "takeable": true,
  "stackable": true,
  "quantity": 50,
  "weight": 0.5
}
```

#### Cursed Items
```json
"cursed_ring": {
  "name": "Cursed Ring",
  "description": "An ancient ring that seems cursed.",
  "takeable": true,
  "cursed": true,
  "weight": 0.1
}
```

## NPCs

Non-player characters that the player can interact with.

```json
"npcs": {
  "npc_id": {
    "name": "string",
    "description": "string",
    "location": "string",
    "friendly": boolean,
    "canTrade": boolean
  }
}
```

### NPC Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `name` | string | Required | NPC's display name |
| `description` | string | Required | Physical description |
| `location` | string | Required | Starting room ID |
| `friendly` | boolean | `true` | Is NPC friendly? |
| `canTrade` | boolean | `false` | Can they trade items? |

### NPC Examples

#### Friendly NPC
```json
"merchant": {
  "name": "Traveling Merchant",
  "description": "A weathered traveler with many goods.",
  "location": "marketplace",
  "friendly": true,
  "canTrade": true
}
```

#### Hostile NPC
```json
"guard": {
  "name": "Castle Guard",
  "description": "An armored guard, looking stern.",
  "location": "gate",
  "friendly": false,
  "canTrade": false
}
```

## Game Variables

Initial game state variables accessible throughout the game.

```json
"variables": {
  "puzzle_solved": false,
  "player_level": 1,
  "story_chapter": "beginning",
  "items_collected": 0
}
```

Variables can be:
- **Boolean**: `true` or `false`
- **Number**: Integer or decimal (`42` or `3.14`)
- **String**: `"text value"`
- **Array**: `[1, 2, 3]` (for multiple values)

## Complete Example

```json
{
  "title": "The Lost Temple",
  "description": "Explore an ancient temple to find the Sacred Artifact",
  "version": "1.0.0",
  "author": "Game Creator",
  "startingRoom": "entrance",
  "maxInventory": 15,
  "maxCarryWeight": 60.0,
  "variables": {
    "temple_opened": false,
    "guardian_defeated": false,
    "torches_lit": 0
  },
  "rooms": {
    "entrance": {
      "name": "Temple Entrance",
      "description": "You stand before an ancient temple. Its stone walls are covered in moss and vines. A heavy wooden door blocks the entrance.",
      "exits": {
        "north": "courtyard"
      },
      "items": ["old_key"],
      "npcs": ["monk"],
      "locked": false
    },
    "courtyard": {
      "name": "Temple Courtyard",
      "description": "A large courtyard with a crumbling fountain in the center. Stone statues watch from the corners.",
      "exits": {
        "south": "entrance",
        "east": "library",
        "west": "guard_room"
      },
      "items": ["torch"],
      "npcs": [],
      "locked": false
    },
    "library": {
      "name": "Ancient Library",
      "description": "Shelves of ancient books line the walls. Dust fills the air.",
      "exits": {
        "west": "courtyard",
        "down": "vault"
      },
      "items": ["scroll", "dusty_tome"],
      "npcs": ["librarian_ghost"],
      "locked": false
    },
    "vault": {
      "name": "Treasure Vault",
      "description": "A secure vault containing the temple's greatest treasures.",
      "exits": {
        "up": "library"
      },
      "items": ["artifact"],
      "npcs": [],
      "locked": true,
      "lockItem": "ancient_key"
    }
  },
  "items": {
    "old_key": {
      "name": "Rusty Key",
      "description": "An old key, covered in rust.",
      "takeable": true,
      "useable": true,
      "weight": 0.2
    },
    "torch": {
      "name": "Torch",
      "description": "A burning torch provides light.",
      "takeable": true,
      "useable": true,
      "weight": 1.0
    },
    "scroll": {
      "name": "Ancient Scroll",
      "description": "A scroll with instructions for the temple.",
      "takeable": true,
      "weight": 0.5
    },
    "dusty_tome": {
      "name": "Dusty Book",
      "description": "A book covered in centuries of dust.",
      "takeable": true,
      "hidden": true,
      "weight": 1.5
    },
    "ancient_key": {
      "name": "Ancient Key",
      "description": "A key made of unknown metal, glowing faintly.",
      "takeable": true,
      "useable": true,
      "weight": 0.3
    },
    "artifact": {
      "name": "Sacred Artifact",
      "description": "The legendary artifact you've been seeking!",
      "takeable": true,
      "weight": 0.5
    }
  },
  "npcs": {
    "monk": {
      "name": "Wise Monk",
      "description": "An elderly monk in meditation.",
      "location": "entrance",
      "friendly": true,
      "canTrade": false
    },
    "librarian_ghost": {
      "name": "Ghostly Librarian",
      "description": "The spirit of the temple's ancient librarian.",
      "location": "library",
      "friendly": true,
      "canTrade": false
    }
  }
}
```

## Validation Rules

Games must follow these rules:

1. **startingRoom** must be defined in rooms
2. All room **exits** must point to valid rooms
3. All room **items** must be defined in items
4. All room **npcs** must be defined in npcs
5. NPC **location** must be a valid room
6. Container **lockItem** must be defined in items
7. Room **lockItem** must be defined in items

## Best Practices

1. **Use meaningful IDs**: Use lowercase with underscores (e.g., `treasure_chest`)
2. **Provide detail**: Write engaging descriptions
3. **Balance complexity**: Mix simple and complex puzzles
4. **Test connections**: Ensure all exits form coherent paths
5. **Hide hints**: Use hidden items and descriptions for puzzle solutions
6. **Limit inventory**: Consider player convenience with weight limits
7. **Document**: Add comments in your JSON or separate documentation

## Limits

- Maximum rooms: Unlimited (tested up to 1000+)
- Maximum items: Unlimited (tested up to 500+)
- Maximum NPCs: Unlimited (tested up to 100+)
- Maximum inventory: Configurable (default 20 items)
- Maximum weight: Configurable (default 50 pounds)
- Maximum string length: No limit

## File Size Guidelines

- Small game: <100KB (10-20 rooms)
- Medium game: 100KB-500KB (30-100 rooms)
- Large game: 500KB+ (100+ rooms)

## Testing Your Game File

```bash
# The game will validate on load and report errors
./textadventure your-game.json
```

Common error messages:
- "room not found" - An exit points to non-existent room
- "item not found" - Room or NPC references non-existent item
- "missing required field" - A required property is missing

## Advanced Features (Future)

These features are planned for future versions:

- Dialogue trees and branching conversations
- Custom scripts for complex logic
- Time-based events and timers
- Procedural generation
- Dynamic difficulty adjustment
- Multiplayer interactions
- Save files integration

For now, use game variables and event handlers for complex logic.

---

**Ready to create your game? Start with the example above and expand!**
