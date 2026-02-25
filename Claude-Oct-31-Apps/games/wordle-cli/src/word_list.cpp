#include "word_list.h"
#include <fstream>
#include <algorithm>
#include <cctype>
#include <random>

void WordList::loadFromFiles(const std::string& validPath, const std::string& answerPath) {
    clear();
    loadWordsFromFile(validPath, &answerWords, nullptr);
    answerWords.clear();
    loadWordsFromFile(answerPath, &answerWords, nullptr);
    loadWordsFromFile(validPath, nullptr, &validWords);
}

bool WordList::isValidWord(const std::string& word) const {
    std::string normalized = normalize(word);
    return validWords.find(normalized) != validWords.end();
}

bool WordList::isAnswerWord(const std::string& word) const {
    std::string normalized = normalize(word);
    return std::find(answerWords.begin(), answerWords.end(), normalized)
           != answerWords.end();
}

std::string WordList::getRandomAnswerWord() const {
    if (answerWords.empty()) {
        return "";
    }
    static std::random_device rd;
    static std::mt19937 gen(rd());
    std::uniform_int_distribution<> dis(0, answerWords.size() - 1);
    return answerWords[dis(gen)];
}

std::string WordList::getAnswerWord(size_t index) const {
    if (index >= answerWords.size()) {
        return "";
    }
    return answerWords[index];
}

size_t WordList::getAnswerCount() const {
    return answerWords.size();
}

size_t WordList::getValidCount() const {
    return validWords.size();
}

void WordList::clear() {
    answerWords.clear();
    validWords.clear();
}

std::string WordList::normalize(const std::string& word) const {
    std::string result = word;
    std::transform(result.begin(), result.end(), result.begin(), ::toupper);
    // Trim whitespace
    size_t first = result.find_first_not_of(" \t\n\r");
    if (first == std::string::npos) return "";
    size_t last = result.find_last_not_of(" \t\n\r");
    return result.substr(first, (last - first + 1));
}

void WordList::loadWordsFromFile(const std::string& path,
                                std::vector<std::string>* destVector,
                                std::unordered_set<std::string>* destSet) {
    std::ifstream file(path);
    if (!file.is_open()) {
        throw std::runtime_error("Could not open word list file: " + path);
    }

    std::string word;
    while (std::getline(file, word)) {
        std::string normalized = normalize(word);
        if (!normalized.empty() && normalized.length() == 5) {
            if (destVector) {
                destVector->push_back(normalized);
            }
            if (destSet) {
                destSet->insert(normalized);
            }
        }
    }

    file.close();
}
