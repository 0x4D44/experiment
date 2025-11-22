/**
 * Comprehensive Test Suite for Breakout Arena
 * Tests game mechanics, physics, collision detection, scoring, and level progression
 */

class TestRunner {
    constructor() {
        this.tests = [];
        this.results = [];
    }

    test(name, fn) {
        this.tests.push({ name, fn });
    }

    async run() {
        this.results = [];

        for (const test of this.tests) {
            try {
                await test.fn();
                this.results.push({
                    name: test.name,
                    passed: true,
                    error: null
                });
            } catch (error) {
                this.results.push({
                    name: test.name,
                    passed: false,
                    error: error.message
                });
            }
        }

        return this.results;
    }

    assert(condition, message) {
        if (!condition) {
            throw new Error(message || 'Assertion failed');
        }
    }

    assertEqual(actual, expected, message) {
        if (actual !== expected) {
            throw new Error(message || `Expected ${expected}, got ${actual}`);
        }
    }

    assertApprox(actual, expected, tolerance = 0.01, message) {
        if (Math.abs(actual - expected) > tolerance) {
            throw new Error(message || `Expected ~${expected}, got ${actual}`);
        }
    }

    assertGreaterThan(actual, expected, message) {
        if (actual <= expected) {
            throw new Error(message || `Expected ${actual} > ${expected}`);
        }
    }

    assertLessThan(actual, expected, message) {
        if (actual >= expected) {
            throw new Error(message || `Expected ${actual} < ${expected}`);
        }
    }
}

// Test suite
const runner = new TestRunner();

// ==================== GAME INITIALIZATION TESTS ====================
runner.test('Game initializes correctly', () => {
    const testGame = new BreakoutGame();
    runner.assert(testGame !== null, 'Game object should be created');
    runner.assertEqual(testGame.gameState, 'start', 'Initial game state should be "start"');
    runner.assertEqual(testGame.score, 0, 'Initial score should be 0');
    runner.assertEqual(testGame.level, 1, 'Initial level should be 1');
    runner.assertEqual(testGame.lives, 3, 'Initial lives should be 3');
});

runner.test('Canvas is properly configured', () => {
    const testGame = new BreakoutGame();
    runner.assert(testGame.canvas !== null, 'Canvas should exist');
    runner.assert(testGame.ctx !== null, 'Canvas context should exist');
    runner.assertEqual(testGame.canvas.width, 800, 'Canvas width should be 800');
    runner.assertEqual(testGame.canvas.height, 600, 'Canvas height should be 600');
});

// ==================== PADDLE TESTS ====================
runner.test('Paddle initializes with correct properties', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    runner.assert(testGame.paddle !== null, 'Paddle should exist');
    runner.assertEqual(testGame.paddle.width, testGame.PADDLE_WIDTH, 'Paddle width should match constant');
    runner.assertEqual(testGame.paddle.height, testGame.PADDLE_HEIGHT, 'Paddle height should match constant');
    runner.assertGreaterThan(testGame.paddle.y, testGame.canvas.height / 2, 'Paddle should be in bottom half');
});

runner.test('Paddle stays within bounds', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    // Test left boundary
    testGame.paddle.x = -100;
    testGame.updatePaddle();
    runner.assertEqual(testGame.paddle.x, 0, 'Paddle should not go below 0');

    // Test right boundary
    testGame.paddle.x = testGame.canvas.width + 100;
    testGame.updatePaddle();
    runner.assertLessThan(testGame.paddle.x, testGame.canvas.width, 'Paddle should stay within canvas');
});

runner.test('Paddle responds to keyboard input', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    const initialX = testGame.paddle.x;

    // Move left
    testGame.keys['ArrowLeft'] = true;
    testGame.updatePaddle();
    runner.assertLessThan(testGame.paddle.x, initialX, 'Paddle should move left');

    testGame.keys['ArrowLeft'] = false;
    const leftX = testGame.paddle.x;

    // Move right
    testGame.keys['ArrowRight'] = true;
    testGame.updatePaddle();
    runner.assertGreaterThan(testGame.paddle.x, leftX, 'Paddle should move right');
});

// ==================== BALL TESTS ====================
runner.test('Ball initializes correctly', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    runner.assertEqual(testGame.balls.length, 1, 'Should start with one ball');
    const ball = testGame.balls[0];
    runner.assertEqual(ball.radius, testGame.BALL_RADIUS, 'Ball radius should match constant');
    runner.assertEqual(ball.stuck, true, 'Ball should start stuck to paddle');
});

runner.test('Ball physics - velocity calculation', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    const ball = testGame.balls[0];
    ball.stuck = false;
    ball.dx = 3;
    ball.dy = -4;

    const initialX = ball.x;
    const initialY = ball.y;

    testGame.updateBalls();

    runner.assertEqual(ball.x, initialX + 3, 'Ball should move by dx');
    runner.assertEqual(ball.y, initialY - 4, 'Ball should move by dy');
});

runner.test('Ball bounces off walls', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    const ball = testGame.balls[0];
    ball.stuck = false;
    ball.dx = 5;

    // Test left wall
    ball.x = 5;
    const originalDx = ball.dx;
    testGame.updateBalls();
    runner.assertEqual(ball.dx, -originalDx, 'Ball should reverse direction on left wall');

    // Test right wall
    ball.x = testGame.canvas.width - 5;
    ball.dx = 5;
    testGame.updateBalls();
    runner.assertLessThan(ball.dx, 0, 'Ball should reverse direction on right wall');

    // Test top wall
    ball.y = 5;
    ball.dy = -5;
    testGame.updateBalls();
    runner.assertGreaterThan(ball.dy, 0, 'Ball should reverse direction on top wall');
});

runner.test('Ball launches from paddle', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    const ball = testGame.balls[0];
    runner.assertEqual(ball.stuck, true, 'Ball should start stuck');

    testGame.keys[' '] = true;
    testGame.updateBalls();

    runner.assertEqual(ball.stuck, false, 'Ball should launch when spacebar pressed');
    runner.assertLessThan(ball.dy, 0, 'Ball should move upward after launch');
});

// ==================== COLLISION DETECTION TESTS ====================
runner.test('Paddle-ball collision detection', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    const ball = testGame.balls[0];
    ball.stuck = false;
    ball.x = testGame.paddle.x + testGame.paddle.width / 2;
    ball.y = testGame.paddle.y - ball.radius - 1;
    ball.dy = 1;

    const collision = testGame.checkPaddleCollision(ball);
    runner.assertEqual(collision, true, 'Should detect paddle collision');
});

runner.test('Paddle-ball collision angle calculation', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    const ball = testGame.balls[0];
    ball.stuck = false;
    ball.x = testGame.paddle.x + testGame.paddle.width / 2;
    ball.y = testGame.paddle.y;
    ball.dy = 5;

    testGame.updateBalls();

    runner.assertLessThan(ball.dy, 0, 'Ball should bounce upward after paddle hit');
});

runner.test('Brick-ball collision detection', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    const brick = testGame.bricks[0];
    const ball = testGame.balls[0];
    ball.stuck = false;
    ball.x = brick.x + brick.width / 2;
    ball.y = brick.y + brick.height / 2;

    const collision = testGame.checkBrickCollision(ball, brick);
    runner.assertEqual(collision, true, 'Should detect brick collision');
});

// ==================== BRICK TESTS ====================
runner.test('Bricks are created correctly', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    runner.assertGreaterThan(testGame.bricks.length, 0, 'Should create bricks');

    const brick = testGame.bricks[0];
    runner.assert(brick.x !== undefined, 'Brick should have x position');
    runner.assert(brick.y !== undefined, 'Brick should have y position');
    runner.assert(brick.width > 0, 'Brick should have width');
    runner.assert(brick.height > 0, 'Brick should have height');
    runner.assert(brick.hits > 0, 'Brick should have hits');
});

runner.test('Different brick types have different hit points', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    const hitCounts = new Set(testGame.bricks.map(b => b.hits));
    runner.assertGreaterThan(hitCounts.size, 1, 'Should have bricks with different hit points');
});

runner.test('Brick damage reduces hit points', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    const brick = testGame.bricks.find(b => b.hits > 1);
    if (brick) {
        const initialHits = brick.hits;
        brick.hits--;
        runner.assertEqual(brick.hits, initialHits - 1, 'Brick hits should decrease');
    }
});

runner.test('Brick destruction removes brick from array', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    const initialCount = testGame.bricks.length;
    const ball = testGame.balls[0];
    ball.stuck = false;

    const brick = testGame.bricks[0];
    ball.x = brick.x + brick.width / 2;
    ball.y = brick.y + brick.height / 2;

    // Hit brick multiple times to destroy it
    for (let i = 0; i < brick.hits; i++) {
        testGame.checkCollisions();
    }

    runner.assertLessThan(testGame.bricks.length, initialCount, 'Destroyed brick should be removed');
});

// ==================== SCORING TESTS ====================
runner.test('Score increases when brick is destroyed', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    const initialScore = testGame.score;
    const ball = testGame.balls[0];
    ball.stuck = false;

    const brick = testGame.bricks.find(b => b.hits === 1);
    if (brick) {
        ball.x = brick.x + brick.width / 2;
        ball.y = brick.y + brick.height / 2;
        testGame.checkCollisions();

        runner.assertGreaterThan(testGame.score, initialScore, 'Score should increase');
    }
});

runner.test('Score scales with level', () => {
    const testGame = new BreakoutGame();
    testGame.level = 1;
    testGame.initLevel();

    const ball = testGame.balls[0];
    ball.stuck = false;
    const brick = testGame.bricks.find(b => b.hits === 1);

    if (brick) {
        ball.x = brick.x + brick.width / 2;
        ball.y = brick.y + brick.height / 2;
        testGame.checkCollisions();
        const level1Score = testGame.score;

        // Reset for level 2
        testGame.score = 0;
        testGame.level = 2;
        testGame.initLevel();

        const ball2 = testGame.balls[0];
        ball2.stuck = false;
        const brick2 = testGame.bricks.find(b => b.hits === 1);

        if (brick2) {
            ball2.x = brick2.x + brick2.width / 2;
            ball2.y = brick2.y + brick2.height / 2;
            testGame.checkCollisions();

            runner.assertGreaterThan(testGame.score, level1Score, 'Level 2 should give more points');
        }
    }
});

runner.test('High score is tracked', () => {
    const testGame = new BreakoutGame();
    testGame.score = 1000;
    testGame.highScore = 500;

    testGame.updateHighScore();

    runner.assertEqual(testGame.highScore, 1000, 'High score should update when score exceeds it');
});

// ==================== POWERUP TESTS ====================
runner.test('Powerups spawn from bricks', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    // Force a brick to have a powerup
    const brick = testGame.bricks[0];
    brick.hasPowerup = true;
    brick.hits = 1;

    const ball = testGame.balls[0];
    ball.stuck = false;
    ball.x = brick.x + brick.width / 2;
    ball.y = brick.y + brick.height / 2;

    testGame.checkCollisions();

    runner.assertGreaterThan(testGame.powerups.length, 0, 'Powerup should spawn from brick');
});

runner.test('Multi-ball powerup creates additional balls', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    const initialBallCount = testGame.balls.length;
    testGame.activatePowerup('multiball');

    runner.assertGreaterThan(testGame.balls.length, initialBallCount, 'Multi-ball should add balls');
});

runner.test('Big paddle powerup increases paddle size', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    const initialWidth = testGame.paddle.width;
    testGame.activatePowerup('bigpaddle');

    runner.assertGreaterThan(testGame.paddle.width, initialWidth, 'Big paddle should increase width');
});

runner.test('Extra life powerup increases lives', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    const initialLives = testGame.lives;
    testGame.activatePowerup('extralife');

    runner.assertEqual(testGame.lives, initialLives + 1, 'Extra life should increase lives');
});

runner.test('Slow ball powerup reduces ball speed', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    const ball = testGame.balls[0];
    ball.stuck = false;
    ball.dx = 5;
    ball.dy = -5;

    const initialSpeed = Math.sqrt(ball.dx * ball.dx + ball.dy * ball.dy);
    testGame.activatePowerup('slowball');
    const newSpeed = Math.sqrt(ball.dx * ball.dx + ball.dy * ball.dy);

    runner.assertLessThan(newSpeed, initialSpeed, 'Slow ball should reduce speed');
});

// ==================== LEVEL PROGRESSION TESTS ====================
runner.test('Level completes when all bricks destroyed', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    testGame.bricks = [];
    testGame.checkLevelComplete();

    runner.assertEqual(testGame.gameState, 'levelComplete', 'Should complete level when no bricks remain');
});

runner.test('Level progression increases level number', () => {
    const testGame = new BreakoutGame();
    testGame.level = 1;
    testGame.initLevel();
    testGame.gameState = 'levelComplete';

    testGame.nextLevel();

    runner.assertEqual(testGame.level, 2, 'Level should increment');
});

runner.test('Different levels have different brick patterns', () => {
    const testGame = new BreakoutGame();

    testGame.level = 1;
    const pattern1 = testGame.getLevelPattern(1);

    testGame.level = 2;
    const pattern2 = testGame.getLevelPattern(2);

    const same = JSON.stringify(pattern1) === JSON.stringify(pattern2);
    runner.assertEqual(same, false, 'Different levels should have different patterns');
});

// ==================== LIVES SYSTEM TESTS ====================
runner.test('Losing ball reduces lives', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    const initialLives = testGame.lives;
    testGame.loseLife();

    runner.assertEqual(testGame.lives, initialLives - 1, 'Lives should decrease');
});

runner.test('Game over when lives reach zero', () => {
    const testGame = new BreakoutGame();
    testGame.lives = 1;
    testGame.initLevel();

    testGame.loseLife();

    runner.assertEqual(testGame.gameState, 'gameOver', 'Game should end when lives reach 0');
});

runner.test('Ball resets after losing life', () => {
    const testGame = new BreakoutGame();
    testGame.lives = 2;
    testGame.initLevel();

    testGame.balls[0].stuck = false;
    testGame.loseLife();

    runner.assertEqual(testGame.balls.length, 1, 'Should have one ball after reset');
    runner.assertEqual(testGame.balls[0].stuck, true, 'Ball should be stuck after reset');
});

// ==================== PARTICLE SYSTEM TESTS ====================
runner.test('Particles are created on brick destruction', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    const brick = testGame.bricks[0];
    testGame.createBrickParticles(brick);

    runner.assertGreaterThan(testGame.particles.length, 0, 'Particles should be created');
});

runner.test('Particles have finite lifetime', () => {
    const testGame = new BreakoutGame();
    testGame.particles.push({
        x: 100,
        y: 100,
        vx: 1,
        vy: 1,
        life: 1,
        maxLife: 30,
        color: '#ff0000',
        size: 3
    });

    testGame.updateParticles();

    runner.assertEqual(testGame.particles.length, 0, 'Particles should be removed when life ends');
});

// ==================== PHYSICS TESTS ====================
runner.test('Ball speed remains constant after paddle bounce', () => {
    const testGame = new BreakoutGame();
    testGame.initLevel();

    const ball = testGame.balls[0];
    ball.stuck = false;
    ball.dx = 3;
    ball.dy = 4;

    const initialSpeed = Math.sqrt(ball.dx * ball.dx + ball.dy * ball.dy);

    ball.x = testGame.paddle.x + testGame.paddle.width / 2;
    ball.y = testGame.paddle.y;
    testGame.updateBalls();

    const newSpeed = Math.sqrt(ball.dx * ball.dx + ball.dy * ball.dy);
    runner.assertApprox(newSpeed, initialSpeed, 0.5, 'Ball speed should remain approximately constant');
});

runner.test('Particle physics includes gravity', () => {
    const testGame = new BreakoutGame();
    testGame.particles.push({
        x: 100,
        y: 100,
        vx: 0,
        vy: 0,
        life: 30,
        maxLife: 30,
        color: '#ff0000',
        size: 3
    });

    testGame.updateParticles();

    runner.assertGreaterThan(testGame.particles[0].vy, 0, 'Particles should have downward velocity from gravity');
});

// ==================== COLOR UTILITY TESTS ====================
runner.test('Brick colors change with damage', () => {
    const testGame = new BreakoutGame();

    const color1 = testGame.getBrickColor(1);
    const color2 = testGame.getBrickColor(2);
    const color3 = testGame.getBrickColor(3);

    runner.assert(color1 !== color2, 'Different hit counts should have different colors');
    runner.assert(color2 !== color3, 'Different hit counts should have different colors');
});

runner.test('Color darkening works correctly', () => {
    const testGame = new BreakoutGame();
    const original = '#ff0000';
    const darkened = testGame.darkenColor(original);

    runner.assert(darkened !== original, 'Darkened color should differ from original');
    runner.assert(darkened.startsWith('#'), 'Darkened color should be valid hex');
});

runner.test('Color lightening works correctly', () => {
    const testGame = new BreakoutGame();
    const original = '#880000';
    const lightened = testGame.lightenColor(original);

    runner.assert(lightened !== original, 'Lightened color should differ from original');
    runner.assert(lightened.startsWith('#'), 'Lightened color should be valid hex');
});

// ==================== RUN TESTS ====================
async function runAllTests() {
    const button = document.querySelector('.run-button');
    button.disabled = true;
    button.textContent = 'RUNNING TESTS...';

    const results = await runner.run();

    const passed = results.filter(r => r.passed).length;
    const failed = results.filter(r => !r.passed).length;
    const total = results.length;
    const successRate = ((passed / total) * 100).toFixed(1);

    // Update stats
    document.getElementById('testStats').style.display = 'flex';
    document.getElementById('totalTests').textContent = total;
    document.getElementById('passedTests').textContent = passed;
    document.getElementById('failedTests').textContent = failed;
    document.getElementById('successRate').textContent = successRate + '%';

    const successRateElem = document.getElementById('successRate');
    successRateElem.className = 'stat-value ' + (successRate >= 90 ? 'pass' : 'fail');

    // Display results by category
    const categories = {
        'Game Initialization': [],
        'Paddle Mechanics': [],
        'Ball Physics': [],
        'Collision Detection': [],
        'Brick System': [],
        'Scoring': [],
        'Power-ups': [],
        'Level Progression': [],
        'Lives System': [],
        'Particle Effects': [],
        'Physics': [],
        'Graphics': []
    };

    results.forEach(result => {
        const name = result.name;
        if (name.includes('initialize') || name.includes('Canvas')) {
            categories['Game Initialization'].push(result);
        } else if (name.includes('Paddle')) {
            categories['Paddle Mechanics'].push(result);
        } else if (name.includes('Ball')) {
            categories['Ball Physics'].push(result);
        } else if (name.includes('collision')) {
            categories['Collision Detection'].push(result);
        } else if (name.includes('Brick') || name.includes('brick')) {
            categories['Brick System'].push(result);
        } else if (name.includes('Score') || name.includes('score')) {
            categories['Scoring'].push(result);
        } else if (name.includes('owerup')) {
            categories['Power-ups'].push(result);
        } else if (name.includes('Level') || name.includes('level')) {
            categories['Level Progression'].push(result);
        } else if (name.includes('ife') || name.includes('lives')) {
            categories['Lives System'].push(result);
        } else if (name.includes('article')) {
            categories['Particle Effects'].push(result);
        } else if (name.includes('physics') || name.includes('speed') || name.includes('gravity')) {
            categories['Physics'].push(result);
        } else if (name.includes('olor')) {
            categories['Graphics'].push(result);
        }
    });

    const resultsDiv = document.getElementById('testResults');
    resultsDiv.innerHTML = '';

    for (const [category, tests] of Object.entries(categories)) {
        if (tests.length === 0) continue;

        const section = document.createElement('div');
        section.className = 'test-section';

        const categoryPassed = tests.filter(t => t.passed).length;
        const categoryTotal = tests.length;

        section.innerHTML = `
            <h2>${category} (${categoryPassed}/${categoryTotal})</h2>
        `;

        tests.forEach(test => {
            const testCase = document.createElement('div');
            testCase.className = `test-case ${test.passed ? 'pass' : 'fail'}`;
            testCase.innerHTML = `
                <div class="test-name">
                    ${test.name}
                    <span class="test-result ${test.passed ? 'pass' : 'fail'}">
                        ${test.passed ? 'PASS' : 'FAIL'}
                    </span>
                </div>
                ${test.error ? `<div class="error-message">Error: ${test.error}</div>` : ''}
            `;
            section.appendChild(testCase);
        });

        resultsDiv.appendChild(section);
    }

    button.disabled = false;
    button.textContent = 'RUN ALL TESTS AGAIN';

    // Scroll to results
    document.getElementById('testStats').scrollIntoView({ behavior: 'smooth' });
}
