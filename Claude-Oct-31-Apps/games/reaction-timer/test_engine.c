/**
 * test_engine.c - Game logic and test execution
 *
 * Implements all 7 test types and session management.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "test_engine.h"
#include "display.h"
#include "input.h"
#include "timer.h"
#include "rng.h"

/**
 * Initialize test session.
 */
TestSession* test_engine_init(const TestConfig *config) {
    if (config == NULL) {
        return NULL;
    }

    TestSession *session = malloc(sizeof(TestSession));
    if (session == NULL) {
        return NULL;
    }

    memset(session, 0, sizeof(TestSession));

    memcpy(&session->config, config, sizeof(TestConfig));
    session->results = malloc(config->num_trials * sizeof(TrialResult));

    if (session->results == NULL) {
        free(session);
        return NULL;
    }

    session->session_start = time(NULL);
    return session;
}

/**
 * Run a test session.
 */
int test_engine_run(TestSession *session) {
    if (session == NULL) {
        return -1;
    }

    display_clear();
    display_draw_box("REACTION TIMER",
                     "\nStarting test...\n\n",
                     50);

    input_flush();

    for (uint32_t trial = 1; trial <= session->config.num_trials; trial++) {
        TrialResult result = {0};
        result.trial_number = trial;

        int ret = test_engine_run_trial(session, trial, &result);

        if (ret != 0) {
            session->completed_trials = trial - 1;
            return ret;  /* User quit or error */
        }

        session->results[trial - 1] = result;
        session->completed_trials = trial;

        /* Show progress */
        display_show_progress_bar(trial, session->config.num_trials, 20);
        display_show_result((double)result.reaction_time_us / 1000.0,
                           result.is_correct, result.is_false_start);
    }

    return 0;
}

/**
 * Run a single trial.
 */
int test_engine_run_trial(TestSession *session, uint32_t trial_number,
                           TrialResult *result) {
    if (session == NULL || result == NULL) {
        return -1;
    }

    RNGState *rng = rng_init(session->config.random_seed + trial_number);
    if (rng == NULL) {
        return -1;
    }

    /* Show ready state */
    display_show_ready();

    /* Generate random delay */
    uint32_t delay_ms = rng_next_delay_ms(rng,
                                          session->config.min_delay_ms,
                                          session->config.max_delay_ms);

    /* Wait with animation */
    display_show_waiting(delay_ms);

    /* Generate stimulus */
    result->stimulus = test_engine_generate_stimulus(
        session->config.test_type,
        session->config.random_seed,
        trial_number);

    /* Get stimulus time */
    uint64_t stimulus_time_us = timer_get_time_us();

    /* Display stimulus */
    display_show_stimulus(session->config.test_type, result->stimulus, 0);

    /* Wait for input */
    InputEvent event = {0};
    if (input_wait_timeout(3000, &event) != 0) {
        /* Timeout */
        result->reaction_time_us = 3000000;
        result->is_valid = 0;
        rng_free(rng);
        return 0;
    }

    /* Calculate reaction time */
    result->reaction_time_us = timer_elapsed_us(stimulus_time_us,
                                                 event.timestamp_us);

    /* Validate measurement */
    result->is_valid = timer_validate_measurement(result->reaction_time_us);

    /* Check for false start */
    result->is_false_start = input_is_false_start(event.timestamp_us,
                                                   stimulus_time_us,
                                                   50000);

    /* Check response correctness */
    result->is_correct = test_engine_check_response(
        session->config.test_type,
        result->stimulus,
        event.key);

    rng_free(rng);
    return 0;
}

/**
 * Generate stimulus.
 */
char test_engine_generate_stimulus(int test_type, uint32_t seed,
                                   uint32_t trial_number) {
    RNGState *rng = rng_init(seed + trial_number);
    if (rng == NULL) {
        return 'X';
    }

    char stim;

    switch (test_type) {
        case TEST_SIMPLE_REACTION:
            stim = ' ';  /* Any key works */
            break;

        case TEST_CHOICE_REACTION:
            stim = rng_select_choice(rng, "ASDF", 4);
            break;

        case TEST_COLOR_REACTION:
            stim = rng_select_choice(rng, "RGBY", 4);
            break;

        case TEST_SEQUENCE:
            stim = '0' + rng_next_u32(rng) % 10;
            break;

        case TEST_PATTERN:
            stim = rng_select_choice(rng, "+-X", 3);
            break;

        case TEST_AUDIO:
            stim = '\a';  /* ASCII bell */
            break;

        case TEST_INHIBITION:
            stim = rng_select_choice(rng, "XO", 2);
            break;

        default:
            stim = 'X';
    }

    rng_free(rng);
    return stim;
}

/**
 * Check response correctness.
 */
int test_engine_check_response(int test_type, char stimulus,
                                char response_key) {
    switch (test_type) {
        case TEST_SIMPLE_REACTION:
            return 1;  /* Any key is correct */

        case TEST_CHOICE_REACTION:
            return (response_key == stimulus) ? 1 : 0;

        case TEST_COLOR_REACTION:
            /* Only red is correct */
            return (stimulus == 'R' && response_key == ' ') ? 1 : 0;

        case TEST_INHIBITION:
            /* Only press for 'X', not for 'O' */
            return (stimulus == 'X' && response_key != '\0') ? 1 : 0;

        default:
            return 1;
    }
}

/**
 * Adjust difficulty.
 */
int test_engine_adjust_difficulty(int difficulty, uint32_t base_min,
                                   uint32_t base_max, uint32_t *adjusted_min,
                                   uint32_t *adjusted_max) {
    if (adjusted_min == NULL || adjusted_max == NULL) {
        return -1;
    }

    /* Difficulty 1-10: reduces delay range */
    double factor = 1.0 - ((difficulty - 1) * 0.08);  /* 1.0 to 0.28 */

    *adjusted_min = (uint32_t)(base_min * factor);
    *adjusted_max = (uint32_t)(base_max * factor);

    if (*adjusted_min < 100) {
        *adjusted_min = 100;
    }
    if (*adjusted_max < *adjusted_min) {
        *adjusted_max = *adjusted_min;
    }

    return 0;
}

/**
 * Get test type string.
 */
const char* test_engine_type_string(int test_type) {
    switch (test_type) {
        case TEST_SIMPLE_REACTION:
            return "Simple Reaction";
        case TEST_CHOICE_REACTION:
            return "Choice Reaction";
        case TEST_COLOR_REACTION:
            return "Color Reaction";
        case TEST_SEQUENCE:
            return "Sequence Memory";
        case TEST_PATTERN:
            return "Pattern Recognition";
        case TEST_AUDIO:
            return "Audio Reaction";
        case TEST_INHIBITION:
            return "Inhibition Test";
        default:
            return "Unknown";
    }
}

/**
 * Get test mode string.
 */
const char* test_engine_mode_string(int test_mode) {
    switch (test_mode) {
        case MODE_QUICK:
            return "Quick";
        case MODE_STANDARD:
            return "Standard";
        case MODE_MARATHON:
            return "Marathon";
        case MODE_ENDURANCE:
            return "Endurance";
        case MODE_DAILY_CHALLENGE:
            return "Daily";
        case MODE_REFLEX_TRAINING:
            return "Training";
        default:
            return "Unknown";
    }
}

/**
 * Finalize session and compute statistics.
 */
int test_engine_finalize(TestSession *session) {
    if (session == NULL || session->results == NULL) {
        return -1;
    }

    uint64_t *samples = malloc(session->completed_trials * sizeof(uint64_t));
    if (samples == NULL) {
        return -1;
    }

    for (uint32_t i = 0; i < session->completed_trials; i++) {
        samples[i] = session->results[i].reaction_time_us;
    }

    int ret = stats_calculate(samples, session->completed_trials,
                             &session->stats);

    free(samples);
    session->session_end = time(NULL);
    session->session_duration_us = (uint64_t)(session->session_end -
                                              session->session_start) *
                                   1000000;

    return ret;
}

/**
 * Display results.
 */
int test_engine_display_results(const TestSession *session) {
    if (session == NULL) {
        return -1;
    }

    display_clear();
    display_show_session_summary(session);
    display_show_progress_bar(session->completed_trials,
                             session->config.num_trials, 20);

    return 0;
}

/**
 * Free session.
 */
void test_engine_free(TestSession *session) {
    if (session == NULL) {
        return;
    }

    if (session->results != NULL) {
        free(session->results);
    }

    stats_free(&session->stats);
    free(session);
}
