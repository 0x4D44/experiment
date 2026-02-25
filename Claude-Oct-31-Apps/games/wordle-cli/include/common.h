#ifndef WORDLE_COMMON_H
#define WORDLE_COMMON_H

#include <string>
#include <vector>
#include <array>
#include <unordered_map>
#include <unordered_set>
#include <memory>
#include <chrono>
#include <iostream>

// Enum for letter feedback status
enum class LetterStatus {
    GRAY,      // Not in word
    YELLOW,    // In word, wrong position
    GREEN      // Correct position
};

// Struct for individual letter feedback
struct LetterFeedback {
    char letter;
    LetterStatus status;
    int position;

    LetterFeedback() : letter(' '), status(LetterStatus::GRAY), position(-1) {}
    LetterFeedback(char l, LetterStatus s, int p)
        : letter(l), status(s), position(p) {}

    bool operator==(const LetterFeedback& other) const {
        return letter == other.letter &&
               status == other.status &&
               position == other.position;
    }
};

// Game state structure
struct GameState {
    std::string gameId;
    std::string answerWord;
    std::string gameMode;  // "daily" or "practice"

    std::vector<std::string> guesses;
    std::vector<std::vector<LetterFeedback>> feedbacks;

    int remainingGuesses;
    bool isGameOver;
    bool isWon;
    bool hardMode;

    std::chrono::system_clock::time_point startTime;
    std::chrono::system_clock::time_point endTime;
    std::unordered_set<char> revealedLetters;

    GameState()
        : remainingGuesses(6),
          isGameOver(false),
          isWon(false),
          hardMode(false) {}

    int getGuessCount() const {
        return guesses.size();
    }
};

// Statistics structure
struct Statistics {
    int gamesPlayed;
    int gamesWon;
    int currentStreak;
    int maxStreak;
    std::array<int, 6> guessDistribution;
    std::string lastPlayedDate;

    Statistics()
        : gamesPlayed(0),
          gamesWon(0),
          currentStreak(0),
          maxStreak(0),
          guessDistribution({0, 0, 0, 0, 0, 0}) {}

    float getWinPercentage() const {
        if (gamesPlayed == 0) return 0.0f;
        return (gamesWon * 100.0f) / gamesPlayed;
    }

    int getAverageGuesses() const {
        if (gamesWon == 0) return 0;
        int total = 0;
        for (int i = 0; i < 6; ++i) {
            total += (i + 1) * guessDistribution[i];
        }
        return total / gamesWon;
    }
};

#endif // WORDLE_COMMON_H
