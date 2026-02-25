# Asteroid Defender - Gameplay Guide

## Quick Start

1. Open `game.html` in a modern web browser (Chrome, Firefox, Safari, Edge)
2. Press **R** to start the game
3. Use Arrow Keys or WASD to move your spaceship
4. Move your mouse to aim
5. Click to fire projectiles at asteroids

## Game Objective

Survive **5 waves** of asteroids and enemy ships. Each wave gets progressively harder. The final challenge is a boss fight that will test all your skills!

## Controls

| Action | Keys |
|--------|------|
| **Move Up** | ↑ or W |
| **Move Down** | ↓ or S |
| **Move Left** | ← or A |
| **Move Right** | → or D |
| **Aim** | Mouse Movement |
| **Fire Weapon** | Left Click or Space (after aiming) |
| **Pause Game** | Spacebar |
| **Restart** | R |

## Game Mechanics

### Player Ship
- Starts with **100 health**
- Moves in 4 cardinal directions
- Can rotate to face any direction via mouse
- Fires projectiles in the direction you're aiming
- Weapon has a cooldown (fires once per 0.15 seconds)

### Asteroids
- **Large** asteroids: 30 health, move moderately fast
- Broken into smaller pieces when destroyed
- Spin slowly for visual effect
- Wrap around screen edges
- Drop resources when destroyed

### Projectiles
- Travel in a straight line
- Deal **10 damage** per hit
- Last for 3 seconds before disappearing
- Fast movement (500 pixels/second)

### Wave Progression
| Wave | Asteroid Count | Difficulty | Asteroids Speed |
|------|---|---|---|
| 1 | 3 | Easy | 100 px/s |
| 2 | 4 | Easy-Medium | 120 px/s |
| 3 | 5 | Medium | 140 px/s |
| 4 | 6 | Medium-Hard | 160 px/s |
| 5 | 7+ | Hard | 180+ px/s |

## Scoring System

| Action | Points |
|--------|--------|
| Destroy Asteroid | +10 |
| Complete Wave | +100 (bonus) |
| No Damage Taken | +50 per wave |

## Strategy Tips

### Positioning
- Stay in the center area - don't get cornered at edges
- Keep moving constantly to avoid collisions
- Use screen wrapping to your advantage (go off one edge, appear on opposite edge)

### Shooting
- Lead your targets - predict where asteroids will be
- Focus fire on large asteroids to break them into smaller pieces
- Small asteroids are easier to hit when they're split

### Defensive Play
- Maintain distance from asteroid clusters
- Use the upper portion of the screen to spot incoming asteroids
- Never let your health drop too low - stay aware of your health bar

### Wave Progression
- Take a moment after each wave completes to assess damage
- Plan your strategy for the next wave's starting positions
- If health is critical, prioritize survival over score

## Wave-by-Wave Breakdown

### Wave 1
- **Difficulty**: Tutorial level
- **Asteroids**: 3 large asteroids
- **Speed**: Moderate
- **Recommendation**: Get comfortable with controls, practice aiming

### Waves 2-3
- **Difficulty**: Easy to Medium
- **Asteroids**: Multiple asteroids spawning in spiral pattern
- **Speed**: Increasing
- **Recommendation**: Focus on accuracy, maintain distance

### Waves 4-5
- **Difficulty**: Hard
- **Asteroids**: Many fast-moving asteroids
- **Speed**: Very fast
- **Recommendation**: Prioritize survival, spam shots when threatened

## Game End Conditions

**Victory**: Destroy all asteroids in all 5 waves
**Defeat**: Player health reaches 0
**Pause**: Press Space anytime to pause/resume

## Performance Tips

- Game runs at 60 FPS on most modern browsers
- If experiencing lag, close other browser tabs
- Works best in Chrome or Firefox
- Minimum screen size: 800x600 pixels

## Advanced Techniques

### Screen Wrapping
- Move off the left edge to appear on the right
- Use this to escape dangerous situations
- Perfect for getting behind large asteroid clusters

### Burst Firing
- Click rapidly to fire the maximum number of projectiles
- Useful against large asteroid clusters
- Use cooldown timing to your advantage

### Predictive Aiming
- Anticipate asteroid trajectories
- Aim ahead of moving targets
- Practice leading your shots

## Troubleshooting

**Game won't start**
- Refresh the page
- Try a different browser (Chrome, Firefox, Safari)
- Ensure JavaScript is enabled

**Controls not responding**
- Click on the game canvas first to focus it
- Try using different keys (Arrow Keys vs WASD)
- Ensure your keyboard isn't in gaming mode

**Game runs slowly**
- Close other browser tabs and applications
- Reduce browser zoom level
- Try a different browser

## About the Game

**Asteroid Defender** is a classic space shooter with modern design. It combines:
- Arcade-style action gameplay
- Strategic wave-based progression
- Physics-based entity management
- Score-based challenge system

The game features a fully-implemented TypeScript game engine with:
- Robust collision detection
- Smooth physics simulation
- Entity management system
- Efficient rendering pipeline

---

**Have fun destroying asteroids!** 🚀

Remember: **It's not about the score, it's about surviving the waves!**
