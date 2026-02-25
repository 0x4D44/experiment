#include <gtest/gtest.h>
#include "word_list.h"
#include <fstream>
#include <filesystem>

namespace fs = std::filesystem;

class WordListTest : public ::testing::Test {
protected:
    WordListTest() = default;
    ~WordListTest() override = default;

    void SetUp() override {
        // Create temporary test word files
        testDir = fs::temp_directory_path() / "wordle_tests";
        fs::create_directories(testDir);

        validWordsPath = testDir / "valid_words.txt";
        answerWordsPath = testDir / "answer_words.txt";

        // Write test valid words
        std::ofstream validFile(validWordsPath);
        validFile << "STARE\n";
        validFile << "PLANT\n";
        validFile << "ABOUT\n";
        validFile << "SPEED\n";
        validFile << "STEAL\n";
        validFile.close();

        // Write test answer words
        std::ofstream answerFile(answerWordsPath);
        answerFile << "PLANT\n";
        answerFile << "STARE\n";
        answerFile << "STEAL\n";
        answerFile.close();
    }

    void TearDown() override {
        if (fs::exists(testDir)) {
            fs::remove_all(testDir);
        }
    }

    fs::path testDir;
    fs::path validWordsPath;
    fs::path answerWordsPath;
};

// Test: Load word lists successfully
TEST_F(WordListTest, LoadWordsSuccessfully) {
    WordList wordList;
    wordList.loadFromFiles(validWordsPath.string(), answerWordsPath.string());

    EXPECT_EQ(wordList.getValidCount(), 5);
    EXPECT_EQ(wordList.getAnswerCount(), 3);
}

// Test: Check if word is valid
TEST_F(WordListTest, IsValidWord) {
    WordList wordList;
    wordList.loadFromFiles(validWordsPath.string(), answerWordsPath.string());

    EXPECT_TRUE(wordList.isValidWord("STARE"));
    EXPECT_TRUE(wordList.isValidWord("PLANT"));
    EXPECT_TRUE(wordList.isValidWord("ABOUT"));
    EXPECT_FALSE(wordList.isValidWord("INVALID"));
    EXPECT_FALSE(wordList.isValidWord("NOTHERE"));
}

// Test: Check if word is answer word
TEST_F(WordListTest, IsAnswerWord) {
    WordList wordList;
    wordList.loadFromFiles(validWordsPath.string(), answerWordsPath.string());

    EXPECT_TRUE(wordList.isAnswerWord("PLANT"));
    EXPECT_TRUE(wordList.isAnswerWord("STARE"));
    EXPECT_TRUE(wordList.isAnswerWord("STEAL"));
    EXPECT_FALSE(wordList.isAnswerWord("SPEED"));  // Valid but not answer
    EXPECT_FALSE(wordList.isAnswerWord("NOTHERE"));
}

// Test: Case insensitivity
TEST_F(WordListTest, CaseInsensitivity) {
    WordList wordList;
    wordList.loadFromFiles(validWordsPath.string(), answerWordsPath.string());

    EXPECT_TRUE(wordList.isValidWord("stare"));
    EXPECT_TRUE(wordList.isValidWord("STARE"));
    EXPECT_TRUE(wordList.isValidWord("StArE"));
    EXPECT_TRUE(wordList.isAnswerWord("plant"));
    EXPECT_TRUE(wordList.isAnswerWord("PLANT"));
}

// Test: Get answer word at index
TEST_F(WordListTest, GetAnswerWordAtIndex) {
    WordList wordList;
    wordList.loadFromFiles(validWordsPath.string(), answerWordsPath.string());

    // Should not crash with valid indices
    std::string word0 = wordList.getAnswerWord(0);
    std::string word1 = wordList.getAnswerWord(1);
    std::string word2 = wordList.getAnswerWord(2);

    EXPECT_FALSE(word0.empty());
    EXPECT_FALSE(word1.empty());
    EXPECT_FALSE(word2.empty());
    EXPECT_EQ(word0.length(), 5);
    EXPECT_EQ(word1.length(), 5);
}

// Test: Get random answer word
TEST_F(WordListTest, GetRandomAnswerWord) {
    WordList wordList;
    wordList.loadFromFiles(validWordsPath.string(), answerWordsPath.string());

    std::string randomWord = wordList.getRandomAnswerWord();
    EXPECT_FALSE(randomWord.empty());
    EXPECT_EQ(randomWord.length(), 5);
    EXPECT_TRUE(wordList.isAnswerWord(randomWord));
}

// Test: Clear words
TEST_F(WordListTest, ClearWords) {
    WordList wordList;
    wordList.loadFromFiles(validWordsPath.string(), answerWordsPath.string());

    EXPECT_GT(wordList.getValidCount(), 0);
    EXPECT_GT(wordList.getAnswerCount(), 0);

    wordList.clear();

    EXPECT_EQ(wordList.getValidCount(), 0);
    EXPECT_EQ(wordList.getAnswerCount(), 0);
}

// Test: Multiple loads (should append)
TEST_F(WordListTest, MultipleLoads) {
    WordList wordList;
    wordList.loadFromFiles(validWordsPath.string(), answerWordsPath.string());

    size_t firstLoadCount = wordList.getValidCount();

    // Load again (implementation might accumulate or replace)
    wordList.loadFromFiles(validWordsPath.string(), answerWordsPath.string());

    // Should handle multiple loads gracefully
    EXPECT_GT(wordList.getValidCount(), 0);
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
