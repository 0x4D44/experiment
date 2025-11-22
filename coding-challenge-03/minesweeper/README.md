# 💣 Minesweeper - Classic Game

A fully functional, beautifully designed Minesweeper game built with vanilla HTML, CSS, and JavaScript. Features smooth animations, sound effects, multiple difficulty levels, and persistent high scores.

## 🎮 Play Now

Simply open `index.html` in any modern web browser. No installation, build process, or dependencies required!

```bash
# Navigate to the directory
cd minesweeper

# Open in your default browser (Linux/Mac)
open index.html

# Or (Windows)
start index.html

# Or just double-click index.html in your file explorer
```

## ✨ Features

### Core Gameplay
- **Three Difficulty Levels**
  - 🟢 Beginner: 9×9 grid with 10 mines
  - 🟡 Intermediate: 16×16 grid with 40 mines
  - 🔴 Expert: 30×16 grid with 99 mines

- **Complete Minesweeper Logic**
  - Left-click to reveal cells
  - Right-click to flag/unflag suspected mines
  - First click is always safe (board regenerates if needed)
  - Recursive flood-fill for empty cells
  - Smart number calculation showing adjacent mine counts
  - Chord clicking (middle-click or shift+click on revealed numbers)

### Polish & User Experience
- **Beautiful Modern UI**
  - Gradient backgrounds and smooth animations
  - Cell reveal animations with scaling effect
  - Color-coded numbers (classic Minesweeper colors)
  - Responsive design that works on all screen sizes

- **Game Feedback**
  - Animated smiley face button showing game state (😊 → 😵 → 😎)
  - Live mine counter showing remaining flags
  - Timer tracking elapsed time
  - Game over modal with results

- **Sound Effects**
  - Cell reveal sound
  - Flag placement sound
  - Victory fanfare
  - Explosion sound on loss

- **High Score System**
  - Best times saved for each difficulty
  - Persistent storage using localStorage
  - Visual display of personal records

## 🎯 How to Play

### Basic Controls
1. **Left Click**: Reveal a cell
   - If it's a mine: Game over! 💥
   - If it's empty: Automatically reveals adjacent empty cells
   - If it has a number: Shows count of adjacent mines (1-8)

2. **Right Click**: Flag/unflag a cell
   - Mark cells you think contain mines
   - Flag counter updates automatically
   - Prevents accidental left-clicks on flagged cells

3. **Chord Click** (Middle-click or Shift+Left-click on revealed number):
   - Quick way to reveal all adjacent unflagged cells
   - Only works if correct number of flags are placed around the number
   - Be careful: wrong flags will detonate mines!

### Winning
- Reveal all non-mine cells
- Mines don't need to be flagged (but it helps!)
- Your time is automatically saved if it's a new record

### Tips
- Start from corners or edges for safer plays
- Use the number patterns to deduce mine locations
- Flag mines as you find them to keep track
- The first click is always safe, so don't worry!

## 🧪 Testing

The project includes a comprehensive test suite covering all game logic:

```bash
# Open the test suite
open test.html
```

### Test Coverage
The test suite validates:
- ✅ Mine placement logic (correct count, uniqueness, first-click safety)
- ✅ Number calculation (adjacent mine counting for all cell types)
- ✅ Flood fill algorithm (recursive reveal, boundary handling)
- ✅ Win/lose detection
- ✅ Flag counting
- ✅ Boundary validation
- ✅ Difficulty configurations
- ✅ Reveal logic edge cases

**30+ test cases** ensuring game quality and correctness!

## 🏗️ Architecture

### File Structure
```
minesweeper/
├── index.html      # Main game (single file, fully self-contained)
├── test.html       # Comprehensive test suite
└── README.md       # This file
```

### Code Organization

The game is built as a single-file application with clean separation of concerns:

**HTML Structure**
- Game container with header
- Difficulty selector buttons
- Game info panel (mine counter, smiley button, timer)
- Dynamic grid rendering
- Game over modal overlay
- High scores display

**CSS Styling**
- Modern gradient backgrounds
- Smooth animations and transitions
- Responsive grid layout
- Color-coded cell states
- Mobile-friendly design

**JavaScript Game Logic**
```javascript
class Minesweeper {
  // Core game state
  - grid[][]          // Mine positions and numbers
  - revealed[][]      // Which cells are revealed
  - flagged[][]       // Which cells are flagged

  // Game mechanics
  - placeMines()      // Random mine placement with first-click safety
  - revealCell()      // Recursive flood-fill algorithm
  - checkWin()        // Victory condition detection
  - handleClicks()    // Left, right, and chord clicking

  // UI & Feedback
  - updateTimer()     // Real-time timer updates
  - playSound()       // Web Audio API sound effects
  - saveHighScore()   // localStorage persistence
}
```

## 🎨 Customization

The game is easy to customize:

### Change Colors
Edit the CSS gradients in the `<style>` section:
```css
body {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}
```

### Adjust Cell Size
Modify the `.cell` class:
```css
.cell {
    width: 30px;   /* Change this */
    height: 30px;  /* And this */
}
```

### Add Custom Difficulties
Extend the `difficulties` object:
```javascript
this.difficulties = {
    custom: { rows: 20, cols: 20, mines: 50 }
};
```

### Disable Sounds
Comment out sound calls in the game logic:
```javascript
// this.sounds.reveal();
```

## 🔧 Technical Details

### Browser Compatibility
- ✅ Chrome/Edge (latest)
- ✅ Firefox (latest)
- ✅ Safari (latest)
- ✅ Mobile browsers (iOS Safari, Chrome Mobile)

### Technologies Used
- **Vanilla JavaScript (ES6+)**: No frameworks or dependencies
- **CSS3**: Gradients, animations, flexbox, grid
- **Web Audio API**: Dynamic sound generation
- **localStorage API**: High score persistence
- **HTML5**: Semantic markup

### Performance
- Instant load time (single HTML file)
- Smooth 60fps animations
- Efficient flood-fill algorithm with proper recursion
- Minimal memory footprint

## 🐛 Known Limitations

- Sound effects require user interaction (browser security policy)
- Very large custom grids may affect performance on older devices
- High scores are stored per-browser (not synced across devices)

## 🚀 Future Enhancements

Potential features for future versions:
- Custom difficulty creator
- Statistics tracking (games played, win rate)
- Dark/light theme toggle
- Question mark flags (? marker)
- Hint system
- Replay/undo functionality
- Online leaderboards
- Mobile-optimized touch controls
- Accessibility improvements (keyboard navigation, screen readers)

## 📝 License

This project is free to use, modify, and distribute. Built as a coding challenge demonstration.

## 🏆 Credits

Created as a fully functional coding challenge submission. Implements classic Minesweeper gameplay with modern web technologies and user experience enhancements.

---

**Enjoy the game! Happy mine sweeping! 💣🚩**
