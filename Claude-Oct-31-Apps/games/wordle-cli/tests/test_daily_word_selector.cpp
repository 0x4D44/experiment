#include <gtest/gtest.h>
#include "daily_word_selector.h"
#include "word_list.h"
#include <fstream>
#include <filesystem>

namespace fs = std::filesystem;

class DailyWordSelectorTest : public ::testing::Test {
protected:
    DailyWordSelectorTest() = default;
    ~DailyWordSelectorTest() override = default;

    void SetUp() override {
        // Create temporary test word files
        testDir = fs::temp_directory_path() / "wordle_tests";
        fs::create_directories(testDir);

        validWordsPath = testDir / "valid_words.txt";
        answerWordsPath = testDir / "answer_words.txt";

        // Write test answer words
        std::ofstream answerFile(answerWordsPath);
        for (int i = 0; i < 100; ++i) {
            answerFile << "WORD" << (i < 10 ? "0" : "") << i << "\n";
        }
        answerFile.close();

        std::ofstream validFile(validWordsPath);
        validFile << "DUMMY\n";
        validFile.close();

        wordList.loadFromFiles(validWordsPath.string(), answerWordsPath.string());
    }

    void TearDown() override {
        if (fs::exists(testDir)) {
            fs::remove_all(testDir);
        }
    }

    fs::path testDir;
    fs::path validWordsPath;
    fs::path answerWordsPath;
    WordList wordList;
};

// Test: Daily word is consistent
TEST_F(DailyWordSelectorTest, DailyWordConsistency) {
    std::string word1 = DailyWordSelector::getDailyWord(wordList);
    std::string word2 = DailyWordSelector::getDailyWord(wordList);

    EXPECT_EQ(word1, word2);
    EXPECT_EQ(word1.length(), 5);
}

// Test: Different dates give different seeds
TEST_F(DailyWordSelectorTest, DifferentDatesDifferentSeeds) {
    auto today = DailyWordSelector::getToday();
    auto tomorrow = today + std::chrono::hours(24);

    uint32_t seedToday = DailyWordSelector::getDaySeed();
    uint32_t seedTomorrow = DailyWordSelector::getSeedForDate(tomorrow);

    EXPECT_NE(seedToday, seedTomorrow);
}

// Test: Seed format is correct
TEST_F(DailyWordSelectorTest, SeedIsValid) {
    uint32_t seed = DailyWordSelector::getDaySeed();
    EXPECT_GT(seed, 0);
}

// Test: Get word for specific date
TEST_F(DailyWordSelectorTest, GetWordForDate) {
    auto today = DailyWordSelector::getToday();
    std::string word = DailyWordSelector::getWordForDate(wordList, today);

    EXPECT_EQ(word.length(), 5);
    EXPECT_TRUE(wordList.isAnswerWord(word));
}

// Test: Same date returns same word
TEST_F(DailyWordSelectorTest, SameDateReturnsSameWord) {
    auto today = DailyWordSelector::getToday();
    std::string word1 = DailyWordSelector::getWordForDate(wordList, today);
    std::string word2 = DailyWordSelector::getWordForDate(wordList, today);

    EXPECT_EQ(word1, word2);
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
