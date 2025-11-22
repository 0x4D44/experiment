/**
 * 2048 Game Application
 * Handles UI updates, user input, and game state management
 */

class GameApp {
    constructor() {
        this.game = new Game();
        this.tileContainer = document.getElementById('tile-container');
        this.scoreElement = document.getElementById('score');
        this.bestScoreElement = document.getElementById('best-score');
        this.scoreAddition = document.getElementById('score-addition');
        this.gameMessage = document.getElementById('game-message');
        this.newGameButton = document.getElementById('new-game');
        this.undoButton = document.getElementById('undo-button');
        this.retryButton = document.getElementById('retry-button');
        this.keepPlayingButton = document.getElementById('keep-playing-button');

        this.touchStartX = 0;
        this.touchStartY = 0;
        this.touchEndX = 0;
        this.touchEndY = 0;
        this.minSwipeDistance = 30;

        this.setupEventListeners();
        this.startNewGame();
    }

    /**
     * Setup all event listeners
     */
    setupEventListeners() {
        // Keyboard controls
        document.addEventListener('keydown', (e) => this.handleKeyPress(e));

        // Touch controls
        this.tileContainer.addEventListener('touchstart', (e) => this.handleTouchStart(e), { passive: true });
        this.tileContainer.addEventListener('touchmove', (e) => this.handleTouchMove(e), { passive: true });
        this.tileContainer.addEventListener('touchend', (e) => this.handleTouchEnd(e));

        // Button controls
        this.newGameButton.addEventListener('click', () => this.startNewGame());
        this.undoButton.addEventListener('click', () => this.undo());
        this.retryButton.addEventListener('click', () => this.startNewGame());
        this.keepPlayingButton.addEventListener('click', () => this.keepPlaying());
    }

    /**
     * Start a new game
     */
    startNewGame() {
        this.game.startNewGame();
        this.updateUI();
        this.hideMessage();
    }

    /**
     * Continue playing after winning
     */
    keepPlaying() {
        this.game.continueGame();
        this.hideMessage();
    }

    /**
     * Undo last move
     */
    undo() {
        if (this.game.restoreState()) {
            this.updateUI();
            this.hideMessage();
        }
    }

    /**
     * Handle keyboard input
     */
    handleKeyPress(event) {
        // Prevent default for arrow keys to avoid page scrolling
        if (['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(event.key)) {
            event.preventDefault();
        }

        const directionMap = {
            'ArrowUp': 'up',
            'ArrowDown': 'down',
            'ArrowLeft': 'left',
            'ArrowRight': 'right'
        };

        const direction = directionMap[event.key];

        if (direction) {
            this.move(direction);
        }
    }

    /**
     * Handle touch start
     */
    handleTouchStart(event) {
        this.touchStartX = event.changedTouches[0].screenX;
        this.touchStartY = event.changedTouches[0].screenY;
    }

    /**
     * Handle touch move
     */
    handleTouchMove(event) {
        this.touchEndX = event.changedTouches[0].screenX;
        this.touchEndY = event.changedTouches[0].screenY;
    }

    /**
     * Handle touch end
     */
    handleTouchEnd(event) {
        const deltaX = this.touchEndX - this.touchStartX;
        const deltaY = this.touchEndY - this.touchStartY;

        // Determine if it's a horizontal or vertical swipe
        if (Math.abs(deltaX) > Math.abs(deltaY)) {
            // Horizontal swipe
            if (Math.abs(deltaX) > this.minSwipeDistance) {
                if (deltaX > 0) {
                    this.move('right');
                } else {
                    this.move('left');
                }
            }
        } else {
            // Vertical swipe
            if (Math.abs(deltaY) > this.minSwipeDistance) {
                if (deltaY > 0) {
                    this.move('down');
                } else {
                    this.move('up');
                }
            }
        }
    }

    /**
     * Execute a move in the specified direction
     */
    move(direction) {
        const result = this.game.move(direction);

        if (result.moved) {
            this.updateUI();
            this.animateScore(result.scoreGained);

            // Check for win/lose conditions
            if (this.game.isWon()) {
                this.showWinMessage();
            } else if (this.game.isOver()) {
                this.showGameOverMessage();
            }
        }
    }

    /**
     * Update the entire UI
     */
    updateUI() {
        this.updateScore();
        this.updateTiles();
        this.updateUndoButton();
    }

    /**
     * Update score display
     */
    updateScore() {
        this.scoreElement.textContent = this.game.getScore();
        this.bestScoreElement.textContent = this.game.getBestScore();
    }

    /**
     * Animate score increase
     */
    animateScore(scoreGained) {
        if (scoreGained > 0) {
            this.scoreAddition.textContent = '+' + scoreGained;
            this.scoreAddition.classList.remove('active');

            // Trigger reflow to restart animation
            void this.scoreAddition.offsetWidth;

            this.scoreAddition.classList.add('active');
        }
    }

    /**
     * Update undo button state
     */
    updateUndoButton() {
        this.undoButton.disabled = !this.game.canUndo();
    }

    /**
     * Update tile display
     */
    updateTiles() {
        // Clear existing tiles
        this.tileContainer.innerHTML = '';

        // Get all tiles from the game
        const tiles = this.game.getAllTiles();

        // Create tile elements
        tiles.forEach(tile => {
            const tileElement = this.createTileElement(tile);
            this.tileContainer.appendChild(tileElement);
        });
    }

    /**
     * Create a tile DOM element
     */
    createTileElement(tile) {
        const element = document.createElement('div');
        element.classList.add('tile', `tile-${tile.value}`, `tile-position-${tile.row}-${tile.col}`);
        element.textContent = tile.value;

        // Add new tile animation
        if (tile.isNew) {
            element.classList.add('tile-new');
        }

        // Add merge animation
        if (tile.merged) {
            element.classList.add('tile-merged');
        }

        return element;
    }

    /**
     * Show win message
     */
    showWinMessage() {
        this.gameMessage.classList.add('game-won');
        this.gameMessage.querySelector('p').textContent = 'You win!';
        this.keepPlayingButton.style.display = 'inline-block';
        this.retryButton.style.display = 'inline-block';
    }

    /**
     * Show game over message
     */
    showGameOverMessage() {
        this.gameMessage.classList.add('game-over');
        this.gameMessage.querySelector('p').textContent = 'Game over!';
        this.keepPlayingButton.style.display = 'none';
        this.retryButton.style.display = 'inline-block';
    }

    /**
     * Hide game message
     */
    hideMessage() {
        this.gameMessage.classList.remove('game-won', 'game-over');
    }
}

// Initialize the game when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    const app = new GameApp();
});
