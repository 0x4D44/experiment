#include <gtest/gtest.h>
#include "input_validator.h"

class InputValidatorTest : public ::testing::Test {
protected:
    InputValidatorTest() = default;
    ~InputValidatorTest() override = default;
};

// Test: Valid format
TEST_F(InputValidatorTest, ValidFormat) {
    EXPECT_TRUE(InputValidator::isValidGuessFormat("STARE"));
    EXPECT_TRUE(InputValidator::isValidGuessFormat("stare"));
    EXPECT_TRUE(InputValidator::isValidGuessFormat("Plant"));
}

// Test: Invalid - wrong length
TEST_F(InputValidatorTest, InvalidLength) {
    EXPECT_FALSE(InputValidator::isValidGuessFormat("TEST"));    // 4 letters
    EXPECT_FALSE(InputValidator::isValidGuessFormat("PLANTS"));  // 6 letters
    EXPECT_FALSE(InputValidator::isValidGuessFormat(""));        // empty
}

// Test: Invalid - contains non-alphabetic
TEST_F(InputValidatorTest, InvalidNonAlphabetic) {
    EXPECT_FALSE(InputValidator::isValidGuessFormat("TEST1"));
    EXPECT_FALSE(InputValidator::isValidGuessFormat("TEST!"));
    EXPECT_FALSE(InputValidator::isValidGuessFormat("TE ST"));
    EXPECT_FALSE(InputValidator::isValidGuessFormat("TEST-"));
}

// Test: Sanitize input
TEST_F(InputValidatorTest, Sanitize) {
    EXPECT_EQ(InputValidator::sanitize("stare"), "STARE");
    EXPECT_EQ(InputValidator::sanitize("  PLANT  "), "PLANT");
    EXPECT_EQ(InputValidator::sanitize("  test  "), "TEST");
    EXPECT_EQ(InputValidator::sanitize("PlAnT"), "PLANT");
}

// Test: Check if alphabetic
TEST_F(InputValidatorTest, IsAlphabetic) {
    EXPECT_TRUE(InputValidator::isAlphabetic("ABCDE"));
    EXPECT_TRUE(InputValidator::isAlphabetic("abcde"));
    EXPECT_TRUE(InputValidator::isAlphabetic("StArE"));
    EXPECT_FALSE(InputValidator::isAlphabetic("ABC1E"));
    EXPECT_FALSE(InputValidator::isAlphabetic("ABC E"));
    EXPECT_FALSE(InputValidator::isAlphabetic("ABC!"));
}

// Test: Convert to uppercase
TEST_F(InputValidatorTest, ToUpperCase) {
    EXPECT_EQ(InputValidator::toUpperCase("stare"), "STARE");
    EXPECT_EQ(InputValidator::toUpperCase("STARE"), "STARE");
    EXPECT_EQ(InputValidator::toUpperCase("StArE"), "STARE");
    EXPECT_EQ(InputValidator::toUpperCase("a"), "A");
    EXPECT_EQ(InputValidator::toUpperCase("Z"), "Z");
}

// Test: Correct length check
TEST_F(InputValidatorTest, IsCorrectLength) {
    EXPECT_TRUE(InputValidator::isCorrectLength("STARE"));
    EXPECT_TRUE(InputValidator::isCorrectLength("plant"));
    EXPECT_FALSE(InputValidator::isCorrectLength("TEST"));
    EXPECT_FALSE(InputValidator::isCorrectLength("PLANTS"));
    EXPECT_FALSE(InputValidator::isCorrectLength(""));
}

// Test: Edge case - spaces around
TEST_F(InputValidatorTest, EdgeCaseSpaces) {
    EXPECT_TRUE(InputValidator::isValidGuessFormat("  STARE  "));
    EXPECT_EQ(InputValidator::sanitize("  STARE  "), "STARE");
}

// Test: Edge case - mixed case with spaces
TEST_F(InputValidatorTest, EdgeCaseMixedCaseSpaces) {
    EXPECT_TRUE(InputValidator::isValidGuessFormat("  sTaRe  "));
    EXPECT_EQ(InputValidator::sanitize("  sTaRe  "), "STARE");
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
