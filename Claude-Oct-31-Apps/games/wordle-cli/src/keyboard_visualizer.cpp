#include "keyboard_visualizer.h"
#include <iostream>

const std::string KeyboardVisualizer::KEYBOARD_ROW1 = "QWERTYUIOP";
const std::string KeyboardVisualizer::KEYBOARD_ROW2 = "ASDFGHJKL";
const std::string KeyboardVisualizer::KEYBOARD_ROW3 = "ZXCVBNM";

const std::string KeyboardVisualizer::COLOR_GREEN = "\033[42m";
const std::string KeyboardVisualizer::COLOR_YELLOW = "\033[43m";
const std::string KeyboardVisualizer::COLOR_GRAY = "\033[47m";
const std::string KeyboardVisualizer::COLOR_RESET = "\033[0m";

KeyboardVisualizer::KeyboardVisualizer() {
    initialize();
}

void KeyboardVisualizer::initialize() {
    for (char c : KEYBOARD_ROW1) letterStatus[c] = LetterStatus::GRAY;
    for (char c : KEYBOARD_ROW2) letterStatus[c] = LetterStatus::GRAY;
    for (char c : KEYBOARD_ROW3) letterStatus[c] = LetterStatus::GRAY;
}

void KeyboardVisualizer::updateLetter(char letter, LetterStatus status) {
    char upperLetter = ::toupper(letter);
    auto it = letterStatus.find(upperLetter);
    if (it != letterStatus.end()) {
        // GREEN takes precedence, otherwise update to new status
        if (it->second != LetterStatus::GREEN) {
            it->second = status;
        }
    }
}

void KeyboardVisualizer::reset() {
    initialize();
}

void KeyboardVisualizer::render() const {
    std::cout << "\n";

    // Row 1
    for (char c : KEYBOARD_ROW1) {
        auto it = letterStatus.find(c);
        LetterStatus status = (it != letterStatus.end()) ? it->second : LetterStatus::GRAY;
        std::cout << getColorCode(status) << " " << c << " " << COLOR_RESET;
    }
    std::cout << "\n";

    // Row 2 (indented)
    std::cout << " ";
    for (char c : KEYBOARD_ROW2) {
        auto it = letterStatus.find(c);
        LetterStatus status = (it != letterStatus.end()) ? it->second : LetterStatus::GRAY;
        std::cout << getColorCode(status) << " " << c << " " << COLOR_RESET;
    }
    std::cout << "\n";

    // Row 3 (more indented)
    std::cout << "  ";
    for (char c : KEYBOARD_ROW3) {
        auto it = letterStatus.find(c);
        LetterStatus status = (it != letterStatus.end()) ? it->second : LetterStatus::GRAY;
        std::cout << getColorCode(status) << " " << c << " " << COLOR_RESET;
    }
    std::cout << "\n\n";
}

LetterStatus KeyboardVisualizer::getLetterStatus(char letter) const {
    char upperLetter = ::toupper(letter);
    auto it = letterStatus.find(upperLetter);
    if (it != letterStatus.end()) {
        return it->second;
    }
    return LetterStatus::GRAY;
}

std::vector<char> KeyboardVisualizer::getLettersWithStatus(LetterStatus status) const {
    std::vector<char> result;
    for (const auto& pair : letterStatus) {
        if (pair.second == status) {
            result.push_back(pair.first);
        }
    }
    return result;
}

std::string KeyboardVisualizer::getColorCode(LetterStatus status) const {
    switch (status) {
        case LetterStatus::GREEN:
            return COLOR_GREEN;
        case LetterStatus::YELLOW:
            return COLOR_YELLOW;
        case LetterStatus::GRAY:
        default:
            return COLOR_GRAY;
    }
}

std::string KeyboardVisualizer::getEmoji(LetterStatus status) const {
    switch (status) {
        case LetterStatus::GREEN:
            return "🟩";
        case LetterStatus::YELLOW:
            return "🟨";
        case LetterStatus::GRAY:
        default:
            return "⬜";
    }
}
