#ifndef WORDLE_GUESS_EVALUATOR_H
#define WORDLE_GUESS_EVALUATOR_H

#include "common.h"

class GuessEvaluator {
private:
    std::string answerWord;

public:
    explicit GuessEvaluator(const std::string& answer);
    ~GuessEvaluator() = default;

    // Main evaluation function - returns feedback for each letter
    std::vector<LetterFeedback> evaluate(const std::string& guess);

    // Check if answer contains a letter
    bool hasLetter(char letter) const;

    // Get count of a letter in answer
    int getLetterCount(char letter) const;

    // Get the answer word
    std::string getAnswerWord() const;

private:
    // Helper to evaluate a single letter at a position
    LetterStatus evaluateLetterStatus(const std::string& guess, int position) const;

    // Handle repeated letters correctly
    void handleRepeatedLetters(std::vector<LetterFeedback>& feedback,
                               const std::string& guess);
};

#endif // WORDLE_GUESS_EVALUATOR_H
