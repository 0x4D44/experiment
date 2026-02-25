#include <gtest/gtest.h>
#include "statistics_manager.h"
#include <filesystem>

namespace fs = std::filesystem;

class StatisticsManagerTest : public ::testing::Test {
protected:
    StatisticsManagerTest() = default;
    ~StatisticsManagerTest() override = default;

    void SetUp() override {
        testDir = fs::temp_directory_path() / "wordle_stats_tests";
        fs::create_directories(testDir);
        statsFile = testDir / "stats.json";
    }

    void TearDown() override {
        if (fs::exists(testDir)) {
            fs::remove_all(testDir);
        }
    }

    fs::path testDir;
    fs::path statsFile;
};

// Test: Initialize statistics
TEST_F(StatisticsManagerTest, InitializeStatistics) {
    StatisticsManager manager;
    const auto& stats = manager.getStats();

    EXPECT_EQ(stats.gamesPlayed, 0);
    EXPECT_EQ(stats.gamesWon, 0);
    EXPECT_EQ(stats.currentStreak, 0);
    EXPECT_EQ(stats.maxStreak, 0);
    EXPECT_EQ(stats.getWinPercentage(), 0.0f);
}

// Test: Record a win
TEST_F(StatisticsManagerTest, RecordWin) {
    StatisticsManager manager;

    GameState gameState;
    gameState.isWon = true;
    gameState.guesses = {"STARE", "PLANT"};
    gameState.remainingGuesses = 4;

    manager.recordGame(gameState);

    const auto& stats = manager.getStats();
    EXPECT_EQ(stats.gamesPlayed, 1);
    EXPECT_EQ(stats.gamesWon, 1);
    EXPECT_EQ(stats.guessDistribution[1], 1);  // 2 guesses (index 1)
}

// Test: Record multiple wins
TEST_F(StatisticsManagerTest, RecordMultipleWins) {
    StatisticsManager manager;

    for (int i = 0; i < 3; ++i) {
        GameState gameState;
        gameState.isWon = true;
        gameState.guesses = {"STARE"};
        manager.recordGame(gameState);
    }

    const auto& stats = manager.getStats();
    EXPECT_EQ(stats.gamesPlayed, 3);
    EXPECT_EQ(stats.gamesWon, 3);
    EXPECT_EQ(stats.currentStreak, 3);
}

// Test: Record a loss
TEST_F(StatisticsManagerTest, RecordLoss) {
    StatisticsManager manager;

    GameState gameState;
    gameState.isWon = false;
    gameState.guesses = {"G1", "G2", "G3", "G4", "G5", "G6"};

    manager.recordGame(gameState);

    const auto& stats = manager.getStats();
    EXPECT_EQ(stats.gamesPlayed, 1);
    EXPECT_EQ(stats.gamesWon, 0);
    EXPECT_EQ(stats.currentStreak, 0);
}

// Test: Win percentage calculation
TEST_F(StatisticsManagerTest, WinPercentageCalculation) {
    StatisticsManager manager;
    auto& stats = manager.getStatsMutable();

    stats.gamesPlayed = 10;
    stats.gamesWon = 7;

    float percentage = stats.getWinPercentage();
    EXPECT_FLOAT_EQ(percentage, 70.0f);
}

// Test: Average guesses calculation
TEST_F(StatisticsManagerTest, AverageGuessesCalculation) {
    StatisticsManager manager;
    auto& stats = manager.getStatsMutable();

    // 2 games won in 3 guesses = average 3
    stats.gamesWon = 2;
    stats.guessDistribution[2] = 2;  // 3 guesses at index 2

    int average = stats.getAverageGuesses();
    EXPECT_EQ(average, 3);
}

// Test: Streak increases on win
TEST_F(StatisticsManagerTest, StreakIncreasesOnWin) {
    StatisticsManager manager;

    GameState gameState;
    gameState.isWon = true;
    gameState.guesses = {"STARE"};

    manager.recordGame(gameState);
    EXPECT_EQ(manager.getStats().currentStreak, 1);

    manager.recordGame(gameState);
    EXPECT_EQ(manager.getStats().currentStreak, 2);
}

// Test: Streak resets on loss
TEST_F(StatisticsManagerTest, StreakResetsOnLoss) {
    StatisticsManager manager;
    auto& stats = manager.getStatsMutable();
    stats.currentStreak = 5;

    GameState gameState;
    gameState.isWon = false;
    gameState.guesses = {"G1", "G2", "G3", "G4", "G5", "G6"};

    manager.recordGame(gameState);
    EXPECT_EQ(manager.getStats().currentStreak, 0);
}

// Test: Max streak tracking
TEST_F(StatisticsManagerTest, MaxStreakTracking) {
    StatisticsManager manager;

    GameState gameState;
    gameState.isWon = true;
    gameState.guesses = {"STARE"};

    // Create 3-game win streak
    for (int i = 0; i < 3; ++i) {
        manager.recordGame(gameState);
    }

    EXPECT_EQ(manager.getStats().maxStreak, 3);

    // Loss resets current but not max
    gameState.isWon = false;
    gameState.guesses = {"G1", "G2", "G3", "G4", "G5", "G6"};
    manager.recordGame(gameState);

    EXPECT_EQ(manager.getStats().currentStreak, 0);
    EXPECT_EQ(manager.getStats().maxStreak, 3);
}

// Test: Guess distribution tracking
TEST_F(StatisticsManagerTest, GuessDistributionTracking) {
    StatisticsManager manager;

    GameState game1;
    game1.isWon = true;
    game1.guesses = {"STARE"};  // 1 guess
    manager.recordGame(game1);

    GameState game2;
    game2.isWon = true;
    game2.guesses = {"STARE", "PLANT"};  // 2 guesses
    manager.recordGame(game2);

    const auto& stats = manager.getStats();
    EXPECT_EQ(stats.guessDistribution[0], 1);  // 1 guess
    EXPECT_EQ(stats.guessDistribution[1], 1);  // 2 guesses
}

// Test: Share emoji generation
TEST_F(StatisticsManagerTest, GenerateShareText) {
    StatisticsManager manager;

    GameState gameState;
    gameState.isWon = true;
    gameState.feedbacks.push_back({
        LetterFeedback('A', LetterStatus::GREEN, 0),
        LetterFeedback('B', LetterStatus::YELLOW, 1),
        LetterFeedback('C', LetterStatus::GRAY, 2),
        LetterFeedback('D', LetterStatus::GREEN, 3),
        LetterFeedback('E', LetterStatus::GREEN, 4)
    });

    std::string shareText = manager.generateShareText(gameState);
    EXPECT_FALSE(shareText.empty());
    // Should contain emoji symbols
    EXPECT_TRUE(shareText.find("🟩") != std::string::npos ||
                shareText.find("🟨") != std::string::npos);
}

// Test: Reset statistics
TEST_F(StatisticsManagerTest, ResetStatistics) {
    StatisticsManager manager;
    auto& stats = manager.getStatsMutable();

    stats.gamesPlayed = 42;
    stats.gamesWon = 31;
    stats.currentStreak = 5;

    manager.reset();

    EXPECT_EQ(manager.getStats().gamesPlayed, 0);
    EXPECT_EQ(manager.getStats().gamesWon, 0);
    EXPECT_EQ(manager.getStats().currentStreak, 0);
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
