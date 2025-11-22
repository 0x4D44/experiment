/**
 * Tower Defense Game - Main Game Engine
 * A fully functional tower defense game with multiple tower types, enemies, and waves
 */

// ============================================================================
// CONSTANTS AND CONFIGURATION
// ============================================================================

const GRID_SIZE = 40; // Size of each grid cell
const GRID_WIDTH = 20; // Number of cells horizontally
const GRID_HEIGHT = 15; // Number of cells vertically
const CANVAS_WIDTH = GRID_SIZE * GRID_WIDTH;
const CANVAS_HEIGHT = GRID_SIZE * GRID_HEIGHT;
const MAX_PARTICLES = 500; // Maximum number of particles to prevent performance issues

// Tower configurations
const TOWER_TYPES = {
    basic: {
        name: 'Basic Tower',
        cost: 50,
        damage: 15,
        range: 120,
        fireRate: 1.0, // seconds
        color: '#4299e1',
        projectileColor: '#4299e1',
        projectileSpeed: 5,
        icon: '🎯'
    },
    rapid: {
        name: 'Rapid Tower',
        cost: 70,
        damage: 8,
        range: 100,
        fireRate: 0.3,
        color: '#f6e05e',
        projectileColor: '#f6e05e',
        projectileSpeed: 7,
        icon: '⚡'
    },
    splash: {
        name: 'Splash Tower',
        cost: 100,
        damage: 25,
        range: 110,
        fireRate: 1.5,
        color: '#f56565',
        projectileColor: '#f56565',
        projectileSpeed: 4,
        splashRadius: 50,
        icon: '💣'
    },
    sniper: {
        name: 'Sniper Tower',
        cost: 120,
        damage: 50,
        range: 200,
        fireRate: 2.0,
        color: '#9f7aea',
        projectileColor: '#9f7aea',
        projectileSpeed: 10,
        icon: '🔭'
    },
    slow: {
        name: 'Frost Tower',
        cost: 80,
        damage: 10,
        range: 90,
        fireRate: 0.8,
        color: '#63b3ed',
        projectileColor: '#90cdf4',
        projectileSpeed: 6,
        slowEffect: 0.5, // 50% slow
        slowDuration: 2.0, // seconds
        icon: '❄️'
    }
};

// Enemy configurations
const ENEMY_TYPES = {
    basic: {
        name: 'Basic',
        health: 100,
        speed: 1.0,
        reward: 10,
        color: '#fc8181',
        size: 12,
        icon: '🔴'
    },
    fast: {
        name: 'Fast',
        health: 80,
        speed: 1.5,
        reward: 15,
        color: '#68d391',
        size: 10,
        icon: '🟢'
    },
    tank: {
        name: 'Tank',
        health: 250,
        speed: 0.7,
        reward: 30,
        color: '#63b3ed',
        size: 16,
        icon: '🔵'
    },
    swarm: {
        name: 'Swarm',
        health: 50,
        speed: 1.2,
        reward: 8,
        color: '#f6e05e',
        size: 8,
        icon: '🟡'
    },
    boss: {
        name: 'Boss',
        health: 500,
        speed: 0.5,
        reward: 100,
        color: '#b794f4',
        size: 20,
        icon: '🟣'
    }
};

// Wave configurations
const WAVES = [
    { enemies: [{ type: 'basic', count: 10, interval: 1.0 }] },
    { enemies: [{ type: 'basic', count: 15, interval: 0.8 }] },
    { enemies: [{ type: 'basic', count: 10, interval: 0.7 }, { type: 'fast', count: 5, interval: 0.7 }] },
    { enemies: [{ type: 'fast', count: 15, interval: 0.6 }] },
    { enemies: [{ type: 'basic', count: 10, interval: 0.5 }, { type: 'tank', count: 3, interval: 2.0 }] },
    { enemies: [{ type: 'swarm', count: 30, interval: 0.4 }] },
    { enemies: [{ type: 'tank', count: 5, interval: 1.5 }, { type: 'fast', count: 10, interval: 0.6 }] },
    { enemies: [{ type: 'basic', count: 15, interval: 0.4 }, { type: 'fast', count: 10, interval: 0.4 }, { type: 'tank', count: 5, interval: 1.0 }] },
    { enemies: [{ type: 'tank', count: 8, interval: 1.0 }, { type: 'swarm', count: 20, interval: 0.3 }] },
    { enemies: [{ type: 'boss', count: 3, interval: 3.0 }, { type: 'tank', count: 5, interval: 1.5 }, { type: 'fast', count: 15, interval: 0.5 }] }
];

// ============================================================================
// GAME STATE
// ============================================================================

const gameState = {
    gold: 200,
    lives: 20,
    wave: 0,
    score: 0,
    paused: false,
    gameOver: false,
    gameWon: false,
    selectedTowerType: null,
    selectedTower: null,
    gameSpeed: 1,
    waveInProgress: false,
    enemiesSpawned: 0,
    totalEnemiesInWave: 0
};

// ============================================================================
// PATH FINDING AND MAP GENERATION
// ============================================================================

// Create a path for enemies to follow
function generatePath() {
    const path = [];
    const startX = 0;
    const startY = Math.floor(GRID_HEIGHT / 2);

    // Create a winding path from left to right
    let x = startX;
    let y = startY;

    path.push({ x, y });

    // Move right with some vertical variation
    while (x < GRID_WIDTH - 1) {
        // Randomly choose to move right or change y
        const choice = Math.random();

        if (choice < 0.7 && x < GRID_WIDTH - 1) {
            x++;
        } else if (choice < 0.85 && y > 2) {
            y--;
        } else if (y < GRID_HEIGHT - 3) {
            y++;
        } else {
            x++;
        }

        path.push({ x, y });
    }

    // Ensure path reaches the end (but stay within grid bounds)
    while (x < GRID_WIDTH - 1) {
        x++;
        path.push({ x, y });
    }

    return path;
}

const PATH = generatePath();

// Convert grid path to pixel coordinates
function getPathCoordinates() {
    return PATH.map(p => ({
        x: p.x * GRID_SIZE + GRID_SIZE / 2,
        y: p.y * GRID_SIZE + GRID_SIZE / 2
    }));
}

const PATH_COORDS = getPathCoordinates();

// ============================================================================
// GAME ENTITIES
// ============================================================================

class Enemy {
    constructor(type) {
        const config = ENEMY_TYPES[type];
        this.type = type;
        this.health = config.health;
        this.maxHealth = config.health;
        this.speed = config.speed;
        this.reward = config.reward;
        this.color = config.color;
        this.size = config.size;

        this.pathIndex = 0;
        this.x = PATH_COORDS[0].x;
        this.y = PATH_COORDS[0].y;
        this.targetX = PATH_COORDS[1].x;
        this.targetY = PATH_COORDS[1].y;

        this.slowEffect = 1.0; // Multiplier for speed
        this.slowTimer = 0;

        this.alive = true;
        this.reachedEnd = false;
    }

    update(deltaTime) {
        if (!this.alive || this.reachedEnd) return;

        // Update slow effect
        if (this.slowTimer > 0) {
            this.slowTimer -= deltaTime;
            if (this.slowTimer <= 0) {
                this.slowEffect = 1.0;
            }
        }

        // Calculate movement
        const effectiveSpeed = this.speed * this.slowEffect * gameState.gameSpeed;
        const dx = this.targetX - this.x;
        const dy = this.targetY - this.y;
        const distance = Math.sqrt(dx * dx + dy * dy);

        if (distance < effectiveSpeed) {
            // Reached waypoint, move to next
            this.pathIndex++;

            if (this.pathIndex >= PATH_COORDS.length) {
                this.reachedEnd = true;
                this.alive = false;
                return;
            }

            this.x = this.targetX;
            this.y = this.targetY;
            this.targetX = PATH_COORDS[this.pathIndex].x;
            this.targetY = PATH_COORDS[this.pathIndex].y;
        } else {
            // Move towards target
            this.x += (dx / distance) * effectiveSpeed;
            this.y += (dy / distance) * effectiveSpeed;
        }
    }

    takeDamage(damage) {
        this.health -= damage;
        if (this.health <= 0) {
            this.alive = false;
            return true; // Enemy killed
        }
        return false;
    }

    applySlow(slowAmount, duration) {
        // Keep the strongest slow effect (lowest multiplier) and maximum duration
        if (slowAmount < this.slowEffect || this.slowTimer <= 0) {
            this.slowEffect = slowAmount;
        }
        // Always extend duration to the maximum
        this.slowTimer = Math.max(this.slowTimer, duration);
    }

    draw(ctx) {
        if (!this.alive) return;

        // Draw enemy circle
        ctx.fillStyle = this.color;
        ctx.beginPath();
        ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2);
        ctx.fill();

        // Draw border
        ctx.strokeStyle = '#000';
        ctx.lineWidth = 2;
        ctx.stroke();

        // Draw health bar
        const barWidth = this.size * 2;
        const barHeight = 4;
        const barX = this.x - barWidth / 2;
        const barY = this.y - this.size - 8;

        // Background
        ctx.fillStyle = '#333';
        ctx.fillRect(barX, barY, barWidth, barHeight);

        // Health
        const healthPercent = this.health / this.maxHealth;
        ctx.fillStyle = healthPercent > 0.5 ? '#48bb78' : healthPercent > 0.25 ? '#f6e05e' : '#f56565';
        ctx.fillRect(barX, barY, barWidth * healthPercent, barHeight);

        // Draw slow effect indicator
        if (this.slowTimer > 0) {
            ctx.fillStyle = 'rgba(99, 179, 237, 0.5)';
            ctx.beginPath();
            ctx.arc(this.x, this.y, this.size + 4, 0, Math.PI * 2);
            ctx.fill();
        }
    }
}

class Tower {
    constructor(type, gridX, gridY) {
        const config = TOWER_TYPES[type];
        this.type = type;
        this.gridX = gridX;
        this.gridY = gridY;
        this.x = gridX * GRID_SIZE + GRID_SIZE / 2;
        this.y = gridY * GRID_SIZE + GRID_SIZE / 2;

        this.level = 1;
        this.damage = config.damage;
        this.range = config.range;
        this.fireRate = config.fireRate;
        this.color = config.color;
        this.projectileColor = config.projectileColor;
        this.projectileSpeed = config.projectileSpeed;

        this.splashRadius = config.splashRadius || 0;
        this.slowEffect = config.slowEffect || 0;
        this.slowDuration = config.slowDuration || 0;

        this.fireTimer = 0;
        this.target = null;
        this.kills = 0;
        this.totalCost = config.cost;
    }

    update(deltaTime, enemies) {
        this.fireTimer -= deltaTime * gameState.gameSpeed;

        // Find target
        if (!this.target || !this.target.alive || !this.isInRange(this.target)) {
            this.target = this.findTarget(enemies);
        }

        // Fire at target
        if (this.target && this.fireTimer <= 0) {
            this.fire();
            this.fireTimer = this.fireRate;
        }
    }

    findTarget(enemies) {
        let closestEnemy = null;
        let maxProgress = -1;

        for (const enemy of enemies) {
            if (!enemy.alive) continue;

            if (this.isInRange(enemy)) {
                // Target enemy furthest along the path
                if (enemy.pathIndex > maxProgress) {
                    maxProgress = enemy.pathIndex;
                    closestEnemy = enemy;
                }
            }
        }

        return closestEnemy;
    }

    isInRange(enemy) {
        const dx = enemy.x - this.x;
        const dy = enemy.y - this.y;
        const distance = Math.sqrt(dx * dx + dy * dy);
        return distance <= this.range;
    }

    fire() {
        if (!this.target) return;

        const projectile = new Projectile(
            this.x,
            this.y,
            this.target,
            this.damage,
            this.projectileColor,
            this.projectileSpeed,
            this.splashRadius,
            this.slowEffect,
            this.slowDuration,
            this
        );

        projectiles.push(projectile);

        // Play sound
        playSound('shoot');
    }

    upgrade() {
        this.level++;
        this.damage = Math.floor(this.damage * 1.5);
        this.range = Math.floor(this.range * 1.1);
        this.fireRate = this.fireRate * 0.9;

        const upgradeCost = this.getUpgradeCost();
        this.totalCost += upgradeCost;
    }

    getUpgradeCost() {
        return Math.floor(TOWER_TYPES[this.type].cost * this.level * 0.7);
    }

    getSellValue() {
        return Math.floor(this.totalCost * 0.7);
    }

    draw(ctx) {
        // Draw range (if selected)
        if (gameState.selectedTower === this) {
            ctx.strokeStyle = this.color;
            ctx.globalAlpha = 0.2;
            ctx.lineWidth = 2;
            ctx.beginPath();
            ctx.arc(this.x, this.y, this.range, 0, Math.PI * 2);
            ctx.stroke();
            ctx.globalAlpha = 1.0;
        }

        // Draw tower base
        const size = GRID_SIZE * 0.7;
        ctx.fillStyle = this.color;
        ctx.fillRect(
            this.x - size / 2,
            this.y - size / 2,
            size,
            size
        );

        // Draw border
        ctx.strokeStyle = '#000';
        ctx.lineWidth = 2;
        ctx.strokeRect(
            this.x - size / 2,
            this.y - size / 2,
            size,
            size
        );

        // Draw tower icon
        ctx.fillStyle = '#fff';
        ctx.font = `${size * 0.5}px Arial`;
        ctx.textAlign = 'center';
        ctx.textBaseline = 'middle';
        ctx.fillText(TOWER_TYPES[this.type].icon, this.x, this.y);

        // Draw level indicator
        if (this.level > 1) {
            ctx.fillStyle = '#ffd700';
            ctx.font = 'bold 12px Arial';
            ctx.fillText(`L${this.level}`, this.x, this.y + size / 2 + 10);
        }

        // Draw targeting line
        if (this.target && this.target.alive) {
            ctx.strokeStyle = this.color;
            ctx.globalAlpha = 0.3;
            ctx.lineWidth = 2;
            ctx.beginPath();
            ctx.moveTo(this.x, this.y);
            ctx.lineTo(this.target.x, this.target.y);
            ctx.stroke();
            ctx.globalAlpha = 1.0;
        }
    }
}

class Projectile {
    constructor(x, y, target, damage, color, speed, splashRadius, slowEffect, slowDuration, tower) {
        this.x = x;
        this.y = y;
        this.target = target;
        this.damage = damage;
        this.color = color;
        this.speed = speed;
        this.splashRadius = splashRadius;
        this.slowEffect = slowEffect;
        this.slowDuration = slowDuration;
        this.tower = tower;

        this.active = true;
    }

    update(deltaTime) {
        if (!this.active || !this.target.alive) {
            this.active = false;
            return;
        }

        const effectiveSpeed = this.speed * gameState.gameSpeed;
        const dx = this.target.x - this.x;
        const dy = this.target.y - this.y;
        const distance = Math.sqrt(dx * dx + dy * dy);

        if (distance < effectiveSpeed) {
            // Hit target
            this.hit();
            this.active = false;
        } else {
            // Move towards target
            this.x += (dx / distance) * effectiveSpeed;
            this.y += (dy / distance) * effectiveSpeed;
        }
    }

    hit() {
        if (this.splashRadius > 0) {
            // Splash damage
            for (const enemy of enemies) {
                if (!enemy.alive) continue;

                const dx = enemy.x - this.target.x;
                const dy = enemy.y - this.target.y;
                const distance = Math.sqrt(dx * dx + dy * dy);

                if (distance <= this.splashRadius) {
                    const killed = enemy.takeDamage(this.damage);
                    if (killed) {
                        this.tower.kills++;
                        gameState.gold += enemy.reward;
                        gameState.score += enemy.reward * 10;
                        playSound('kill');
                    }

                    // Apply slow effect
                    if (this.slowEffect > 0) {
                        enemy.applySlow(this.slowEffect, this.slowDuration);
                    }

                    // Damage numbers
                    damageNumbers.push(new DamageNumber(enemy.x, enemy.y, this.damage));
                }
            }

            // Explosion effect
            particles.push(...createExplosion(this.target.x, this.target.y, this.color));
            playSound('explosion');
        } else {
            // Single target damage
            const killed = this.target.takeDamage(this.damage);
            if (killed) {
                this.tower.kills++;
                gameState.gold += this.target.reward;
                gameState.score += this.target.reward * 10;
                playSound('kill');
            }

            // Apply slow effect
            if (this.slowEffect > 0) {
                this.target.applySlow(this.slowEffect, this.slowDuration);
            }

            // Damage numbers
            damageNumbers.push(new DamageNumber(this.target.x, this.target.y, this.damage));

            // Hit effect
            particles.push(...createHitEffect(this.target.x, this.target.y, this.color));
            playSound('hit');
        }
    }

    draw(ctx) {
        if (!this.active) return;

        ctx.fillStyle = this.color;
        ctx.beginPath();
        ctx.arc(this.x, this.y, 4, 0, Math.PI * 2);
        ctx.fill();

        // Draw trail
        ctx.strokeStyle = this.color;
        ctx.globalAlpha = 0.5;
        ctx.lineWidth = 2;
        ctx.lineCap = 'round';

        const dx = this.target.x - this.x;
        const dy = this.target.y - this.y;
        const distance = Math.sqrt(dx * dx + dy * dy);
        const trailLength = 10;

        if (distance > 0) {
            const trailX = this.x - (dx / distance) * trailLength;
            const trailY = this.y - (dy / distance) * trailLength;

            ctx.beginPath();
            ctx.moveTo(this.x, this.y);
            ctx.lineTo(trailX, trailY);
            ctx.stroke();
        }

        ctx.globalAlpha = 1.0;
    }
}

class Particle {
    constructor(x, y, vx, vy, color, life) {
        this.x = x;
        this.y = y;
        this.vx = vx;
        this.vy = vy;
        this.color = color;
        this.life = life;
        this.maxLife = life;
        this.size = Math.random() * 3 + 2;
    }

    update(deltaTime) {
        this.x += this.vx * gameState.gameSpeed;
        this.y += this.vy * gameState.gameSpeed;
        this.life -= deltaTime * gameState.gameSpeed;
    }

    draw(ctx) {
        const alpha = this.life / this.maxLife;
        ctx.fillStyle = this.color;
        ctx.globalAlpha = alpha;
        ctx.beginPath();
        ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2);
        ctx.fill();
        ctx.globalAlpha = 1.0;
    }
}

class DamageNumber {
    constructor(x, y, damage) {
        this.x = x;
        this.y = y;
        this.damage = Math.floor(damage);
        this.life = 1.0;
        this.vy = -1;
    }

    update(deltaTime) {
        this.y += this.vy * gameState.gameSpeed;
        this.life -= deltaTime * gameState.gameSpeed;
    }

    draw(ctx) {
        ctx.fillStyle = `rgba(255, 255, 255, ${this.life})`;
        ctx.font = 'bold 16px Arial';
        ctx.textAlign = 'center';
        ctx.strokeStyle = `rgba(0, 0, 0, ${this.life})`;
        ctx.lineWidth = 3;
        ctx.strokeText(this.damage.toString(), this.x, this.y);
        ctx.fillText(this.damage.toString(), this.x, this.y);
    }
}

// ============================================================================
// PARTICLE EFFECTS
// ============================================================================

function createExplosion(x, y, color) {
    const newParticles = [];
    const count = 20;

    // Check if we have room for more particles
    const availableSlots = MAX_PARTICLES - particles.length;
    if (availableSlots <= 0) return newParticles;

    const actualCount = Math.min(count, availableSlots);

    for (let i = 0; i < actualCount; i++) {
        const angle = (Math.PI * 2 * i) / actualCount;
        const speed = Math.random() * 2 + 1;
        const vx = Math.cos(angle) * speed;
        const vy = Math.sin(angle) * speed;

        newParticles.push(new Particle(x, y, vx, vy, color, 0.5));
    }

    return newParticles;
}

function createHitEffect(x, y, color) {
    const newParticles = [];
    const count = 5;

    // Check if we have room for more particles
    const availableSlots = MAX_PARTICLES - particles.length;
    if (availableSlots <= 0) return newParticles;

    const actualCount = Math.min(count, availableSlots);

    for (let i = 0; i < actualCount; i++) {
        const angle = Math.random() * Math.PI * 2;
        const speed = Math.random() * 1.5 + 0.5;
        const vx = Math.cos(angle) * speed;
        const vy = Math.sin(angle) * speed;

        newParticles.push(new Particle(x, y, vx, vy, color, 0.3));
    }

    return newParticles;
}

// ============================================================================
// SOUND SYSTEM
// ============================================================================

const audioContext = new (window.AudioContext || window.webkitAudioContext)();

function playSound(type) {
    if (!audioContext) return;

    const oscillator = audioContext.createOscillator();
    const gainNode = audioContext.createGain();

    oscillator.connect(gainNode);
    gainNode.connect(audioContext.destination);

    switch (type) {
        case 'shoot':
            oscillator.frequency.value = 400;
            gainNode.gain.setValueAtTime(0.1, audioContext.currentTime);
            gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.1);
            oscillator.start(audioContext.currentTime);
            oscillator.stop(audioContext.currentTime + 0.1);
            break;

        case 'hit':
            oscillator.frequency.value = 200;
            gainNode.gain.setValueAtTime(0.15, audioContext.currentTime);
            gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.15);
            oscillator.start(audioContext.currentTime);
            oscillator.stop(audioContext.currentTime + 0.15);
            break;

        case 'explosion':
            oscillator.type = 'sawtooth';
            oscillator.frequency.value = 100;
            gainNode.gain.setValueAtTime(0.2, audioContext.currentTime);
            gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.3);
            oscillator.start(audioContext.currentTime);
            oscillator.stop(audioContext.currentTime + 0.3);
            break;

        case 'kill':
            oscillator.frequency.value = 600;
            gainNode.gain.setValueAtTime(0.15, audioContext.currentTime);
            gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.2);
            oscillator.start(audioContext.currentTime);
            oscillator.stop(audioContext.currentTime + 0.2);
            break;

        case 'place':
            oscillator.frequency.value = 500;
            gainNode.gain.setValueAtTime(0.1, audioContext.currentTime);
            gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.15);
            oscillator.start(audioContext.currentTime);
            oscillator.stop(audioContext.currentTime + 0.15);
            break;

        case 'upgrade':
            oscillator.frequency.value = 800;
            gainNode.gain.setValueAtTime(0.15, audioContext.currentTime);
            gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.25);
            oscillator.start(audioContext.currentTime);
            oscillator.stop(audioContext.currentTime + 0.25);
            break;

        case 'lose':
            oscillator.type = 'sawtooth';
            oscillator.frequency.value = 200;
            gainNode.gain.setValueAtTime(0.2, audioContext.currentTime);
            gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 1.0);
            oscillator.start(audioContext.currentTime);
            oscillator.stop(audioContext.currentTime + 1.0);
            break;

        case 'win':
            oscillator.frequency.value = 800;
            gainNode.gain.setValueAtTime(0.2, audioContext.currentTime);
            gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.5);
            oscillator.start(audioContext.currentTime);
            oscillator.stop(audioContext.currentTime + 0.5);
            break;
    }
}

// ============================================================================
// GAME COLLECTIONS
// ============================================================================

let towers = [];
let enemies = [];
let projectiles = [];
let particles = [];
let damageNumbers = [];

// ============================================================================
// WAVE MANAGEMENT
// ============================================================================

let waveSpawnQueue = [];
let waveSpawnTimer = 0;

function startWave() {
    if (gameState.waveInProgress || gameState.wave >= WAVES.length) return;

    gameState.wave++;
    gameState.waveInProgress = true;

    // Build spawn queue
    waveSpawnQueue = [];
    const wave = WAVES[gameState.wave - 1];

    for (const group of wave.enemies) {
        for (let i = 0; i < group.count; i++) {
            waveSpawnQueue.push({
                type: group.type,
                delay: i * group.interval
            });
        }
    }

    // Sort by delay
    waveSpawnQueue.sort((a, b) => a.delay - b.delay);

    gameState.enemiesSpawned = 0;
    gameState.totalEnemiesInWave = waveSpawnQueue.length;
    waveSpawnTimer = 0;

    updateUI();
}

function updateWaveSpawning(deltaTime) {
    if (!gameState.waveInProgress || waveSpawnQueue.length === 0) return;

    waveSpawnTimer += deltaTime * gameState.gameSpeed;

    while (waveSpawnQueue.length > 0 && waveSpawnTimer >= waveSpawnQueue[0].delay) {
        const spawn = waveSpawnQueue.shift();
        enemies.push(new Enemy(spawn.type));
        gameState.enemiesSpawned++;
    }

    // Check if wave is complete
    if (waveSpawnQueue.length === 0 && enemies.length === 0) {
        gameState.waveInProgress = false;

        // Check win condition
        if (gameState.wave >= WAVES.length) {
            gameWin();
        }
    }
}

// ============================================================================
// RENDERING
// ============================================================================

function drawGrid(ctx) {
    // Draw grid background
    ctx.fillStyle = '#1a202c';
    ctx.fillRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);

    // Draw grid lines
    ctx.strokeStyle = '#2d3748';
    ctx.lineWidth = 1;

    for (let x = 0; x <= GRID_WIDTH; x++) {
        ctx.beginPath();
        ctx.moveTo(x * GRID_SIZE, 0);
        ctx.lineTo(x * GRID_SIZE, CANVAS_HEIGHT);
        ctx.stroke();
    }

    for (let y = 0; y <= GRID_HEIGHT; y++) {
        ctx.beginPath();
        ctx.moveTo(0, y * GRID_SIZE);
        ctx.lineTo(CANVAS_WIDTH, y * GRID_SIZE);
        ctx.stroke();
    }
}

function drawPath(ctx) {
    ctx.strokeStyle = '#4a5568';
    ctx.lineWidth = GRID_SIZE * 0.8;
    ctx.lineCap = 'round';
    ctx.lineJoin = 'round';

    ctx.beginPath();
    for (let i = 0; i < PATH_COORDS.length; i++) {
        const point = PATH_COORDS[i];
        if (i === 0) {
            ctx.moveTo(point.x, point.y);
        } else {
            ctx.lineTo(point.x, point.y);
        }
    }
    ctx.stroke();

    // Draw path borders
    ctx.strokeStyle = '#2d3748';
    ctx.lineWidth = GRID_SIZE * 0.8 + 4;
    ctx.beginPath();
    for (let i = 0; i < PATH_COORDS.length; i++) {
        const point = PATH_COORDS[i];
        if (i === 0) {
            ctx.moveTo(point.x, point.y);
        } else {
            ctx.lineTo(point.x, point.y);
        }
    }
    ctx.stroke();

    // Redraw path
    ctx.strokeStyle = '#4a5568';
    ctx.lineWidth = GRID_SIZE * 0.8;
    ctx.beginPath();
    for (let i = 0; i < PATH_COORDS.length; i++) {
        const point = PATH_COORDS[i];
        if (i === 0) {
            ctx.moveTo(point.x, point.y);
        } else {
            ctx.lineTo(point.x, point.y);
        }
    }
    ctx.stroke();

    // Draw start marker
    ctx.fillStyle = '#48bb78';
    ctx.beginPath();
    ctx.arc(PATH_COORDS[0].x, PATH_COORDS[0].y, 15, 0, Math.PI * 2);
    ctx.fill();
    ctx.fillStyle = '#fff';
    ctx.font = 'bold 16px Arial';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillText('S', PATH_COORDS[0].x, PATH_COORDS[0].y);

    // Draw end marker
    const endPoint = PATH_COORDS[PATH_COORDS.length - 1];
    ctx.fillStyle = '#f56565';
    ctx.beginPath();
    ctx.arc(endPoint.x, endPoint.y, 15, 0, Math.PI * 2);
    ctx.fill();
    ctx.fillStyle = '#fff';
    ctx.fillText('E', endPoint.x, endPoint.y);
}

function drawPlacementPreview(ctx) {
    if (!gameState.selectedTowerType) return;

    const mousePos = getMouseGridPos();
    if (!mousePos) return;

    const x = mousePos.x * GRID_SIZE + GRID_SIZE / 2;
    const y = mousePos.y * GRID_SIZE + GRID_SIZE / 2;

    const canPlace = isValidPlacement(mousePos.x, mousePos.y);
    const config = TOWER_TYPES[gameState.selectedTowerType];

    // Draw range preview
    ctx.strokeStyle = canPlace ? config.color : '#f56565';
    ctx.globalAlpha = 0.3;
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.arc(x, y, config.range, 0, Math.PI * 2);
    ctx.stroke();
    ctx.globalAlpha = 1.0;

    // Draw tower preview
    const size = GRID_SIZE * 0.7;
    ctx.fillStyle = canPlace ? config.color : '#f56565';
    ctx.globalAlpha = 0.5;
    ctx.fillRect(x - size / 2, y - size / 2, size, size);
    ctx.globalAlpha = 1.0;

    // Draw icon
    ctx.fillStyle = '#fff';
    ctx.font = `${size * 0.5}px Arial`;
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillText(config.icon, x, y);
}

// ============================================================================
// INPUT HANDLING
// ============================================================================

let mouseGridPos = null;

function getMouseGridPos() {
    return mouseGridPos;
}

function isValidPlacement(gridX, gridY) {
    // Check if on path
    for (const pathPoint of PATH) {
        if (pathPoint.x === gridX && pathPoint.y === gridY) {
            return false;
        }
    }

    // Check if tower already exists
    for (const tower of towers) {
        if (tower.gridX === gridX && tower.gridY === gridY) {
            return false;
        }
    }

    // Check bounds
    if (gridX < 0 || gridX >= GRID_WIDTH || gridY < 0 || gridY >= GRID_HEIGHT) {
        return false;
    }

    return true;
}

function getEventCoordinates(event) {
    const rect = canvas.getBoundingClientRect();
    let clientX, clientY;

    // Handle both mouse and touch events
    if (event.touches && event.touches.length > 0) {
        clientX = event.touches[0].clientX;
        clientY = event.touches[0].clientY;
    } else if (event.changedTouches && event.changedTouches.length > 0) {
        clientX = event.changedTouches[0].clientX;
        clientY = event.changedTouches[0].clientY;
    } else {
        clientX = event.clientX;
        clientY = event.clientY;
    }

    return {
        x: clientX - rect.left,
        y: clientY - rect.top
    };
}

function handleCanvasClick(event) {
    const coords = getEventCoordinates(event);
    const gridX = Math.floor(coords.x / GRID_SIZE);
    const gridY = Math.floor(coords.y / GRID_SIZE);

    // Check if clicking on existing tower
    for (const tower of towers) {
        if (tower.gridX === gridX && tower.gridY === gridY) {
            selectTower(tower);
            return;
        }
    }

    // Place tower
    if (gameState.selectedTowerType) {
        const config = TOWER_TYPES[gameState.selectedTowerType];

        if (gameState.gold >= config.cost && isValidPlacement(gridX, gridY)) {
            const tower = new Tower(gameState.selectedTowerType, gridX, gridY);
            towers.push(tower);
            gameState.gold -= config.cost;
            playSound('place');
            updateUI();
        }
    } else {
        // Deselect tower if clicking empty space
        deselectTower();
    }
}

function handleCanvasMouseMove(event) {
    const coords = getEventCoordinates(event);
    const gridX = Math.floor(coords.x / GRID_SIZE);
    const gridY = Math.floor(coords.y / GRID_SIZE);

    mouseGridPos = { x: gridX, y: gridY };
}

// Touch event handlers
function handleCanvasTouchStart(event) {
    event.preventDefault(); // Prevent scrolling
    handleCanvasClick(event);
}

function handleCanvasTouchMove(event) {
    event.preventDefault(); // Prevent scrolling
    handleCanvasMouseMove(event);
}

function handleCanvasTouchEnd(event) {
    event.preventDefault();
    // Touch ended, could add additional logic here if needed
}

// ============================================================================
// UI MANAGEMENT
// ============================================================================

function updateUI() {
    document.getElementById('gold').textContent = gameState.gold;
    document.getElementById('lives').textContent = gameState.lives;
    document.getElementById('wave').textContent = `${gameState.wave}/${WAVES.length}`;
    document.getElementById('score').textContent = gameState.score;

    // Update tower cards
    const towerCards = document.querySelectorAll('.tower-card');
    towerCards.forEach(card => {
        const type = card.dataset.towerType;
        const config = TOWER_TYPES[type];

        if (gameState.gold < config.cost) {
            card.classList.add('disabled');
        } else {
            card.classList.remove('disabled');
        }

        if (gameState.selectedTowerType === type) {
            card.classList.add('selected');
        } else {
            card.classList.remove('selected');
        }
    });

    // Update start wave button
    const startWaveBtn = document.getElementById('start-wave-btn');
    if (gameState.waveInProgress) {
        startWaveBtn.disabled = true;
        startWaveBtn.textContent = `🌊 Wave ${gameState.wave} in Progress`;
    } else if (gameState.wave >= WAVES.length) {
        startWaveBtn.disabled = true;
        startWaveBtn.textContent = '🎉 All Waves Complete!';
    } else {
        startWaveBtn.disabled = false;
        startWaveBtn.textContent = `🚀 Start Wave ${gameState.wave + 1}`;
    }
}

function selectTower(tower) {
    gameState.selectedTower = tower;
    gameState.selectedTowerType = null;

    const detailPanel = document.getElementById('tower-detail');
    detailPanel.classList.remove('hidden');

    document.getElementById('detail-type').textContent = TOWER_TYPES[tower.type].name;
    document.getElementById('detail-level').textContent = tower.level;
    document.getElementById('detail-damage').textContent = Math.floor(tower.damage);
    document.getElementById('detail-range').textContent = Math.floor(tower.range);
    document.getElementById('detail-firerate').textContent = tower.fireRate.toFixed(2) + 's';
    document.getElementById('detail-kills').textContent = tower.kills;

    const upgradeCost = tower.getUpgradeCost();
    const sellValue = tower.getSellValue();

    document.getElementById('upgrade-cost').textContent = upgradeCost;
    document.getElementById('sell-value').textContent = sellValue;

    const upgradeBtn = document.getElementById('upgrade-btn');
    upgradeBtn.disabled = gameState.gold < upgradeCost;

    updateUI();
}

function deselectTower() {
    gameState.selectedTower = null;
    document.getElementById('tower-detail').classList.add('hidden');
    updateUI();
}

function showOverlay(title, message, buttons) {
    const overlay = document.getElementById('game-overlay');
    const overlayTitle = document.getElementById('overlay-title');
    const overlayMessage = document.getElementById('overlay-message');
    const overlayButtons = document.getElementById('overlay-buttons');

    overlayTitle.textContent = title;
    overlayMessage.textContent = message;
    overlayButtons.innerHTML = '';

    buttons.forEach(btn => {
        const button = document.createElement('button');
        button.className = `overlay-btn ${btn.class}`;
        button.textContent = btn.text;
        button.onclick = btn.onClick;
        overlayButtons.appendChild(button);
    });

    overlay.classList.remove('hidden');
}

function hideOverlay() {
    document.getElementById('game-overlay').classList.add('hidden');
}

// ============================================================================
// GAME LOOP
// ============================================================================

let lastTime = 0;

function gameLoop(timestamp) {
    const deltaTime = Math.min((timestamp - lastTime) / 1000, 0.1); // Cap at 100ms
    lastTime = timestamp;

    if (!gameState.paused && !gameState.gameOver) {
        update(deltaTime);
    }

    render();

    requestAnimationFrame(gameLoop);
}

function update(deltaTime) {
    // Update wave spawning
    updateWaveSpawning(deltaTime);

    // Update enemies
    for (let i = enemies.length - 1; i >= 0; i--) {
        const enemy = enemies[i];
        enemy.update(deltaTime);

        if (enemy.reachedEnd) {
            gameState.lives--;
            enemies.splice(i, 1);

            if (gameState.lives <= 0) {
                gameOver();
            }

            updateUI();
        } else if (!enemy.alive) {
            enemies.splice(i, 1);
        }
    }

    // Update towers
    for (const tower of towers) {
        tower.update(deltaTime, enemies);
    }

    // Update projectiles
    for (let i = projectiles.length - 1; i >= 0; i--) {
        projectiles[i].update(deltaTime);
        if (!projectiles[i].active) {
            projectiles.splice(i, 1);
        }
    }

    // Update particles
    for (let i = particles.length - 1; i >= 0; i--) {
        particles[i].update(deltaTime);
        if (particles[i].life <= 0) {
            particles.splice(i, 1);
        }
    }

    // Update damage numbers
    for (let i = damageNumbers.length - 1; i >= 0; i--) {
        damageNumbers[i].update(deltaTime);
        if (damageNumbers[i].life <= 0) {
            damageNumbers.splice(i, 1);
        }
    }
}

function render() {
    drawGrid(ctx);
    drawPath(ctx);

    // Draw towers
    for (const tower of towers) {
        tower.draw(ctx);
    }

    // Draw enemies
    for (const enemy of enemies) {
        enemy.draw(ctx);
    }

    // Draw projectiles
    for (const projectile of projectiles) {
        projectile.draw(ctx);
    }

    // Draw particles
    for (const particle of particles) {
        particle.draw(ctx);
    }

    // Draw damage numbers
    for (const dmgNum of damageNumbers) {
        dmgNum.draw(ctx);
    }

    // Draw placement preview
    drawPlacementPreview(ctx);
}

// ============================================================================
// GAME STATE FUNCTIONS
// ============================================================================

function gameOver() {
    gameState.gameOver = true;
    gameState.paused = true;
    playSound('lose');

    showOverlay(
        '💀 Game Over',
        `You survived ${gameState.wave} waves and scored ${gameState.score} points!`,
        [
            {
                text: '🔄 Restart Game',
                class: 'primary',
                onClick: () => {
                    hideOverlay();
                    restartGame();
                }
            }
        ]
    );
}

function gameWin() {
    gameState.gameWon = true;
    gameState.paused = true;
    playSound('win');

    showOverlay(
        '🎉 Victory!',
        `Congratulations! You defeated all waves and scored ${gameState.score} points!`,
        [
            {
                text: '🔄 Play Again',
                class: 'primary',
                onClick: () => {
                    hideOverlay();
                    restartGame();
                }
            }
        ]
    );
}

function restartGame() {
    // Reset game state
    gameState.gold = 200;
    gameState.lives = 20;
    gameState.wave = 0;
    gameState.score = 0;
    gameState.paused = false;
    gameState.gameOver = false;
    gameState.gameWon = false;
    gameState.selectedTowerType = null;
    gameState.selectedTower = null;
    gameState.gameSpeed = 1;
    gameState.waveInProgress = false;

    // Clear all entities
    towers = [];
    enemies = [];
    projectiles = [];
    particles = [];
    damageNumbers = [];
    waveSpawnQueue = [];

    deselectTower();
    updateUI();

    document.getElementById('speed-btn').textContent = '▶️ 1x';
}

function togglePause() {
    if (gameState.gameOver) return;

    gameState.paused = !gameState.paused;

    const pauseBtn = document.getElementById('pause-btn');

    if (gameState.paused) {
        pauseBtn.textContent = '▶️ Resume';
        showOverlay(
            '⏸️ Game Paused',
            'Take your time to plan your defense!',
            [
                {
                    text: '▶️ Resume',
                    class: 'primary',
                    onClick: () => {
                        hideOverlay();
                        togglePause();
                    }
                },
                {
                    text: '🔄 Restart',
                    class: 'secondary',
                    onClick: () => {
                        hideOverlay();
                        restartGame();
                    }
                }
            ]
        );
    } else {
        pauseBtn.textContent = '⏸️ Pause';
        hideOverlay();
    }
}

function toggleSpeed() {
    if (gameState.paused || gameState.gameOver) return;

    const speeds = [1, 1.5, 2];
    const currentIndex = speeds.indexOf(gameState.gameSpeed);
    const nextIndex = (currentIndex + 1) % speeds.length;
    gameState.gameSpeed = speeds[nextIndex];

    document.getElementById('speed-btn').textContent = `▶️ ${gameState.gameSpeed}x`;
}

// ============================================================================
// EVENT LISTENERS
// ============================================================================

// Track event listeners for cleanup
const eventListeners = [];

function addTrackedEventListener(element, event, handler, options) {
    element.addEventListener(event, handler, options);
    eventListeners.push({ element, event, handler, options });
}

// Cleanup function to remove all event listeners
function cleanup() {
    eventListeners.forEach(({ element, event, handler, options }) => {
        element.removeEventListener(event, handler, options);
    });
    eventListeners.length = 0;
}

// Canvas setup
const canvas = document.getElementById('game-canvas');
const ctx = canvas.getContext('2d');
canvas.width = CANVAS_WIDTH;
canvas.height = CANVAS_HEIGHT;

const canvasMouseLeaveHandler = () => {
    mouseGridPos = null;
};

// Mouse events
addTrackedEventListener(canvas, 'click', handleCanvasClick);
addTrackedEventListener(canvas, 'mousemove', handleCanvasMouseMove);
addTrackedEventListener(canvas, 'mouseleave', canvasMouseLeaveHandler);

// Touch events for mobile/tablet support
addTrackedEventListener(canvas, 'touchstart', handleCanvasTouchStart, { passive: false });
addTrackedEventListener(canvas, 'touchmove', handleCanvasTouchMove, { passive: false });
addTrackedEventListener(canvas, 'touchend', handleCanvasTouchEnd, { passive: false });

// Tower selection
document.querySelectorAll('.tower-card').forEach(card => {
    const towerCardClickHandler = () => {
        const type = card.dataset.towerType;
        const config = TOWER_TYPES[type];

        if (gameState.gold >= config.cost) {
            gameState.selectedTowerType = type;
            deselectTower();
            updateUI();
        }
    };
    addTrackedEventListener(card, 'click', towerCardClickHandler);
});

// Tower actions
const upgradeBtnClickHandler = () => {
    if (!gameState.selectedTower) return;

    const upgradeCost = gameState.selectedTower.getUpgradeCost();
    if (gameState.gold >= upgradeCost) {
        gameState.gold -= upgradeCost;
        gameState.selectedTower.upgrade();
        playSound('upgrade');
        selectTower(gameState.selectedTower);
        updateUI();
    }
};
addTrackedEventListener(document.getElementById('upgrade-btn'), 'click', upgradeBtnClickHandler);

const sellBtnClickHandler = () => {
    if (!gameState.selectedTower) return;

    const sellValue = gameState.selectedTower.getSellValue();
    gameState.gold += sellValue;

    const index = towers.indexOf(gameState.selectedTower);
    if (index > -1) {
        towers.splice(index, 1);
    }

    deselectTower();
    updateUI();
};
addTrackedEventListener(document.getElementById('sell-btn'), 'click', sellBtnClickHandler);

// Control buttons
addTrackedEventListener(document.getElementById('start-wave-btn'), 'click', startWave);
addTrackedEventListener(document.getElementById('pause-btn'), 'click', togglePause);
addTrackedEventListener(document.getElementById('speed-btn'), 'click', toggleSpeed);

const restartBtnClickHandler = () => {
    if (confirm('Are you sure you want to restart? All progress will be lost.')) {
        restartGame();
    }
};
addTrackedEventListener(document.getElementById('restart-btn'), 'click', restartBtnClickHandler);

// Keyboard shortcuts
const keydownHandler = (e) => {
    switch (e.key) {
        case ' ':
        case 'p':
        case 'P':
            e.preventDefault();
            togglePause();
            break;
        case 'Escape':
            gameState.selectedTowerType = null;
            deselectTower();
            updateUI();
            break;
        case 's':
        case 'S':
            if (!gameState.waveInProgress && gameState.wave < WAVES.length) {
                startWave();
            }
            break;
        case '1':
            gameState.selectedTowerType = 'basic';
            deselectTower();
            updateUI();
            break;
        case '2':
            gameState.selectedTowerType = 'rapid';
            deselectTower();
            updateUI();
            break;
        case '3':
            gameState.selectedTowerType = 'splash';
            deselectTower();
            updateUI();
            break;
        case '4':
            gameState.selectedTowerType = 'sniper';
            deselectTower();
            updateUI();
            break;
        case '5':
            gameState.selectedTowerType = 'slow';
            deselectTower();
            updateUI();
            break;
    }
};
addTrackedEventListener(document, 'keydown', keydownHandler);

// Cleanup on page unload
const beforeUnloadHandler = () => {
    cleanup();
};
addTrackedEventListener(window, 'beforeunload', beforeUnloadHandler);

// ============================================================================
// INITIALIZATION
// ============================================================================

function init() {
    updateUI();
    requestAnimationFrame(gameLoop);

    // Show welcome message
    showOverlay(
        '🏰 Welcome to Tower Defense!',
        'Build towers to defend against enemy waves. Good luck!',
        [
            {
                text: '🚀 Start Playing',
                class: 'primary',
                onClick: hideOverlay
            }
        ]
    );
}

// Start the game
init();
