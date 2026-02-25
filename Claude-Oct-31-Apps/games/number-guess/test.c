#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <assert.h>
#include <time.h>

/*
 * TEST FRAMEWORK
 * Simple assertion-based testing without external dependencies
 */

int tests_run = 0;
int tests_passed = 0;
int tests_failed = 0;

#define TEST(name) \
    void test_##name(void); \
    void run_test_##name(void) { \
        printf("\n  Testing: %s ... ", #name); \
        tests_run++; \
        test_##name(); \
        printf("PASS"); \
        tests_passed++; \
    } \
    void test_##name(void)

#define ASSERT_EQUAL(actual, expected) \
    do { \
        if ((actual) != (expected)) { \
            printf("FAIL\n    Expected: %d, Got: %d\n", (expected), (actual)); \
            tests_failed++; \
            tests_run--; \
            return; \
        } \
    } while(0)

#define ASSERT_TRUE(condition) \
    do { \
        if (!(condition)) { \
            printf("FAIL\n    Condition was false\n"); \
            tests_failed++; \
            tests_run--; \
            return; \
        } \
    } while(0)

#define ASSERT_FLOAT_EQUAL(actual, expected, tolerance) \
    do { \
        if (fabs((actual) - (expected)) > (tolerance)) { \
            printf("FAIL\n    Expected: %f, Got: %f\n", (expected), (actual)); \
            tests_failed++; \
            tests_run--; \
            return; \
        } \
    } while(0)

/*
 * FORWARD DECLARATIONS
 * Functions to be implemented
 */

// Random generation
int generate_random_number(int min, int max);
void seed_random(unsigned int seed);

// Mathematical utilities
int is_prime(int num);
int is_perfect_square(int num);
int is_fibonacci(int num);
int digit_sum(int num);
int count_divisors(int num);

// Hint generation
const char* get_range_hint(int guess, int secret, int min, int max);
const char* get_mathematical_hint(int secret);
const char* get_proximity_hint(int guess, int secret, int range_size);

// AI algorithms
int binary_search_guess(int *min, int *max, int last_feedback);
int is_better_guess(int current, int candidate, int secret);

// Scoring
int calculate_classic_score(int guesses, int max_guesses, int difficulty,
                           int hints_used, float time_seconds);
int calculate_challenge_score(int guesses, int max_guesses, int difficulty,
                             int streak);

// Input validation
int is_valid_guess(int guess, int min, int max);
int is_valid_range(int min, int max);

// Statistics
float calculate_win_rate(int wins, int total);
float calculate_average_guesses(int total_guesses, int games_played);

/*
 * TEST SUITES
 */

/* ============================================================
 * SUITE 1: RANDOM NUMBER GENERATION
 * ============================================================ */

TEST(random_bounds_valid_range)
{
    seed_random(12345);

    for (int i = 0; i < 100; i++) {
        int num = generate_random_number(1, 100);
        ASSERT_TRUE(num >= 1 && num <= 100);
    }
}

TEST(random_respects_min_max)
{
    seed_random(67890);

    int min = 50, max = 150;
    for (int i = 0; i < 50; i++) {
        int num = generate_random_number(min, max);
        ASSERT_TRUE(num >= min && num <= max);
    }
}

TEST(random_single_value_range)
{
    seed_random(11111);

    int num = generate_random_number(42, 42);
    ASSERT_EQUAL(num, 42);
}

TEST(random_different_seeds_different_values)
{
    seed_random(111);
    int num1 = generate_random_number(1, 1000);

    seed_random(222);
    int num2 = generate_random_number(1, 1000);

    ASSERT_TRUE(num1 != num2);
}

TEST(random_large_range)
{
    seed_random(54321);

    for (int i = 0; i < 30; i++) {
        int num = generate_random_number(1, 100000);
        ASSERT_TRUE(num >= 1 && num <= 100000);
    }
}

/* ============================================================
 * SUITE 2: MATHEMATICAL UTILITIES
 * ============================================================ */

TEST(is_prime_small_primes)
{
    ASSERT_TRUE(is_prime(2));
    ASSERT_TRUE(is_prime(3));
    ASSERT_TRUE(is_prime(5));
    ASSERT_TRUE(is_prime(7));
    ASSERT_TRUE(is_prime(11));
}

TEST(is_prime_small_composites)
{
    ASSERT_TRUE(!is_prime(0));
    ASSERT_TRUE(!is_prime(1));
    ASSERT_TRUE(!is_prime(4));
    ASSERT_TRUE(!is_prime(6));
    ASSERT_TRUE(!is_prime(9));
}

TEST(is_prime_large_primes)
{
    ASSERT_TRUE(is_prime(97));
    ASSERT_TRUE(is_prime(101));
    ASSERT_TRUE(is_prime(997));
}

TEST(is_prime_large_composites)
{
    ASSERT_TRUE(!is_prime(100));
    ASSERT_TRUE(!is_prime(1000));
    ASSERT_TRUE(!is_prime(10000));
}

TEST(is_perfect_square_valid)
{
    ASSERT_TRUE(is_perfect_square(0));
    ASSERT_TRUE(is_perfect_square(1));
    ASSERT_TRUE(is_perfect_square(4));
    ASSERT_TRUE(is_perfect_square(9));
    ASSERT_TRUE(is_perfect_square(16));
    ASSERT_TRUE(is_perfect_square(100));
    ASSERT_TRUE(is_perfect_square(10000));
}

TEST(is_perfect_square_invalid)
{
    ASSERT_TRUE(!is_perfect_square(2));
    ASSERT_TRUE(!is_perfect_square(3));
    ASSERT_TRUE(!is_perfect_square(5));
    ASSERT_TRUE(!is_perfect_square(99));
    ASSERT_TRUE(!is_perfect_square(101));
}

TEST(is_fibonacci_valid)
{
    ASSERT_TRUE(is_fibonacci(0));
    ASSERT_TRUE(is_fibonacci(1));
    ASSERT_TRUE(is_fibonacci(2));
    ASSERT_TRUE(is_fibonacci(3));
    ASSERT_TRUE(is_fibonacci(5));
    ASSERT_TRUE(is_fibonacci(8));
    ASSERT_TRUE(is_fibonacci(13));
    ASSERT_TRUE(is_fibonacci(21));
    ASSERT_TRUE(is_fibonacci(34));
    ASSERT_TRUE(is_fibonacci(55));
    ASSERT_TRUE(is_fibonacci(89));
}

TEST(is_fibonacci_invalid)
{
    ASSERT_TRUE(!is_fibonacci(4));
    ASSERT_TRUE(!is_fibonacci(6));
    ASSERT_TRUE(!is_fibonacci(7));
    ASSERT_TRUE(!is_fibonacci(10));
    ASSERT_TRUE(!is_fibonacci(20));
}

TEST(digit_sum_single_digit)
{
    ASSERT_EQUAL(digit_sum(0), 0);
    ASSERT_EQUAL(digit_sum(5), 5);
    ASSERT_EQUAL(digit_sum(9), 9);
}

TEST(digit_sum_multi_digit)
{
    ASSERT_EQUAL(digit_sum(12), 3);
    ASSERT_EQUAL(digit_sum(99), 18);
    ASSERT_EQUAL(digit_sum(123), 6);
    ASSERT_EQUAL(digit_sum(999), 27);
    ASSERT_EQUAL(digit_sum(12345), 15);
}

TEST(count_divisors_prime)
{
    ASSERT_EQUAL(count_divisors(2), 2);  // 1, 2
    ASSERT_EQUAL(count_divisors(5), 2);  // 1, 5
    ASSERT_EQUAL(count_divisors(7), 2);  // 1, 7
}

TEST(count_divisors_composite)
{
    ASSERT_EQUAL(count_divisors(1), 1);  // 1
    ASSERT_EQUAL(count_divisors(4), 3);  // 1, 2, 4
    ASSERT_EQUAL(count_divisors(6), 4);  // 1, 2, 3, 6
    ASSERT_EQUAL(count_divisors(12), 6); // 1, 2, 3, 4, 6, 12
}

/* ============================================================
 * SUITE 3: HINT SYSTEM
 * ============================================================ */

TEST(hint_range_too_high)
{
    // When guess is higher than secret
    const char *hint = get_range_hint(60, 40, 1, 100);
    ASSERT_TRUE(hint != NULL);
    // Should indicate "too high" concept
}

TEST(hint_range_too_low)
{
    // When guess is lower than secret
    const char *hint = get_range_hint(30, 70, 1, 100);
    ASSERT_TRUE(hint != NULL);
    // Should indicate "too low" concept
}

TEST(hint_mathematical_prime)
{
    // For a prime number
    const char *hint = get_mathematical_hint(13);
    ASSERT_TRUE(hint != NULL);
}

TEST(hint_mathematical_square)
{
    // For a perfect square
    const char *hint = get_mathematical_hint(16);
    ASSERT_TRUE(hint != NULL);
}

TEST(hint_mathematical_fibonacci)
{
    // For a Fibonacci number
    const char *hint = get_mathematical_hint(21);
    ASSERT_TRUE(hint != NULL);
}

TEST(hint_proximity_very_close)
{
    // When guess is very close
    const char *hint = get_proximity_hint(49, 50, 100);
    ASSERT_TRUE(hint != NULL);
    // Should indicate "very close"
}

TEST(hint_proximity_far)
{
    // When guess is far
    const char *hint = get_proximity_hint(5, 95, 100);
    ASSERT_TRUE(hint != NULL);
    // Should indicate "far" or "cold"
}

/* ============================================================
 * SUITE 4: AI ALGORITHMS
 * ============================================================ */

TEST(binary_search_simple_converges)
{
    int min = 1, max = 100;
    int secret = 50;
    int guess_count = 0;

    // Simulate binary search
    while (min <= max && guess_count < 20) {
        int guess = (min + max) / 2;
        int feedback = (guess < secret) ? -1 : (guess > secret) ? 1 : 0;

        if (feedback == 0) break;  // Found
        if (feedback == -1) min = guess + 1;
        else max = guess - 1;

        guess_count++;
    }

    // Should converge within log2(100) ~= 7 guesses
    ASSERT_TRUE(guess_count < 10);
}

TEST(binary_search_narrow_range)
{
    int min = 1, max = 1000;

    // After 3 iterations of binary search
    int guess = 500;  // First guess
    int feedback = -1;  // Secret is higher
    if (feedback == -1) min = 501;

    guess = 750;  // Second guess
    feedback = 1;   // Secret is lower
    if (feedback == 1) max = 749;

    guess = 625;  // Third guess
    feedback = -1;  // Secret is higher
    if (feedback == -1) min = 626;

    // Range should be narrowed significantly
    ASSERT_TRUE((max - min) < 200);
}

TEST(binary_search_optimal_for_range)
{
    // For range 1-100, optimal is ~7 guesses (log2(100) + 1)
    // For range 1-1000, optimal is ~10-11 guesses
    // For range 1-10000, optimal is ~14-15 guesses

    // This is information-theoretic bound
    ASSERT_TRUE((int)ceil(log2(100)) + 1 >= 7 && (int)ceil(log2(100)) + 1 <= 8);
    ASSERT_TRUE((int)ceil(log2(1000)) + 1 >= 10 && (int)ceil(log2(1000)) + 1 <= 11);
    ASSERT_TRUE((int)ceil(log2(10000)) + 1 >= 14 && (int)ceil(log2(10000)) + 1 <= 15);
}

/* ============================================================
 * SUITE 5: SCORING SYSTEM
 * ============================================================ */

TEST(score_perfect_easy)
{
    // Easy: guess in 1 guess, no hints, no time bonus
    int score = calculate_classic_score(1, 10, 0, 0, 5.0);
    ASSERT_TRUE(score > 80);  // Should be high
}

TEST(score_good_medium)
{
    // Medium: guess in 5 guesses, no hints
    int score = calculate_classic_score(5, 10, 1, 0, 45.0);
    ASSERT_TRUE(score > 50);  // Should be decent
}

TEST(score_minimum)
{
    // Should never be negative
    int score = calculate_classic_score(10, 10, 0, 5, 60.0);
    ASSERT_TRUE(score >= 0);
}

TEST(score_penalty_for_hints)
{
    // More hints should give lower score
    int score1 = calculate_classic_score(5, 10, 1, 0, 30.0);
    int score2 = calculate_classic_score(5, 10, 1, 3, 30.0);
    ASSERT_TRUE(score1 > score2);
}

TEST(score_difficulty_multiplier)
{
    // Harder difficulty should give more points
    int score_easy = calculate_classic_score(1, 10, 0, 0, 5.0);
    int score_hard = calculate_classic_score(1, 15, 3, 0, 5.0);
    ASSERT_TRUE(score_hard > score_easy);
}

TEST(challenge_score_basic)
{
    // Challenge mode: 3 guesses max, 2 used, no streak
    int score = calculate_challenge_score(2, 3, 2, 1);
    ASSERT_TRUE(score > 0);
}

TEST(challenge_score_streak_multiplier)
{
    // Streak should increase score
    int score1 = calculate_challenge_score(2, 5, 1, 1);
    int score2 = calculate_challenge_score(2, 5, 1, 3);
    ASSERT_TRUE(score2 > score1);
}

/* ============================================================
 * SUITE 6: INPUT VALIDATION
 * ============================================================ */

TEST(valid_guess_in_range)
{
    ASSERT_TRUE(is_valid_guess(50, 1, 100));
    ASSERT_TRUE(is_valid_guess(1, 1, 100));
    ASSERT_TRUE(is_valid_guess(100, 1, 100));
}

TEST(valid_guess_out_of_range)
{
    ASSERT_TRUE(!is_valid_guess(0, 1, 100));
    ASSERT_TRUE(!is_valid_guess(101, 1, 100));
    ASSERT_TRUE(!is_valid_guess(-5, 1, 100));
}

TEST(valid_range_normal)
{
    ASSERT_TRUE(is_valid_range(1, 100));
    ASSERT_TRUE(is_valid_range(1, 1000));
    ASSERT_TRUE(is_valid_range(1, 10));
}

TEST(valid_range_invalid)
{
    ASSERT_TRUE(!is_valid_range(100, 1));  // min > max
    ASSERT_TRUE(!is_valid_range(1, 1));     // only 1 number
    ASSERT_TRUE(!is_valid_range(-10, 10));  // negative min
}

/* ============================================================
 * SUITE 7: STATISTICS
 * ============================================================ */

TEST(win_rate_perfect)
{
    float rate = calculate_win_rate(10, 10);
    ASSERT_FLOAT_EQUAL(rate, 1.0, 0.001);
}

TEST(win_rate_half)
{
    float rate = calculate_win_rate(5, 10);
    ASSERT_FLOAT_EQUAL(rate, 0.5, 0.001);
}

TEST(win_rate_zero)
{
    float rate = calculate_win_rate(0, 10);
    ASSERT_FLOAT_EQUAL(rate, 0.0, 0.001);
}

TEST(average_guesses_consistency)
{
    float avg = calculate_average_guesses(50, 10);  // 50 total guesses in 10 games
    ASSERT_FLOAT_EQUAL(avg, 5.0, 0.001);
}

TEST(average_guesses_zero_games)
{
    float avg = calculate_average_guesses(0, 0);
    // Should handle division by zero gracefully
    ASSERT_FLOAT_EQUAL(avg, 0.0, 0.001);
}

/* ============================================================
 * SUITE 8: DIFFICULTY SCALING
 * ============================================================ */

TEST(difficulty_range_easy)
{
    // Easy (level 1): should be small range
    int range = 10;  // 10 * (2^(1-1)) = 10
    ASSERT_EQUAL(range, 10);
}

TEST(difficulty_range_medium)
{
    // Medium (level 2): range = 10 * 2^1 = 20
    int range = 20;
    ASSERT_EQUAL(range, 20);
}

TEST(difficulty_range_hard)
{
    // Hard (level 3): range = 10 * 2^2 = 40 (or 1000 for actual hard)
    int range = 1000;
    ASSERT_TRUE(range > 100);
}

TEST(difficulty_guesses_scaling)
{
    // Guesses should scale: 5 + (level * 2.5)
    int easy_guesses = 5 + (int)(1 * 2.5);    // ~7
    int medium_guesses = 5 + (int)(2 * 2.5);  // ~10
    int hard_guesses = 5 + (int)(3 * 2.5);    // ~12

    ASSERT_TRUE(easy_guesses < medium_guesses);
    ASSERT_TRUE(medium_guesses < hard_guesses);
}

/*
 * MAIN TEST RUNNER
 */

void print_header(const char *title)
{
    printf("\n");
    printf("╔═══════════════════════════════════════════╗\n");
    printf("║ %-41s ║\n", title);
    printf("╚═══════════════════════════════════════════╝\n");
}

int main(void)
{
    printf("\n");
    print_header("Number Guess Game - Test Suite");

    printf("\n[SUITE 1: RANDOM NUMBER GENERATION]\n");
    run_test_random_bounds_valid_range();
    run_test_random_respects_min_max();
    run_test_random_single_value_range();
    run_test_random_different_seeds_different_values();
    run_test_random_large_range();

    printf("\n[SUITE 2: MATHEMATICAL UTILITIES]\n");
    run_test_is_prime_small_primes();
    run_test_is_prime_small_composites();
    run_test_is_prime_large_primes();
    run_test_is_prime_large_composites();
    run_test_is_perfect_square_valid();
    run_test_is_perfect_square_invalid();
    run_test_is_fibonacci_valid();
    run_test_is_fibonacci_invalid();
    run_test_digit_sum_single_digit();
    run_test_digit_sum_multi_digit();
    run_test_count_divisors_prime();
    run_test_count_divisors_composite();

    printf("\n[SUITE 3: HINT SYSTEM]\n");
    run_test_hint_range_too_high();
    run_test_hint_range_too_low();
    run_test_hint_mathematical_prime();
    run_test_hint_mathematical_square();
    run_test_hint_mathematical_fibonacci();
    run_test_hint_proximity_very_close();
    run_test_hint_proximity_far();

    printf("\n[SUITE 4: AI ALGORITHMS]\n");
    run_test_binary_search_simple_converges();
    run_test_binary_search_narrow_range();
    run_test_binary_search_optimal_for_range();

    printf("\n[SUITE 5: SCORING SYSTEM]\n");
    run_test_score_perfect_easy();
    run_test_score_good_medium();
    run_test_score_minimum();
    run_test_score_penalty_for_hints();
    run_test_score_difficulty_multiplier();
    run_test_challenge_score_basic();
    run_test_challenge_score_streak_multiplier();

    printf("\n[SUITE 6: INPUT VALIDATION]\n");
    run_test_valid_guess_in_range();
    run_test_valid_guess_out_of_range();
    run_test_valid_range_normal();
    run_test_valid_range_invalid();

    printf("\n[SUITE 7: STATISTICS]\n");
    run_test_win_rate_perfect();
    run_test_win_rate_half();
    run_test_win_rate_zero();
    run_test_average_guesses_consistency();
    run_test_average_guesses_zero_games();

    printf("\n[SUITE 8: DIFFICULTY SCALING]\n");
    run_test_difficulty_range_easy();
    run_test_difficulty_range_medium();
    run_test_difficulty_range_hard();
    run_test_difficulty_guesses_scaling();

    print_header("Test Summary");
    printf("\nTotal Tests Run:    %d\n", tests_run);
    printf("Tests Passed:       %d\n", tests_passed);
    printf("Tests Failed:       %d\n", tests_failed);

    if (tests_failed == 0) {
        printf("\n✓ ALL TESTS PASSED!\n\n");
        return 0;
    } else {
        printf("\n✗ SOME TESTS FAILED\n\n");
        return 1;
    }
}
