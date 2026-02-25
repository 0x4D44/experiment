#ifndef WORDLE_STATISTICS_MANAGER_H
#define WORDLE_STATISTICS_MANAGER_H

#include "common.h"

class StatisticsManager {
private:
    Statistics stats;
    std::string statsFilePath;

public:
    StatisticsManager();
    ~StatisticsManager() = default;

    // Load statistics from file
    void loadFromFile(const std::string& path);

    // Save statistics to file
    void saveToFile() const;

    // Record a completed game
    void recordGame(const GameState& gameState);

    // Update streak after a game
    void updateStreak(bool won);

    // Get current statistics
    const Statistics& getStats() const;

    // Get mutable statistics (for tests)
    Statistics& getStatsMutable();

    // Generate share text with emoji grid
    std::string generateShareText(const GameState& gameState) const;

    // Get emoji for letter status
    static std::string getStatusEmoji(LetterStatus status);

    // Reset statistics (for testing)
    void reset();

private:
    // Create directory if it doesn't exist
    bool ensureDirectoryExists(const std::string& path) const;

    // Ensure data directory exists
    void ensureDataDirectory();

    // Parse statistics from JSON
    bool parseStatsJson(const std::string& jsonStr);

    // Generate JSON from statistics
    std::string generateStatsJson() const;
};

#endif // WORDLE_STATISTICS_MANAGER_H
