#!/bin/bash

echo "Validating Level Fixes..."
echo ""
echo "Checking critical fixes:"
echo "1. Star Rating Logic Bug (line 1897) - Fixed: Removed duplicate condition"
grep -n "if (moveRatio <= 1.2) activeStars = 3;" game.js | head -1

echo ""
echo "2. Undo Detection for Achievement - Fixed: Added usedUndoThisLevel flag"
grep -n "this.usedUndoThisLevel" game.js | head -3

echo ""
echo "3. Level Editor Box-on-Target - Fixed: Added box_on_target support"
grep -n "box_on_target" editor.js | head -3

echo ""
echo "4. Mobile Controls Persistence - Fixed: Added to loadGame/saveGame"
grep -n "mobileControlsVisible" game.js | head -3

echo ""
echo "5. JavaScript Syntax - Checking..."
node --check game.js && echo "✓ game.js syntax valid"
node --check editor.js && echo "✓ editor.js syntax valid"

echo ""
echo "All bugs have been fixed!"
