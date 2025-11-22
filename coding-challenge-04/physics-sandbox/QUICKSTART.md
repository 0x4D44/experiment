# Physics Sandbox - Quick Start

## Fastest Way to Run

### Option 1: Direct Browser (Easiest)
Simply double-click `index.html` to open in your default browser!

### Option 2: Use Start Script (Recommended)
```bash
./start.sh
```
Then open http://localhost:8000 in your browser.

### Option 3: Python Server
```bash
python3 -m http.server 8000
```
Then open http://localhost:8000 in your browser.

### Option 4: Node.js Server
```bash
npx http-server -p 8000
```
Then open http://localhost:8000 in your browser.

## First 60 Seconds

1. **Open the app** - You'll see a beautiful dark space with physics objects
2. **Click anywhere** - Spawn colorful circles
3. **Drag objects** - Click and drag to throw them
4. **Press R** - Watch rain of 50 objects fall
5. **Press G** - Toggle gravity and watch objects float
6. **Press Space** - Pause to admire your creation

## Run Tests

```bash
npm test
# or
node physics-engine.test.js
```

Expected output: **36 tests passed, 0 failed**

## File Structure

```
physics-sandbox/
├── index.html              # Main app (open this!)
├── physics-engine.js       # Custom physics engine
├── renderer.js             # Beautiful visual effects
├── app.js                  # Main application logic
├── physics-engine.test.js  # Comprehensive tests
├── start.sh               # Quick start script
├── package.json           # Project metadata
├── README.md              # Full documentation
├── DEMO.md                # Demo walkthrough
└── QUICKSTART.md          # This file
```

## Key Features

- Real physics simulation (gravity, collisions, momentum)
- Stunning visual effects (trails, glows, shadows)
- Interactive controls (drag, throw, spawn)
- 60 FPS smooth animation
- 36 comprehensive unit tests
- No build process or dependencies needed!

## Next Steps

1. Read `DEMO.md` for a guided tour
2. Check `README.md` for full documentation
3. Review code to see clean architecture
4. Run tests to verify everything works

## Keyboard Shortcuts

- `1` - Circle tool
- `2` - Box tool
- `G` - Toggle gravity
- `T` - Toggle trails
- `Space` - Pause
- `C` - Clear all
- `R` - Rain effect

## Support

- All modern browsers supported
- Works on desktop and mobile
- Touch screen compatible
- Responsive design

Enjoy the physics sandbox!
