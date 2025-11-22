/**
 * Memory Match Game - Core Game Logic
 * Handles game state, card matching, scoring, and game flow
 */

class MemoryGame {
    constructor() {
        this.difficulty = 'medium';
        this.theme = 'emojis';
        this.cards = [];
        this.flippedCards = [];
        this.matchedPairs = 0;
        this.moves = 0;
        this.timer = 0;
        this.timerInterval = null;
        this.isProcessing = false;
        this.gameStarted = false;
        this.soundEnabled = true;

        // Card themes
        this.themes = {
            emojis: ['🎮', '🎯', '🎲', '🎪', '🎨', '🎭', '🎬', '🎤', '🎧', '🎸', '🎹', '🎺', '🎻', '🎼', '🎵', '🎶', '🎷', '🎰', '🃏', '🎴', '🀄', '🎳', '🎱', '⚽', '⚾', '🏀', '🏐', '🏈', '🏉', '🎾', '🥎', '🏏'],
            animals: ['🐶', '🐱', '🐭', '🐹', '🐰', '🦊', '🐻', '🐼', '🐨', '🐯', '🦁', '🐮', '🐷', '🐸', '🐵', '🐔', '🐧', '🐦', '🐤', '🦆', '🦅', '🦉', '🦇', '🐺', '🐗', '🐴', '🦄', '🐝', '🐛', '🦋', '🐌', '🐞'],
            food: ['🍎', '🍊', '🍋', '🍌', '🍉', '🍇', '🍓', '🍈', '🍒', '🍑', '🥭', '🍍', '🥥', '🥝', '🍅', '🍆', '🥑', '🥦', '🥬', '🥒', '🌶️', '🌽', '🥕', '🧄', '🧅', '🥔', '🍠', '🥐', '🥖', '🍞', '🥨', '🧀'],
            space: ['🌍', '🌎', '🌏', '🌕', '🌖', '🌗', '🌘', '🌑', '🌒', '🌓', '🌔', '🌙', '🌛', '🌜', '⭐', '🌟', '✨', '⚡', '☄️', '💫', '🪐', '🌌', '🔭', '🛸', '🚀', '🛰️', '🌠', '☀️', '⛅', '🌤️', '🌈', '☁️'],
            sports: ['⚽', '🏀', '🏈', '⚾', '🥎', '🎾', '🏐', '🏉', '🥏', '🎱', '🪀', '🏓', '🏸', '🏒', '🏑', '🥍', '🏏', '🪃', '🥅', '⛳', '🪁', '🏹', '🎣', '🤿', '🥊', '🥋', '🎽', '🛹', '🛼', '⛸️', '🥌', '🎿']
        };

        // Difficulty settings
        this.difficulties = {
            easy: { cols: 4, rows: 4, pairs: 8 },
            medium: { cols: 6, rows: 6, pairs: 18 },
            hard: { cols: 8, rows: 8, pairs: 32 }
        };

        this.loadBestScore();
    }

    /**
     * Initialize a new game
     */
    init(difficulty = this.difficulty, theme = this.theme) {
        this.difficulty = difficulty;
        this.theme = theme;
        this.cards = [];
        this.flippedCards = [];
        this.matchedPairs = 0;
        this.moves = 0;
        this.timer = 0;
        this.gameStarted = false;
        this.isProcessing = false;

        this.stopTimer();
        this.generateCards();
        this.shuffleCards();

        return this.cards;
    }

    /**
     * Generate card pairs based on difficulty and theme
     */
    generateCards() {
        const config = this.difficulties[this.difficulty];
        const totalPairs = config.pairs;
        const themeEmojis = this.themes[this.theme];

        // Select random emojis from theme
        const selectedEmojis = this.getRandomEmojis(themeEmojis, totalPairs);

        // Create pairs
        this.cards = [];
        let id = 0;

        for (const emoji of selectedEmojis) {
            this.cards.push({
                id: id++,
                value: emoji,
                flipped: false,
                matched: false
            });
            this.cards.push({
                id: id++,
                value: emoji,
                flipped: false,
                matched: false
            });
        }
    }

    /**
     * Get random emojis from theme
     */
    getRandomEmojis(emojis, count) {
        const shuffled = [...emojis].sort(() => Math.random() - 0.5);
        return shuffled.slice(0, count);
    }

    /**
     * Shuffle cards using Fisher-Yates algorithm
     */
    shuffleCards() {
        for (let i = this.cards.length - 1; i > 0; i--) {
            const j = Math.floor(Math.random() * (i + 1));
            [this.cards[i], this.cards[j]] = [this.cards[j], this.cards[i]];
        }
    }

    /**
     * Handle card flip
     */
    async flipCard(cardId) {
        // Start timer on first flip
        if (!this.gameStarted) {
            this.startTimer();
            this.gameStarted = true;
        }

        // Prevent flipping during processing or if card is already flipped/matched
        const card = this.cards.find(c => c.id === cardId);
        if (!card || card.flipped || card.matched || this.isProcessing) {
            return false;
        }

        // Flip the card
        card.flipped = true;
        this.flippedCards.push(card);

        // Check if two cards are flipped
        if (this.flippedCards.length === 2) {
            this.isProcessing = true;
            this.moves++;

            const [card1, card2] = this.flippedCards;

            if (this.checkMatch(card1, card2)) {
                // Match found
                await this.handleMatch(card1, card2);
            } else {
                // No match
                await this.handleMismatch(card1, card2);
            }

            this.flippedCards = [];
            this.isProcessing = false;

            // Check win condition
            if (this.checkWin()) {
                this.stopTimer();
                return 'win';
            }
        }

        return true;
    }

    /**
     * Check if two cards match
     */
    checkMatch(card1, card2) {
        return card1.value === card2.value && card1.id !== card2.id;
    }

    /**
     * Handle matched cards
     */
    async handleMatch(card1, card2) {
        return new Promise(resolve => {
            setTimeout(() => {
                card1.matched = true;
                card2.matched = true;
                this.matchedPairs++;
                resolve();
            }, 600);
        });
    }

    /**
     * Handle mismatched cards
     */
    async handleMismatch(card1, card2) {
        return new Promise(resolve => {
            setTimeout(() => {
                card1.flipped = false;
                card2.flipped = false;
                resolve();
            }, 1000);
        });
    }

    /**
     * Check if game is won
     */
    checkWin() {
        const config = this.difficulties[this.difficulty];
        return this.matchedPairs === config.pairs;
    }

    /**
     * Start game timer
     */
    startTimer() {
        this.timerInterval = setInterval(() => {
            this.timer++;
        }, 1000);
    }

    /**
     * Stop game timer
     */
    stopTimer() {
        if (this.timerInterval) {
            clearInterval(this.timerInterval);
            this.timerInterval = null;
        }
    }

    /**
     * Format timer for display
     */
    getFormattedTime() {
        const minutes = Math.floor(this.timer / 60);
        const seconds = this.timer % 60;
        return `${minutes}:${seconds.toString().padStart(2, '0')}`;
    }

    /**
     * Calculate game score
     * Lower is better: score = moves * 10 + time in seconds
     */
    calculateScore() {
        return (this.moves * 10) + this.timer;
    }

    /**
     * Get total pairs for current difficulty
     */
    getTotalPairs() {
        return this.difficulties[this.difficulty].pairs;
    }

    /**
     * Save best score to localStorage
     */
    saveBestScore() {
        const score = this.calculateScore();
        const key = `bestScore_${this.difficulty}_${this.theme}`;
        const currentBest = localStorage.getItem(key);

        if (!currentBest || score < parseInt(currentBest)) {
            localStorage.setItem(key, score.toString());
            return true; // New record
        }

        return false;
    }

    /**
     * Load best score from localStorage
     */
    loadBestScore() {
        const key = `bestScore_${this.difficulty}_${this.theme}`;
        const score = localStorage.getItem(key);
        return score ? parseInt(score) : null;
    }

    /**
     * Get game statistics
     */
    getStats() {
        return {
            moves: this.moves,
            time: this.timer,
            formattedTime: this.getFormattedTime(),
            matchedPairs: this.matchedPairs,
            totalPairs: this.getTotalPairs(),
            score: this.calculateScore(),
            bestScore: this.loadBestScore()
        };
    }

    /**
     * Toggle sound
     */
    toggleSound() {
        this.soundEnabled = !this.soundEnabled;
        return this.soundEnabled;
    }

    /**
     * Reset game state
     */
    reset() {
        this.stopTimer();
        this.cards = [];
        this.flippedCards = [];
        this.matchedPairs = 0;
        this.moves = 0;
        this.timer = 0;
        this.gameStarted = false;
        this.isProcessing = false;
    }
}

// Export for use in other files and tests
if (typeof module !== 'undefined' && module.exports) {
    module.exports = MemoryGame;
}
