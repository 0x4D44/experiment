# NEON TETRIS - Visual Preview

## What the Game Looks Like

### Main Screen Layout

```
╔════════════════════════════════════════════════════════════╗
║                                                            ║
║              N E O N   T E T R I S                        ║
║         (Glowing gradient text animation)                 ║
║                                                            ║
║  ┌─────────────────────┐      ┌──────────────┐           ║
║  │                     │      │    NEXT      │           ║
║  │                     │      │  ┌────────┐  │           ║
║  │    GAME BOARD       │      │  │ [    ] │  │           ║
║  │   (10×20 Grid)      │      │  └────────┘  │           ║
║  │                     │      └──────────────┘           ║
║  │  [Falling Pieces]   │                                 ║
║  │  [Ghost Preview]    │      ┌──────────────┐           ║
║  │  [Locked Blocks]    │      │    HOLD      │           ║
║  │                     │      │  ┌────────┐  │           ║
║  │   Neon Grid Lines   │      │  │ [  ]   │  │           ║
║  │   Glowing Blocks    │      │  └────────┘  │           ║
║  │                     │      └──────────────┘           ║
║  │                     │                                 ║
║  │                     │      ┌──────────────┐           ║
║  │                     │      │    STATS     │           ║
║  │                     │      │ SCORE: 1,250 │           ║
║  │                     │      │ LINES: 15    │           ║
║  └─────────────────────┘      │ LEVEL: 2     │           ║
║   (Cyan border + glow)        │ HIGH: 5,000  │           ║
║                               └──────────────┘           ║
║                                                            ║
║                               ┌──────────────┐           ║
║                               │  CONTROLS    │           ║
║                               │  ←/→  Move   │           ║
║                               │  ↓    Drop   │           ║
║                               │  ↑/Z  Rotate │           ║
║                               │ SPACE Hard   │           ║
║                               │   C   Hold   │           ║
║                               │  [START]     │           ║
║                               └──────────────┘           ║
╚════════════════════════════════════════════════════════════╝
```

## Color Scheme

### Piece Colors (All with neon glow)
- **I Piece**: Bright Cyan (#00ffff) - Horizontal/Vertical bar
- **O Piece**: Bright Yellow (#ffff00) - Square block
- **T Piece**: Magenta (#ff00ff) - T-shape
- **S Piece**: Bright Green (#00ff00) - S-shape
- **Z Piece**: Bright Red (#ff0000) - Z-shape
- **J Piece**: Bright Blue (#0000ff) - J-shape
- **L Piece**: Bright Orange (#ff8800) - L-shape

### UI Colors
- **Background**: Dark gradient (black → dark purple → dark blue)
- **Grid Lines**: Transparent cyan (rgba(0,255,255,0.1))
- **Main Border**: Cyan with glow effect
- **Panel Borders**: Magenta with glow effect
- **Text Labels**: Magenta with glow
- **Text Values**: Cyan with glow
- **Buttons**: Gradient magenta → cyan

## Visual Effects

### Active Animations
1. **Title Animation**
   - Gradient color shift (magenta ↔ cyan)
   - Constant smooth transition
   - Glowing text shadow

2. **Scanline Effect**
   - Subtle horizontal lines across entire screen
   - Slow vertical scrolling
   - CRT monitor aesthetic

3. **Block Rendering**
   - Solid color base
   - Gradient highlight (top-left to bottom-right)
   - Glowing border
   - Shadow blur effect

4. **Ghost Piece**
   - Same shape as current piece
   - 20% opacity
   - Shows landing position
   - Helps player aim

5. **Line Clear Animation**
   - Flashes white
   - 0.1 second duration
   - Then vanishes
   - Pieces above fall down

6. **Combo Notifications**
   - Large text appears center screen
   - "DOUBLE!" / "TRIPLE!" / "TETRIS!"
   - Yellow color with glow
   - Scales up then floats away
   - Fades out after 1 second

7. **Game Over Modal**
   - Slides in from top
   - Dark background overlay
   - Magenta border with glow
   - Pulsing "GAME OVER" text
   - Shows final score

## Interface Details

### Game Board (Main Canvas)
```
┌───────────────────────┐
│ Row 0  ▓▓▓▓▓▓▓▓▓▓    │ <- Top (game over line)
│ Row 1                  │
│ Row 2     ▓▓           │ <- Locked pieces (glowing)
│ Row 3                  │
│ ...                    │
│ Row 15    ▓▓           │
│ Row 16 ▓▓ ▓▓    ▓▓     │
│ Row 17 ▓▓▓▓▓▓▓▓▓▓     │ <- Current piece (solid)
│ Row 18 ▓▓▓▓▓▓▓▓▓▓     │
│ Row 19 ████████████    │ <- Ghost piece (transparent)
└───────────────────────┘
  300px wide × 600px tall
  30px blocks
```

### Preview Panels
```
┌──────────┐
│   NEXT   │ <- Magenta label
│ ┌──────┐ │
│ │  ▓▓  │ │ <- Next piece preview
│ │  ▓▓  │ │    Centered, glowing
│ │      │ │    120×120px canvas
│ └──────┘ │
└──────────┘

┌──────────┐
│   HOLD   │ <- Magenta label
│ ┌──────┐ │
│ │ ▓▓▓▓ │ │ <- Held piece
│ │      │ │    or empty
│ │      │ │    120×120px canvas
│ └──────┘ │
└──────────┘
```

### Stats Panel
```
┌──────────────┐
│    STATS     │ <- Cyan label
├──────────────┤
│ SCORE  1,250 │ <- Magenta label, cyan value
│ LINES    15  │
│ LEVEL     2  │
│ HIGH  5,000  │
└──────────────┘
```

## Responsive Design

### Desktop (Wide Screen)
- Side-by-side layout
- Game board on left
- Panels stacked on right
- Optimal spacing

### Tablet (Medium Screen)
- Still side-by-side
- Slightly tighter spacing
- Panels remain stacked

### Mobile (Narrow Screen)
- Stacks vertically
- Game board on top
- Panels below
- Still fully playable with keyboard

## Aesthetic Details

### Neon/Cyberpunk Theme
- **Dark background** for contrast
- **Bright, saturated colors** for pieces
- **Glowing effects** on all elements
- **Grid overlay** like circuit board
- **Scanlines** like old CRT
- **Gradient animations** for dynamism

### Typography
- **Font**: Courier New (monospace)
- **Title**: 3em, uppercase, gradient
- **Labels**: 1.2em, uppercase, glowing
- **Values**: 1.1em, bold, glowing
- **Controls**: 0.9em, readable

### Spacing & Layout
- **Grid gap**: 30px between sections
- **Panel padding**: 20px internal
- **Border width**: 2-3px with glow
- **Border radius**: 10px for panels, 25px for buttons
- **Canvas padding**: 10px around game board

## Game States Visual Feedback

### Playing
- Active piece falling
- Ghost piece visible
- Next/Hold pieces shown
- Stats updating
- Grid glowing

### Paused
- Game frozen in place
- "PAUSE" button highlighted
- No animations running
- Can still see board state

### Game Over
- Dark overlay appears
- Modal slides in
- "GAME OVER" pulsing
- Final score displayed
- "PLAY AGAIN" button glowing

### Line Clearing
- Completed rows flash white
- 0.1s animation
- Rows disappear
- Pieces fall down
- Combo notification appears

## Button States

### Normal
```
┌──────────────┐
│ START GAME   │ <- Gradient background
└──────────────┘    Magenta/cyan gradient
```

### Hover
```
┌──────────────┐
│ START GAME   │ <- Scaled up 1.05x
└──────────────┘    Stronger glow
```

### Active/Pressed
```
┌──────────────┐
│ START GAME   │ <- Scaled down 0.98x
└──────────────┘    Momentary feedback
```

## Summary

The game features a **stunning neon/cyberpunk aesthetic** with:
- Bright, glowing colors on dark background
- Smooth gradient animations
- CRT scanline effects
- Professional layout and spacing
- Clear visual hierarchy
- Excellent readability
- Eye-catching design

**This visual design makes the game stand out and creates an addictive, immersive experience!**
