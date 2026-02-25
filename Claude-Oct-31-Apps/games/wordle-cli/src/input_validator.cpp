#include "input_validator.h"
#include <algorithm>
#include <cctype>

bool InputValidator::isValidGuessFormat(const std::string& input) {
    std::string sanitized = sanitize(input);
    return isCorrectLength(sanitized) && isAlphabetic(sanitized);
}

std::string InputValidator::sanitize(const std::string& input) {
    std::string trimmed = trim(input);
    return toUpperCase(trimmed);
}

bool InputValidator::isAlphabetic(const std::string& input) {
    if (input.empty()) return false;
    return std::all_of(input.begin(), input.end(),
                      [](char c) { return std::isalpha(c); });
}

std::string InputValidator::toUpperCase(const std::string& input) {
    std::string result = input;
    std::transform(result.begin(), result.end(), result.begin(), ::toupper);
    return result;
}

bool InputValidator::isCorrectLength(const std::string& input) {
    return input.length() == 5;
}

std::string InputValidator::trim(const std::string& input) {
    size_t first = input.find_first_not_of(" \t\n\r");
    if (first == std::string::npos) return "";
    size_t last = input.find_last_not_of(" \t\n\r");
    return input.substr(first, (last - first + 1));
}
