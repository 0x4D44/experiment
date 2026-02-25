# Poker Quest - Roguelike Card Combat Game

## Overview

**Poker Quest** is a strategic roguelike game that combines poker hand evaluation with turn-based combat. Draw poker hands to defeat enemies, buy upgrades in shops, and defeat the Dragon Lord to win!

## Features

- **Poker Hand Combat**: Draw and play 5 cards from your deck to create poker hands (Royal Flush, Straight Flush, Four of a Kind, etc.)
- **Strategic Deck Management**: Manage your card deck across encounters, with the ability to refresh it in the shop
- **Enemy Variety**: Fight 10 different enemies including Goblins, Orcs, Trolls, Wizards, Dark Knights, and the final Dragon Lord boss
- **Shop System**: Earn gold from victories and purchase upgrades:
  - Health Boost: +20 Max Health
  - Heal: Restore 50 HP
  - Refresh Deck: Start fresh with a new deck
  - Gold Multiplier: Earn more gold (permanent bonus)
- **Progressive Difficulty**: Enemy health increases with each encounter
- **Boss Battle**: Fight the Dragon Lord with 150 HP at encounter 10
- **Health Management**: Balance offense and defense as you accumulate damage

## How to Play

### Starting the Game
1. Open `poker-quest.html` in a modern web browser
2. Click "START NEW RUN" to begin

### Combat
1. You start with 5 cards in your hand
2. **Select 5 cards** by clicking on them (they will highlight in green)
3. Click **ATTACK** to play your hand
4. The game evaluates your poker hand and calculates damage
5. Enemy plays back, dealing damage to you
6. Continue until the enemy is defeated or your health reaches 0

### Poker Hands (Damage Values)
- **Royal Flush** (100 damage): A-K-Q-J-10, all same suit
- **Straight Flush** (80 damage): Five consecutive cards, all same suit
- **Four of a Kind** (60 damage): Four cards of the same rank
- **Full House** (50 damage): Three of a kind + pair
- **Flush** (35 damage): Five cards of the same suit
- **Straight** (30 damage): Five consecutive cards
- **Three of a Kind** (25 damage): Three cards of the same rank
- **Two Pair** (15 damage): Two different pairs
- **Pair** (8 damage): Two cards of the same rank
- **High Card** (2 damage): No matching cards

### Shop
After each victory (except the final boss), visit the shop to buy upgrades:
- Purchase upgrades with gold earned from battles
- Plan your purchases carefully - you have limited resources
- Healing items are valuable for surviving multiple encounters

## Game Statistics

- **Encounters to Win**: 10
- **Starting Health**: 100 HP
- **Gold Rewards**: 10 + (5 × encounter number)
  - Encounter 1: 15 gold
  - Encounter 2: 20 gold
  - ...
  - Encounter 10: 60 gold
- **Total possible gold**: ~425 gold (without multipliers)

## Technical Details

### Architecture
- Pure TypeScript/JavaScript (no external dependencies)
- HTML5 Canvas-style UI with CSS
- Modular card evaluation system
- Complete game state management

### Poker Hand Evaluation Algorithm
The game uses a comprehensive poker hand evaluation system that:
1. Counts card occurrences (pairs, three of a kind, etc.)
2. Detects straights (including ace-low straights A-2-3-4-5)
3. Detects flushes (all same suit)
4. Ranks hands by strength
5. Assigns consistent damage values

### Files
- `poker-quest.html` - Complete playable game
- `poker-quest.ts` - TypeScript source code (game logic)
- `poker-quest.test.ts` - Comprehensive test suite
- `POKER_QUEST_README.md` - This file

## Testing

The game includes a comprehensive test suite covering:
- Rank value calculations
- Deck creation and card drawing
- All poker hand types (10 different hands)
- Hand damage calculations
- Game initialization and flow
- Card selection mechanics
- Combat resolution
- Enemy defeat and loss conditions
- Shop system and upgrades
- Game status reporting

**To run tests** (requires Jest):
```bash
npm install jest ts-node @types/jest @types/node typescript
npx jest poker-quest.test.ts
```

## Game Strategy Tips

1. **Preserve High Cards**: Aces and face cards are valuable for straights and high cards
2. **Plan Deck Refreshes**: The "Refresh Deck" shop item resets your card distribution
3. **Health Management**: Buy healing items if you're below 50% health
4. **Early Victories**: Win early encounters to accumulate gold for better upgrades
5. **Boss Preparation**: Ensure you have at least 150 HP or have powerful hands ready for the final boss

## Gameplay Flow

```
Title Screen
    ↓
Start New Run (100 HP, Full Deck, 0 Gold)
    ↓
[Encounter 1-9]:
    Generate Enemy
    ↓
    Battle Loop:
        - Draw 5 cards
        - Select cards
        - Attack
        - Enemy attacks
        - If enemy defeated → Victory
        - If player defeated → Game Over
    ↓
    Visit Shop (Buy upgrades)
    ↓
    Next Encounter
    ↓
[Encounter 10]:
    Dragon Lord (150 HP)
    Battle Loop (same as above)
    ↓
    If Defeated → Victory Screen
    If Defeated by Dragon → Game Over Screen
```

## Browser Compatibility

Works on all modern browsers with:
- HTML5 Canvas support
- ES6+ JavaScript
- CSS Grid and Flexbox

Tested on:
- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+

## Future Enhancement Ideas

- Special card abilities (wild cards, damage multipliers)
- More enemy types with unique AI behaviors
- Relic/artifact system for permanent bonuses
- Deck building between encounters (choose specific cards)
- Leaderboards/high scores
- Multiple game modes (endless, challenges)
- Sound effects and background music

## Game Balance

The game is balanced to provide:
- **Difficulty**: Enemies scale from 50 HP to 150 HP
- **Accessibility**: Lucky card draws can win against tough odds
- **Variety**: Different poker hands create different strategies
- **Progression**: Shop upgrades provide meaningful progression

## Credits

Created for Round 3 of the Game Development Competition.

Game Design: Strategic card play meets roguelike progression
Code: Pure TypeScript/JavaScript implementation
Testing: Comprehensive test suite for game logic

---

**Enjoy your poker quest and good luck defeating the Dragon Lord!**
