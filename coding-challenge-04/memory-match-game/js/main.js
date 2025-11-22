/**
 * Main Application Controller
 * Manages UI, user interactions, and game flow
 */

// Initialize game
const game = new MemoryGame();

// DOM Elements
const gameBoard = document.getElementById('gameBoard');
const difficultySelect = document.getElementById('difficulty');
const themeSelect = document.getElementById('theme');
const newGameBtn = document.getElementById('newGame');
const soundToggleBtn = document.getElementById('soundToggle');
const soundIcon = document.getElementById('soundIcon');

// Stats elements
const movesElement = document.getElementById('moves');
const timerElement = document.getElementById('timer');
const matchesElement = document.getElementById('matches');
const totalPairsElement = document.getElementById('totalPairs');
const bestScoreElement = document.getElementById('bestScore');

// Modal elements
const winModal = document.getElementById('winModal');
const playAgainBtn = document.getElementById('playAgain');
const finalMovesElement = document.getElementById('finalMoves');
const finalTimeElement = document.getElementById('finalTime');
const finalScoreElement = document.getElementById('finalScore');
const newRecordElement = document.getElementById('newRecord');

/**
 * Render game board
 */
function renderBoard() {
    gameBoard.innerHTML = '';
    gameBoard.className = `game-board ${game.difficulty}`;

    game.cards.forEach(card => {
        const cardElement = createCardElement(card);
        gameBoard.appendChild(cardElement);
    });

    updateStats();
}

/**
 * Create card HTML element
 */
function createCardElement(card) {
    const cardDiv = document.createElement('div');
    cardDiv.className = 'card';
    cardDiv.dataset.id = card.id;

    if (card.flipped) {
        cardDiv.classList.add('flipped');
    }

    if (card.matched) {
        cardDiv.classList.add('matched');
    }

    cardDiv.innerHTML = `
        <div class="card-inner">
            <div class="card-back"></div>
            <div class="card-front">${card.value}</div>
        </div>
    `;

    cardDiv.addEventListener('click', () => handleCardClick(card.id));

    return cardDiv;
}

/**
 * Handle card click
 */
async function handleCardClick(cardId) {
    // Check if this will be the second card before flipping
    const willProcessPair = game.flippedCards.length === 1 && !game.isProcessing;

    // Get the first card ID if this will be a pair
    const firstCardId = willProcessPair ? game.flippedCards[0].id : null;

    const result = await game.flipCard(cardId);

    if (result === false) return;

    // Play flip sound
    soundManager.playFlip();

    // Update UI
    updateCardUI(cardId);

    // If we just processed a pair, check for match/mismatch
    if (willProcessPair) {
        const card1 = game.cards.find(c => c.id === firstCardId);
        const card2 = game.cards.find(c => c.id === cardId);

        if (card1 && card2) {
            // Check if cards matched or not
            setTimeout(() => {
                if (card1.matched && card2.matched) {
                    // Match found
                    soundManager.playMatch();
                    animateMatch(card1.id, card2.id);
                } else {
                    // No match, cards flipped back
                    soundManager.playMismatch();
                    animateMismatch(card1.id, card2.id);
                }
            }, 100);
        }
    }

    // Update stats
    updateStats();

    // Check for win
    if (result === 'win') {
        setTimeout(() => showWinModal(), 800);
    }
}

/**
 * Update card UI
 */
function updateCardUI(cardId) {
    const cardElement = gameBoard.querySelector(`[data-id="${cardId}"]`);
    if (!cardElement) return;

    const card = game.cards.find(c => c.id === cardId);
    if (!card) return;

    cardElement.className = 'card';

    if (card.flipped) {
        cardElement.classList.add('flipped');
    }

    if (card.matched) {
        cardElement.classList.add('matched');
    }
}

/**
 * Animate matched cards
 */
function animateMatch(card1Id, card2Id) {
    const card1Element = gameBoard.querySelector(`[data-id="${card1Id}"]`);
    const card2Element = gameBoard.querySelector(`[data-id="${card2Id}"]`);

    if (card1Element) {
        card1Element.classList.add('matched');
    }

    if (card2Element) {
        card2Element.classList.add('matched');
    }
}

/**
 * Animate mismatched cards
 */
function animateMismatch(card1Id, card2Id) {
    const card1Element = gameBoard.querySelector(`[data-id="${card1Id}"]`);
    const card2Element = gameBoard.querySelector(`[data-id="${card2Id}"]`);

    if (card1Element) {
        card1Element.classList.add('shake');
        setTimeout(() => {
            card1Element.classList.remove('shake');
        }, 500);
        setTimeout(() => {
            card1Element.classList.remove('flipped');
        }, 1000);
    }

    if (card2Element) {
        card2Element.classList.add('shake');
        setTimeout(() => {
            card2Element.classList.remove('shake');
        }, 500);
        setTimeout(() => {
            card2Element.classList.remove('flipped');
        }, 1000);
    }
}

/**
 * Update game stats display
 */
function updateStats() {
    const stats = game.getStats();

    movesElement.textContent = stats.moves;
    timerElement.textContent = stats.formattedTime;
    matchesElement.textContent = stats.matchedPairs;
    totalPairsElement.textContent = stats.totalPairs;

    const bestScore = stats.bestScore;
    bestScoreElement.textContent = bestScore !== null ? bestScore : '--';
}

/**
 * Start timer update interval
 */
function startTimerUpdate() {
    // Update timer display every second
    setInterval(() => {
        if (game.gameStarted && game.timerInterval) {
            timerElement.textContent = game.getFormattedTime();
        }
    }, 100);
}

/**
 * Show win modal
 */
function showWinModal() {
    const stats = game.getStats();

    // Update modal content
    finalMovesElement.textContent = stats.moves;
    finalTimeElement.textContent = stats.formattedTime;
    finalScoreElement.textContent = stats.score;

    // Check for new record
    const isNewRecord = game.saveBestScore();
    if (isNewRecord) {
        newRecordElement.classList.remove('hidden');
    } else {
        newRecordElement.classList.add('hidden');
    }

    // Show modal with animation
    winModal.classList.add('show');

    // Play win sound and confetti
    soundManager.playWin();
    confetti.start(5000);

    // Update best score display
    updateStats();
}

/**
 * Hide win modal
 */
function hideWinModal() {
    winModal.classList.remove('show');
    confetti.clear();
}

/**
 * Start new game
 */
function startNewGame() {
    // Play sound
    soundManager.playNewGame();

    // Hide modal if showing
    hideWinModal();

    // Get selected difficulty and theme
    const difficulty = difficultySelect.value;
    const theme = themeSelect.value;

    // Initialize new game
    game.init(difficulty, theme);

    // Render board
    renderBoard();
}

/**
 * Toggle sound
 */
function toggleSound() {
    soundManager.toggle();
    game.toggleSound();

    // Update icon
    soundIcon.textContent = soundManager.enabled ? '🔊' : '🔇';

    // Play click sound if enabled
    if (soundManager.enabled) {
        soundManager.playClick();
    }
}

/**
 * Handle difficulty change
 */
function handleDifficultyChange() {
    soundManager.playClick();
    // Don't auto-start new game, let user click "New Game"
    updateStats();
}

/**
 * Handle theme change
 */
function handleThemeChange() {
    soundManager.playClick();
    // Don't auto-start new game, let user click "New Game"
    updateStats();
}

/**
 * Initialize application
 */
function init() {
    // Event listeners
    newGameBtn.addEventListener('click', startNewGame);
    playAgainBtn.addEventListener('click', startNewGame);
    soundToggleBtn.addEventListener('click', toggleSound);
    difficultySelect.addEventListener('change', handleDifficultyChange);
    themeSelect.addEventListener('change', handleThemeChange);

    // Start timer updates
    startTimerUpdate();

    // Start initial game
    startNewGame();

    // Add keyboard shortcuts
    document.addEventListener('keydown', (e) => {
        if (e.key === 'Escape' && winModal.classList.contains('show')) {
            hideWinModal();
        }
        if (e.key === 'n' || e.key === 'N') {
            startNewGame();
        }
    });

    // Close modal when clicking outside
    winModal.addEventListener('click', (e) => {
        if (e.target === winModal) {
            hideWinModal();
        }
    });
}

// Start the application when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    init();
}
