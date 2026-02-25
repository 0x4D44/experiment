#ifndef WORDLE_KEYBOARD_VISUALIZER_H
#define WORDLE_KEYBOARD_VISUALIZER_H

#include "common.h"

class KeyboardVisualizer {
private:
    std::unordered_map<char, LetterStatus> letterStatus;

public:
    KeyboardVisualizer();
    ~KeyboardVisualizer() = default;

    // Update a letter's status
    void updateLetter(char letter, LetterStatus status);

    // Reset keyboard
    void reset();

    // Render keyboard to console
    void render() const;

    // Get status of a letter
    LetterStatus getLetterStatus(char letter) const;

    // Get all letters with a specific status
    std::vector<char> getLettersWithStatus(LetterStatus status) const;

private:
    // Initialize keyboard with all letters
    void initialize();

    // Get ANSI color code for status
    std::string getColorCode(LetterStatus status) const;

    // Get emoji for status
    std::string getEmoji(LetterStatus status) const;

    static const std::string KEYBOARD_ROW1;
    static const std::string KEYBOARD_ROW2;
    static const std::string KEYBOARD_ROW3;

    static const std::string COLOR_GREEN;
    static const std::string COLOR_YELLOW;
    static const std::string COLOR_GRAY;
    static const std::string COLOR_RESET;
};

#endif // WORDLE_KEYBOARD_VISUALIZER_H
