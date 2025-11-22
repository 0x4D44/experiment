/**
 * ULTIMATE SNAKE GAME - Competition Edition
 * A modern, feature-rich implementation of the classic Snake game
 * with multiple game modes, power-ups, themes, and smooth animations
 */

class SnakeGame {
    constructor() {
        // Canvas setup
        this.canvas = document.getElementById('game-canvas');
        this.ctx = this.canvas.getContext('2d');
        this.gridSize = 20;
        this.tileCount = 30;
        this.canvas.width = this.gridSize * this.tileCount;
        this.canvas.height = this.gridSize * this.tileCount;

        // Game state
        this.gameState = 'menu'; // menu, playing, paused, gameover
        this.gameMode = 'classic'; // classic, timed, endless, obstacle
        this.difficulty = 'medium';
        this.score = 0;
        this.combo = 1;
        this.lastComboTime = 0;
        this.comboTimeout = 3000; // 3 seconds
        this.lastSpeedMilestone = 0; // Track last speed increase milestone

        // Snake
        this.snake = [];
        this.snakeDirection = { x: 1, y: 0 };
        this.nextDirection = { x: 1, y: 0 };
        this.snakeSpeed = 150;
        this.lastMoveTime = 0;
        this.smoothProgress = 0;

        // Food and power-ups
        this.food = null;
        this.powerups = [];
        this.activePowerups = [];
        this.obstacles = [];

        // Timer for timed mode
        this.timeLeft = 60;
        this.timerInterval = null;

        // Settings
        this.settings = {
            theme: 'classic',
            sound: true,
            particles: true,
            gridLines: true
        };

        // Statistics
        this.stats = this.loadStats();

        // Power-up types
        this.powerupTypes = [
            { type: 'speed', icon: '⚡', name: 'Speed Boost', duration: 5000, color: '#ffff00' },
            { type: 'slow', icon: '🐌', name: 'Slow Motion', duration: 7000, color: '#87ceeb' },
            { type: 'invincible', icon: '🛡️', name: 'Invincibility', duration: 5000, color: '#ffd700' },
            { type: 'shrink', icon: '📉', name: 'Shrink', duration: 0, color: '#ff69b4' },
            { type: 'multiplier', icon: '✖️', name: '2x Points', duration: 10000, color: '#00ff00' },
            { type: 'ghost', icon: '👻', name: 'Ghost Mode', duration: 5000, color: '#9370db' }
        ];

        // Mobile touch controls
        this.touchStartX = 0;
        this.touchStartY = 0;
        this.minSwipeDistance = 50;

        // Particle system
        this.particles = [];

        // Audio context (reuse to prevent memory leak)
        this.audioContext = null;

        // Initialize
        this.init();
    }

    init() {
        this.setupEventListeners();
        this.createBackgroundParticles();
        this.loadSettings();
        this.updateStatisticsDisplay();
        this.showMainMenu();
    }

    // ========================================
    // EVENT LISTENERS
    // ========================================

    setupEventListeners() {
        // Store listener references for cleanup
        this.keydownHandler = (e) => this.handleKeyPress(e);
        this.touchStartHandler = (e) => this.handleTouchStart(e);
        this.touchEndHandler = (e) => this.handleTouchEnd(e);
        this.touchMoveHandler = (e) => e.preventDefault();
        this.contextMenuHandler = (e) => e.preventDefault();

        // Keyboard controls
        document.addEventListener('keydown', this.keydownHandler);

        // Touch controls for mobile
        this.canvas.addEventListener('touchstart', this.touchStartHandler, { passive: false });
        this.canvas.addEventListener('touchend', this.touchEndHandler, { passive: false });
        this.canvas.addEventListener('touchmove', this.touchMoveHandler, { passive: false });

        // Prevent context menu on canvas
        this.canvas.addEventListener('contextmenu', this.contextMenuHandler);
    }

    cleanup() {
        // Remove event listeners to prevent memory leaks
        if (this.keydownHandler) {
            document.removeEventListener('keydown', this.keydownHandler);
        }
        if (this.touchStartHandler) {
            this.canvas.removeEventListener('touchstart', this.touchStartHandler);
        }
        if (this.touchEndHandler) {
            this.canvas.removeEventListener('touchend', this.touchEndHandler);
        }
        if (this.touchMoveHandler) {
            this.canvas.removeEventListener('touchmove', this.touchMoveHandler);
        }
        if (this.contextMenuHandler) {
            this.canvas.removeEventListener('contextmenu', this.contextMenuHandler);
        }

        // Close audio context
        if (this.audioContext) {
            this.audioContext.close();
            this.audioContext = null;
        }

        // Clear timer
        if (this.timerInterval) {
            clearInterval(this.timerInterval);
        }
    }

    handleKeyPress(e) {
        if (this.gameState !== 'playing' && this.gameState !== 'paused') return;

        // Arrow keys and WASD
        const key = e.key.toLowerCase();

        if (key === 'arrowup' || key === 'w') {
            if (this.snakeDirection.y === 0) {
                this.nextDirection = { x: 0, y: -1 };
            }
            e.preventDefault();
        } else if (key === 'arrowdown' || key === 's') {
            if (this.snakeDirection.y === 0) {
                this.nextDirection = { x: 0, y: 1 };
            }
            e.preventDefault();
        } else if (key === 'arrowleft' || key === 'a') {
            if (this.snakeDirection.x === 0) {
                this.nextDirection = { x: -1, y: 0 };
            }
            e.preventDefault();
        } else if (key === 'arrowright' || key === 'd') {
            if (this.snakeDirection.x === 0) {
                this.nextDirection = { x: 1, y: 0 };
            }
            e.preventDefault();
        } else if (key === ' ' || key === 'p') {
            this.togglePause();
            e.preventDefault();
        } else if (key === 'escape') {
            this.exitToMenu();
            e.preventDefault();
        }
    }

    handleTouchStart(e) {
        if (e.touches.length === 1) {
            const touch = e.touches[0];
            this.touchStartX = touch.clientX;
            this.touchStartY = touch.clientY;
        } else if (e.touches.length > 1 && this.gameState === 'playing') {
            // Multi-touch for pause
            this.togglePause();
        }
    }

    handleTouchEnd(e) {
        if (this.gameState !== 'playing') return;

        const touch = e.changedTouches[0];
        const deltaX = touch.clientX - this.touchStartX;
        const deltaY = touch.clientY - this.touchStartY;

        // Check if it's a tap (small movement)
        if (Math.abs(deltaX) < 10 && Math.abs(deltaY) < 10) {
            this.togglePause();
            return;
        }

        // Check if swipe distance is sufficient
        if (Math.abs(deltaX) < this.minSwipeDistance && Math.abs(deltaY) < this.minSwipeDistance) {
            return;
        }

        // Determine swipe direction
        if (Math.abs(deltaX) > Math.abs(deltaY)) {
            // Horizontal swipe
            if (deltaX > 0 && this.snakeDirection.x === 0) {
                this.nextDirection = { x: 1, y: 0 };
            } else if (deltaX < 0 && this.snakeDirection.x === 0) {
                this.nextDirection = { x: -1, y: 0 };
            }
        } else {
            // Vertical swipe
            if (deltaY > 0 && this.snakeDirection.y === 0) {
                this.nextDirection = { x: 0, y: 1 };
            } else if (deltaY < 0 && this.snakeDirection.y === 0) {
                this.nextDirection = { x: 0, y: -1 };
            }
        }
    }

    // ========================================
    // MENU NAVIGATION
    // ========================================

    showMainMenu() {
        this.hideAllMenus();
        document.getElementById('main-menu').classList.add('active');
        this.gameState = 'menu';
    }

    showModeSelection() {
        this.hideAllMenus();
        document.getElementById('mode-menu').classList.add('active');
    }

    showDifficultySelection() {
        this.hideAllMenus();
        document.getElementById('difficulty-menu').classList.add('active');
    }

    showSettings() {
        this.hideAllMenus();
        document.getElementById('settings-menu').classList.add('active');
    }

    showStatistics() {
        this.hideAllMenus();
        this.updateStatisticsDisplay();
        document.getElementById('stats-menu').classList.add('active');
    }

    showControls() {
        this.hideAllMenus();
        document.getElementById('controls-menu').classList.add('active');
    }

    showGameScreen() {
        this.hideAllMenus();
        document.getElementById('game-screen').classList.add('active');
    }

    hideAllMenus() {
        document.querySelectorAll('.menu, .game-screen').forEach(menu => {
            menu.classList.remove('active');
        });
    }

    selectMode(mode) {
        this.gameMode = mode;
        this.showDifficultySelection();
    }

    selectDifficulty(difficulty) {
        this.difficulty = difficulty;
        this.startGame();
    }

    // ========================================
    // GAME INITIALIZATION
    // ========================================

    startGame() {
        this.showGameScreen();
        this.gameState = 'playing';

        // Reset game state
        this.score = 0;
        this.combo = 1;
        this.lastComboTime = Date.now();
        this.activePowerups = [];
        this.obstacles = [];
        this.particles = [];
        this.lastSpeedMilestone = 0;
        this.gameStartTime = Date.now(); // Track game start time for total time stat

        // Initialize snake
        const startX = Math.floor(this.tileCount / 2);
        const startY = Math.floor(this.tileCount / 2);
        this.snake = [
            { x: startX, y: startY },
            { x: startX - 1, y: startY },
            { x: startX - 2, y: startY }
        ];
        this.snakeDirection = { x: 1, y: 0 };
        this.nextDirection = { x: 1, y: 0 };

        // Set speed based on difficulty
        switch (this.difficulty) {
            case 'easy':
                this.snakeSpeed = 200;
                break;
            case 'medium':
                this.snakeSpeed = 150;
                break;
            case 'hard':
                this.snakeSpeed = 100;
                break;
            case 'insane':
                this.snakeSpeed = 60;
                break;
        }

        // Setup timed mode
        if (this.gameMode === 'timed') {
            this.timeLeft = 60;
            document.getElementById('timer-display').style.display = 'flex';
            this.startTimer();
        } else {
            document.getElementById('timer-display').style.display = 'none';
        }

        // Spawn initial food
        this.spawnFood();

        // Update UI
        this.updateUI();

        // Hide overlays
        document.getElementById('pause-overlay').classList.remove('active');
        document.getElementById('gameover-overlay').classList.remove('active');

        // Start game loop
        this.lastMoveTime = Date.now();
        this.gameLoop();
    }

    // ========================================
    // GAME LOOP
    // ========================================

    gameLoop() {
        const currentTime = Date.now();
        const deltaTime = currentTime - this.lastMoveTime;

        // Update smooth progress for animation
        const effectiveSpeed = this.getEffectiveSpeed();
        this.smoothProgress = Math.min(deltaTime / effectiveSpeed, 1);

        // Move snake when enough time has passed
        if (deltaTime >= effectiveSpeed) {
            this.update();
            this.lastMoveTime = currentTime;
            this.smoothProgress = 0;
        }

        // Update combo timer
        if (currentTime - this.lastComboTime > this.comboTimeout) {
            this.combo = 1;
            this.updateUI();
        }

        // Update power-ups
        this.updatePowerups(currentTime);

        // Update particles
        this.updateParticles();

        // Render
        this.render();

        // Continue loop ONLY if still playing
        if (this.gameState === 'playing') {
            requestAnimationFrame(() => this.gameLoop());
        }
    }

    update() {
        // Update direction
        this.snakeDirection = { ...this.nextDirection };

        // Calculate new head position
        const head = { ...this.snake[0] };
        head.x += this.snakeDirection.x;
        head.y += this.snakeDirection.y;

        // Handle wall collision based on game mode
        if (this.gameMode === 'endless' || this.hasActivePowerup('ghost')) {
            // Wrap around walls
            if (head.x < 0) head.x = this.tileCount - 1;
            if (head.x >= this.tileCount) head.x = 0;
            if (head.y < 0) head.y = this.tileCount - 1;
            if (head.y >= this.tileCount) head.y = 0;
        } else {
            // Die on wall collision
            if (head.x < 0 || head.x >= this.tileCount || head.y < 0 || head.y >= this.tileCount) {
                this.gameOver();
                return;
            }
        }

        // Check obstacle collision
        if (this.checkObstacleCollision(head)) {
            if (!this.hasActivePowerup('invincible')) {
                this.gameOver();
                return;
            }
        }

        // Check self collision
        if (this.checkSelfCollision(head)) {
            if (!this.hasActivePowerup('invincible')) {
                this.gameOver();
                return;
            }
        }

        // Add new head
        this.snake.unshift(head);

        // Check food collision
        if (head.x === this.food.x && head.y === this.food.y) {
            this.eatFood();
        } else {
            // Remove tail if no food eaten
            this.snake.pop();
        }

        // Check power-up collision
        for (let i = this.powerups.length - 1; i >= 0; i--) {
            const powerup = this.powerups[i];
            if (head.x === powerup.x && head.y === powerup.y) {
                this.collectPowerup(powerup);
                this.powerups.splice(i, 1);
            }
        }

        // Spawn obstacles in obstacle mode
        if (this.gameMode === 'obstacle' && Math.random() < 0.02) {
            this.spawnObstacle();
        }

        // Spawn power-ups randomly
        if (this.powerups.length < 3 && Math.random() < 0.01) {
            this.spawnPowerup();
        }

        // Progressive difficulty - only trigger once per milestone
        if (this.score > 0 && this.score % 50 === 0 && this.score !== this.lastSpeedMilestone && this.snakeSpeed > 50) {
            this.snakeSpeed -= 2;
            this.lastSpeedMilestone = this.score;
        }

        this.updateUI();
    }

    // ========================================
    // COLLISION DETECTION
    // ========================================

    checkSelfCollision(head) {
        for (let i = 1; i < this.snake.length; i++) {
            if (head.x === this.snake[i].x && head.y === this.snake[i].y) {
                return true;
            }
        }
        return false;
    }

    checkObstacleCollision(pos) {
        return this.obstacles.some(obs => obs.x === pos.x && obs.y === pos.y);
    }

    // ========================================
    // FOOD & POWER-UPS
    // ========================================

    spawnFood() {
        let position;
        do {
            position = {
                x: Math.floor(Math.random() * this.tileCount),
                y: Math.floor(Math.random() * this.tileCount)
            };
        } while (this.isPositionOccupied(position));

        this.food = position;
    }

    spawnPowerup() {
        let position;
        do {
            position = {
                x: Math.floor(Math.random() * this.tileCount),
                y: Math.floor(Math.random() * this.tileCount)
            };
        } while (this.isPositionOccupied(position));

        const type = this.powerupTypes[Math.floor(Math.random() * this.powerupTypes.length)];
        this.powerups.push({ ...position, ...type });
    }

    spawnObstacle() {
        if (this.obstacles.length >= 10) return;

        let position;
        let attempts = 0;
        do {
            position = {
                x: Math.floor(Math.random() * this.tileCount),
                y: Math.floor(Math.random() * this.tileCount)
            };
            attempts++;
        } while (this.isPositionOccupied(position) && attempts < 50);

        if (attempts < 50) {
            this.obstacles.push(position);
        }
    }

    isPositionOccupied(pos) {
        // Check snake
        if (this.snake.some(segment => segment.x === pos.x && segment.y === pos.y)) {
            return true;
        }
        // Check food
        if (this.food && this.food.x === pos.x && this.food.y === pos.y) {
            return true;
        }
        // Check power-ups
        if (this.powerups.some(p => p.x === pos.x && p.y === pos.y)) {
            return true;
        }
        // Check obstacles
        if (this.obstacles.some(o => o.x === pos.x && o.y === pos.y)) {
            return true;
        }
        return false;
    }

    eatFood() {
        // Calculate score with combo multiplier
        const multiplier = this.hasActivePowerup('multiplier') ? 2 : 1;
        const points = 10 * this.combo * multiplier;
        this.score += points;

        // Update combo
        this.combo++;
        this.lastComboTime = Date.now();

        // Update statistics
        this.stats.totalFood++;
        this.saveStats();

        // Create particles
        if (this.settings.particles) {
            this.createParticles(this.food.x, this.food.y, this.getThemeColor('food'), 15);
        }

        // Play sound
        this.playSound('eat');

        // Spawn new food
        this.spawnFood();

        this.updateUI();
    }

    collectPowerup(powerup) {
        // Handle shrink power-up immediately
        if (powerup.type === 'shrink') {
            const shrinkAmount = Math.max(0, Math.min(3, this.snake.length - 3));
            for (let i = 0; i < shrinkAmount; i++) {
                this.snake.pop();
            }
            this.score += 20; // Bonus for taking the risk
        } else {
            // Add timed power-up
            this.activePowerups.push({
                ...powerup,
                startTime: Date.now(),
                endTime: Date.now() + powerup.duration
            });
        }

        // Update statistics
        this.stats.powerupsUsed++;
        this.saveStats();

        // Create particles
        if (this.settings.particles) {
            this.createParticles(powerup.x, powerup.y, powerup.color, 20);
        }

        // Play sound
        this.playSound('powerup');

        this.updateActivePowerupsDisplay();
    }

    updatePowerups(currentTime) {
        // Remove expired power-ups
        this.activePowerups = this.activePowerups.filter(p => currentTime < p.endTime);
        this.updateActivePowerupsDisplay();
    }

    hasActivePowerup(type) {
        return this.activePowerups.some(p => p.type === type);
    }

    getEffectiveSpeed() {
        let speed = this.snakeSpeed;

        if (this.hasActivePowerup('speed')) {
            speed *= 0.5; // Twice as fast
        }
        if (this.hasActivePowerup('slow')) {
            speed *= 2; // Half speed
        }

        return speed;
    }

    // ========================================
    // RENDERING
    // ========================================

    render() {
        // Clear canvas
        this.ctx.fillStyle = this.getThemeColor('background');
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

        // Draw grid
        if (this.settings.gridLines) {
            this.drawGrid();
        }

        // Draw obstacles
        this.drawObstacles();

        // Draw food with glow
        this.drawFood();

        // Draw power-ups
        this.drawPowerups();

        // Draw snake with smooth interpolation
        this.drawSnake();

        // Draw particles
        this.drawParticles();
    }

    drawGrid() {
        this.ctx.strokeStyle = this.getThemeColor('grid');
        this.ctx.lineWidth = 1;

        for (let i = 0; i <= this.tileCount; i++) {
            // Vertical lines
            this.ctx.beginPath();
            this.ctx.moveTo(i * this.gridSize, 0);
            this.ctx.lineTo(i * this.gridSize, this.canvas.height);
            this.ctx.stroke();

            // Horizontal lines
            this.ctx.beginPath();
            this.ctx.moveTo(0, i * this.gridSize);
            this.ctx.lineTo(this.canvas.width, i * this.gridSize);
            this.ctx.stroke();
        }
    }

    drawSnake() {
        for (let i = 0; i < this.snake.length; i++) {
            const segment = this.snake[i];
            let x = segment.x * this.gridSize;
            let y = segment.y * this.gridSize;

            // Smooth interpolation for head
            if (i === 0) {
                const prevX = x - this.snakeDirection.x * this.gridSize * this.smoothProgress;
                const prevY = y - this.snakeDirection.y * this.gridSize * this.smoothProgress;
                x = prevX;
                y = prevY;
            }

            // Color gradient from head to tail
            const opacity = 1 - (i / this.snake.length) * 0.5;
            const color = this.getThemeColor('snake');

            // Draw segment with glow effect
            if (this.hasActivePowerup('invincible')) {
                this.ctx.shadowBlur = 20;
                this.ctx.shadowColor = '#ffd700';
            } else if (this.hasActivePowerup('ghost')) {
                this.ctx.shadowBlur = 20;
                this.ctx.shadowColor = '#9370db';
            } else {
                this.ctx.shadowBlur = 10;
                this.ctx.shadowColor = color;
            }

            this.ctx.fillStyle = color;
            this.ctx.globalAlpha = opacity;

            if (i === 0) {
                // Draw head with rounded corners
                this.roundRect(x + 1, y + 1, this.gridSize - 2, this.gridSize - 2, 5);

                // Draw eyes
                this.ctx.shadowBlur = 0;
                this.ctx.fillStyle = '#000';
                this.ctx.globalAlpha = 1;
                const eyeSize = 3;
                if (this.snakeDirection.x === 1) {
                    this.ctx.fillRect(x + this.gridSize - 8, y + 5, eyeSize, eyeSize);
                    this.ctx.fillRect(x + this.gridSize - 8, y + this.gridSize - 8, eyeSize, eyeSize);
                } else if (this.snakeDirection.x === -1) {
                    this.ctx.fillRect(x + 5, y + 5, eyeSize, eyeSize);
                    this.ctx.fillRect(x + 5, y + this.gridSize - 8, eyeSize, eyeSize);
                } else if (this.snakeDirection.y === -1) {
                    this.ctx.fillRect(x + 5, y + 5, eyeSize, eyeSize);
                    this.ctx.fillRect(x + this.gridSize - 8, y + 5, eyeSize, eyeSize);
                } else {
                    this.ctx.fillRect(x + 5, y + this.gridSize - 8, eyeSize, eyeSize);
                    this.ctx.fillRect(x + this.gridSize - 8, y + this.gridSize - 8, eyeSize, eyeSize);
                }
            } else {
                // Draw body segment
                this.roundRect(x + 2, y + 2, this.gridSize - 4, this.gridSize - 4, 3);
            }

            this.ctx.globalAlpha = 1;
            this.ctx.shadowBlur = 0;
        }
    }

    drawFood() {
        const x = this.food.x * this.gridSize;
        const y = this.food.y * this.gridSize;
        const color = this.getThemeColor('food');

        // Pulsing glow effect
        const pulse = Math.sin(Date.now() / 200) * 0.3 + 0.7;
        this.ctx.shadowBlur = 20 * pulse;
        this.ctx.shadowColor = color;
        this.ctx.fillStyle = color;

        // Draw food as circle
        this.ctx.beginPath();
        this.ctx.arc(
            x + this.gridSize / 2,
            y + this.gridSize / 2,
            this.gridSize / 2 - 2,
            0,
            Math.PI * 2
        );
        this.ctx.fill();

        this.ctx.shadowBlur = 0;
    }

    drawPowerups() {
        for (const powerup of this.powerups) {
            const x = powerup.x * this.gridSize;
            const y = powerup.y * this.gridSize;

            // Floating animation
            const float = Math.sin(Date.now() / 300) * 2;

            this.ctx.shadowBlur = 15;
            this.ctx.shadowColor = powerup.color;
            this.ctx.fillStyle = powerup.color;

            // Draw star shape
            this.drawStar(x + this.gridSize / 2, y + this.gridSize / 2 + float, this.gridSize / 3, 5);

            this.ctx.shadowBlur = 0;

            // Draw icon
            this.ctx.fillStyle = '#fff';
            this.ctx.font = '12px Arial';
            this.ctx.textAlign = 'center';
            this.ctx.textBaseline = 'middle';
            this.ctx.fillText(powerup.icon, x + this.gridSize / 2, y + this.gridSize / 2 + float);
        }
    }

    drawObstacles() {
        this.ctx.fillStyle = '#666';
        this.ctx.shadowBlur = 5;
        this.ctx.shadowColor = '#000';

        for (const obstacle of this.obstacles) {
            const x = obstacle.x * this.gridSize;
            const y = obstacle.y * this.gridSize;
            this.ctx.fillRect(x + 1, y + 1, this.gridSize - 2, this.gridSize - 2);
        }

        this.ctx.shadowBlur = 0;
    }

    drawParticles() {
        for (const particle of this.particles) {
            this.ctx.fillStyle = particle.color;
            this.ctx.globalAlpha = particle.life;
            this.ctx.beginPath();
            this.ctx.arc(particle.x, particle.y, particle.size, 0, Math.PI * 2);
            this.ctx.fill();
            this.ctx.globalAlpha = 1;
        }
    }

    // Drawing helper functions
    roundRect(x, y, width, height, radius) {
        this.ctx.beginPath();
        this.ctx.moveTo(x + radius, y);
        this.ctx.lineTo(x + width - radius, y);
        this.ctx.quadraticCurveTo(x + width, y, x + width, y + radius);
        this.ctx.lineTo(x + width, y + height - radius);
        this.ctx.quadraticCurveTo(x + width, y + height, x + width - radius, y + height);
        this.ctx.lineTo(x + radius, y + height);
        this.ctx.quadraticCurveTo(x, y + height, x, y + height - radius);
        this.ctx.lineTo(x, y + radius);
        this.ctx.quadraticCurveTo(x, y, x + radius, y);
        this.ctx.closePath();
        this.ctx.fill();
    }

    drawStar(cx, cy, radius, points) {
        const angle = Math.PI / points;
        this.ctx.beginPath();
        for (let i = 0; i < 2 * points; i++) {
            const r = i % 2 === 0 ? radius : radius / 2;
            const x = cx + Math.cos(i * angle) * r;
            const y = cy + Math.sin(i * angle) * r;
            if (i === 0) {
                this.ctx.moveTo(x, y);
            } else {
                this.ctx.lineTo(x, y);
            }
        }
        this.ctx.closePath();
        this.ctx.fill();
    }

    // ========================================
    // PARTICLE SYSTEM
    // ========================================

    createParticles(gridX, gridY, color, count) {
        const x = gridX * this.gridSize + this.gridSize / 2;
        const y = gridY * this.gridSize + this.gridSize / 2;

        for (let i = 0; i < count; i++) {
            this.particles.push({
                x: x,
                y: y,
                vx: (Math.random() - 0.5) * 5,
                vy: (Math.random() - 0.5) * 5,
                size: Math.random() * 3 + 2,
                color: color,
                life: 1
            });
        }
    }

    updateParticles() {
        for (let i = this.particles.length - 1; i >= 0; i--) {
            const particle = this.particles[i];
            particle.x += particle.vx;
            particle.y += particle.vy;
            particle.life -= 0.02;

            if (particle.life <= 0) {
                this.particles.splice(i, 1);
            }
        }
    }

    createBackgroundParticles() {
        const container = document.getElementById('particle-container');
        for (let i = 0; i < 20; i++) {
            const particle = document.createElement('div');
            particle.className = 'bg-particle';
            particle.style.width = Math.random() * 5 + 2 + 'px';
            particle.style.height = particle.style.width;
            particle.style.left = Math.random() * 100 + '%';
            particle.style.top = Math.random() * 100 + '%';
            particle.style.animationDelay = Math.random() * 20 + 's';
            container.appendChild(particle);
        }
    }

    // ========================================
    // GAME CONTROLS
    // ========================================

    togglePause() {
        if (this.gameState === 'playing') {
            this.gameState = 'paused';
            document.getElementById('pause-overlay').classList.add('active');
            if (this.timerInterval) {
                clearInterval(this.timerInterval);
                this.timerInterval = null;
            }
        } else if (this.gameState === 'paused') {
            this.gameState = 'playing';
            document.getElementById('pause-overlay').classList.remove('active');
            if (this.gameMode === 'timed') {
                this.startTimer();
            }
            this.lastMoveTime = Date.now();
            this.gameLoop();
        }
    }

    exitToMenu() {
        this.gameState = 'menu';
        if (this.timerInterval) {
            clearInterval(this.timerInterval);
            this.timerInterval = null;
        }
        this.showMainMenu();
    }

    restart() {
        this.selectDifficulty(this.difficulty);
    }

    gameOver() {
        this.gameState = 'gameover';

        if (this.timerInterval) {
            clearInterval(this.timerInterval);
            this.timerInterval = null;
        }

        // Update statistics
        this.stats.gamesPlayed++;
        if (this.score > this.stats.highScore) {
            this.stats.highScore = this.score;
            document.getElementById('new-highscore').style.display = 'block';
        } else {
            document.getElementById('new-highscore').style.display = 'none';
        }
        if (this.snake.length > this.stats.longestSnake) {
            this.stats.longestSnake = this.snake.length;
        }
        // Update total time statistic (in seconds)
        if (this.gameStartTime) {
            const gameTime = Math.floor((Date.now() - this.gameStartTime) / 1000);
            this.stats.totalTime += gameTime;
        }
        this.saveStats();

        // Show game over overlay
        document.getElementById('final-score').textContent = this.score;
        document.getElementById('final-length').textContent = this.snake.length;

        // Set appropriate title based on game mode
        let title = '💀 GAME OVER';
        if (this.gameMode === 'timed' && this.timeLeft <= 0) {
            title = '⏰ TIME\'S UP!';
        }
        document.getElementById('gameover-title').textContent = title;

        document.getElementById('gameover-overlay').classList.add('active');

        // Create death particles
        if (this.settings.particles) {
            const head = this.snake[0];
            this.createParticles(head.x, head.y, this.getThemeColor('snake'), 30);
        }

        // Play sound
        this.playSound('death');
    }

    // ========================================
    // TIMER (for timed mode)
    // ========================================

    startTimer() {
        // Clear any existing timer before creating a new one
        if (this.timerInterval) {
            clearInterval(this.timerInterval);
            this.timerInterval = null;
        }

        this.timerInterval = setInterval(() => {
            this.timeLeft--;
            document.getElementById('timer').textContent = this.timeLeft;

            if (this.timeLeft <= 0) {
                this.gameOver();
            }
        }, 1000);
    }

    // ========================================
    // UI UPDATES
    // ========================================

    updateUI() {
        document.getElementById('score').textContent = this.score;
        document.getElementById('length').textContent = this.snake.length;
        document.getElementById('combo').textContent = 'x' + this.combo;
    }

    updateActivePowerupsDisplay() {
        const container = document.getElementById('active-powerups');
        container.innerHTML = '';

        for (const powerup of this.activePowerups) {
            const timeLeft = Math.ceil((powerup.endTime - Date.now()) / 1000);

            const div = document.createElement('div');
            div.className = 'powerup-item';
            div.innerHTML = `
                <span class="powerup-icon">${powerup.icon}</span>
                <span class="powerup-name">${powerup.name}</span>
                <span class="powerup-timer">${timeLeft}s</span>
            `;
            container.appendChild(div);
        }
    }

    // ========================================
    // SETTINGS
    // ========================================

    changeTheme(theme) {
        this.settings.theme = theme;
        document.body.className = 'theme-' + theme;
        this.saveSettings();
    }

    toggleSound(enabled) {
        this.settings.sound = enabled;
        this.saveSettings();
    }

    toggleParticles(enabled) {
        this.settings.particles = enabled;
        this.saveSettings();
    }

    toggleGrid(enabled) {
        this.settings.gridLines = enabled;
        this.saveSettings();
    }

    loadSettings() {
        const saved = localStorage.getItem('snakeGameSettings');
        if (saved) {
            this.settings = JSON.parse(saved);
            document.body.className = 'theme-' + this.settings.theme;
            document.getElementById('theme-select').value = this.settings.theme;
            document.getElementById('sound-toggle').checked = this.settings.sound;
            document.getElementById('particle-toggle').checked = this.settings.particles;
            document.getElementById('grid-toggle').checked = this.settings.gridLines;
        }
    }

    saveSettings() {
        localStorage.setItem('snakeGameSettings', JSON.stringify(this.settings));
    }

    getThemeColor(type) {
        const theme = this.settings.theme;
        const colors = {
            classic: {
                background: '#0a0a0a',
                snake: '#4ecca3',
                food: '#ff6b6b',
                grid: 'rgba(255, 255, 255, 0.05)'
            },
            neon: {
                background: '#000',
                snake: '#00ffff',
                food: '#ff0080',
                grid: 'rgba(255, 0, 255, 0.1)'
            },
            retro: {
                background: '#1a1a00',
                snake: '#90ee90',
                food: '#ff4500',
                grid: 'rgba(255, 165, 0, 0.1)'
            },
            nature: {
                background: '#0d1f0d',
                snake: '#32cd32',
                food: '#ff6347',
                grid: 'rgba(144, 238, 144, 0.05)'
            }
        };

        return colors[theme][type];
    }

    // ========================================
    // STATISTICS
    // ========================================

    loadStats() {
        const saved = localStorage.getItem('snakeGameStats');
        if (saved) {
            return JSON.parse(saved);
        }
        return {
            gamesPlayed: 0,
            highScore: 0,
            longestSnake: 0,
            totalFood: 0,
            powerupsUsed: 0,
            totalTime: 0
        };
    }

    saveStats() {
        localStorage.setItem('snakeGameStats', JSON.stringify(this.stats));
    }

    updateStatisticsDisplay() {
        document.getElementById('stat-games').textContent = this.stats.gamesPlayed;
        document.getElementById('stat-highscore').textContent = this.stats.highScore;
        document.getElementById('stat-longest').textContent = this.stats.longestSnake;
        document.getElementById('stat-food').textContent = this.stats.totalFood;
        document.getElementById('stat-powerups').textContent = this.stats.powerupsUsed;
        document.getElementById('stat-time').textContent = Math.floor(this.stats.totalTime / 60);
    }

    resetStats() {
        if (confirm('Are you sure you want to reset all statistics? This cannot be undone.')) {
            this.stats = {
                gamesPlayed: 0,
                highScore: 0,
                longestSnake: 0,
                totalFood: 0,
                powerupsUsed: 0,
                totalTime: 0
            };
            this.saveStats();
            this.updateStatisticsDisplay();
        }
    }

    // ========================================
    // SOUND EFFECTS
    // ========================================

    playSound(type) {
        if (!this.settings.sound) return;

        // Create audio context once and reuse it to prevent memory leak
        if (!this.audioContext) {
            this.audioContext = new (window.AudioContext || window.webkitAudioContext)();
        }

        const oscillator = this.audioContext.createOscillator();
        const gainNode = this.audioContext.createGain();

        oscillator.connect(gainNode);
        gainNode.connect(this.audioContext.destination);

        switch (type) {
            case 'eat':
                oscillator.frequency.value = 600;
                gainNode.gain.value = 0.1;
                oscillator.start();
                oscillator.stop(this.audioContext.currentTime + 0.1);
                break;
            case 'powerup':
                oscillator.frequency.value = 800;
                gainNode.gain.value = 0.1;
                oscillator.start();
                oscillator.stop(this.audioContext.currentTime + 0.15);
                break;
            case 'death':
                oscillator.frequency.value = 200;
                gainNode.gain.value = 0.15;
                oscillator.start();
                oscillator.stop(this.audioContext.currentTime + 0.3);
                break;
        }
    }
}

// Initialize game when DOM is loaded
let game;
document.addEventListener('DOMContentLoaded', () => {
    game = new SnakeGame();
});
