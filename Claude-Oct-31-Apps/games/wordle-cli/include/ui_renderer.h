#ifndef WORDLE_UI_RENDERER_H
#define WORDLE_UI_RENDERER_H

#include "common.h"

class UIRenderer {
public:
    // Game display functions
    static void renderGameGrid(const GameState& state);
    static void renderGuessLine(const std::string& guess,
                               const std::vector<LetterFeedback>& feedback);
    static void renderEmptyLines(int count);
    static void renderEmptyGuessLine();
    static void renderStats(const Statistics& stats);
    static void renderGameOver(bool won, const std::string& answer, int guesses);
    static void renderWelcome();
    static void renderMenu();

    // Message display
    static void showMessage(const std::string& msg);
    static void showError(const std::string& msg);
    static void showSuccess(const std::string& msg);
    static void showInfo(const std::string& msg);

    // Screen control
    static void clearScreen();
    static void showCursor();
    static void hideCursor();

    // Utility functions
    static std::string colorizeCell(char letter, LetterStatus status);
    static std::string getStatusColor(LetterStatus status);

    // ANSI color constants
    static const std::string GREEN;
    static const std::string YELLOW;
    static const std::string GRAY;
    static const std::string WHITE;
    static const std::string RESET;
    static const std::string BOLD;
    static const std::string DIM;
    static const std::string BLACK_TEXT;

private:
    // Helper functions
    static void renderRow(const std::string& row);
    static std::string centerText(const std::string& text, int width = 80);
};

#endif // WORDLE_UI_RENDERER_H
