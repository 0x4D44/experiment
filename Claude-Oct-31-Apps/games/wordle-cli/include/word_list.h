#ifndef WORDLE_WORD_LIST_H
#define WORDLE_WORD_LIST_H

#include "common.h"

class WordList {
private:
    std::vector<std::string> answerWords;
    std::unordered_set<std::string> validWords;

public:
    WordList() = default;
    ~WordList() = default;

    // Load word lists from files
    void loadFromFiles(const std::string& validPath, const std::string& answerPath);

    // Check if a word is valid (can be guessed)
    bool isValidWord(const std::string& word) const;

    // Check if a word is an answer word (can be the answer)
    bool isAnswerWord(const std::string& word) const;

    // Get a random answer word
    std::string getRandomAnswerWord() const;

    // Get answer word at specific index
    std::string getAnswerWord(size_t index) const;

    // Get count of answer words
    size_t getAnswerCount() const;

    // Get count of valid words
    size_t getValidCount() const;

    // Clear all words
    void clear();

private:
    // Helper function to normalize word (uppercase, trim)
    std::string normalize(const std::string& word) const;

    // Load words from a single file
    void loadWordsFromFile(const std::string& path,
                          std::vector<std::string>* destVector,
                          std::unordered_set<std::string>* destSet = nullptr);
};

#endif // WORDLE_WORD_LIST_H
