/**
 * 2048 Game - Application Controller
 * Handles UI updates, animations, and user input
 */

class HTMLActuator {
  constructor() {
    this.tileContainer = document.getElementById('tile-container');
    this.scoreContainer = document.getElementById('score');
    this.bestContainer = document.getElementById('best-score');
    this.messageContainer = document.getElementById('game-message');
  }

  actuate(grid, metadata) {
    window.requestAnimationFrame(() => {
      this.clearContainer(this.tileContainer);

      grid.cells.forEach((row) => {
        row.forEach((cell) => {
          if (cell) {
            this.addTile(cell);
          }
        });
      });

      this.updateScore(metadata.score);
      this.updateBestScore(metadata.bestScore);

      if (metadata.terminated) {
        if (metadata.over) {
          this.message(false); // You lose
        } else if (metadata.won) {
          this.message(true); // You win!
        }
      }
    });
  }

  clearContainer(container) {
    while (container.firstChild) {
      container.removeChild(container.firstChild);
    }
  }

  addTile(tile) {
    const wrapper = document.createElement('div');
    const inner = document.createElement('div');
    const position = tile.previousPosition || { row: tile.row, col: tile.col };
    const positionClass = this.positionClass(position);

    // Use tile-super class for tiles beyond 2048
    const valueClass = tile.value > 2048 ? 'tile-super' : `tile-${tile.value}`;
    const classes = ['tile', valueClass, positionClass];

    if (tile.previousPosition) {
      // Tile moved
      window.requestAnimationFrame(() => {
        classes[2] = this.positionClass({ row: tile.row, col: tile.col });
        this.applyClasses(wrapper, classes);
      });
    } else if (tile.mergedFrom) {
      // Tile merged
      classes.push('tile-merged');
      this.applyClasses(wrapper, classes);

      // Render the tiles that merged
      tile.mergedFrom.forEach((merged) => {
        this.addTile(merged);
      });
    } else {
      // New tile
      classes.push('tile-new');
      this.applyClasses(wrapper, classes);
    }

    inner.classList.add('tile-inner');
    inner.textContent = tile.value;

    wrapper.appendChild(inner);
    this.tileContainer.appendChild(wrapper);
  }

  applyClasses(element, classes) {
    element.className = classes.join(' ');
  }

  normalizePosition(position) {
    return { row: position.row, col: position.col };
  }

  positionClass(position) {
    position = this.normalizePosition(position);
    return `tile-position-${position.row}-${position.col}`;
  }

  updateScore(score) {
    this.clearContainer(this.scoreContainer);
    this.scoreContainer.textContent = score;
  }

  updateBestScore(bestScore) {
    this.clearContainer(this.bestContainer);
    this.bestContainer.textContent = bestScore;
  }

  message(won) {
    const type = won ? 'game-won' : 'game-over';
    const message = won ? 'You win!' : 'Game over!';

    this.messageContainer.classList.add(type);
    this.messageContainer.querySelector('p').textContent = message;
  }

  clearMessage() {
    this.messageContainer.classList.remove('game-won');
    this.messageContainer.classList.remove('game-over');
  }
}

class KeyboardInputManager {
  constructor() {
    this.events = {};
    this.listen();
  }

  on(event, callback) {
    if (!this.events[event]) {
      this.events[event] = [];
    }
    this.events[event].push(callback);
  }

  emit(event, data) {
    const callbacks = this.events[event];
    if (callbacks) {
      callbacks.forEach((callback) => {
        callback(data);
      });
    }
  }

  listen() {
    const map = {
      38: 0, // Up
      39: 1, // Right
      40: 2, // Down
      37: 3, // Left
      87: 0, // W
      68: 1, // D
      83: 2, // S
      65: 3  // A
    };

    document.addEventListener('keydown', (event) => {
      const modifiers = event.altKey || event.ctrlKey || event.metaKey || event.shiftKey;
      const mapped = map[event.which];

      if (!modifiers && mapped !== undefined) {
        event.preventDefault();
        this.emit('move', mapped);
      }

      // R key to restart
      if (!modifiers && event.which === 82) {
        this.restart.call(this, event);
      }
    });

    // Restart button
    const restartButton = document.getElementById('restart-button');
    restartButton.addEventListener('click', this.restart.bind(this));

    // Retry button
    const retryButton = document.getElementById('retry-button');
    retryButton.addEventListener('click', this.restart.bind(this));

    // Keep playing button
    const keepPlayingButton = document.getElementById('keep-playing-button');
    keepPlayingButton.addEventListener('click', this.keepPlaying.bind(this));
  }

  restart(event) {
    event.preventDefault();
    this.emit('restart');
  }

  keepPlaying(event) {
    event.preventDefault();
    this.emit('keepPlaying');
  }
}

class LocalStorageManager {
  constructor() {
    this.bestScoreKey = 'bestScore';
    this.gameStateKey = 'gameState';
  }

  getBestScore() {
    return parseInt(localStorage.getItem(this.bestScoreKey)) || 0;
  }

  setBestScore(score) {
    localStorage.setItem(this.bestScoreKey, score);
  }

  getGameState() {
    const stateJSON = localStorage.getItem(this.gameStateKey);
    return stateJSON ? JSON.parse(stateJSON) : null;
  }

  setGameState(gameState) {
    localStorage.setItem(this.gameStateKey, JSON.stringify(gameState));
  }

  clearGameState() {
    localStorage.removeItem(this.gameStateKey);
  }
}

class Application {
  constructor() {
    this.size = 4;
    this.inputManager = new KeyboardInputManager();
    this.storageManager = new LocalStorageManager();
    this.actuator = new HTMLActuator();

    this.inputManager.on('move', this.move.bind(this));
    this.inputManager.on('restart', this.restart.bind(this));
    this.inputManager.on('keepPlaying', this.keepPlaying.bind(this));

    this.setup();

    // Add CSS for tile positions dynamically
    this.addTilePositionStyles();
  }

  setup() {
    this.game = new GameManager(this.size);
    this.bestScore = this.storageManager.getBestScore();
    this.actuate();
  }

  restart() {
    this.actuator.clearMessage();
    this.storageManager.clearGameState();
    this.setup();
  }

  keepPlaying() {
    this.game.keepPlayingAfterWin();
    this.actuator.clearMessage();
  }

  move(direction) {
    if (this.game.move(direction)) {
      this.actuate();
    }
  }

  actuate() {
    if (this.game.score > this.bestScore) {
      this.bestScore = this.game.score;
      this.storageManager.setBestScore(this.bestScore);
    }

    this.actuator.actuate(this.game.grid, {
      score: this.game.score,
      bestScore: this.bestScore,
      over: this.game.over,
      won: this.game.won,
      terminated: this.game.isGameTerminated()
    });
  }

  addTilePositionStyles() {
    let styles = '';
    for (let row = 0; row < this.size; row++) {
      for (let col = 0; col < this.size; col++) {
        const x = col * 121.25; // 106.25 + 15 (cell width + margin)
        const y = row * 121.25;
        styles += `.tile-position-${row}-${col} { transform: translate(${x}px, ${y}px); }\n`;
      }
    }

    const styleSheet = document.createElement('style');
    styleSheet.textContent = styles;
    document.head.appendChild(styleSheet);
  }
}

// Initialize the game when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
  new Application();
});
