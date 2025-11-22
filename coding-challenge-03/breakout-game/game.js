/**
 * BREAKOUT ARENA - A Modern Take on Classic Breakout
 * Complete arcade game with physics, power-ups, levels, and visual effects
 */

class BreakoutGame {
    constructor() {
        this.canvas = document.getElementById('gameCanvas');
        this.ctx = this.canvas.getContext('2d');

        // Game state
        this.gameState = 'start'; // start, playing, paused, levelComplete, gameOver
        this.score = 0;
        this.highScore = parseInt(localStorage.getItem('breakoutHighScore')) || 0;
        this.level = 1;
        this.lives = 3;

        // Game objects
        this.paddle = null;
        this.balls = [];
        this.bricks = [];
        this.powerups = [];
        this.particles = [];

        // Power-up effects
        this.activePowerups = new Map();

        // Input handling
        this.keys = {};
        this.mouseX = this.canvas.width / 2;

        // Animation frame
        this.lastTime = 0;
        this.animationId = null;

        // Constants
        this.PADDLE_WIDTH = 100;
        this.PADDLE_HEIGHT = 20;
        this.PADDLE_SPEED = 8;
        this.BALL_RADIUS = 8;
        this.BALL_SPEED = 5;
        this.BRICK_ROWS = 6;
        this.BRICK_COLS = 10;
        this.BRICK_WIDTH = 70;
        this.BRICK_HEIGHT = 25;
        this.BRICK_PADDING = 5;
        this.BRICK_OFFSET_TOP = 60;
        this.BRICK_OFFSET_LEFT = 35;

        this.init();
    }

    init() {
        this.setupEventListeners();
        this.updateHighScoreDisplay();
        document.getElementById('startScreen').style.display = 'block';
    }

    setupEventListeners() {
        // Keyboard controls
        document.addEventListener('keydown', (e) => {
            this.keys[e.key] = true;
            if (e.key === ' ') {
                e.preventDefault();
                if (this.gameState === 'playing') {
                    this.pause();
                } else if (this.gameState === 'paused') {
                    this.resume();
                }
            }
        });

        document.addEventListener('keyup', (e) => {
            this.keys[e.key] = false;
        });

        // Mouse controls
        this.canvas.addEventListener('mousemove', (e) => {
            const rect = this.canvas.getBoundingClientRect();
            this.mouseX = e.clientX - rect.left;
        });

        // Touch controls for mobile
        this.canvas.addEventListener('touchmove', (e) => {
            e.preventDefault();
            const rect = this.canvas.getBoundingClientRect();
            this.mouseX = e.touches[0].clientX - rect.left;
        });
    }

    start() {
        document.getElementById('startScreen').style.display = 'none';
        this.gameState = 'playing';
        this.score = 0;
        this.lives = 3;
        this.level = 1;
        this.initLevel();
        this.gameLoop();
    }

    initLevel() {
        // Create paddle
        this.paddle = {
            x: this.canvas.width / 2 - this.PADDLE_WIDTH / 2,
            y: this.canvas.height - 40,
            width: this.PADDLE_WIDTH,
            height: this.PADDLE_HEIGHT,
            speed: this.PADDLE_SPEED
        };

        // Create ball
        this.balls = [{
            x: this.canvas.width / 2,
            y: this.canvas.height - 60,
            dx: (Math.random() - 0.5) * this.BALL_SPEED,
            dy: -this.BALL_SPEED,
            radius: this.BALL_RADIUS,
            stuck: true
        }];

        // Create bricks
        this.createBricks();

        // Clear powerups and particles
        this.powerups = [];
        this.particles = [];
        this.activePowerups.clear();

        this.updateDisplay();
    }

    createBricks() {
        this.bricks = [];
        const patterns = this.getLevelPattern(this.level);

        for (let row = 0; row < this.BRICK_ROWS; row++) {
            for (let col = 0; col < this.BRICK_COLS; col++) {
                const brickType = patterns[row % patterns.length][col % patterns[0].length];

                if (brickType > 0) {
                    const brick = {
                        x: col * (this.BRICK_WIDTH + this.BRICK_PADDING) + this.BRICK_OFFSET_LEFT,
                        y: row * (this.BRICK_HEIGHT + this.BRICK_PADDING) + this.BRICK_OFFSET_TOP,
                        width: this.BRICK_WIDTH,
                        height: this.BRICK_HEIGHT,
                        hits: brickType,
                        maxHits: brickType,
                        color: this.getBrickColor(brickType),
                        hasPowerup: Math.random() < 0.15 // 15% chance of powerup
                    };
                    this.bricks.push(brick);
                }
            }
        }
    }

    getLevelPattern(level) {
        const patterns = [
            // Level 1 - Simple
            [
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                [2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                [2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                [2, 2, 2, 2, 2, 2, 2, 2, 2, 2]
            ],
            // Level 2 - Checkered
            [
                [1, 2, 1, 2, 1, 2, 1, 2, 1, 2],
                [2, 3, 2, 3, 2, 3, 2, 3, 2, 3],
                [1, 2, 1, 2, 1, 2, 1, 2, 1, 2],
                [2, 3, 2, 3, 2, 3, 2, 3, 2, 3],
                [1, 2, 1, 2, 1, 2, 1, 2, 1, 2],
                [2, 3, 2, 3, 2, 3, 2, 3, 2, 3]
            ],
            // Level 3 - Diamond
            [
                [0, 0, 0, 0, 3, 3, 0, 0, 0, 0],
                [0, 0, 0, 2, 2, 2, 2, 0, 0, 0],
                [0, 0, 1, 1, 1, 1, 1, 1, 0, 0],
                [0, 2, 2, 2, 2, 2, 2, 2, 2, 0],
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                [3, 3, 3, 3, 3, 3, 3, 3, 3, 3]
            ],
            // Level 4 - Pyramid
            [
                [0, 0, 0, 0, 3, 3, 0, 0, 0, 0],
                [0, 0, 0, 3, 2, 2, 3, 0, 0, 0],
                [0, 0, 3, 2, 1, 1, 2, 3, 0, 0],
                [0, 3, 2, 1, 1, 1, 1, 2, 3, 0],
                [3, 2, 1, 1, 1, 1, 1, 1, 2, 3],
                [3, 3, 3, 3, 3, 3, 3, 3, 3, 3]
            ],
            // Level 5+ - Advanced patterns
            [
                [3, 0, 3, 0, 3, 3, 0, 3, 0, 3],
                [0, 3, 0, 3, 2, 2, 3, 0, 3, 0],
                [3, 2, 3, 2, 1, 1, 2, 3, 2, 3],
                [2, 1, 2, 1, 1, 1, 1, 2, 1, 2],
                [3, 2, 3, 2, 2, 2, 2, 3, 2, 3],
                [3, 3, 3, 3, 3, 3, 3, 3, 3, 3]
            ]
        ];

        const patternIndex = Math.min(level - 1, patterns.length - 1);
        return patterns[patternIndex];
    }

    getBrickColor(hits) {
        const colors = [
            null,
            '#00ff00', // 1 hit - Green
            '#ffaa00', // 2 hits - Orange
            '#ff0000'  // 3 hits - Red
        ];
        return colors[hits] || colors[3];
    }

    gameLoop(timestamp = 0) {
        const deltaTime = timestamp - this.lastTime;
        this.lastTime = timestamp;

        if (this.gameState === 'playing') {
            this.update(deltaTime);
            this.render();
        } else if (this.gameState === 'paused') {
            // Render paused state
            this.render();
        }

        this.animationId = requestAnimationFrame((ts) => this.gameLoop(ts));
    }

    update(deltaTime) {
        this.updatePaddle();
        this.updateBalls();
        this.updatePowerups();
        this.updateParticles();
        this.checkCollisions();
        this.checkLevelComplete();
    }

    updatePaddle() {
        // Mouse control (priority)
        this.paddle.x = this.mouseX - this.paddle.width / 2;

        // Keyboard control
        if (this.keys['ArrowLeft']) {
            this.paddle.x -= this.paddle.speed;
        }
        if (this.keys['ArrowRight']) {
            this.paddle.x += this.paddle.speed;
        }

        // Keep paddle in bounds
        this.paddle.x = Math.max(0, Math.min(this.canvas.width - this.paddle.width, this.paddle.x));
    }

    updateBalls() {
        for (let i = this.balls.length - 1; i >= 0; i--) {
            const ball = this.balls[i];

            if (ball.stuck) {
                // Ball stuck to paddle
                ball.x = this.paddle.x + this.paddle.width / 2;
                ball.y = this.paddle.y - ball.radius;

                // Launch ball on click or space
                if (this.keys[' '] || this.keys['ArrowUp']) {
                    ball.stuck = false;
                    ball.dy = -this.BALL_SPEED;
                    ball.dx = (Math.random() - 0.5) * this.BALL_SPEED;
                }
            } else {
                // Update ball position
                ball.x += ball.dx;
                ball.y += ball.dy;

                // Wall collisions
                if (ball.x - ball.radius <= 0 || ball.x + ball.radius >= this.canvas.width) {
                    ball.dx = -ball.dx;
                    ball.x = Math.max(ball.radius, Math.min(this.canvas.width - ball.radius, ball.x));
                    this.createImpactParticles(ball.x, ball.y, '#00ffff');
                    this.playSound('wall');
                }

                if (ball.y - ball.radius <= 0) {
                    ball.dy = -ball.dy;
                    ball.y = ball.radius;
                    this.createImpactParticles(ball.x, ball.y, '#00ffff');
                    this.playSound('wall');
                }

                // Bottom collision (lose life)
                if (ball.y - ball.radius > this.canvas.height) {
                    this.balls.splice(i, 1);

                    if (this.balls.length === 0) {
                        this.loseLife();
                    }
                }

                // Paddle collision
                if (this.checkPaddleCollision(ball)) {
                    // Calculate bounce angle based on where ball hits paddle
                    const hitPos = (ball.x - this.paddle.x) / this.paddle.width;
                    const angle = (hitPos - 0.5) * Math.PI * 0.6; // -54° to +54°

                    const speed = Math.sqrt(ball.dx * ball.dx + ball.dy * ball.dy);
                    ball.dx = speed * Math.sin(angle);
                    ball.dy = -Math.abs(speed * Math.cos(angle));

                    ball.y = this.paddle.y - ball.radius;
                    this.createImpactParticles(ball.x, ball.y, '#00ff00');
                    this.playSound('paddle');
                }
            }
        }
    }

    checkPaddleCollision(ball) {
        return ball.x > this.paddle.x &&
               ball.x < this.paddle.x + this.paddle.width &&
               ball.y + ball.radius > this.paddle.y &&
               ball.y - ball.radius < this.paddle.y + this.paddle.height &&
               ball.dy > 0;
    }

    checkCollisions() {
        const isFireball = this.activePowerups.has('fireball');

        for (const ball of this.balls) {
            if (ball.stuck) continue;

            for (let i = this.bricks.length - 1; i >= 0; i--) {
                const brick = this.bricks[i];

                if (this.checkBrickCollision(ball, brick)) {
                    // Fireball pierces through bricks without bouncing
                    if (!isFireball) {
                        // Determine collision side for normal bounce
                        const ballCenterX = ball.x;
                        const ballCenterY = ball.y;
                        const brickCenterX = brick.x + brick.width / 2;
                        const brickCenterY = brick.y + brick.height / 2;

                        const dx = ballCenterX - brickCenterX;
                        const dy = ballCenterY - brickCenterY;

                        const absDx = Math.abs(dx);
                        const absDy = Math.abs(dy);

                        // Bounce ball
                        if (absDx > absDy) {
                            ball.dx = -ball.dx;
                        } else {
                            ball.dy = -ball.dy;
                        }
                    }

                    // Fireball instantly destroys bricks
                    if (isFireball) {
                        brick.hits = 0;
                    } else {
                        brick.hits--;
                    }

                    if (brick.hits <= 0) {
                        // Brick destroyed
                        this.score += brick.maxHits * 10 * this.level;
                        this.createBrickParticles(brick);

                        // Spawn powerup
                        if (brick.hasPowerup) {
                            this.spawnPowerup(brick.x + brick.width / 2, brick.y + brick.height / 2);
                        }

                        this.bricks.splice(i, 1);
                        this.playSound('brick');
                    } else {
                        // Brick damaged
                        brick.color = this.getBrickColor(brick.hits);
                        this.score += 5 * this.level;
                        this.createImpactParticles(ball.x, ball.y, brick.color);
                        this.playSound('hit');
                    }

                    this.updateDisplay();

                    // Only break if not fireball (fireball continues through bricks)
                    if (!isFireball) {
                        break;
                    }
                }
            }
        }
    }

    checkBrickCollision(ball, brick) {
        const closestX = Math.max(brick.x, Math.min(ball.x, brick.x + brick.width));
        const closestY = Math.max(brick.y, Math.min(ball.y, brick.y + brick.height));

        const distanceX = ball.x - closestX;
        const distanceY = ball.y - closestY;

        return (distanceX * distanceX + distanceY * distanceY) < (ball.radius * ball.radius);
    }

    spawnPowerup(x, y) {
        const types = ['multiball', 'bigpaddle', 'slowball', 'fireball', 'extralife'];
        const type = types[Math.floor(Math.random() * types.length)];

        const powerup = {
            x: x,
            y: y,
            width: 30,
            height: 30,
            speed: 2,
            type: type,
            color: this.getPowerupColor(type),
            icon: this.getPowerupIcon(type)
        };

        this.powerups.push(powerup);
    }

    getPowerupColor(type) {
        const colors = {
            multiball: '#ff00ff',
            bigpaddle: '#00ff00',
            slowball: '#00ffff',
            fireball: '#ff0000',
            extralife: '#ffff00'
        };
        return colors[type] || '#ffffff';
    }

    getPowerupIcon(type) {
        const icons = {
            multiball: '●●',
            bigpaddle: '━━',
            slowball: '⏱',
            fireball: '🔥',
            extralife: '❤'
        };
        return icons[type] || '?';
    }

    updatePowerups() {
        for (let i = this.powerups.length - 1; i >= 0; i--) {
            const powerup = this.powerups[i];
            powerup.y += powerup.speed;

            // Check paddle collision
            if (powerup.x + powerup.width > this.paddle.x &&
                powerup.x < this.paddle.x + this.paddle.width &&
                powerup.y + powerup.height > this.paddle.y &&
                powerup.y < this.paddle.y + this.paddle.height) {

                this.activatePowerup(powerup.type);
                this.powerups.splice(i, 1);
                this.playSound('powerup');
            }
            // Remove if off screen
            else if (powerup.y > this.canvas.height) {
                this.powerups.splice(i, 1);
            }
        }

        // Update active powerup durations
        for (const [type, endTime] of this.activePowerups.entries()) {
            if (Date.now() > endTime) {
                this.deactivatePowerup(type);
                this.activePowerups.delete(type);
            }
        }
    }

    activatePowerup(type) {
        switch (type) {
            case 'multiball':
                // Add 2 extra balls
                if (this.balls.length > 0) {
                    const ball = this.balls[0];
                    for (let i = 0; i < 2; i++) {
                        this.balls.push({
                            x: ball.x,
                            y: ball.y,
                            dx: (Math.random() - 0.5) * this.BALL_SPEED,
                            dy: -this.BALL_SPEED,
                            radius: this.BALL_RADIUS,
                            stuck: false
                        });
                    }
                    this.showPowerupMessage('Multi-Ball!');
                }
                break;

            case 'bigpaddle':
                this.paddle.width = this.PADDLE_WIDTH * 1.5;
                this.activePowerups.set('bigpaddle', Date.now() + 10000);
                this.showPowerupMessage('Big Paddle! (10s)');
                break;

            case 'slowball':
                for (const ball of this.balls) {
                    ball.dx *= 0.7;
                    ball.dy *= 0.7;
                }
                this.activePowerups.set('slowball', Date.now() + 8000);
                this.showPowerupMessage('Slow Ball! (8s)');
                break;

            case 'fireball':
                this.activePowerups.set('fireball', Date.now() + 12000);
                this.showPowerupMessage('Fire Ball! (12s)');
                break;

            case 'extralife':
                this.lives++;
                this.updateDisplay();
                this.showPowerupMessage('Extra Life!');
                break;
        }
    }

    deactivatePowerup(type) {
        switch (type) {
            case 'bigpaddle':
                this.paddle.width = this.PADDLE_WIDTH;
                break;

            case 'slowball':
                // Restore original speed (reverse the 0.7 multiplier)
                for (const ball of this.balls) {
                    ball.dx /= 0.7;
                    ball.dy /= 0.7;
                }
                break;
        }
    }

    showPowerupMessage(message) {
        const info = document.getElementById('powerupInfo');
        info.textContent = message;
        info.style.display = 'block';

        setTimeout(() => {
            info.style.display = 'none';
        }, 2000);
    }

    createBrickParticles(brick) {
        const count = 20;
        for (let i = 0; i < count; i++) {
            this.particles.push({
                x: brick.x + brick.width / 2,
                y: brick.y + brick.height / 2,
                vx: (Math.random() - 0.5) * 6,
                vy: (Math.random() - 0.5) * 6,
                life: 60,
                maxLife: 60,
                color: brick.color,
                size: Math.random() * 4 + 2
            });
        }
    }

    createImpactParticles(x, y, color) {
        const count = 8;
        for (let i = 0; i < count; i++) {
            this.particles.push({
                x: x,
                y: y,
                vx: (Math.random() - 0.5) * 4,
                vy: (Math.random() - 0.5) * 4,
                life: 30,
                maxLife: 30,
                color: color,
                size: Math.random() * 3 + 1
            });
        }
    }

    updateParticles() {
        for (let i = this.particles.length - 1; i >= 0; i--) {
            const p = this.particles[i];
            p.x += p.vx;
            p.y += p.vy;
            p.vy += 0.2; // Gravity
            p.life--;

            if (p.life <= 0) {
                this.particles.splice(i, 1);
            }
        }
    }

    checkLevelComplete() {
        if (this.bricks.length === 0) {
            this.levelComplete();
        }
    }

    levelComplete() {
        this.gameState = 'levelComplete';
        const bonus = this.lives * 1000;
        this.score += bonus;

        document.getElementById('levelCompleteText').innerHTML =
            `Level ${this.level} Complete!<br>` +
            `Bonus: ${bonus} points<br>` +
            `Total Score: ${this.score}`;

        document.getElementById('levelComplete').style.display = 'block';
        this.updateHighScore();
        this.updateDisplay();
        this.playSound('levelComplete');
    }

    nextLevel() {
        document.getElementById('levelComplete').style.display = 'none';
        this.level++;
        this.gameState = 'playing';
        this.initLevel();
    }

    loseLife() {
        this.lives--;
        this.updateDisplay();

        if (this.lives <= 0) {
            this.gameOver();
        } else {
            // Reset ball
            this.balls = [{
                x: this.canvas.width / 2,
                y: this.canvas.height - 60,
                dx: 0,
                dy: -this.BALL_SPEED,
                radius: this.BALL_RADIUS,
                stuck: true
            }];
            this.playSound('loseLife');
        }
    }

    gameOver() {
        this.gameState = 'gameOver';
        this.updateHighScore();

        document.getElementById('finalScore').innerHTML =
            `Final Score: ${this.score}<br>` +
            `Level Reached: ${this.level}<br>` +
            (this.score === this.highScore ? '<br>NEW HIGH SCORE!' : '');

        document.getElementById('gameOver').style.display = 'block';
        this.playSound('gameOver');
    }

    restart() {
        document.getElementById('gameOver').style.display = 'none';
        this.start();
    }

    pause() {
        this.gameState = 'paused';
    }

    resume() {
        this.gameState = 'playing';
    }

    updateDisplay() {
        document.getElementById('score').textContent = this.score;
        document.getElementById('level').textContent = this.level;

        // Update lives display
        const livesContainer = document.getElementById('lives');
        livesContainer.innerHTML = '';
        for (let i = 0; i < this.lives; i++) {
            const life = document.createElement('div');
            life.className = 'life-icon';
            livesContainer.appendChild(life);
        }
    }

    updateHighScore() {
        if (this.score > this.highScore) {
            this.highScore = this.score;
            localStorage.setItem('breakoutHighScore', this.highScore);
            this.updateHighScoreDisplay();
        }
    }

    updateHighScoreDisplay() {
        document.getElementById('highScore').textContent = this.highScore;
    }

    playSound(type) {
        // Simple sound effects using Web Audio API
        if (!this.audioContext) {
            try {
                this.audioContext = new (window.AudioContext || window.webkitAudioContext)();
            } catch (e) {
                return; // Audio not supported
            }
        }

        const ctx = this.audioContext;
        const oscillator = ctx.createOscillator();
        const gainNode = ctx.createGain();

        oscillator.connect(gainNode);
        gainNode.connect(ctx.destination);

        switch (type) {
            case 'paddle':
                oscillator.frequency.value = 300;
                gainNode.gain.value = 0.1;
                oscillator.start();
                oscillator.stop(ctx.currentTime + 0.1);
                break;

            case 'brick':
                oscillator.frequency.value = 500;
                gainNode.gain.value = 0.15;
                oscillator.start();
                oscillator.stop(ctx.currentTime + 0.1);
                break;

            case 'hit':
                oscillator.frequency.value = 400;
                gainNode.gain.value = 0.1;
                oscillator.start();
                oscillator.stop(ctx.currentTime + 0.08);
                break;

            case 'wall':
                oscillator.frequency.value = 200;
                gainNode.gain.value = 0.08;
                oscillator.start();
                oscillator.stop(ctx.currentTime + 0.05);
                break;

            case 'powerup':
                oscillator.frequency.value = 800;
                gainNode.gain.value = 0.15;
                oscillator.start();
                gainNode.gain.exponentialRampToValueAtTime(0.01, ctx.currentTime + 0.3);
                oscillator.stop(ctx.currentTime + 0.3);
                break;

            case 'loseLife':
                oscillator.frequency.value = 150;
                gainNode.gain.value = 0.2;
                oscillator.start();
                gainNode.gain.exponentialRampToValueAtTime(0.01, ctx.currentTime + 0.5);
                oscillator.stop(ctx.currentTime + 0.5);
                break;

            case 'levelComplete':
                oscillator.frequency.value = 600;
                gainNode.gain.value = 0.2;
                oscillator.start();
                oscillator.frequency.exponentialRampToValueAtTime(1000, ctx.currentTime + 0.3);
                gainNode.gain.exponentialRampToValueAtTime(0.01, ctx.currentTime + 0.3);
                oscillator.stop(ctx.currentTime + 0.3);
                break;

            case 'gameOver':
                oscillator.frequency.value = 300;
                gainNode.gain.value = 0.2;
                oscillator.start();
                oscillator.frequency.exponentialRampToValueAtTime(100, ctx.currentTime + 0.5);
                gainNode.gain.exponentialRampToValueAtTime(0.01, ctx.currentTime + 0.5);
                oscillator.stop(ctx.currentTime + 0.5);
                break;
        }
    }

    render() {
        // Clear canvas
        this.ctx.fillStyle = '#000';
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

        // Draw background grid
        this.drawBackground();

        // Draw particles
        this.drawParticles();

        // Draw bricks
        this.drawBricks();

        // Draw powerups
        this.drawPowerups();

        // Draw paddle
        this.drawPaddle();

        // Draw balls
        this.drawBalls();

        // Draw pause overlay
        if (this.gameState === 'paused') {
            this.drawPauseOverlay();
        }
    }

    drawBackground() {
        this.ctx.strokeStyle = 'rgba(138, 43, 226, 0.1)';
        this.ctx.lineWidth = 1;

        // Vertical lines
        for (let x = 0; x < this.canvas.width; x += 40) {
            this.ctx.beginPath();
            this.ctx.moveTo(x, 0);
            this.ctx.lineTo(x, this.canvas.height);
            this.ctx.stroke();
        }

        // Horizontal lines
        for (let y = 0; y < this.canvas.height; y += 40) {
            this.ctx.beginPath();
            this.ctx.moveTo(0, y);
            this.ctx.lineTo(this.canvas.width, y);
            this.ctx.stroke();
        }
    }

    drawBricks() {
        for (const brick of this.bricks) {
            // Brick gradient
            const gradient = this.ctx.createLinearGradient(
                brick.x, brick.y,
                brick.x, brick.y + brick.height
            );

            const color = brick.color;
            gradient.addColorStop(0, color);
            gradient.addColorStop(1, this.darkenColor(color));

            this.ctx.fillStyle = gradient;
            this.ctx.fillRect(brick.x, brick.y, brick.width, brick.height);

            // Brick border
            this.ctx.strokeStyle = this.lightenColor(color);
            this.ctx.lineWidth = 2;
            this.ctx.strokeRect(brick.x, brick.y, brick.width, brick.height);

            // Shine effect
            this.ctx.fillStyle = 'rgba(255, 255, 255, 0.2)';
            this.ctx.fillRect(brick.x, brick.y, brick.width, brick.height / 3);

            // Show hit points
            if (brick.hits > 1) {
                this.ctx.fillStyle = '#fff';
                this.ctx.font = 'bold 12px monospace';
                this.ctx.textAlign = 'center';
                this.ctx.textBaseline = 'middle';
                this.ctx.fillText(
                    brick.hits.toString(),
                    brick.x + brick.width / 2,
                    brick.y + brick.height / 2
                );
            }
        }
    }

    drawPaddle() {
        // Paddle gradient
        const gradient = this.ctx.createLinearGradient(
            this.paddle.x, this.paddle.y,
            this.paddle.x, this.paddle.y + this.paddle.height
        );
        gradient.addColorStop(0, '#00ffff');
        gradient.addColorStop(1, '#0088ff');

        this.ctx.fillStyle = gradient;
        this.ctx.fillRect(this.paddle.x, this.paddle.y, this.paddle.width, this.paddle.height);

        // Paddle border
        this.ctx.strokeStyle = '#ffffff';
        this.ctx.lineWidth = 2;
        this.ctx.strokeRect(this.paddle.x, this.paddle.y, this.paddle.width, this.paddle.height);

        // Paddle glow
        this.ctx.shadowBlur = 20;
        this.ctx.shadowColor = '#00ffff';
        this.ctx.strokeRect(this.paddle.x, this.paddle.y, this.paddle.width, this.paddle.height);
        this.ctx.shadowBlur = 0;
    }

    drawBalls() {
        for (const ball of this.balls) {
            // Ball gradient
            const gradient = this.ctx.createRadialGradient(
                ball.x - ball.radius / 3, ball.y - ball.radius / 3, 0,
                ball.x, ball.y, ball.radius
            );

            const isFireball = this.activePowerups.has('fireball');
            if (isFireball) {
                gradient.addColorStop(0, '#ffff00');
                gradient.addColorStop(0.5, '#ff8800');
                gradient.addColorStop(1, '#ff0000');
            } else {
                gradient.addColorStop(0, '#ffffff');
                gradient.addColorStop(0.5, '#ff00ff');
                gradient.addColorStop(1, '#8800ff');
            }

            this.ctx.fillStyle = gradient;
            this.ctx.beginPath();
            this.ctx.arc(ball.x, ball.y, ball.radius, 0, Math.PI * 2);
            this.ctx.fill();

            // Ball glow
            this.ctx.shadowBlur = 15;
            this.ctx.shadowColor = isFireball ? '#ff0000' : '#ff00ff';
            this.ctx.fill();
            this.ctx.shadowBlur = 0;

            // Trail effect for fireball
            if (isFireball && !ball.stuck) {
                this.particles.push({
                    x: ball.x,
                    y: ball.y,
                    vx: -ball.dx * 0.1,
                    vy: -ball.dy * 0.1,
                    life: 20,
                    maxLife: 20,
                    color: '#ff6600',
                    size: ball.radius * 0.7
                });
            }
        }
    }

    drawPowerups() {
        for (const powerup of this.powerups) {
            // Powerup glow
            this.ctx.shadowBlur = 15;
            this.ctx.shadowColor = powerup.color;

            // Powerup box
            this.ctx.fillStyle = powerup.color;
            this.ctx.fillRect(powerup.x, powerup.y, powerup.width, powerup.height);

            // Powerup border
            this.ctx.strokeStyle = '#fff';
            this.ctx.lineWidth = 2;
            this.ctx.strokeRect(powerup.x, powerup.y, powerup.width, powerup.height);

            this.ctx.shadowBlur = 0;

            // Powerup icon
            this.ctx.fillStyle = '#fff';
            this.ctx.font = 'bold 16px monospace';
            this.ctx.textAlign = 'center';
            this.ctx.textBaseline = 'middle';
            this.ctx.fillText(
                powerup.icon,
                powerup.x + powerup.width / 2,
                powerup.y + powerup.height / 2
            );
        }
    }

    drawParticles() {
        for (const p of this.particles) {
            const alpha = p.life / p.maxLife;

            // Convert hex color to rgba
            let fillStyle;
            if (p.color.startsWith('#')) {
                const hex = p.color.replace('#', '');
                const r = parseInt(hex.substring(0, 2), 16);
                const g = parseInt(hex.substring(2, 4), 16);
                const b = parseInt(hex.substring(4, 6), 16);
                fillStyle = `rgba(${r}, ${g}, ${b}, ${alpha})`;
            } else {
                // Already in rgb format
                fillStyle = p.color.replace(')', `, ${alpha})`).replace('rgb', 'rgba');
            }

            this.ctx.fillStyle = fillStyle;
            this.ctx.beginPath();
            this.ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
            this.ctx.fill();
        }
    }

    drawPauseOverlay() {
        this.ctx.fillStyle = 'rgba(0, 0, 0, 0.7)';
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

        this.ctx.fillStyle = '#fff';
        this.ctx.font = 'bold 48px monospace';
        this.ctx.textAlign = 'center';
        this.ctx.textBaseline = 'middle';
        this.ctx.fillText('PAUSED', this.canvas.width / 2, this.canvas.height / 2);

        this.ctx.font = '20px monospace';
        this.ctx.fillText('Press SPACE to resume', this.canvas.width / 2, this.canvas.height / 2 + 50);
    }

    darkenColor(color) {
        const factor = 0.6;
        const num = parseInt(color.replace('#', ''), 16);
        const r = Math.floor((num >> 16) * factor);
        const g = Math.floor(((num >> 8) & 0x00FF) * factor);
        const b = Math.floor((num & 0x0000FF) * factor);
        return '#' + (r << 16 | g << 8 | b).toString(16).padStart(6, '0');
    }

    lightenColor(color) {
        const factor = 1.3;
        const num = parseInt(color.replace('#', ''), 16);
        const r = Math.min(255, Math.floor((num >> 16) * factor));
        const g = Math.min(255, Math.floor(((num >> 8) & 0x00FF) * factor));
        const b = Math.min(255, Math.floor((num & 0x0000FF) * factor));
        return '#' + (r << 16 | g << 8 | b).toString(16).padStart(6, '0');
    }
}

// Initialize game when page loads
let game;
window.addEventListener('load', () => {
    game = new BreakoutGame();
});
