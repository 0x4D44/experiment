/**
 * Flappy Bird Game - Complete Implementation
 * Features: Physics, Collision Detection, Particle Effects, Sound, Progressive Difficulty
 */

class FlappyBirdGame {
    constructor() {
        this.canvas = document.getElementById('gameCanvas');
        this.ctx = this.canvas.getContext('2d');

        // Game constants
        this.GRAVITY = 0.5;
        this.FLAP_POWER = -9;
        this.PIPE_WIDTH = 80;
        this.PIPE_GAP = 180;
        this.PIPE_SPEED = 3;
        this.BIRD_SIZE = 34;
        this.GROUND_HEIGHT = 100;

        // Game state
        this.gameState = 'start'; // start, playing, gameOver
        this.score = 0;
        this.highScore = this.loadHighScore();
        this.frame = 0;
        this.soundEnabled = true;

        // Bird
        this.bird = {
            x: 100,
            y: this.canvas.height / 2,
            velocity: 0,
            rotation: 0
        };

        // Pipes
        this.pipes = [];
        this.pipeSpawnTimer = 0;
        this.pipeSpawnInterval = 90; // frames between pipes

        // Particles
        this.particles = [];

        // Background layers for parallax
        this.bgLayers = [
            { offset: 0, speed: 0.5, color: '#87CEEB' },
            { offset: 0, speed: 1, color: '#9CDDEF' },
            { offset: 0, speed: 1.5, color: '#B0E2F7' }
        ];

        // Day/night cycle
        this.timeOfDay = 0; // 0-1, 0 = day, 0.5 = night

        // Initialize
        this.setupEventListeners();
        this.generateSounds();
        this.gameLoop();
    }

    // ==================== INPUT HANDLING ====================

    setupEventListeners() {
        // Space key
        document.addEventListener('keydown', (e) => {
            if (e.code === 'Space') {
                e.preventDefault();
                this.handleInput();
            }
        });

        // Click/Touch
        this.canvas.addEventListener('click', () => this.handleInput());
        this.canvas.addEventListener('touchstart', (e) => {
            e.preventDefault();
            this.handleInput();
        });

        // UI Buttons
        document.getElementById('startButton').addEventListener('click', () => this.startGame());
        document.getElementById('restartButton').addEventListener('click', () => this.restartGame());
        document.getElementById('soundToggle').addEventListener('click', () => this.toggleSound());
    }

    handleInput() {
        if (this.gameState === 'playing') {
            this.flap();
        }
    }

    // ==================== GAME STATE MANAGEMENT ====================

    startGame() {
        this.gameState = 'playing';
        this.score = 0;
        this.bird.y = this.canvas.height / 2;
        this.bird.velocity = 0;
        this.pipes = [];
        this.particles = [];
        this.pipeSpawnTimer = 0;
        this.frame = 0;

        document.getElementById('startScreen').classList.remove('active');
        document.getElementById('scoreDisplay').style.display = 'block';
        document.getElementById('highScoreDisplay').style.display = 'block';

        this.updateScoreDisplay();
    }

    restartGame() {
        document.getElementById('gameOverScreen').classList.remove('active');
        this.startGame();
    }

    endGame() {
        this.gameState = 'gameOver';
        this.playSound('die');
        this.createExplosion(this.bird.x, this.bird.y);

        // Update high score
        if (this.score > this.highScore) {
            this.highScore = this.score;
            this.saveHighScore();
        }

        // Show game over screen
        setTimeout(() => {
            document.getElementById('finalScore').textContent = this.score;
            document.getElementById('finalHighScore').textContent = this.highScore;
            document.getElementById('gameOverScreen').classList.add('active');
            document.getElementById('scoreDisplay').style.display = 'none';
            document.getElementById('highScoreDisplay').style.display = 'none';
        }, 500);
    }

    // ==================== PHYSICS ====================

    flap() {
        this.bird.velocity = this.FLAP_POWER;
        this.playSound('flap');

        // Create flap particles
        for (let i = 0; i < 5; i++) {
            this.particles.push({
                x: this.bird.x - 10,
                y: this.bird.y + Math.random() * 20 - 10,
                vx: -Math.random() * 2 - 1,
                vy: Math.random() * 4 - 2,
                size: Math.random() * 4 + 2,
                life: 1,
                decay: 0.02,
                color: 'rgba(255, 255, 255, 0.8)'
            });
        }
    }

    updatePhysics() {
        if (this.gameState !== 'playing') return;

        // Apply gravity
        this.bird.velocity += this.GRAVITY;
        this.bird.y += this.bird.velocity;

        // Update bird rotation based on velocity
        this.bird.rotation = Math.min(Math.max(this.bird.velocity * 3, -30), 90);

        // Check ceiling and floor collision
        if (this.bird.y < 0) {
            this.bird.y = 0;
            this.bird.velocity = 0;
        }

        if (this.bird.y + this.BIRD_SIZE > this.canvas.height - this.GROUND_HEIGHT) {
            this.playSound('hit');
            this.endGame();
        }
    }

    // ==================== PIPE MANAGEMENT ====================

    updatePipes() {
        if (this.gameState !== 'playing') return;

        // Spawn new pipes
        this.pipeSpawnTimer++;
        if (this.pipeSpawnTimer >= this.pipeSpawnInterval) {
            this.spawnPipe();
            this.pipeSpawnTimer = 0;

            // Progressive difficulty - pipes get slightly faster and closer
            if (this.score > 0 && this.score % 5 === 0) {
                this.pipeSpawnInterval = Math.max(70, this.pipeSpawnInterval - 1);
            }
        }

        // Update existing pipes
        for (let i = this.pipes.length - 1; i >= 0; i--) {
            const pipe = this.pipes[i];

            // Move pipe
            pipe.x -= this.PIPE_SPEED + Math.floor(this.score / 10) * 0.2; // Progressive speed

            // Score when bird passes pipe
            if (!pipe.scored && pipe.x + this.PIPE_WIDTH < this.bird.x) {
                pipe.scored = true;
                this.score++;
                this.playSound('score');
                this.updateScoreDisplay();

                // Spawn score particles
                for (let j = 0; j < 10; j++) {
                    this.particles.push({
                        x: this.bird.x,
                        y: this.bird.y,
                        vx: Math.random() * 4 - 2,
                        vy: Math.random() * 4 - 2,
                        size: Math.random() * 3 + 1,
                        life: 1,
                        decay: 0.02,
                        color: `hsl(${Math.random() * 60 + 30}, 100%, 50%)`
                    });
                }
            }

            // Remove off-screen pipes
            if (pipe.x + this.PIPE_WIDTH < 0) {
                this.pipes.splice(i, 1);
            }
        }
    }

    spawnPipe() {
        const minGapY = 100;
        const maxGapY = this.canvas.height - this.GROUND_HEIGHT - this.PIPE_GAP - 100;
        const gapY = Math.random() * (maxGapY - minGapY) + minGapY;

        this.pipes.push({
            x: this.canvas.width,
            gapY: gapY,
            scored: false
        });
    }

    // ==================== COLLISION DETECTION ====================

    checkCollisions() {
        if (this.gameState !== 'playing') return;

        const birdLeft = this.bird.x - this.BIRD_SIZE / 2;
        const birdRight = this.bird.x + this.BIRD_SIZE / 2;
        const birdTop = this.bird.y - this.BIRD_SIZE / 2;
        const birdBottom = this.bird.y + this.BIRD_SIZE / 2;

        for (const pipe of this.pipes) {
            const pipeLeft = pipe.x;
            const pipeRight = pipe.x + this.PIPE_WIDTH;

            // Check if bird is within pipe's x range
            if (birdRight > pipeLeft && birdLeft < pipeRight) {
                // Check collision with top pipe
                if (birdTop < pipe.gapY) {
                    this.playSound('hit');
                    this.endGame();
                    return;
                }

                // Check collision with bottom pipe
                if (birdBottom > pipe.gapY + this.PIPE_GAP) {
                    this.playSound('hit');
                    this.endGame();
                    return;
                }
            }
        }
    }

    // ==================== PARTICLE SYSTEM ====================

    updateParticles() {
        for (let i = this.particles.length - 1; i >= 0; i--) {
            const p = this.particles[i];

            p.x += p.vx;
            p.y += p.vy;
            p.vy += 0.2; // Gravity
            p.life -= p.decay;

            if (p.life <= 0) {
                this.particles.splice(i, 1);
            }
        }
    }

    createExplosion(x, y) {
        for (let i = 0; i < 30; i++) {
            const angle = (Math.PI * 2 * i) / 30;
            const speed = Math.random() * 5 + 3;

            this.particles.push({
                x: x,
                y: y,
                vx: Math.cos(angle) * speed,
                vy: Math.sin(angle) * speed,
                size: Math.random() * 6 + 3,
                life: 1,
                decay: 0.015,
                color: `hsl(${Math.random() * 60}, 100%, 50%)`
            });
        }
    }

    // ==================== RENDERING ====================

    render() {
        // Clear canvas
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

        // Render background with day/night cycle
        this.renderBackground();

        // Render pipes
        this.renderPipes();

        // Render ground
        this.renderGround();

        // Render bird
        this.renderBird();

        // Render particles
        this.renderParticles();
    }

    renderBackground() {
        // Day/night cycle
        this.timeOfDay = (Math.sin(this.frame * 0.001) + 1) / 2;

        // Sky gradient
        const gradient = this.ctx.createLinearGradient(0, 0, 0, this.canvas.height);

        if (this.timeOfDay < 0.5) {
            // Day
            const t = this.timeOfDay * 2;
            gradient.addColorStop(0, this.interpolateColor('#87CEEB', '#1e3a8a', t));
            gradient.addColorStop(1, this.interpolateColor('#B0E2F7', '#3b82f6', t));
        } else {
            // Night
            const t = (this.timeOfDay - 0.5) * 2;
            gradient.addColorStop(0, this.interpolateColor('#1e3a8a', '#0f172a', t));
            gradient.addColorStop(1, this.interpolateColor('#3b82f6', '#1e293b', t));
        }

        this.ctx.fillStyle = gradient;
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

        // Parallax clouds/stars
        this.renderParallaxElements();
    }

    renderParallaxElements() {
        if (this.gameState !== 'playing') return;

        // Clouds during day, stars during night
        if (this.timeOfDay < 0.5) {
            // Clouds
            this.ctx.fillStyle = 'rgba(255, 255, 255, 0.3)';
            for (let i = 0; i < 5; i++) {
                const x = ((this.frame * 0.3 + i * 150) % (this.canvas.width + 100)) - 50;
                const y = 50 + i * 30;
                this.drawCloud(x, y);
            }
        } else {
            // Stars
            this.ctx.fillStyle = 'rgba(255, 255, 255, 0.8)';
            for (let i = 0; i < 20; i++) {
                const x = (i * 73) % this.canvas.width;
                const y = (i * 41) % (this.canvas.height - this.GROUND_HEIGHT);
                const twinkle = Math.sin(this.frame * 0.1 + i) * 0.5 + 0.5;
                this.ctx.globalAlpha = twinkle;
                this.ctx.fillRect(x, y, 2, 2);
            }
            this.ctx.globalAlpha = 1;
        }
    }

    drawCloud(x, y) {
        this.ctx.beginPath();
        this.ctx.arc(x, y, 15, 0, Math.PI * 2);
        this.ctx.arc(x + 15, y - 5, 20, 0, Math.PI * 2);
        this.ctx.arc(x + 30, y, 15, 0, Math.PI * 2);
        this.ctx.fill();
    }

    renderBird() {
        this.ctx.save();
        this.ctx.translate(this.bird.x, this.bird.y);
        this.ctx.rotate((this.bird.rotation * Math.PI) / 180);

        // Bird body
        this.ctx.fillStyle = '#FFD700';
        this.ctx.beginPath();
        this.ctx.arc(0, 0, this.BIRD_SIZE / 2, 0, Math.PI * 2);
        this.ctx.fill();

        // Bird outline
        this.ctx.strokeStyle = '#FFA500';
        this.ctx.lineWidth = 3;
        this.ctx.stroke();

        // Eye
        this.ctx.fillStyle = 'white';
        this.ctx.beginPath();
        this.ctx.arc(8, -5, 6, 0, Math.PI * 2);
        this.ctx.fill();

        this.ctx.fillStyle = 'black';
        this.ctx.beginPath();
        this.ctx.arc(10, -5, 3, 0, Math.PI * 2);
        this.ctx.fill();

        // Beak
        this.ctx.fillStyle = '#FF6347';
        this.ctx.beginPath();
        this.ctx.moveTo(15, 0);
        this.ctx.lineTo(25, -3);
        this.ctx.lineTo(25, 3);
        this.ctx.closePath();
        this.ctx.fill();

        // Wing
        this.ctx.fillStyle = '#FFA500';
        this.ctx.beginPath();
        this.ctx.ellipse(-5, 5, 10, 15, Math.sin(this.frame * 0.2) * 0.3, 0, Math.PI * 2);
        this.ctx.fill();

        this.ctx.restore();
    }

    renderPipes() {
        for (const pipe of this.pipes) {
            // Top pipe
            const topGradient = this.ctx.createLinearGradient(pipe.x, 0, pipe.x + this.PIPE_WIDTH, 0);
            topGradient.addColorStop(0, '#4CAF50');
            topGradient.addColorStop(0.5, '#45a049');
            topGradient.addColorStop(1, '#3d8b40');

            this.ctx.fillStyle = topGradient;
            this.ctx.fillRect(pipe.x, 0, this.PIPE_WIDTH, pipe.gapY);

            // Top pipe cap
            this.ctx.fillRect(pipe.x - 5, pipe.gapY - 30, this.PIPE_WIDTH + 10, 30);

            // Bottom pipe
            const bottomGradient = this.ctx.createLinearGradient(pipe.x, 0, pipe.x + this.PIPE_WIDTH, 0);
            bottomGradient.addColorStop(0, '#4CAF50');
            bottomGradient.addColorStop(0.5, '#45a049');
            bottomGradient.addColorStop(1, '#3d8b40');

            this.ctx.fillStyle = bottomGradient;
            this.ctx.fillRect(pipe.x, pipe.gapY + this.PIPE_GAP, this.PIPE_WIDTH,
                this.canvas.height - this.GROUND_HEIGHT - pipe.gapY - this.PIPE_GAP);

            // Bottom pipe cap
            this.ctx.fillRect(pipe.x - 5, pipe.gapY + this.PIPE_GAP, this.PIPE_WIDTH + 10, 30);

            // Pipe highlights
            this.ctx.strokeStyle = 'rgba(255, 255, 255, 0.3)';
            this.ctx.lineWidth = 2;
            this.ctx.strokeRect(pipe.x + 5, 5, 3, pipe.gapY - 40);
            this.ctx.strokeRect(pipe.x + 5, pipe.gapY + this.PIPE_GAP + 35, 3,
                this.canvas.height - this.GROUND_HEIGHT - pipe.gapY - this.PIPE_GAP - 40);
        }
    }

    renderGround() {
        // Ground gradient
        const gradient = this.ctx.createLinearGradient(0, this.canvas.height - this.GROUND_HEIGHT,
            0, this.canvas.height);
        gradient.addColorStop(0, '#8B4513');
        gradient.addColorStop(0.5, '#654321');
        gradient.addColorStop(1, '#3d2817');

        this.ctx.fillStyle = gradient;
        this.ctx.fillRect(0, this.canvas.height - this.GROUND_HEIGHT, this.canvas.width, this.GROUND_HEIGHT);

        // Ground pattern
        this.ctx.fillStyle = 'rgba(139, 69, 19, 0.5)';
        for (let i = 0; i < this.canvas.width; i += 40) {
            const offset = this.gameState === 'playing' ? (this.frame * 3) % 40 : 0;
            this.ctx.fillRect(i - offset, this.canvas.height - this.GROUND_HEIGHT, 20, 10);
        }

        // Grass on top of ground
        this.ctx.fillStyle = '#228B22';
        this.ctx.fillRect(0, this.canvas.height - this.GROUND_HEIGHT, this.canvas.width, 10);
    }

    renderParticles() {
        for (const p of this.particles) {
            this.ctx.globalAlpha = p.life;
            this.ctx.fillStyle = p.color;
            this.ctx.beginPath();
            this.ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
            this.ctx.fill();
        }
        this.ctx.globalAlpha = 1;
    }

    // ==================== SOUND SYSTEM ====================

    generateSounds() {
        this.sounds = {
            flap: this.createSound(150, 0.1, 'sine'),
            score: this.createSound(800, 0.15, 'square'),
            hit: this.createSound(100, 0.3, 'sawtooth'),
            die: this.createSound(200, 0.5, 'triangle')
        };
    }

    createSound(frequency, duration, type = 'sine') {
        return { frequency, duration, type };
    }

    playSound(name) {
        if (!this.soundEnabled) return;

        const sound = this.sounds[name];
        if (!sound) return;

        try {
            const audioContext = new (window.AudioContext || window.webkitAudioContext)();
            const oscillator = audioContext.createOscillator();
            const gainNode = audioContext.createGain();

            oscillator.connect(gainNode);
            gainNode.connect(audioContext.destination);

            oscillator.type = sound.type;
            oscillator.frequency.value = sound.frequency;

            gainNode.gain.setValueAtTime(0.3, audioContext.currentTime);
            gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + sound.duration);

            oscillator.start(audioContext.currentTime);
            oscillator.stop(audioContext.currentTime + sound.duration);
        } catch (e) {
            console.warn('Audio not supported:', e);
        }
    }

    toggleSound() {
        this.soundEnabled = !this.soundEnabled;
        const btn = document.getElementById('soundToggle');
        btn.textContent = this.soundEnabled ? '🔊 Sound ON' : '🔇 Sound OFF';
    }

    // ==================== UTILITIES ====================

    updateScoreDisplay() {
        document.getElementById('scoreDisplay').textContent = this.score;
        document.getElementById('highScoreDisplay').textContent = `High: ${this.highScore}`;
    }

    loadHighScore() {
        const saved = localStorage.getItem('flappyBirdHighScore');
        return saved ? parseInt(saved) : 0;
    }

    saveHighScore() {
        localStorage.setItem('flappyBirdHighScore', this.highScore.toString());
    }

    interpolateColor(color1, color2, factor) {
        const c1 = this.hexToRgb(color1);
        const c2 = this.hexToRgb(color2);

        const r = Math.round(c1.r + (c2.r - c1.r) * factor);
        const g = Math.round(c1.g + (c2.g - c1.g) * factor);
        const b = Math.round(c1.b + (c2.b - c1.b) * factor);

        return `rgb(${r}, ${g}, ${b})`;
    }

    hexToRgb(hex) {
        const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
        return result ? {
            r: parseInt(result[1], 16),
            g: parseInt(result[2], 16),
            b: parseInt(result[3], 16)
        } : { r: 0, g: 0, b: 0 };
    }

    // ==================== GAME LOOP ====================

    gameLoop() {
        this.frame++;

        // Update
        this.updatePhysics();
        this.updatePipes();
        this.checkCollisions();
        this.updateParticles();

        // Render
        this.render();

        // Continue loop
        requestAnimationFrame(() => this.gameLoop());
    }
}

// Start the game when page loads
window.addEventListener('load', () => {
    new FlappyBirdGame();
});
