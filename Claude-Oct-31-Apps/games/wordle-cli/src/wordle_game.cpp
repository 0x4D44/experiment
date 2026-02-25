#include "wordle_game.h"
#include "daily_word_selector.h"
#include "input_validator.h"
#include <iostream>
#include <memory>

WordleGame::WordleGame() {
    wordList.loadFromFiles(
        "/home/md/language/ClaudeApps/games/wordle-cli/resources/valid_words.txt",
        "/home/md/language/ClaudeApps/games/wordle-cli/resources/answer_words.txt"
    );
}

void WordleGame::initialize(const std::string& mode) {
    currentGame.gameMode = mode;
    currentGame.remainingGuesses = 6;
    currentGame.isGameOver = false;
    currentGame.isWon = false;
    currentGame.guesses.clear();
    currentGame.feedbacks.clear();
    currentGame.revealedLetters.clear();

    if (mode == "daily") {
        currentGame.answerWord = DailyWordSelector::getDailyWord(wordList);
    } else {
        currentGame.answerWord = wordList.getRandomAnswerWord();
    }

    evaluator = std::make_unique<GuessEvaluator>(currentGame.answerWord);
    keyboard.reset();
    currentGame.startTime = std::chrono::system_clock::now();
}

void WordleGame::run() {
    displayWelcome();
    initialize("daily");
    displayGameState();

    while (!currentGame.isGameOver) {
        std::string input = getUserInput();

        if (input == "Q" || input == "q") {
            break;
        }

        if (!validateGuess(input)) {
            continue;
        }

        evaluateGuess(input);
        updateGameState();
        displayGameState();
    }

    if (currentGame.isGameOver) {
        displayResults();
    }
}

void WordleGame::processGuess(const std::string& guess) {
    if (validateGuess(guess)) {
        evaluateGuess(guess);
        updateGameState();
    }
}

bool WordleGame::validateGuess(const std::string& guess) {
    if (!InputValidator::isValidGuessFormat(guess)) {
        handleInvalidInput("Must be 5 letters");
        return false;
    }

    std::string sanitized = InputValidator::sanitize(guess);

    if (!wordList.isValidWord(sanitized)) {
        handleInvalidInput("Not in word list");
        return false;
    }

    if (currentGame.hardMode && !isHardModeValid(sanitized)) {
        handleInvalidInput("Must use revealed letters");
        return false;
    }

    return true;
}

void WordleGame::evaluateGuess(const std::string& guess) {
    std::string sanitized = InputValidator::sanitize(guess);
    currentGame.guesses.push_back(sanitized);

    auto feedback = evaluator->evaluate(sanitized);
    currentGame.feedbacks.push_back(feedback);

    // Update keyboard
    for (const auto& letter : feedback) {
        keyboard.updateLetter(letter.letter, letter.status);
        if (letter.status == LetterStatus::GREEN ||
            letter.status == LetterStatus::YELLOW) {
            currentGame.revealedLetters.insert(letter.letter);
        }
    }
}

void WordleGame::updateGameState() {
    currentGame.remainingGuesses--;

    // Check for win
    if (!currentGame.feedbacks.empty()) {
        const auto& lastFeedback = currentGame.feedbacks.back();
        bool allGreen = true;
        for (const auto& letter : lastFeedback) {
            if (letter.status != LetterStatus::GREEN) {
                allGreen = false;
                break;
            }
        }

        if (allGreen) {
            currentGame.isGameOver = true;
            currentGame.isWon = true;
            currentGame.endTime = std::chrono::system_clock::now();
            saveGame();
            return;
        }
    }

    // Check for loss
    if (currentGame.remainingGuesses <= 0) {
        currentGame.isGameOver = true;
        currentGame.isWon = false;
        currentGame.endTime = std::chrono::system_clock::now();
        saveGame();
    }
}

void WordleGame::displayGameState() {
    UIRenderer::renderGameGrid(currentGame);
    keyboard.render();
}

void WordleGame::endGame(bool won) {
    currentGame.isGameOver = true;
    currentGame.isWon = won;
    currentGame.endTime = std::chrono::system_clock::now();
    saveGame();
}

void WordleGame::displayResults() {
    UIRenderer::renderGameOver(currentGame.isWon, currentGame.answerWord,
                              currentGame.getGuessCount());
    UIRenderer::renderStats(statsManager.getStats());

    std::cout << "\nShare: \n";
    std::cout << statsManager.generateShareText(currentGame) << "\n";
}

void WordleGame::saveGame() {
    statsManager.recordGame(currentGame);
}

const GameState& WordleGame::getCurrentGame() const {
    return currentGame;
}

const Statistics& WordleGame::getStatistics() const {
    return statsManager.getStats();
}

const WordList& WordleGame::getWordList() const {
    return wordList;
}

void WordleGame::displayWelcome() {
    UIRenderer::renderWelcome();
}

void WordleGame::handleInvalidInput(const std::string& reason) {
    UIRenderer::showError(reason);
}

bool WordleGame::isHardModeValid(const std::string& guess) const {
    for (char revealedLetter : currentGame.revealedLetters) {
        if (guess.find(revealedLetter) == std::string::npos) {
            return false;
        }
    }
    return true;
}

std::string WordleGame::getUserInput() {
    std::cout << "> ";
    std::string input;
    std::getline(std::cin, input);
    return input;
}

void WordleGame::cleanup() {
    // Any cleanup needed
}
