#include "guess_evaluator.h"
#include <algorithm>
#include <cctype>

GuessEvaluator::GuessEvaluator(const std::string& answer)
    : answerWord(answer) {
    // Normalize to uppercase
    std::transform(answerWord.begin(), answerWord.end(),
                  answerWord.begin(), ::toupper);
}

std::vector<LetterFeedback> GuessEvaluator::evaluate(const std::string& guess) {
    std::string normalizedGuess = guess;
    std::transform(normalizedGuess.begin(), normalizedGuess.end(),
                  normalizedGuess.begin(), ::toupper);

    std::vector<LetterFeedback> feedback;

    // First pass: Mark all exact matches (GREEN)
    std::vector<bool> answerUsed(5, false);
    for (int i = 0; i < 5; ++i) {
        if (normalizedGuess[i] == answerWord[i]) {
            feedback.push_back(LetterFeedback(normalizedGuess[i],
                                             LetterStatus::GREEN, i));
            answerUsed[i] = true;
        } else {
            feedback.push_back(LetterFeedback(normalizedGuess[i],
                                             LetterStatus::GRAY, i));
        }
    }

    // Second pass: Mark wrong positions (YELLOW) for non-exact matches
    for (int i = 0; i < 5; ++i) {
        if (feedback[i].status == LetterStatus::GRAY) {  // Not an exact match
            bool found = false;
            for (int j = 0; j < 5; ++j) {
                if (!answerUsed[j] && answerWord[j] == normalizedGuess[i]) {
                    feedback[i].status = LetterStatus::YELLOW;
                    answerUsed[j] = true;
                    found = true;
                    break;
                }
            }
        }
    }

    return feedback;
}

bool GuessEvaluator::hasLetter(char letter) const {
    char upperLetter = ::toupper(letter);
    return answerWord.find(upperLetter) != std::string::npos;
}

int GuessEvaluator::getLetterCount(char letter) const {
    char upperLetter = ::toupper(letter);
    int count = 0;
    for (char c : answerWord) {
        if (c == upperLetter) {
            ++count;
        }
    }
    return count;
}

std::string GuessEvaluator::getAnswerWord() const {
    return answerWord;
}
