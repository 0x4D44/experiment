#include <gtest/gtest.h>
#include "guess_evaluator.h"

class GuessEvaluatorTest : public ::testing::Test {
protected:
    GuessEvaluatorTest() = default;
    ~GuessEvaluatorTest() override = default;
};

// Test: Simple case - all correct letters in correct positions
TEST_F(GuessEvaluatorTest, AllGreen) {
    GuessEvaluator evaluator("PLANT");
    auto feedback = evaluator.evaluate("PLANT");

    EXPECT_EQ(feedback.size(), 5);
    EXPECT_EQ(feedback[0].letter, 'P');
    EXPECT_EQ(feedback[0].status, LetterStatus::GREEN);
    EXPECT_EQ(feedback[1].letter, 'L');
    EXPECT_EQ(feedback[1].status, LetterStatus::GREEN);
    EXPECT_EQ(feedback[2].letter, 'A');
    EXPECT_EQ(feedback[2].status, LetterStatus::GREEN);
    EXPECT_EQ(feedback[3].letter, 'N');
    EXPECT_EQ(feedback[3].status, LetterStatus::GREEN);
    EXPECT_EQ(feedback[4].letter, 'T');
    EXPECT_EQ(feedback[4].status, LetterStatus::GREEN);
}

// Test: No correct letters
TEST_F(GuessEvaluatorTest, AllGray) {
    GuessEvaluator evaluator("PLANT");
    auto feedback = evaluator.evaluate("SWORD");

    EXPECT_EQ(feedback.size(), 5);
    for (const auto& letter : feedback) {
        EXPECT_EQ(letter.status, LetterStatus::GRAY);
    }
}

// Test: Letters in word but wrong positions
TEST_F(GuessEvaluatorTest, AllYellow) {
    GuessEvaluator evaluator("STARE");
    auto feedback = evaluator.evaluate("EARTS");

    EXPECT_EQ(feedback.size(), 5);
    for (const auto& letter : feedback) {
        EXPECT_EQ(letter.status, LetterStatus::YELLOW);
    }
}

// Test: Mixed feedback
TEST_F(GuessEvaluatorTest, MixedFeedback) {
    GuessEvaluator evaluator("ROBOT");
    auto feedback = evaluator.evaluate("FLOOR");

    EXPECT_EQ(feedback.size(), 5);
    EXPECT_EQ(feedback[0].letter, 'F');
    EXPECT_EQ(feedback[0].status, LetterStatus::GRAY);

    EXPECT_EQ(feedback[1].letter, 'L');
    EXPECT_EQ(feedback[1].status, LetterStatus::GRAY);

    EXPECT_EQ(feedback[2].letter, 'O');
    EXPECT_EQ(feedback[2].status, LetterStatus::YELLOW);

    EXPECT_EQ(feedback[3].letter, 'O');
    EXPECT_EQ(feedback[3].status, LetterStatus::YELLOW);

    EXPECT_EQ(feedback[4].letter, 'R');
    EXPECT_EQ(feedback[4].status, LetterStatus::GREEN);
}

// Test: Repeated letters - exact match takes precedence
TEST_F(GuessEvaluatorTest, RepeatedLettersExactMatch) {
    GuessEvaluator evaluator("SPEED");
    auto feedback = evaluator.evaluate("ERASE");

    EXPECT_EQ(feedback.size(), 5);
    // E at position 0 should be YELLOW (in word but wrong position)
    EXPECT_EQ(feedback[0].letter, 'E');
    EXPECT_EQ(feedback[0].status, LetterStatus::YELLOW);

    // R is GRAY
    EXPECT_EQ(feedback[1].letter, 'R');
    EXPECT_EQ(feedback[1].status, LetterStatus::GRAY);

    // A is GRAY
    EXPECT_EQ(feedback[2].letter, 'A');
    EXPECT_EQ(feedback[2].status, LetterStatus::GRAY);

    // S is YELLOW
    EXPECT_EQ(feedback[3].letter, 'S');
    EXPECT_EQ(feedback[3].status, LetterStatus::YELLOW);

    // E at position 4 should be GREEN
    EXPECT_EQ(feedback[4].letter, 'E');
    EXPECT_EQ(feedback[4].status, LetterStatus::GREEN);
}

// Test: Repeated letters in guess, only one in answer
TEST_F(GuessEvaluatorTest, RepeatedGuessLettersSingleAnswer) {
    GuessEvaluator evaluator("ROBOT");
    auto feedback = evaluator.evaluate("OOMPH");

    EXPECT_EQ(feedback.size(), 5);
    // O at position 0 should be YELLOW (one O in ROBOT at position 1)
    EXPECT_EQ(feedback[0].letter, 'O');
    EXPECT_EQ(feedback[0].status, LetterStatus::YELLOW);

    // O at position 1 should be GREEN
    EXPECT_EQ(feedback[1].letter, 'O');
    EXPECT_EQ(feedback[1].status, LetterStatus::GREEN);

    // M is GRAY
    EXPECT_EQ(feedback[2].letter, 'M');
    EXPECT_EQ(feedback[2].status, LetterStatus::GRAY);

    // P is GRAY
    EXPECT_EQ(feedback[3].letter, 'P');
    EXPECT_EQ(feedback[3].status, LetterStatus::GRAY);

    // H is GRAY
    EXPECT_EQ(feedback[4].letter, 'H');
    EXPECT_EQ(feedback[4].status, LetterStatus::GRAY);
}

// Test: Repeated letters - complex case
TEST_F(GuessEvaluatorTest, RepeatedLettersComplex) {
    GuessEvaluator evaluator("STEAL");
    auto feedback = evaluator.evaluate("SLEEK");

    EXPECT_EQ(feedback.size(), 5);
    // S at position 0 should be YELLOW
    EXPECT_EQ(feedback[0].letter, 'S');
    EXPECT_EQ(feedback[0].status, LetterStatus::YELLOW);

    // L at position 1 should be YELLOW
    EXPECT_EQ(feedback[1].letter, 'L');
    EXPECT_EQ(feedback[1].status, LetterStatus::YELLOW);

    // E at position 2 should be GREEN
    EXPECT_EQ(feedback[2].letter, 'E');
    EXPECT_EQ(feedback[2].status, LetterStatus::GREEN);

    // E at position 3 should be YELLOW (second E, matches position 4 of STEAL)
    EXPECT_EQ(feedback[3].letter, 'E');
    EXPECT_EQ(feedback[3].status, LetterStatus::YELLOW);

    // K is GRAY
    EXPECT_EQ(feedback[4].letter, 'K');
    EXPECT_EQ(feedback[4].status, LetterStatus::GRAY);
}

// Test: Single letter word contains multiple instances
TEST_F(GuessEvaluatorTest, SingleLetterMultipleInstances) {
    GuessEvaluator evaluator("SISSY");
    auto feedback = evaluator.evaluate("SASSY");

    EXPECT_EQ(feedback.size(), 5);
    // S at position 0 is GREEN
    EXPECT_EQ(feedback[0].status, LetterStatus::GREEN);

    // A at position 1 is GRAY
    EXPECT_EQ(feedback[1].status, LetterStatus::GRAY);

    // S at position 2 is YELLOW (another S exists at position 2 in SISSY, but this is position 2)
    // Actually position 2 in SISSY is I, so this S is at wrong position
    EXPECT_EQ(feedback[2].status, LetterStatus::YELLOW);

    // S at position 3 is YELLOW
    EXPECT_EQ(feedback[3].status, LetterStatus::YELLOW);

    // Y at position 4 is GREEN
    EXPECT_EQ(feedback[4].status, LetterStatus::GREEN);
}

// Test: Helper function - hasLetter
TEST_F(GuessEvaluatorTest, HasLetter) {
    GuessEvaluator evaluator("STARE");

    EXPECT_TRUE(evaluator.hasLetter('S'));
    EXPECT_TRUE(evaluator.hasLetter('T'));
    EXPECT_TRUE(evaluator.hasLetter('A'));
    EXPECT_TRUE(evaluator.hasLetter('R'));
    EXPECT_TRUE(evaluator.hasLetter('E'));
    EXPECT_FALSE(evaluator.hasLetter('X'));
    EXPECT_FALSE(evaluator.hasLetter('Z'));
}

// Test: Helper function - getLetterCount
TEST_F(GuessEvaluatorTest, GetLetterCount) {
    GuessEvaluator evaluator("SPEED");

    EXPECT_EQ(evaluator.getLetterCount('S'), 1);
    EXPECT_EQ(evaluator.getLetterCount('P'), 1);
    EXPECT_EQ(evaluator.getLetterCount('E'), 2);
    EXPECT_EQ(evaluator.getLetterCount('D'), 1);
    EXPECT_EQ(evaluator.getLetterCount('X'), 0);
}

// Test: Case insensitivity
TEST_F(GuessEvaluatorTest, CaseInsensitivity) {
    GuessEvaluator evaluator("PLANT");

    // Should work with lowercase
    auto feedback = evaluator.evaluate("plant");
    EXPECT_EQ(feedback.size(), 5);
    for (const auto& letter : feedback) {
        EXPECT_EQ(letter.status, LetterStatus::GREEN);
    }
}

// Test: Answer getter
TEST_F(GuessEvaluatorTest, AnswerGetter) {
    GuessEvaluator evaluator("HELLO");
    EXPECT_EQ(evaluator.getAnswerWord(), "HELLO");
}

// Test: Two of same letter in guess, one in answer, one exact match
TEST_F(GuessEvaluatorTest, RepeatedLetterWithExactMatch) {
    GuessEvaluator evaluator("FLOOR");
    auto feedback = evaluator.evaluate("LLAMA");

    EXPECT_EQ(feedback.size(), 5);
    // L at position 0 should be YELLOW
    EXPECT_EQ(feedback[0].letter, 'L');
    EXPECT_EQ(feedback[0].status, LetterStatus::YELLOW);

    // L at position 1 should be GREEN
    EXPECT_EQ(feedback[1].letter, 'L');
    EXPECT_EQ(feedback[1].status, LetterStatus::GREEN);
}

// Test: Common Wordle scenario with double letters
TEST_F(GuessEvaluatorTest, CommonDoubleLetterScenario) {
    GuessEvaluator evaluator("ABBEY");
    auto feedback = evaluator.evaluate("LABEL");

    EXPECT_EQ(feedback.size(), 5);
    // L at position 0 is GRAY
    EXPECT_EQ(feedback[0].letter, 'L');
    EXPECT_EQ(feedback[0].status, LetterStatus::GRAY);

    // A at position 1 is GREEN
    EXPECT_EQ(feedback[1].letter, 'A');
    EXPECT_EQ(feedback[1].status, LetterStatus::GREEN);

    // B at position 2 is YELLOW
    EXPECT_EQ(feedback[2].letter, 'B');
    EXPECT_EQ(feedback[2].status, LetterStatus::YELLOW);

    // E at position 3 is YELLOW
    EXPECT_EQ(feedback[3].letter, 'E');
    EXPECT_EQ(feedback[3].status, LetterStatus::YELLOW);

    // L at position 4 is GRAY
    EXPECT_EQ(feedback[4].letter, 'L');
    EXPECT_EQ(feedback[4].status, LetterStatus::GRAY);
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
