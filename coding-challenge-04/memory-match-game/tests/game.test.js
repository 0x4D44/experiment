/**
 * Memory Game Unit Tests
 * Tests for core game logic and functionality
 */

// Simple test framework
class TestRunner {
    constructor() {
        this.tests = [];
        this.passed = 0;
        this.failed = 0;
    }

    test(name, fn) {
        this.tests.push({ name, fn });
    }

    async run() {
        console.log('\n🧪 Running Memory Game Tests\n');
        console.log('='.repeat(50));

        for (const test of this.tests) {
            try {
                await test.fn();
                this.passed++;
                console.log(`✅ PASS: ${test.name}`);
            } catch (error) {
                this.failed++;
                console.log(`❌ FAIL: ${test.name}`);
                console.log(`   Error: ${error.message}`);
            }
        }

        console.log('='.repeat(50));
        console.log(`\n📊 Results: ${this.passed} passed, ${this.failed} failed\n`);

        return this.failed === 0;
    }
}

// Assertion helper
function assert(condition, message) {
    if (!condition) {
        throw new Error(message || 'Assertion failed');
    }
}

function assertEqual(actual, expected, message) {
    if (actual !== expected) {
        throw new Error(message || `Expected ${expected}, got ${actual}`);
    }
}

function assertNotEqual(actual, expected, message) {
    if (actual === expected) {
        throw new Error(message || `Expected value to not equal ${expected}`);
    }
}

function assertGreaterThan(actual, expected, message) {
    if (actual <= expected) {
        throw new Error(message || `Expected ${actual} to be greater than ${expected}`);
    }
}

// Mock localStorage for Node.js environment
if (typeof localStorage === 'undefined') {
    global.localStorage = {
        storage: {},
        getItem(key) {
            return this.storage[key] || null;
        },
        setItem(key, value) {
            this.storage[key] = value.toString();
        },
        removeItem(key) {
            delete this.storage[key];
        },
        clear() {
            this.storage = {};
        }
    };
}

// Load MemoryGame class (for Node.js environment)
let MemoryGame;
if (typeof require !== 'undefined') {
    try {
        MemoryGame = require('../js/game.js');
    } catch (e) {
        console.log('Running in browser environment');
    }
}

// Test Suite
const runner = new TestRunner();

// Initialization Tests
runner.test('Game initializes with default values', () => {
    const game = new MemoryGame();
    assertEqual(game.difficulty, 'medium');
    assertEqual(game.theme, 'emojis');
    assertEqual(game.moves, 0);
    assertEqual(game.timer, 0);
    assertEqual(game.matchedPairs, 0);
    assert(game.cards.length === 0);
});

runner.test('Game generates correct number of cards for easy difficulty', () => {
    const game = new MemoryGame();
    game.init('easy', 'emojis');
    assertEqual(game.cards.length, 16); // 8 pairs * 2
});

runner.test('Game generates correct number of cards for medium difficulty', () => {
    const game = new MemoryGame();
    game.init('medium', 'emojis');
    assertEqual(game.cards.length, 36); // 18 pairs * 2
});

runner.test('Game generates correct number of cards for hard difficulty', () => {
    const game = new MemoryGame();
    game.init('hard', 'emojis');
    assertEqual(game.cards.length, 64); // 32 pairs * 2
});

runner.test('All cards have valid properties', () => {
    const game = new MemoryGame();
    game.init('easy', 'emojis');

    game.cards.forEach(card => {
        assert(typeof card.id === 'number', 'Card has numeric id');
        assert(typeof card.value === 'string', 'Card has string value');
        assert(card.flipped === false, 'Card starts unflipped');
        assert(card.matched === false, 'Card starts unmatched');
    });
});

runner.test('Cards are shuffled (not in order)', () => {
    const game = new MemoryGame();
    game.init('easy', 'emojis');

    // Check that cards are not in sequential pairs
    let inOrder = true;
    for (let i = 0; i < game.cards.length - 1; i += 2) {
        if (game.cards[i].value !== game.cards[i + 1].value) {
            inOrder = false;
            break;
        }
    }

    assert(!inOrder, 'Cards should be shuffled');
});

runner.test('Each card value appears exactly twice', () => {
    const game = new MemoryGame();
    game.init('easy', 'emojis');

    const valueCount = {};
    game.cards.forEach(card => {
        valueCount[card.value] = (valueCount[card.value] || 0) + 1;
    });

    Object.values(valueCount).forEach(count => {
        assertEqual(count, 2, 'Each value should appear exactly twice');
    });
});

// Game Logic Tests
runner.test('Flipping a card marks it as flipped', async () => {
    const game = new MemoryGame();
    game.init('easy', 'emojis');

    const cardId = game.cards[0].id;
    await game.flipCard(cardId);

    const card = game.cards.find(c => c.id === cardId);
    assert(card.flipped === true, 'Card should be flipped');
});

runner.test('Flipping first card starts the timer', async () => {
    const game = new MemoryGame();
    game.init('easy', 'emojis');

    assertEqual(game.gameStarted, false);

    await game.flipCard(game.cards[0].id);

    assert(game.gameStarted === true, 'Game should start');
    assert(game.timerInterval !== null, 'Timer should be running');

    game.stopTimer();
});

runner.test('Cannot flip already flipped card', async () => {
    const game = new MemoryGame();
    game.init('easy', 'emojis');

    const cardId = game.cards[0].id;
    await game.flipCard(cardId);
    const result = await game.flipCard(cardId);

    assertEqual(result, false, 'Should not be able to flip already flipped card');
});

runner.test('Cannot flip matched card', async () => {
    const game = new MemoryGame();
    game.init('easy', 'emojis');

    game.cards[0].matched = true;
    const result = await game.flipCard(game.cards[0].id);

    assertEqual(result, false, 'Should not be able to flip matched card');
});

runner.test('Matching cards are marked as matched', async () => {
    const game = new MemoryGame();
    game.init('easy', 'emojis');

    // Find two cards with same value
    const firstValue = game.cards[0].value;
    const matchingCard = game.cards.find((c, i) => i > 0 && c.value === firstValue);

    await game.flipCard(game.cards[0].id);
    await game.flipCard(matchingCard.id);

    // Wait for match animation
    await new Promise(resolve => setTimeout(resolve, 700));

    const card1 = game.cards.find(c => c.id === game.cards[0].id);
    const card2 = game.cards.find(c => c.id === matchingCard.id);

    assert(card1.matched === true, 'First card should be matched');
    assert(card2.matched === true, 'Second card should be matched');
    assertEqual(game.matchedPairs, 1, 'Matched pairs should increment');
});

runner.test('Non-matching cards flip back', async () => {
    const game = new MemoryGame();
    game.init('easy', 'emojis');

    // Find two cards with different values
    let card1, card2;
    for (let i = 0; i < game.cards.length; i++) {
        for (let j = i + 1; j < game.cards.length; j++) {
            if (game.cards[i].value !== game.cards[j].value) {
                card1 = game.cards[i];
                card2 = game.cards[j];
                break;
            }
        }
        if (card1 && card2) break;
    }

    await game.flipCard(card1.id);
    await game.flipCard(card2.id);

    // Wait for mismatch animation
    await new Promise(resolve => setTimeout(resolve, 1100));

    const updatedCard1 = game.cards.find(c => c.id === card1.id);
    const updatedCard2 = game.cards.find(c => c.id === card2.id);

    assert(updatedCard1.flipped === false, 'First card should flip back');
    assert(updatedCard2.flipped === false, 'Second card should flip back');
});

runner.test('Move counter increments correctly', async () => {
    const game = new MemoryGame();
    game.init('easy', 'emojis');

    assertEqual(game.moves, 0);

    await game.flipCard(game.cards[0].id);
    assertEqual(game.moves, 0, 'Moves should not increment on first card');

    await game.flipCard(game.cards[1].id);
    assertEqual(game.moves, 1, 'Moves should increment when second card is flipped');

    await new Promise(resolve => setTimeout(resolve, 1100));

    await game.flipCard(game.cards[2].id);
    await game.flipCard(game.cards[3].id);
    assertEqual(game.moves, 2, 'Moves should continue to increment');
});

runner.test('Check win condition returns correct value', () => {
    const game = new MemoryGame();
    game.init('easy', 'emojis');

    assertEqual(game.checkWin(), false, 'Should not win initially');

    game.matchedPairs = 7;
    assertEqual(game.checkWin(), false, 'Should not win with 7 pairs');

    game.matchedPairs = 8;
    assertEqual(game.checkWin(), true, 'Should win with all 8 pairs matched');
});

// Score and Timer Tests
runner.test('Timer increments correctly', async () => {
    const game = new MemoryGame();
    game.init('easy', 'emojis');

    game.startTimer();

    await new Promise(resolve => setTimeout(resolve, 1500));

    assert(game.timer >= 1, 'Timer should increment');

    game.stopTimer();
});

runner.test('Timer formats correctly', () => {
    const game = new MemoryGame();

    game.timer = 0;
    assertEqual(game.getFormattedTime(), '0:00');

    game.timer = 30;
    assertEqual(game.getFormattedTime(), '0:30');

    game.timer = 60;
    assertEqual(game.getFormattedTime(), '1:00');

    game.timer = 125;
    assertEqual(game.getFormattedTime(), '2:05');
});

runner.test('Score calculation is correct', () => {
    const game = new MemoryGame();
    game.init('easy', 'emojis');

    game.moves = 10;
    game.timer = 30;

    const score = game.calculateScore();
    assertEqual(score, 130); // (10 * 10) + 30
});

runner.test('Game statistics are accurate', () => {
    const game = new MemoryGame();
    game.init('easy', 'emojis');

    game.moves = 15;
    game.timer = 45;
    game.matchedPairs = 3;

    const stats = game.getStats();

    assertEqual(stats.moves, 15);
    assertEqual(stats.time, 45);
    assertEqual(stats.formattedTime, '0:45');
    assertEqual(stats.matchedPairs, 3);
    assertEqual(stats.totalPairs, 8);
    assertEqual(stats.score, 195); // (15 * 10) + 45
});

// Theme Tests
runner.test('All themes have sufficient emojis', () => {
    const game = new MemoryGame();

    Object.entries(game.themes).forEach(([theme, emojis]) => {
        assertGreaterThan(
            emojis.length,
            31,
            `Theme ${theme} should have at least 32 emojis for hard mode`
        );
    });
});

runner.test('Theme selection works correctly', () => {
    const game = new MemoryGame();

    game.init('easy', 'animals');
    const animalEmojis = game.themes.animals;

    game.cards.forEach(card => {
        assert(
            animalEmojis.includes(card.value),
            `Card value should be from animals theme`
        );
    });
});

// Reset Tests
runner.test('Game resets correctly', () => {
    const game = new MemoryGame();
    game.init('medium', 'food');

    // Modify game state
    game.moves = 10;
    game.timer = 30;
    game.matchedPairs = 5;
    game.gameStarted = true;

    // Reset
    game.reset();

    assertEqual(game.moves, 0);
    assertEqual(game.timer, 0);
    assertEqual(game.matchedPairs, 0);
    assert(game.gameStarted === false);
    assertEqual(game.cards.length, 0);
});

runner.test('New game initializes fresh state', () => {
    const game = new MemoryGame();
    game.init('easy', 'emojis');

    // Play some moves
    game.moves = 10;
    game.timer = 30;
    game.matchedPairs = 5;

    // Start new game
    game.init('medium', 'food');

    assertEqual(game.moves, 0);
    assertEqual(game.timer, 0);
    assertEqual(game.matchedPairs, 0);
    assertEqual(game.difficulty, 'medium');
    assertEqual(game.theme, 'food');
    assertEqual(game.cards.length, 36);
});

// LocalStorage Tests
runner.test('Best score saves and loads correctly', () => {
    const game = new MemoryGame();
    game.init('easy', 'emojis');

    // Clear any existing score
    localStorage.removeItem('bestScore_easy_emojis');

    game.moves = 10;
    game.timer = 30;

    const isNewRecord = game.saveBestScore();
    assert(isNewRecord === true, 'Should be new record');

    const loadedScore = game.loadBestScore();
    assertEqual(loadedScore, 130);

    // Try a worse score
    game.moves = 20;
    game.timer = 50;

    const isNewRecord2 = game.saveBestScore();
    assert(isNewRecord2 === false, 'Should not be new record');

    const loadedScore2 = game.loadBestScore();
    assertEqual(loadedScore2, 130, 'Best score should remain unchanged');
});

// Run tests
if (typeof window !== 'undefined') {
    // Browser environment
    window.addEventListener('load', () => {
        runner.run().then(success => {
            if (success) {
                console.log('✨ All tests passed!');
            } else {
                console.log('⚠️ Some tests failed');
            }
        });
    });
} else {
    // Node environment
    runner.run().then(success => {
        process.exit(success ? 0 : 1);
    });
}
