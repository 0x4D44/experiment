# Cyber Breach - Network Hacking Puzzle Game

**A sophisticated hacking simulation where you infiltrate networks, solve puzzles, and extract data while evading security systems.**

## Overview

Cyber Breach is a turn-based hacking puzzle game that challenges players to infiltrate virtual networks by exploiting security vulnerabilities. The game combines strategy, puzzle-solving, and stealth mechanics to create an engaging cybersecurity-themed experience.

### Core Gameplay Loop
1. **Load a network** - Each level presents a different network topology
2. **Scan nodes** - Identify vulnerable systems and their security levels
3. **Activate tools** - Use specialized hacking tools to bypass security (limited uses)
4. **Solve puzzles** - Complete mini-puzzles specific to each node type
5. **Extract data** - Successfully hack nodes to collect data points
6. **Manage detection** - Keep your detection level low to avoid triggering alarms
7. **Progress** - Advance through increasingly difficult network levels

## Features

### Network Systems
- **5 Node Types**: Gateway, Server, Database, SecurityHub, DataCache
- **Security Levels**: 1-10 difficulty scaling
- **Network Connections**: Nodes connected through firewall-protected links
- **12 Progressive Levels**: From beginner networks to ultimate security challenges

### Mini-Puzzle Types
- **Password Cracking**: Guess common passwords (admin, 123456, password, etc.)
- **Port Scanning**: Identify open ports and vulnerabilities
- **Encryption Breaking**: Decrypt Caesar cipher messages
- **Privilege Escalation**: Chain exploits to gain higher access
- **Firewall Bypass**: Navigate protocol and rule restrictions

### Hacking Tools (Limited Uses)
- **Port Scanner** (3 uses): Reveals port information for gateways
- **Password Cracker** (2 uses): Reduces difficulty for server hacks
- **Firewall Bypass** (1 use): Bypasses security hub firewalls
- **Privilege Exploit** (2 uses): Gains database access
- **Network Mask** (∞ uses): Reduces detection when active

### Stealth & Detection System
- **Detection Meter**: 0-100% (game over if hits 100%)
- **Failed Attempts**: Increase detection by (Security Level × 2)
- **Successful Hacks**: Increase detection by (Security Level × 3)
- **Network Mask**: Reduces detection increase by 50% when active
- **Detection Levels**: Low (0-25%), Medium (25-50%), High (50-75%), Critical (75-100%)

### Dynamic Level Generation
- Automatically generates networks with increasing complexity
- Security levels scale with difficulty
- Node counts increase per level (3-10 nodes)
- Time limits decrease as levels progress (300s → 60s)
- Random connections create varied network topologies

## How to Play

### Starting the Game
1. Open `cyber-breach.html` in a web browser
2. The game initializes with Level 1
3. Select a level from the dropdown to start a new level

### Basic Commands

**Hack a Node:**
- Enter node ID in the command input (e.g., `gateway_1` or `node_1_2`)
- Click HACK or press ENTER
- Solve the puzzle if successful

**Scan Node Information:**
- Click SCAN NODE
- Enter a node ID to see its details
- Shows security level, type, puzzle type, and data value

**View Network:**
- Click SHOW NETWORK
- Displays network topology information

**Activate Tools:**
- Click on tool buttons in the "Hacking Tools" panel
- Active tools appear highlighted
- Use activates tools when their effect is needed

### Game Objectives
- **Extract Data Points**: Hack nodes to collect enough data to complete the level
- **Avoid Detection**: Keep detection level below 100%
- **Beat the Clock**: Complete objectives before time runs out
- **Progress Through Levels**: Complete all 12 levels for ultimate victory

## Game Mechanics

### Node Types & Puzzles

| Node Type | Puzzle Type | Security | Data Reward |
|-----------|------------|----------|------------|
| Gateway | Port Scan | 1 | 10 points |
| Server | Password Crack | 1-10 | 10-100 points |
| Database | Encryption | 1-10 | 10-100 points |
| SecurityHub | Privilege Escalation | 1-10 | 10-100 points |
| DataCache | Firewall Bypass | 1-10 | 10-100 points |

### Detection Formula
```
Detection Increase = (Node Security Level × Multiplier) / Tool Effect
- Successful hack: Multiplier = 3
- Failed attempt: Multiplier = 2
- Network Mask active: Tool Effect = 2 (50% reduction)
```

### Level Progression
- **Level 1-3**: Tutorial levels with basic networks
- **Level 4-7**: Intermediate complexity, 5-7 nodes each
- **Level 8-10**: Advanced networks, 8-10 nodes, high security
- **Level 11-12**: Bonus levels with maximum difficulty

## Technical Details

### Technology Stack
- **Language**: TypeScript
- **Rendering**: HTML5 (Canvas-ready for future upgrades)
- **Testing**: Jest (64+ unit and integration tests)
- **Architecture**: Modular systems design

### Core Systems

**PuzzleGenerator**
- Generates contextual puzzles based on node type
- Difficulty scales with security level
- Unique puzzle data per node

**NetworkNode**
- Represents individual network systems
- Manages puzzle state and solution attempts
- Calculates security info and access requirements

**ToolKit**
- Manages 5 specialized hacking tools
- Tracks limited uses and active status
- Provides tool status information

**DetectionSystem**
- Tracks security detection (0-100%)
- Classifies threat levels
- Triggers alarm at max detection

**LevelManager**
- Generates procedural networks
- Scales difficulty with level number
- Creates 12+ unique level configurations

**CyberBreachGame**
- Main game engine
- Manages game state and progression
- Handles node hacking and level transitions

## Testing

The game includes a comprehensive test suite:

```bash
npm test -- cyber-breach.test.ts
```

**Test Coverage:**
- 7 Puzzle system tests
- 8 Node system tests
- 8 Tool system tests
- 10 Detection system tests
- 7 Level system tests
- 19 Game engine tests
- 5 Integration tests

**All 64 tests passing** with 100% success rate.

## Gameplay Strategies

### Beginner Strategy
1. Start with Level 1 to learn mechanics
2. Focus on Gateway nodes first (already hacked)
3. Use Password Cracker on Server nodes
4. Monitor detection - restart if it gets too high
5. Complete early levels to earn tool experience

### Intermediate Strategy
1. Plan your infiltration path before hacking
2. Use Network Mask when attempting risky nodes
3. Save high-use tools for high-security targets
4. Hack easier nodes first to build score cushion
5. Watch for detection spikes on hard nodes

### Advanced Strategy
1. Optimize node order to minimize detection
2. Combine multiple tools for maximum effectiveness
3. Balance quick hacks vs. careful approach
4. Use detection decreases to reset position
5. Master Level 11-12 challenges for achievement

## File Structure

```
cyber-breach.ts          - Core game logic (703 lines)
cyber-breach.test.ts     - Test suite (677 lines)
cyber-breach.html        - Interactive web interface
CYBER_BREACH_README.md   - This file
2025.11.07 - JRN - Network Hacker Development.md - Development journal
```

## Level Difficulties

| Level | Difficulty | Time Limit | Nodes | Avg Security |
|-------|-----------|-----------|-------|--------------|
| 1     | Beginner  | 280s      | 3-4   | 2-3          |
| 2     | Beginner  | 260s      | 3-4   | 3-4          |
| 3     | Easy      | 240s      | 4-5   | 4-5          |
| 4     | Easy      | 220s      | 4-5   | 5-6          |
| 5     | Medium    | 200s      | 5-6   | 6-7          |
| 6     | Medium    | 180s      | 5-6   | 7-8          |
| 7     | Hard      | 160s      | 6-7   | 8-9          |
| 8     | Hard      | 140s      | 7-8   | 8-9          |
| 9     | Expert    | 120s      | 8-9   | 9-10         |
| 10    | Expert    | 100s      | 9-10  | 10           |
| 11    | Insane    | 80s       | 10    | 10           |
| 12    | Impossible| 60s       | 10    | 10           |

## Scoring

- **Per Node**: Score = Node Security Level × 10 data points
- **Level Completion**: Reach 50% of total network data points
- **Bonus**: Completing with low detection adds prestige
- **Progression**: Each level's score carries over to next level

## Known Limitations

1. Simple AI puzzle solutions (not full cryptography)
2. No persistent high scores (browser session only)
3. No multiplayer or network features
4. Terminal interface text-based only
5. No sound effects or music

## Future Enhancements

1. **Enhanced Graphics**: Canvas-based network visualization
2. **Achievements**: Badges for specific accomplishments
3. **Leaderboards**: Track high scores and speedruns
4. **Procedural Generation**: Infinite procedural networks
5. **Story Mode**: Campaign with narrative elements
6. **Multiplayer**: Competitive hacking scenarios
7. **Advanced Tools**: More sophisticated hacking equipment
8. **Network AI**: Defensive AI that adapts to player tactics

## Development Notes

This game was built following Test-Driven Development (TDD) principles:
1. Comprehensive test suite written first
2. Game logic implemented to pass all tests
3. UI added as a presentation layer
4. All systems fully decoupled and testable

The architecture supports easy expansion for additional puzzle types, tools, and levels without modifying core systems.

## Credits

**Cyber Breach** - Round 3 Game Development Competition
- Built with TypeScript
- Tested with Jest
- Designed for browser-based gameplay
- Emphasis on strategy and puzzle-solving

## License

This game is provided as competition submission. All code is original and follows standard software engineering practices.

---

**Ready to infiltrate? Load a level and start hacking!**
