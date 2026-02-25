// Rhythm Runner - Complete Game Implementation
class AudioSystem {
  constructor(bpm) {
    this.bpm = bpm;
    this.beatDuration = (60 / bpm) * 1000;
    this.startTime = 0;
    this.isPlaying = false;
  }

  start() {
    if (!this.isPlaying) {
      this.startTime = Date.now();
      this.isPlaying = true;
    }
  }

  stop() { this.isPlaying = false; }
  getElapsedTime() { return !this.isPlaying ? 0 : Date.now() - this.startTime; }
  getBeatPosition() {
    const e = this.getElapsedTime();
    return (e % this.beatDuration) / this.beatDuration;
  }
  getCurrentBeat() { return Math.floor(this.getElapsedTime() / this.beatDuration); }
  checkTiming() {
    const b = this.getBeatPosition();
    const t = Math.abs((b - 0.5) * this.beatDuration);
    return t <= 50 ? 'perfect' : t <= 100 ? 'good' : 'miss';
  }
  setBPM(n) { this.bpm = n; this.beatDuration = (60 / n) * 1000; }
  getBPM() { return this.bpm; }
  reset() { this.startTime = 0; this.isPlaying = false; }
  update() { return false; }
}

class Player {
  constructor(x, y) {
    this.x = x; this.y = y; this.width = 30; this.height = 40;
    this.velocityY = 0; this.grounded = true; this.currentAction = 'idle';
    this.jumpForce = 15; this.slideTimer = 0; this.slideActive = false; this.gravity = 0.6;
  }

  jump() { if (this.grounded && !this.slideActive) { this.velocityY = -this.jumpForce; this.grounded = false; this.currentAction = 'jump'; } }
  slide() { if (this.grounded && !this.slideActive) { this.slideActive = true; this.slideTimer = 0; this.currentAction = 'slide'; this.height = 20; } }

  update(canvasHeight) {
    if (!this.grounded) this.velocityY += this.gravity;
    this.y += this.velocityY;
    const groundY = canvasHeight - 50;
    if (this.y >= groundY) { this.y = groundY; this.velocityY = 0; this.grounded = true; }
    if (this.slideActive) { this.slideTimer++; if (this.slideTimer > 15) { this.slideActive = false; this.height = 40; this.currentAction = 'idle'; } }
    if (this.x < 0) this.x = 0;
    if (this.x + this.width > 1200) this.x = 1200 - this.width;
  }

  moveLeft() { this.x -= 5; }
  moveRight() { this.x += 5; }
  getBounds() { return { x: this.x, y: this.y, width: this.width, height: this.height }; }
  reset(x, y) { this.x = x; this.y = y; this.velocityY = 0; this.grounded = true; this.slideActive = false; this.slideTimer = 0; this.currentAction = 'idle'; this.height = 40; }
}

class ObstacleGenerator {
  constructor() {
    this.obstacles = []; this.lastObstacleX = 800;
    this.minGap = 100; this.maxGap = 150;
    this.obstacleSpeed = 3; this.nextObstacleDistance = 150;
  }

  update(playerX, canvasWidth, canvasHeight) {
    if (this.lastObstacleX < playerX + canvasWidth + 200) this.generateObstacle(playerX, canvasHeight);
    for (const o of this.obstacles) o.x -= o.speed;
    this.obstacles = this.obstacles.filter(o => o.x + o.width > playerX - 100);
  }

  generateObstacle(playerX, canvasHeight) {
    const groundY = canvasHeight - 50;
    const newX = this.lastObstacleX + this.nextObstacleDistance;
    const rand = Math.random();
    const obstacle = rand < 0.5
      ? { x: newX, y: groundY - 40, width: 40, height: 40, type: 'block', speed: this.obstacleSpeed, passed: false }
      : { x: newX, y: groundY - 20, width: 30, height: 20, type: 'spike', speed: this.obstacleSpeed, passed: false };
    this.obstacles.push(obstacle);
    this.lastObstacleX = newX;
    this.nextObstacleDistance = this.minGap + Math.random() * (this.maxGap - this.minGap);
  }

  getObstacles() { return this.obstacles; }
  markObstaclePassed(o) { o.passed = true; }

  setDifficulty(d) {
    if (d === 'easy') { this.obstacleSpeed = 2; this.minGap = 120; this.maxGap = 180; }
    else if (d === 'normal') { this.obstacleSpeed = 3; this.minGap = 100; this.maxGap = 150; }
    else { this.obstacleSpeed = 4; this.minGap = 80; this.maxGap = 120; }
  }

  reset() { this.obstacles = []; this.lastObstacleX = 800; this.nextObstacleDistance = 150; }
}

class ScoringSystem {
  constructor() {
    this.score = 0; this.combo = 0; this.multiplier = 1; this.maxCombo = 0;
    this.perfectHits = 0; this.goodHits = 0; this.missHits = 0;
  }

  registerHit(timing, difficulty = 1) {
    let points = 0;
    switch (timing) {
      case 'perfect': points = 100; this.combo++; this.perfectHits++; break;
      case 'good': points = 50; this.combo++; this.goodHits++; break;
      case 'miss': this.combo = 0; this.missHits++; break;
    }
    this.multiplier = 1 + Math.floor(this.combo / 5) * 0.5;
    const totalPoints = Math.floor(points * this.multiplier * difficulty);
    this.score += totalPoints;
    if (this.combo > this.maxCombo) this.maxCombo = this.combo;
    return totalPoints;
  }

  getScore() { return this.score; }
  getCombo() { return this.combo; }
  getMaxCombo() { return this.maxCombo; }

  getStats() {
    const total = this.perfectHits + this.goodHits + this.missHits;
    const accuracy = total > 0 ? ((this.perfectHits + this.goodHits) / total) * 100 : 0;
    return { total, perfect: this.perfectHits, good: this.goodHits, miss: this.missHits, accuracy: Math.round(accuracy * 100) / 100 };
  }

  reset() { this.score = 0; this.combo = 0; this.multiplier = 1; this.maxCombo = 0; this.perfectHits = 0; this.goodHits = 0; this.missHits = 0; }
}

class Renderer {
  constructor() {
    const canvas = document.getElementById('gameCanvas');
    if (!canvas) throw new Error('Canvas not found');
    this.canvas = canvas;
    const ctx = this.canvas.getContext('2d');
    if (!ctx) throw new Error('Context failed');
    this.ctx = ctx;
    this.canvas.width = 1200;
    this.canvas.height = 600;
    this.feedbackMessages = [];
  }

  render(player, obstacles, score, combo, bpm, beatPosition) {
    this.clearCanvas();
    this.drawBeatIndicator(beatPosition);
    this.drawGround();
    obstacles.forEach(o => this.drawObstacle(o));
    this.drawPlayer(player);
    this.drawFeedback();
    this.drawUI(score, combo, bpm);
  }

  clearCanvas() { this.ctx.fillStyle = '#000'; this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height); }

  drawBeatIndicator(beatPosition) {
    const intensity = Math.sin(beatPosition * Math.PI) * 0.3 + 0.1;
    this.ctx.fillStyle = 'rgba(0,255,0,' + intensity + ')';
    this.ctx.fillRect(0, 0, this.canvas.width, 5);
  }

  drawGround() {
    const groundY = this.canvas.height - 50;
    this.ctx.fillStyle = '#1a1a2e';
    this.ctx.fillRect(0, groundY, this.canvas.width, 50);
    this.ctx.strokeStyle = '#00ff00';
    this.ctx.lineWidth = 2;
    this.ctx.strokeRect(0, groundY, this.canvas.width, 50);
  }

  drawPlayer(player) {
    const b = player.getBounds();
    this.ctx.fillStyle = '#00ff00';
    this.ctx.fillRect(b.x, b.y, b.width, b.height);
    this.ctx.strokeStyle = '#00aa00';
    this.ctx.lineWidth = 2;
    this.ctx.strokeRect(b.x, b.y, b.width, b.height);
  }

  drawObstacle(o) {
    this.ctx.fillStyle = '#ff0000';
    this.ctx.fillRect(o.x, o.y, o.width, o.height);
    this.ctx.strokeStyle = '#aa0000';
    this.ctx.lineWidth = 2;
    this.ctx.strokeRect(o.x, o.y, o.width, o.height);
  }

  drawUI(score, combo, bpm) {
    const s = document.getElementById('score');
    const c = document.getElementById('combo');
    const b = document.getElementById('bpm');
    if (s) s.textContent = score.toString();
    if (c) c.textContent = combo.toString();
    if (b) b.textContent = bpm.toString();
  }

  addFeedback(text, type) { this.feedbackMessages.push({ text, time: 60, type }); }

  drawFeedback() {
    for (let i = this.feedbackMessages.length - 1; i >= 0; i--) {
      const msg = this.feedbackMessages[i];
      msg.time--;
      const opacity = msg.time / 60;
      this.ctx.fillStyle = msg.type === 'perfect' ? 'rgba(255,255,0,' + opacity + ')' : msg.type === 'good' ? 'rgba(0,255,0,' + opacity + ')' : 'rgba(255,0,0,' + opacity + ')';
      this.ctx.font = 'bold 32px Arial';
      this.ctx.textAlign = 'center';
      this.ctx.fillText(msg.text, this.canvas.width / 2, this.canvas.height / 2);
      if (msg.time <= 0) this.feedbackMessages.splice(i, 1);
    }
  }

  getCanvasSize() { return { width: this.canvas.width, height: this.canvas.height }; }
}

class Game {
  constructor() {
    this.audioSystem = new AudioSystem(120);
    this.player = new Player(600, 500);
    this.obstacles = new ObstacleGenerator();
    this.scoring = new ScoringSystem();
    this.renderer = new Renderer();
    this.gameState = 'menu';
    this.gameOver = false;
    this.lastBeatProcessed = -1;
    this.difficulty = 'normal';
    this.keysPressed = new Set();
    this.lives = 3;
    this.lastActionTime = 0;
    this.setupInput();
  }

  startGame(difficulty) {
    this.difficulty = difficulty;
    this.gameState = 'playing';
    this.gameOver = false;
    this.lives = 3;
    const bpmMap = { easy: 100, normal: 120, hard: 140 };
    this.audioSystem.setBPM(bpmMap[difficulty]);
    this.obstacles.setDifficulty(difficulty);
    this.audioSystem.start();
    const menu = document.getElementById('menu');
    if (menu) menu.classList.add('hidden');
  }

  update() {
    if (this.gameState !== 'playing') return;
    const canvasSize = this.renderer.getCanvasSize();
    this.audioSystem.update();
    const currentBeat = this.audioSystem.getCurrentBeat();
    if (currentBeat > this.lastBeatProcessed) this.lastBeatProcessed = currentBeat;
    this.player.update(canvasSize.height);
    this.obstacles.update(this.player.x, canvasSize.width, canvasSize.height);
    this.checkCollisions();
  }

  render() {
    const obstacles = this.obstacles.getObstacles();
    const beatPosition = this.audioSystem.getBeatPosition();
    this.renderer.render(this.player, obstacles, this.scoring.getScore(), this.scoring.getCombo(), this.audioSystem.getBPM(), beatPosition);
  }

  setupInput() {
    document.addEventListener('keydown', (e) => {
      this.keysPressed.add(e.key.toLowerCase());
      if (e.key === ' ') { e.preventDefault(); this.handleJump(); }
      else if (e.key === 's') { e.preventDefault(); this.handleSlide(); }
      else if (e.key === 'ArrowLeft') { e.preventDefault(); this.player.moveLeft(); }
      else if (e.key === 'ArrowRight') { e.preventDefault(); this.player.moveRight(); }
    });
    document.addEventListener('keyup', (e) => { this.keysPressed.delete(e.key.toLowerCase()); });
  }

  handleJump() {
    const now = Date.now();
    if (now - this.lastActionTime < 100) return;
    this.lastActionTime = now;
    const timing = this.audioSystem.checkTiming();
    this.scoring.registerHit(timing, 1);
    this.player.jump();
    this.renderer.addFeedback(timing.toUpperCase(), timing);
  }

  handleSlide() {
    const now = Date.now();
    if (now - this.lastActionTime < 100) return;
    this.lastActionTime = now;
    const timing = this.audioSystem.checkTiming();
    this.scoring.registerHit(timing, 1);
    this.player.slide();
    this.renderer.addFeedback(timing.toUpperCase(), timing);
  }

  checkCollisions() {
    const obstacles = this.obstacles.getObstacles();
    const playerBounds = this.player.getBounds();
    for (const obstacle of obstacles) {
      if (
        playerBounds.x < obstacle.x + obstacle.width &&
        playerBounds.x + playerBounds.width > obstacle.x &&
        playerBounds.y < obstacle.y + obstacle.height &&
        playerBounds.y + playerBounds.height > obstacle.y
      ) {
        if (!obstacle.passed) {
          this.scoring.registerHit('miss', 1);
          this.renderer.addFeedback('HIT!', 'miss');
          this.lives--;
          if (this.lives <= 0) this.endGame();
        }
        obstacle.passed = true;
      } else if (obstacle.x + obstacle.width < playerBounds.x && !obstacle.passed) {
        this.obstacles.markObstaclePassed(obstacle);
        this.scoring.registerHit('good', 1);
        this.renderer.addFeedback('DODGE!', 'good');
      }
    }
  }

  endGame() {
    this.gameState = 'gameOver';
    this.gameOver = true;
    this.audioSystem.stop();
    const stats = this.scoring.getStats();
    const score = this.scoring.getScore();
    const maxCombo = this.scoring.getMaxCombo();
    const menu = document.getElementById('menu');
    if (menu) {
      menu.classList.remove('hidden');
      menu.innerHTML = '<h1>GAME OVER</h1><p style="font-size: 24px;">Score: ' + score + '</p><p>Max Combo: ' + maxCombo + '</p><p>Accuracy: ' + stats.accuracy + '%</p><button class="menu-button" onclick="window.game.reset()">Play Again</button>';
    }
  }

  reset() {
    this.gameState = 'menu';
    this.gameOver = false;
    this.lives = 3;
    this.lastBeatProcessed = -1;
    this.player.reset(600, 500);
    this.obstacles.reset();
    this.scoring.reset();
    this.audioSystem.reset();
    this.keysPressed.clear();
    const menu = document.getElementById('menu');
    if (menu) {
      menu.classList.remove('hidden');
      menu.innerHTML = '<h1>RHYTHM RUNNER</h1><p style="margin-bottom: 30px; color: #aaa;">Sync to the beat, chain perfect hits</p><button class="menu-button" onclick="window.game.startGame(\'easy\')">Easy (100 BPM)</button><button class="menu-button" onclick="window.game.startGame(\'normal\')">Normal (120 BPM)</button><button class="menu-button" onclick="window.game.startGame(\'hard\')">Hard (140 BPM)</button>';
    }
  }

  isGameOver() { return this.gameOver; }
}

let game;

function init() {
  game = new Game();
  window.game = game;
  gameLoop();
}

function gameLoop() {
  game.update();
  game.render();
  requestAnimationFrame(gameLoop);
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}
