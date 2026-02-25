#include "statistics_manager.h"
#include <fstream>
#include <sstream>
#include <filesystem>
#include <cstring>

namespace fs = std::filesystem;

StatisticsManager::StatisticsManager() {
    ensureDataDirectory();
    std::string statsPath = std::getenv("HOME");
    statsPath += "/.wordle/stats.json";
    statsFilePath = statsPath;
    loadFromFile(statsFilePath);
}

void StatisticsManager::loadFromFile(const std::string& path) {
    std::ifstream file(path);
    if (!file.is_open()) {
        // File doesn't exist, use default stats
        stats = Statistics();
        return;
    }

    std::stringstream buffer;
    buffer << file.rdbuf();
    file.close();

    parseStatsJson(buffer.str());
}

void StatisticsManager::saveToFile() const {
    ensureDataDirectory();
    std::ofstream file(statsFilePath);
    if (file.is_open()) {
        file << generateStatsJson();
        file.close();
    }
}

void StatisticsManager::recordGame(const GameState& gameState) {
    stats.gamesPlayed++;

    if (gameState.isWon) {
        stats.gamesWon++;
        int guesses = gameState.getGuessCount();
        if (guesses >= 1 && guesses <= 6) {
            stats.guessDistribution[guesses - 1]++;
        }
    }

    updateStreak(gameState.isWon);
    saveToFile();
}

void StatisticsManager::updateStreak(bool won) {
    if (won) {
        stats.currentStreak++;
        if (stats.currentStreak > stats.maxStreak) {
            stats.maxStreak = stats.currentStreak;
        }
    } else {
        stats.currentStreak = 0;
    }
}

const Statistics& StatisticsManager::getStats() const {
    return stats;
}

Statistics& StatisticsManager::getStatsMutable() {
    return stats;
}

std::string StatisticsManager::generateShareText(const GameState& gameState) const {
    std::stringstream ss;
    ss << "Wordle " << gameState.getGuessCount() << "/6\n\n";

    for (const auto& feedback : gameState.feedbacks) {
        for (const auto& letter : feedback) {
            ss << getStatusEmoji(letter.status);
        }
        ss << "\n";
    }

    return ss.str();
}

std::string StatisticsManager::getStatusEmoji(LetterStatus status) {
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

void StatisticsManager::reset() {
    stats = Statistics();
    saveToFile();
}

bool StatisticsManager::ensureDirectoryExists(const std::string& path) const {
    try {
        fs::path dirPath = path;
        if (dirPath.has_filename()) {
            dirPath = dirPath.parent_path();
        }
        fs::create_directories(dirPath);
        return true;
    } catch (const std::exception& e) {
        return false;
    }
}

void StatisticsManager::ensureDataDirectory() {
    std::string homeDir = std::getenv("HOME") ? std::getenv("HOME") : "";
    if (homeDir.empty()) {
        return;
    }
    std::string wordleDir = homeDir + "/.wordle";
    ensureDirectoryExists(wordleDir + "/");
}

bool StatisticsManager::parseStatsJson(const std::string& jsonStr) {
    // Simple JSON parsing (since we don't want external deps)
    // Expected format: {"gamesPlayed":X,"gamesWon":Y,"currentStreak":Z,"maxStreak":W,"guessDistribution":[...]}

    try {
        // Reset to defaults
        stats = Statistics();

        // Extract gamesPlayed
        size_t pos = jsonStr.find("\"gamesPlayed\":");
        if (pos != std::string::npos) {
            sscanf(jsonStr.c_str() + pos, "\"gamesPlayed\":%d", &stats.gamesPlayed);
        }

        // Extract gamesWon
        pos = jsonStr.find("\"gamesWon\":");
        if (pos != std::string::npos) {
            sscanf(jsonStr.c_str() + pos, "\"gamesWon\":%d", &stats.gamesWon);
        }

        // Extract currentStreak
        pos = jsonStr.find("\"currentStreak\":");
        if (pos != std::string::npos) {
            sscanf(jsonStr.c_str() + pos, "\"currentStreak\":%d", &stats.currentStreak);
        }

        // Extract maxStreak
        pos = jsonStr.find("\"maxStreak\":");
        if (pos != std::string::npos) {
            sscanf(jsonStr.c_str() + pos, "\"maxStreak\":%d", &stats.maxStreak);
        }

        return true;
    } catch (...) {
        return false;
    }
}

std::string StatisticsManager::generateStatsJson() const {
    std::stringstream ss;
    ss << "{\n";
    ss << "  \"gamesPlayed\": " << stats.gamesPlayed << ",\n";
    ss << "  \"gamesWon\": " << stats.gamesWon << ",\n";
    ss << "  \"currentStreak\": " << stats.currentStreak << ",\n";
    ss << "  \"maxStreak\": " << stats.maxStreak << ",\n";
    ss << "  \"guessDistribution\": [";

    for (int i = 0; i < 6; ++i) {
        ss << stats.guessDistribution[i];
        if (i < 5) ss << ", ";
    }

    ss << "]\n}\n";
    return ss.str();
}
