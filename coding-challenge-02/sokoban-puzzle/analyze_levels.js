// Simple script to analyze levels
const fs = require('fs');
const gameJs = fs.readFileSync('game.js', 'utf8');

// Extract level data manually
function countSymbols(gridArray) {
    let players = 0;
    let boxes = 0;
    let targets = 0;

    gridArray.forEach(row => {
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

    return { players, boxes, targets };
}

// Parse the LEVEL_PACKS from the file
const levelPacksMatch = gameJs.match(/const LEVEL_PACKS = \{[\s\S]*?\n\};/);
if (!levelPacksMatch) {
    console.log("Could not find LEVEL_PACKS");
    process.exit(1);
}

// Use eval to parse the object (in a real scenario, use a proper parser)
const LEVEL_PACKS = eval('(' + levelPacksMatch[0].replace('const LEVEL_PACKS = ', '') + ')');

let levelNum = 1;
const brokenLevels = [];

for (const packKey in LEVEL_PACKS) {
    const pack = LEVEL_PACKS[packKey];
    pack.levels.forEach((level, idx) => {
        const counts = countSymbols(level.grid);
        const isBroken = (counts.players !== 1 || counts.boxes !== counts.targets || counts.boxes === 0);

        if (isBroken) {
            brokenLevels.push({
                level: levelNum,
                pack: packKey,
                idx: idx,
                name: level.name,
                ...counts,
                issue: counts.players !== 1 ? 'PLAYER' : counts.boxes !== counts.targets ? 'BOX/TARGET' : 'NO BOXES'
            });
        }

        console.log(`Level ${levelNum} (${level.name}): Players=${counts.players}, Boxes=${counts.boxes}, Targets=${counts.targets} ${isBroken ? '❌ BROKEN' : '✅'}`);

        levelNum++;
    });
}

console.log('\n=== SUMMARY OF BROKEN LEVELS ===');
brokenLevels.forEach(level => {
    console.log(`Level ${level.level} (${level.name}): Players=${level.players}, Boxes=${level.boxes}, Targets=${level.targets} - Issue: ${level.issue}`);
});
console.log(`\nTotal broken: ${brokenLevels.length}`);
