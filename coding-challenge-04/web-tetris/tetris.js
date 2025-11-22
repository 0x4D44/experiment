/**
 * TETRIS - Classic Block Puzzle Game
 * Full-featured Tetris implementation with all classic mechanics
 */

// Game constants
const COLS = 10;
const ROWS = 20;
const BLOCK_SIZE = 30;
const COLORS = {
    I: '#00f0f0',  // Cyan
    O: '#f0f000',  // Yellow
    T: '#a000f0',  // Purple
    S: '#00f000',  // Green
    Z: '#f00000',  // Red
    J: '#0000f0',  // Blue
    L: '#f0a000',  // Orange
    GHOST: 'rgba(255, 255, 255, 0.2)',
    GRID: '#1a1a1a',
    GRID_LINE: '#333333'
};

// Tetromino shapes (using SRS - Super Rotation System)
const SHAPES = {
    I: [
        [[0,0,0,0],
         [1,1,1,1],
         [0,0,0,0],
         [0,0,0,0]],
        [[0,0,1,0],
         [0,0,1,0],
         [0,0,1,0],
         [0,0,1,0]],
        [[0,0,0,0],
         [0,0,0,0],
         [1,1,1,1],
         [0,0,0,0]],
        [[0,1,0,0],
         [0,1,0,0],
         [0,1,0,0],
         [0,1,0,0]]
    ],
    O: [
        [[0,1,1,0],
         [0,1,1,0],
         [0,0,0,0],
         [0,0,0,0]]
    ],
    T: [
        [[0,1,0],
         [1,1,1],
         [0,0,0]],
        [[0,1,0],
         [0,1,1],
         [0,1,0]],
        [[0,0,0],
         [1,1,1],
         [0,1,0]],
        [[0,1,0],
         [1,1,0],
         [0,1,0]]
    ],
    S: [
        [[0,1,1],
         [1,1,0],
         [0,0,0]],
        [[0,1,0],
         [0,1,1],
         [0,0,1]]
    ],
    Z: [
        [[1,1,0],
         [0,1,1],
         [0,0,0]],
        [[0,0,1],
         [0,1,1],
         [0,1,0]]
    ],
    J: [
        [[1,0,0],
         [1,1,1],
         [0,0,0]],
        [[0,1,1],
         [0,1,0],
         [0,1,0]],
        [[0,0,0],
         [1,1,1],
         [0,0,1]],
        [[0,1,0],
         [0,1,0],
         [1,1,0]]
    ],
    L: [
        [[0,0,1],
         [1,1,1],
         [0,0,0]],
        [[0,1,0],
         [0,1,0],
         [0,1,1]],
        [[0,0,0],
         [1,1,1],
         [1,0,0]],
        [[1,1,0],
         [0,1,0],
         [0,1,0]]
    ]
};

// Scoring system (based on original Tetris)
const SCORES = {
    SINGLE: 100,
    DOUBLE: 300,
    TRIPLE: 500,
    TETRIS: 800,
    SOFT_DROP: 1,
    HARD_DROP: 2
};

// Game class
class TetrisGame {
    constructor() {
        this.canvas = document.getElementById('gameCanvas');
        this.ctx = this.canvas.getContext('2d');
        this.nextCanvas = document.getElementById('nextCanvas');
        this.nextCtx = this.nextCanvas.getContext('2d');

        this.grid = this.createGrid();
        this.score = 0;
        this.lines = 0;
        this.level = 1;
        this.gameOver = false;
        this.paused = false;
        this.gameStarted = false;

        this.currentPiece = null;
        this.nextPiece = null;
        this.dropCounter = 0;
        this.dropInterval = 1000; // ms
        this.lastTime = 0;

        this.setupControls();
        this.setupUI();
    }

    createGrid() {
        return Array.from({ length: ROWS }, () => Array(COLS).fill(0));
    }

    setupControls() {
        document.addEventListener('keydown', (e) => {
            if (!this.gameStarted || this.gameOver) return;

            if (e.key === 'p' || e.key === 'P') {
                this.togglePause();
                return;
            }

            if (this.paused) return;

            switch(e.key) {
                case 'ArrowLeft':
                    e.preventDefault();
                    this.movePiece(-1, 0);
                    break;
                case 'ArrowRight':
                    e.preventDefault();
                    this.movePiece(1, 0);
                    break;
                case 'ArrowDown':
                    e.preventDefault();
                    this.softDrop();
                    break;
                case 'ArrowUp':
                    e.preventDefault();
                    this.rotatePiece();
                    break;
                case ' ':
                    e.preventDefault();
                    this.hardDrop();
                    break;
            }
        });
    }

    setupUI() {
        const startButton = document.getElementById('startButton');
        const restartButton = document.getElementById('restartButton');

        startButton.addEventListener('click', () => {
            this.startGame();
        });

        restartButton.addEventListener('click', () => {
            this.resetGame();
            this.startGame();
        });
    }

    startGame() {
        document.getElementById('startScreen').classList.add('hidden');
        this.gameStarted = true;
        this.nextPiece = this.createPiece();
        this.spawnPiece();
        this.updateDisplay();
        this.gameLoop(0);
    }

    resetGame() {
        this.grid = this.createGrid();
        this.score = 0;
        this.lines = 0;
        this.level = 1;
        this.gameOver = false;
        this.paused = false;
        this.dropCounter = 0;
        this.dropInterval = 1000;
        this.currentPiece = null;
        this.nextPiece = null;
        document.getElementById('gameOverOverlay').classList.remove('show');
    }

    togglePause() {
        this.paused = !this.paused;
        const pausedOverlay = document.getElementById('pausedOverlay');
        if (this.paused) {
            pausedOverlay.classList.add('show');
        } else {
            pausedOverlay.classList.remove('show');
            this.lastTime = performance.now();
        }
    }

    createPiece() {
        const types = Object.keys(SHAPES);
        const type = types[Math.floor(Math.random() * types.length)];
        return {
            type: type,
            rotation: 0,
            x: Math.floor(COLS / 2) - 1,
            y: 0,
            color: COLORS[type]
        };
    }

    spawnPiece() {
        this.currentPiece = this.nextPiece;
        this.nextPiece = this.createPiece();

        // Check if spawn position is valid
        if (!this.isValidMove(this.currentPiece.x, this.currentPiece.y, this.currentPiece.rotation)) {
            this.endGame();
        }

        this.drawNextPiece();
    }

    getCurrentShape() {
        const shapes = SHAPES[this.currentPiece.type];
        return shapes[this.currentPiece.rotation % shapes.length];
    }

    getShape(piece) {
        const shapes = SHAPES[piece.type];
        return shapes[piece.rotation % shapes.length];
    }

    isValidMove(newX, newY, newRotation) {
        const shape = SHAPES[this.currentPiece.type][newRotation % SHAPES[this.currentPiece.type].length];

        for (let y = 0; y < shape.length; y++) {
            for (let x = 0; x < shape[y].length; x++) {
                if (shape[y][x]) {
                    const gridX = newX + x;
                    const gridY = newY + y;

                    // Check boundaries
                    if (gridX < 0 || gridX >= COLS || gridY >= ROWS) {
                        return false;
                    }

                    // Check collision with placed pieces (but allow negative Y for spawning)
                    if (gridY >= 0 && this.grid[gridY][gridX]) {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    movePiece(dx, dy) {
        const newX = this.currentPiece.x + dx;
        const newY = this.currentPiece.y + dy;

        if (this.isValidMove(newX, newY, this.currentPiece.rotation)) {
            this.currentPiece.x = newX;
            this.currentPiece.y = newY;
            return true;
        }
        return false;
    }

    rotatePiece() {
        const newRotation = (this.currentPiece.rotation + 1) % SHAPES[this.currentPiece.type].length;

        // Try basic rotation
        if (this.isValidMove(this.currentPiece.x, this.currentPiece.y, newRotation)) {
            this.currentPiece.rotation = newRotation;
            return;
        }

        // Wall kick: try moving left or right
        const kicks = [-1, 1, -2, 2];
        for (let kick of kicks) {
            if (this.isValidMove(this.currentPiece.x + kick, this.currentPiece.y, newRotation)) {
                this.currentPiece.x += kick;
                this.currentPiece.rotation = newRotation;
                return;
            }
        }
    }

    softDrop() {
        if (this.movePiece(0, 1)) {
            this.score += SCORES.SOFT_DROP;
            this.updateDisplay();
        }
    }

    hardDrop() {
        let dropDistance = 0;
        while (this.movePiece(0, 1)) {
            dropDistance++;
        }
        this.score += dropDistance * SCORES.HARD_DROP;
        this.updateDisplay();
        this.lockPiece();
    }

    getGhostPieceY() {
        let ghostY = this.currentPiece.y;
        while (this.isValidMove(this.currentPiece.x, ghostY + 1, this.currentPiece.rotation)) {
            ghostY++;
        }
        return ghostY;
    }

    lockPiece() {
        const shape = this.getCurrentShape();

        for (let y = 0; y < shape.length; y++) {
            for (let x = 0; x < shape[y].length; x++) {
                if (shape[y][x]) {
                    const gridY = this.currentPiece.y + y;
                    const gridX = this.currentPiece.x + x;

                    if (gridY >= 0) {
                        this.grid[gridY][gridX] = this.currentPiece.color;
                    }
                }
            }
        }

        this.clearLines();
        this.spawnPiece();
    }

    clearLines() {
        let linesCleared = 0;

        for (let y = ROWS - 1; y >= 0; y--) {
            if (this.grid[y].every(cell => cell !== 0)) {
                // Remove the line
                this.grid.splice(y, 1);
                // Add new empty line at top
                this.grid.unshift(Array(COLS).fill(0));
                linesCleared++;
                y++; // Check the same row again
            }
        }

        if (linesCleared > 0) {
            this.lines += linesCleared;

            // Score based on lines cleared
            switch(linesCleared) {
                case 1:
                    this.score += SCORES.SINGLE * this.level;
                    break;
                case 2:
                    this.score += SCORES.DOUBLE * this.level;
                    break;
                case 3:
                    this.score += SCORES.TRIPLE * this.level;
                    break;
                case 4:
                    this.score += SCORES.TETRIS * this.level;
                    break;
            }

            // Level up every 10 lines
            this.level = Math.floor(this.lines / 10) + 1;
            this.dropInterval = Math.max(100, 1000 - (this.level - 1) * 100);

            this.updateDisplay();
        }
    }

    updateDisplay() {
        document.getElementById('score').textContent = this.score;
        document.getElementById('lines').textContent = this.lines;
        document.getElementById('level').textContent = this.level;
    }

    endGame() {
        this.gameOver = true;
        document.getElementById('finalScore').textContent = this.score;
        document.getElementById('gameOverOverlay').classList.add('show');
    }

    draw() {
        // Clear canvas
        this.ctx.fillStyle = COLORS.GRID;
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

        // Draw grid lines
        this.ctx.strokeStyle = COLORS.GRID_LINE;
        this.ctx.lineWidth = 1;
        for (let x = 0; x <= COLS; x++) {
            this.ctx.beginPath();
            this.ctx.moveTo(x * BLOCK_SIZE, 0);
            this.ctx.lineTo(x * BLOCK_SIZE, ROWS * BLOCK_SIZE);
            this.ctx.stroke();
        }
        for (let y = 0; y <= ROWS; y++) {
            this.ctx.beginPath();
            this.ctx.moveTo(0, y * BLOCK_SIZE);
            this.ctx.lineTo(COLS * BLOCK_SIZE, y * BLOCK_SIZE);
            this.ctx.stroke();
        }

        // Draw placed pieces
        for (let y = 0; y < ROWS; y++) {
            for (let x = 0; x < COLS; x++) {
                if (this.grid[y][x]) {
                    this.drawBlock(x, y, this.grid[y][x]);
                }
            }
        }

        // Draw ghost piece
        if (this.currentPiece && !this.paused) {
            const ghostY = this.getGhostPieceY();
            const shape = this.getCurrentShape();

            for (let y = 0; y < shape.length; y++) {
                for (let x = 0; x < shape[y].length; x++) {
                    if (shape[y][x]) {
                        const gridX = this.currentPiece.x + x;
                        const gridY = ghostY + y;
                        if (gridY >= 0) {
                            this.drawBlock(gridX, gridY, COLORS.GHOST);
                        }
                    }
                }
            }
        }

        // Draw current piece
        if (this.currentPiece && !this.paused) {
            const shape = this.getCurrentShape();

            for (let y = 0; y < shape.length; y++) {
                for (let x = 0; x < shape[y].length; x++) {
                    if (shape[y][x]) {
                        const gridX = this.currentPiece.x + x;
                        const gridY = this.currentPiece.y + y;
                        if (gridY >= 0) {
                            this.drawBlock(gridX, gridY, this.currentPiece.color);
                        }
                    }
                }
            }
        }
    }

    drawBlock(x, y, color) {
        const px = x * BLOCK_SIZE;
        const py = y * BLOCK_SIZE;

        // Main block
        this.ctx.fillStyle = color;
        this.ctx.fillRect(px + 1, py + 1, BLOCK_SIZE - 2, BLOCK_SIZE - 2);

        // Add 3D effect with highlights and shadows (if not ghost)
        if (color !== COLORS.GHOST) {
            // Highlight
            this.ctx.fillStyle = 'rgba(255, 255, 255, 0.3)';
            this.ctx.fillRect(px + 1, py + 1, BLOCK_SIZE - 2, 4);
            this.ctx.fillRect(px + 1, py + 1, 4, BLOCK_SIZE - 2);

            // Shadow
            this.ctx.fillStyle = 'rgba(0, 0, 0, 0.3)';
            this.ctx.fillRect(px + 1, py + BLOCK_SIZE - 5, BLOCK_SIZE - 2, 4);
            this.ctx.fillRect(px + BLOCK_SIZE - 5, py + 1, 4, BLOCK_SIZE - 2);
        }
    }

    drawNextPiece() {
        // Clear canvas
        this.nextCtx.fillStyle = '#f5f5f5';
        this.nextCtx.fillRect(0, 0, this.nextCanvas.width, this.nextCanvas.height);

        if (!this.nextPiece) return;

        const shape = this.getShape(this.nextPiece);
        const blockSize = 25;

        // Calculate centering offset
        const offsetX = (this.nextCanvas.width - shape[0].length * blockSize) / 2;
        const offsetY = (this.nextCanvas.height - shape.length * blockSize) / 2;

        for (let y = 0; y < shape.length; y++) {
            for (let x = 0; x < shape[y].length; x++) {
                if (shape[y][x]) {
                    const px = offsetX + x * blockSize;
                    const py = offsetY + y * blockSize;

                    // Main block
                    this.nextCtx.fillStyle = this.nextPiece.color;
                    this.nextCtx.fillRect(px + 1, py + 1, blockSize - 2, blockSize - 2);

                    // Highlight
                    this.nextCtx.fillStyle = 'rgba(255, 255, 255, 0.3)';
                    this.nextCtx.fillRect(px + 1, py + 1, blockSize - 2, 3);
                    this.nextCtx.fillRect(px + 1, py + 1, 3, blockSize - 2);

                    // Shadow
                    this.nextCtx.fillStyle = 'rgba(0, 0, 0, 0.3)';
                    this.nextCtx.fillRect(px + 1, py + blockSize - 4, blockSize - 2, 3);
                    this.nextCtx.fillRect(px + blockSize - 4, py + 1, 3, blockSize - 2);
                }
            }
        }
    }

    gameLoop(currentTime) {
        if (this.gameOver) return;

        const deltaTime = currentTime - this.lastTime;
        this.lastTime = currentTime;

        if (!this.paused) {
            this.dropCounter += deltaTime;

            if (this.dropCounter > this.dropInterval) {
                if (!this.movePiece(0, 1)) {
                    this.lockPiece();
                }
                this.dropCounter = 0;
            }
        }

        this.draw();
        requestAnimationFrame((time) => this.gameLoop(time));
    }
}

// Initialize game when page loads
let game;
window.addEventListener('load', () => {
    game = new TetrisGame();
});

// Export for testing
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        TetrisGame,
        SHAPES,
        COLORS,
        SCORES,
        COLS,
        ROWS
    };
}
