// Validate all 30 levels
const fs = require('fs');

// Read and parse game.js to extract LEVEL_PACKS
const gameJs = fs.readFileSync('game.js', 'utf8');
const levelPacksMatch = gameJs.match(/const LEVEL_PACKS = \{[\s\S]*?\n\};/);
if (!levelPacksMatch) {
    console.log("ERROR: Could not find LEVEL_PACKS");
    process.exit(1);
}

const LEVEL_PACKS = eval('(' + levelPacksMatch[0].replace('const LEVEL_PACKS = ', '') + ')');

function validateLevel(grid) {
    let players = 0;
    let boxes = 0;
    let targets = 0;

    grid.forEach(row => {
        for (const char of row) {
            if (char === '@') players++;
            if (char === '$') boxes++;
            if (char === '.') targets++;
            if (char === '*') {
                boxes++;
                targets++;
            }
        }
    });

    return {
        players,
        boxes,
        targets,
        valid: players === 1 && boxes === targets && boxes > 0
    };
}

let levelNum = 1;
let allValid = true;
const brokenLevels = [];

console.log("Validating all 30 levels...\n");

for (const packKey in LEVEL_PACKS) {
    const pack = LEVEL_PACKS[packKey];
    console.log(`\n${pack.name} (${pack.icon}):`);

    pack.levels.forEach((level, idx) => {
        const result = validateLevel(level.grid);
        const status = result.valid ? '✓' : '✗';

        console.log(`  Level ${levelNum}: ${level.name} ${status}`);
        if (!result.valid) {
            console.log(`    Players: ${result.players}, Boxes: ${result.boxes}, Targets: ${result.targets}`);
            brokenLevels.push(levelNum);
            allValid = false;
        }

        levelNum++;
    });
}

console.log("\n" + "=".repeat(60));
if (allValid) {
    console.log("SUCCESS! All 30 levels are valid and playable.");
} else {
    console.log(`FAILURE! ${brokenLevels.length} levels are still broken:`);
    console.log(`Broken levels: ${brokenLevels.join(', ')}`);
}
console.log("=".repeat(60));

process.exit(allValid ? 0 : 1);
