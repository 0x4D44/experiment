#ifndef WORDLE_GAME_H
#define WORDLE_GAME_H

#include "common.h"
#include "word_list.h"
#include "guess_evaluator.h"
#include "statistics_manager.h"
#include "keyboard_visualizer.h"
#include "ui_renderer.h"

class WordleGame {
private:
    GameState currentGame;
    WordList wordList;
    StatisticsManager statsManager;
    KeyboardVisualizer keyboard;
    std::unique_ptr<GuessEvaluator> evaluator;

public:
    WordleGame();
    ~WordleGame() = default;

    // Game lifecycle
    void initialize(const std::string& mode = "daily");
    void run();
    void cleanup();

    // Game actions
    void processGuess(const std::string& guess);
    bool validateGuess(const std::string& guess);
    void evaluateGuess(const std::string& guess);
    void updateGameState();
    void displayGameState();

    // Game ending
    void endGame(bool won);
    void displayResults();
    void saveGame();

    // Getters for testing
    const GameState& getCurrentGame() const;
    const Statistics& getStatistics() const;
    const WordList& getWordList() const;

private:
    void displayWelcome();
    void handleInvalidInput(const std::string& reason);
    bool isHardModeValid(const std::string& guess) const;
    std::string getUserInput();
};

#endif // WORDLE_GAME_H
