#include "ui_renderer.h"
#include <iostream>
#include <iomanip>

const std::string UIRenderer::GREEN = "\033[42m";
const std::string UIRenderer::YELLOW = "\033[43m";
const std::string UIRenderer::GRAY = "\033[47m";
const std::string UIRenderer::WHITE = "\033[47m";
const std::string UIRenderer::RESET = "\033[0m";
const std::string UIRenderer::BOLD = "\033[1m";
const std::string UIRenderer::DIM = "\033[2m";
const std::string UIRenderer::BLACK_TEXT = "\033[30m";

void UIRenderer::clearScreen() {
    std::cout << "\033[2J\033[H";
}

void UIRenderer::showCursor() {
    std::cout << "\033[?25h";
}

void UIRenderer::hideCursor() {
    std::cout << "\033[?25l";
}

void UIRenderer::showMessage(const std::string& msg) {
    std::cout << msg << std::endl;
}

void UIRenderer::showError(const std::string& msg) {
    std::cout << "\033[91m" << "✗ " << msg << RESET << std::endl;
}

void UIRenderer::showSuccess(const std::string& msg) {
    std::cout << "\033[92m" << "✓ " << msg << RESET << std::endl;
}

void UIRenderer::showInfo(const std::string& msg) {
    std::cout << "\033[94m" << "ℹ " << msg << RESET << std::endl;
}

void UIRenderer::renderWelcome() {
    clearScreen();
    std::cout << "\n";
    std::cout << "    " << BOLD << "W O R D L E" << RESET << "\n";
    std::cout << "   " << DIM << "Guess the word in 6 tries" << RESET << "\n\n";
}

void UIRenderer::renderMenu() {
    std::cout << "  [D]aily  [P]ractice  [S]tats  [Q]uit\n\n";
}

void UIRenderer::renderGameGrid(const GameState& state) {
    clearScreen();
    std::cout << BOLD << "WORDLE" << RESET << std::string(26, ' ')
              << "Guess " << state.getGuessCount() << " of 6\n\n";

    // Render guesses
    for (const auto& feedback : state.feedbacks) {
        renderGuessLine(state.guesses[state.feedbacks.size() == state.guesses.size() ?
                                      &feedback - &state.feedbacks[0] : 0], feedback);
    }

    // Render empty lines
    int emptyLines = 6 - state.feedbacks.size();
    for (int i = 0; i < emptyLines; ++i) {
        renderEmptyGuessLine();
    }

    std::cout << "\n";
}

void UIRenderer::renderGuessLine(const std::string& guess,
                                const std::vector<LetterFeedback>& feedback) {
    for (size_t i = 0; i < feedback.size(); ++i) {
        std::cout << colorizeCell(feedback[i].letter, feedback[i].status);
    }
    std::cout << "\n";
}

void UIRenderer::renderEmptyGuessLine() {
    for (int i = 0; i < 5; ++i) {
        std::cout << "\033[47m   \033[0m";
    }
    std::cout << "\n";
}

void UIRenderer::renderEmptyLines(int count) {
    for (int i = 0; i < count; ++i) {
        renderEmptyGuessLine();
    }
}

void UIRenderer::renderStats(const Statistics& stats) {
    std::cout << "\n" << BOLD << "STATISTICS" << RESET << "\n";
    std::cout << std::string(50, '-') << "\n";
    std::cout << "Played: " << stats.gamesPlayed << "\n";
    std::cout << "Won:    " << stats.gamesWon << "\n";
    std::cout << "Win %:  " << std::fixed << std::setprecision(1)
              << stats.getWinPercentage() << "%\n";
    std::cout << "Streak: " << stats.currentStreak << "\n";
    std::cout << "Max:    " << stats.maxStreak << "\n";
    std::cout << "\nGuess Distribution:\n";
    for (int i = 0; i < 6; ++i) {
        std::cout << "  " << (i + 1) << " guess" << (i != 0 ? "es" : " ")
                  << ": " << stats.guessDistribution[i] << "\n";
    }
    std::cout << std::string(50, '-') << "\n\n";
}

void UIRenderer::renderGameOver(bool won, const std::string& answer, int guesses) {
    if (won) {
        std::cout << "\n" << "\033[92m" << BOLD << "YOU WIN!" << RESET << "\n";
        std::cout << "You found it in " << guesses << " guess" << (guesses != 1 ? "es" : "") << "\n";
    } else {
        std::cout << "\n" << "\033[91m" << BOLD << "GAME OVER" << RESET << "\n";
        std::cout << "The answer was: " << BOLD << answer << RESET << "\n";
    }
}

std::string UIRenderer::colorizeCell(char letter, LetterStatus status) {
    std::string colorCode = getStatusColor(status);
    std::string cellStr;
    cellStr += colorCode;
    cellStr += BLACK_TEXT;
    cellStr += " ";
    cellStr += letter;
    cellStr += " ";
    cellStr += RESET;
    return cellStr;
}

std::string UIRenderer::getStatusColor(LetterStatus status) {
    switch (status) {
        case LetterStatus::GREEN:
            return GREEN;
        case LetterStatus::YELLOW:
            return YELLOW;
        case LetterStatus::GRAY:
        default:
            return GRAY;
    }
}

std::string UIRenderer::centerText(const std::string& text, int width) {
    int padding = (width - text.length()) / 2;
    return std::string(padding, ' ') + text;
}
