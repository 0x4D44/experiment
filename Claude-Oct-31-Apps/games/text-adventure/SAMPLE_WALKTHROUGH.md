# Dungeon Escape - Sample Game Walkthrough

## Overview

"Dungeon Escape" is a sample text adventure that demonstrates the capabilities of the Text Adventure Engine. This guide provides a walkthrough of the game, showing various features and mechanics in action.

## Game Objective

**Escape the dungeon!** You wake up in a cold prison cell with no memory of how you got there. Navigate through the dungeon, solve puzzles, collect items, and find your way to freedom.

## Starting the Game

```bash
./bin/textadventure games/dungeon-escape/dungeon-escape.json
```

You'll see the opening message and find yourself in the Dungeon Cell.

## Game Map

```
                    DUNGEON ESCAPE

     [Armory]--[Guard's Chamber]
                      |
    [Cell]--[Corridor]--[Treasury]
                      |
                [Cellar]
                   |  |
        [Deep Cellar]
                   |
            [Escape Tunnel]--[Outside]
```

## Walkthrough: Quick Escape (Minimum Puzzles)

### Step 1: Examine Your Cell

```
> look
```

You're in the Dungeon Cell. Look around carefully.

```
> examine mattress
```

The mattress feels lumpy. This is important!

```
> take key
```

You discover an iron key hidden beneath the mattress. This is essential for escape!

**Puzzle Solved**: Found the first key!

### Step 2: Enter the Corridor

```
> north
```

You move to the Dungeon Corridor. A guard stands here.

```
> take torch
```

Take the torch to light your way through dark passages.

### Step 3: Avoid the Guard and Find a Way Around

```
> north
```

Move north to the Guard's Chamber. The main guard is sleeping.

```
> take coins
```

Grab the gold coins - they might be useful.

### Step 4: Reach the Treasury

Return to the corridor and head east:

```
> south
> east
```

You're now in the Treasury. This is where treasure is kept!

```
> take crown
```

The golden crown is valuable. Take it with you.

### Step 5: Navigate to the Cellar

```
> down
```

You descend to the Cellar, which is damp and dark. The torch helps you see.

### Step 6: Find the Path to Freedom

```
> east
```

You enter the Escape Tunnel!

```
> east
```

You emerge **Outside the Dungeon** and escape to freedom!

**Victory!** You've escaped the dungeon!

## Walkthrough: Full Exploration (All Items)

### Additional Locations

If you want to explore completely, visit these locations:

#### Armory
From Guard's Chamber:
```
> west
```

Here you'll find weapons:
```
> take sword
> take shield
```

These items increase your score for discovery.

#### Deep Cellar
From Cellar:
```
> down
```

In the Deep Cellar, find ancient treasures:
```
> take golden_key
> take ancient_scroll
```

These items contain clues and valuable artifacts.

### Collection Challenge

Try to collect all items:
- Iron Key (Mattress)
- Torch (Corridor)
- Gold Coins (Guard's Chamber)
- Crown (Treasury)
- Sword (Armory)
- Shield (Armory)
- Golden Key (Deep Cellar)
- Ancient Scroll (Deep Cellar)
- Rope (Escape Tunnel)
- Ancient Scroll (Cellar)
- Map (Cellar)

**Maximum Score**: Collect all items for the highest score!

## Command Examples

### Movement

```
> north           # Go north
> south           # Go south
> east            # Go east
> west            # Go west
> n               # Shortcut for north
> s               # Shortcut for south
> up              # Go upstairs
> down            # Go downstairs
> d               # Shortcut for down
```

### Examining Items

```
> examine key           # Look closely at the key
> look at torch         # Alternative way to examine
> x sword               # Shortcut for examine
> inspect crown         # Another way to examine
```

### Inventory Management

```
> inventory       # Show what you're carrying
> i               # Shortcut for inventory
> take key        # Pick up an item
> get sword       # Alternative way to take
> drop coins      # Leave an item behind
> place shield    # Alternative way to drop
```

### Room Information

```
> look            # Look around the room
> l               # Shortcut for look
```

### System Commands

```
> help            # Show available commands
> wait            # Pass time
> z               # Shortcut for wait
> quit            # Exit the game
> exit            # Alternative way to quit
```

## Puzzle Solutions

### Puzzle 1: Finding the Key

**Location**: Dungeon Cell

**Solution**: Examine the mattress closely. The key is hidden beneath it.

```
> examine mattress
> take key
```

### Puzzle 2: Navigating Past Enemies

**Location**: Guard's Chamber

**Solution**: The guard is sleeping, so you can sneak past quietly. Move quickly through the area without disturbing him.

**Alternative**: Use the Armory route as a detour if you want to avoid NPCs.

### Puzzle 3: Finding the Path to Freedom

**Location**: Escape Tunnel

**Solution**: The torches light the way. Follow the path east from the Cellar to the Escape Tunnel, then continue east to reach the Outside and escape!

## Inventory Tips

### Weight Management

Each item has a weight:
- Iron Key: 0.3 lbs
- Torch: 1.0 lbs
- Gold Coins: 0.5 lbs (or 5.0 lbs for stack)
- Crown: 2.0 lbs
- Sword: 3.0 lbs
- Shield: 2.5 lbs
- Golden Key: 0.2 lbs
- Rope: 1.5 lbs

**Maximum Carrying Capacity**: 50 pounds

### Strategy

- Take essential items: Key, Torch, Map
- Take valuables for points: Crown, Golden Key
- Weapons are heavy: Choose wisely

## Item Interactions

### Open Containers

Some items can be containers:

```
> open chest          # Opens a container
> put coins in chest  # Place items in container
> close chest         # Closes container
```

### Use Items

Some items can be used:

```
> use key on door     # Use key to unlock
> use torch on dark   # Light up dark areas
> use rope on wall    # Climb or secure rope
```

## Secrets and Easter Eggs

### Hidden Items

Some items are hidden and require examination to find:
- Hidden diamond in the Deep Cellar (examine carefully)
- Ancient scroll (look for old parchment)

### Secret Paths

- Cellar has multiple exits - explore all directions
- Some rooms have items described in their descriptions

### Score Bonuses

- First key found: 10 points
- Each room visited: 5 points
- Each item collected: 5 points
- Puzzle solved: 25 points
- Game completed: 100 points

## Common Mistakes

### Can't Move North?

**Problem**: "You can't go that way."

**Solution**: Make sure you're in a room that has a north exit. Check available exits with `look`.

### Can't Pick Up Item?

**Problem**: "You don't see that here."

**Solution**:
1. The item might be in a different room
2. You might have the item already
3. Check your inventory with `i`

### Inventory Full?

**Problem**: "Your inventory is full."

**Solution**:
1. Drop non-essential items: `drop coins`
2. Check what you're carrying: `inventory`
3. The weight limit is 50 pounds

### Can't Take Non-Takeable Items?

Some items like walls or heavy objects can't be taken. These are atmospheric elements.

## Learning the Engine

### Command Variations

The parser understands many variations:
- `take sword` = `get sword` = `grab sword` = `pick up sword`
- `examine key` = `look at key` = `x key` = `inspect key`
- `go north` = `move north` = `north` = `n`

### Game State Tracking

The engine remembers:
- Which rooms you've visited
- Which items you've examined
- What's in your inventory
- Game variables and progress

### Event Triggers

When you perform actions, events fire:
- Enter a room → room_entered event
- Take an item → item_taken event
- Examine something → item_examined event

## Extending the Sample Game

You can modify `dungeon-escape.json` to:

### Add More Rooms

```json
"new_room": {
  "name": "New Location",
  "description": "Description of the room",
  "exits": {"north": "existing_room"},
  "items": [],
  "npcs": []
}
```

### Add More Items

```json
"new_item": {
  "name": "Item Name",
  "description": "What it looks like",
  "takeable": true,
  "weight": 1.0
}
```

### Add NPCs with Dialogue

NPCs can be placed in rooms and talked to. The basic version supports friendly/hostile NPCs.

## Advanced Gameplay

### Speedrun Challenge

**Objective**: Escape as quickly as possible

**Target**: Complete in under 5 commands

**Solution**:
```
> take key
> north
> take torch
> south
> east
> down
> east
> east
```

**Time**: ~30 seconds!

### Completionist Challenge

**Objective**: Collect every item and visit every room

**Target**: Maximum score

**Rooms to Visit**:
1. Cell (start)
2. Corridor
3. Guard's Chamber
4. Armory
5. Treasury
6. Cellar
7. Deep Cellar
8. Escape Tunnel
9. Outside (finish)

**Items to Collect**: All 18 items for maximum points

## Game Statistics

- **Rooms**: 9 unique locations
- **Items**: 18 collectible objects
- **NPCs**: 2 characters
- **Possible Commands**: 100+
- **Multiple Paths**: Yes
- **Optional Content**: Yes
- **Estimated Play Time**: 10-30 minutes (depending on exploration)

## Hints and Tips

1. **Examine everything**: Many items are hidden or contain clues
2. **Read descriptions**: They often hint at solutions
3. **Try different commands**: The parser is very flexible
4. **Explore thoroughly**: Not all paths are obvious
5. **Inventory management**: Weight limit is a real constraint
6. **Take your time**: Adventures are meant to be savored

## Troubleshooting

### Game Won't Start

- Ensure game file exists: `games/dungeon-escape/dungeon-escape.json`
- Check file is valid JSON (no syntax errors)
- Verify you're in the correct directory

### Commands Not Working

- Type `help` to see available commands
- Check for typos (though fuzzy matching helps)
- Some items might not be in the current room

### Can't Exit

- Type `quit` or `exit`
- Press Ctrl+C to force quit

## Next Steps

1. **Play the game**: Experience the adventure yourself!
2. **Create your own game**: Use `GAME_FORMAT.md` as a reference
3. **Extend the engine**: Add custom features to the code
4. **Share your creations**: Make unique adventures!

---

**Enjoy your dungeon escape! Good luck, adventurer!**
