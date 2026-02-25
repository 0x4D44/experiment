/**
 * test_engine.h - Game logic and test execution
 *
 * Provides test types, configurations, and execution framework
 * for all seven reaction time test modes.
 */

#ifndef TEST_ENGINE_H
#define TEST_ENGINE_H

#include <stdint.h>
#include <time.h>
#include "statistics.h"

/* Test type enumeration */
enum TestType {
    TEST_SIMPLE_REACTION = 1,   /* Press any key on "GO!" */
    TEST_CHOICE_REACTION = 2,   /* Press correct key from options */
    TEST_COLOR_REACTION = 3,    /* React to specific color only */
    TEST_SEQUENCE = 4,          /* Reproduce shown sequence */
    TEST_PATTERN = 5,           /* React to pattern match */
    TEST_AUDIO = 6,             /* React to audio cue */
    TEST_INHIBITION = 7         /* Go/No-Go task */
};

/* Test mode enumeration */
enum TestMode {
    MODE_QUICK = 1,             /* 5 attempts */
    MODE_STANDARD = 2,          /* 10 attempts */
    MODE_MARATHON = 3,          /* 50 attempts */
    MODE_ENDURANCE = 4,         /* Continuous until miss */
    MODE_DAILY_CHALLENGE = 5,   /* Same seed for all */
    MODE_REFLEX_TRAINING = 6    /* Adaptive difficulty */
};

/* Result of a single trial */
typedef struct {
    uint32_t trial_number;      /* Which trial (1-based) */
    uint64_t reaction_time_us;  /* In microseconds */
    uint8_t is_valid;           /* Passed validation */
    uint8_t is_false_start;     /* Too early */
    uint8_t is_correct;         /* Correct response (choice tests) */
    uint8_t is_outlier;         /* Statistical outlier */
    char stimulus;              /* What was shown (for replay) */
} TrialResult;

/* Test configuration */
typedef struct {
    int test_type;              /* From TestType enum */
    int test_mode;              /* From TestMode enum */
    uint32_t num_trials;        /* How many attempts */
    uint32_t min_delay_ms;      /* Minimum stimulus delay */
    uint32_t max_delay_ms;      /* Maximum stimulus delay */
    uint32_t random_seed;       /* RNG seed for reproducibility */
    int difficulty_level;       /* 1-10 (affects delays/complexity) */
    char player_name[64];       /* For leaderboard entry */
    time_t session_start;       /* Session timestamp */
} TestConfig;

/* Complete session result */
typedef struct {
    TestConfig config;          /* Test parameters */
    TrialResult *results;       /* Array of trial results */
    uint32_t completed_trials;  /* Actual trials completed */
    Statistics stats;           /* Computed statistics */
    time_t session_start;       /* Start timestamp */
    time_t session_end;         /* End timestamp */
    uint64_t session_duration_us; /* Total time including delays */
} TestSession;

/**
 * Initialize test engine with configuration.
 * Validates config and allocates trial result array.
 *
 * config: Test configuration
 *
 * Returns: Pointer to allocated TestSession, NULL on failure
 */
TestSession* test_engine_init(const TestConfig *config);

/**
 * Run a complete test session.
 * Executes configured number of trials with specified test type.
 * Updates TestSession with results.
 *
 * session: Test session to run
 *
 * Returns: 0 on successful completion, -1 on failure, 1 on user quit
 */
int test_engine_run(TestSession *session);

/**
 * Execute a single trial of the test.
 * Shows prompt, waits for delay, displays stimulus, measures response.
 *
 * session: Current test session
 * trial_number: Which trial is running (1-based)
 * result: Output TrialResult
 *
 * Returns: 0 on success, -1 on error, 1 on user quit
 */
int test_engine_run_trial(TestSession *session, uint32_t trial_number,
                           TrialResult *result);

/**
 * Generate random stimulus for current test type.
 * Ensures variety and prevents predictable patterns.
 *
 * test_type: Type of test running
 * seed: Random seed for reproducibility
 * trial_number: Which trial this is
 *
 * Returns: Stimulus character/code
 */
char test_engine_generate_stimulus(int test_type, uint32_t seed,
                                   uint32_t trial_number);

/**
 * Check if response is correct for the stimulus.
 * Varies by test type (simple = any key, choice = specific key, etc.).
 *
 * test_type: Type of test
 * stimulus: What was shown
 * response_key: What key was pressed
 *
 * Returns: 1 if correct, 0 if incorrect
 */
int test_engine_check_response(int test_type, char stimulus, char response_key);

/**
 * Calculate difficulty-adjusted delay range.
 * Higher difficulty = shorter delays for faster reflexes.
 *
 * difficulty: 1-10 level
 * base_min: Baseline minimum
 * base_max: Baseline maximum
 * adjusted_min: Output minimum
 * adjusted_max: Output maximum
 *
 * Returns: 0 on success
 */
int test_engine_adjust_difficulty(int difficulty, uint32_t base_min,
                                   uint32_t base_max, uint32_t *adjusted_min,
                                   uint32_t *adjusted_max);

/**
 * Get display string for test type.
 *
 * test_type: From TestType enum
 * Returns: Human-readable test name
 */
const char* test_engine_type_string(int test_type);

/**
 * Get display string for test mode.
 *
 * test_mode: From TestMode enum
 * Returns: Human-readable mode name
 */
const char* test_engine_mode_string(int test_mode);

/**
 * Finalize session and compute all statistics.
 * Must be called before using stats in the session.
 *
 * session: Completed test session
 *
 * Returns: 0 on success, -1 on failure
 */
int test_engine_finalize(TestSession *session);

/**
 * Display session results to user.
 * Shows statistics, graphs, percentile rankings, etc.
 *
 * session: Completed test session
 *
 * Returns: 0 on success
 */
int test_engine_display_results(const TestSession *session);

/**
 * Free allocated test session memory.
 *
 * session: TestSession to free
 */
void test_engine_free(TestSession *session);

#endif /* TEST_ENGINE_H */
