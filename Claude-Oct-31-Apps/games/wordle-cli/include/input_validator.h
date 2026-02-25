#ifndef WORDLE_INPUT_VALIDATOR_H
#define WORDLE_INPUT_VALIDATOR_H

#include "common.h"

class InputValidator {
public:
    // Check if input is valid guess format
    static bool isValidGuessFormat(const std::string& input);

    // Sanitize input (trim, uppercase)
    static std::string sanitize(const std::string& input);

    // Check if string is alphabetic only
    static bool isAlphabetic(const std::string& input);

    // Convert to uppercase
    static std::string toUpperCase(const std::string& input);

    // Check if exactly 5 characters
    static bool isCorrectLength(const std::string& input);

private:
    // Trim whitespace from both ends
    static std::string trim(const std::string& input);
};

#endif // WORDLE_INPUT_VALIDATOR_H
