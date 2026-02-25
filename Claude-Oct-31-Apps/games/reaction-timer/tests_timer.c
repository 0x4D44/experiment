/**
 * tests_timer.c - Unit tests for timer module
 *
 * Tests timing accuracy, calibration, and platform-specific implementations.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include "timer.h"
#include "test_suite.h"

/* Test: Timer initialization and shutdown */
static int test_timer_init_shutdown(void) {
    test_assert(timer_init() == 0, "Timer init should succeed");
    test_assert(timer_shutdown() == 0, "Timer shutdown should succeed");
    return 0;
}

/* Test: Basic timing measurement */
static int test_timer_basic_measurement(void) {
    timer_init();

    uint64_t start = timer_get_time_us();
    usleep(100000);  /* Sleep 100ms */
    uint64_t end = timer_get_time_us();

    uint64_t elapsed = timer_elapsed_us(start, end);

    /* Should be approximately 100ms (100000 microseconds) */
    /* Allow 10% tolerance for system jitter */
    test_assert(elapsed > 90000 && elapsed < 110000,
                "100ms sleep should measure ~100ms");

    timer_shutdown();
    return 0;
}

/* Test: Millisecond convenience function */
static int test_timer_milliseconds(void) {
    timer_init();

    double start = timer_get_time_ms();
    usleep(50000);  /* Sleep 50ms */
    double end = timer_get_time_ms();

    double elapsed = end - start;

    /* Should be approximately 50ms */
    test_assert(elapsed > 45.0 && elapsed < 55.0,
                "50ms sleep should measure ~50ms");

    timer_shutdown();
    return 0;
}

/* Test: Timer validation */
static int test_timer_validate(void) {
    timer_init();

    /* Valid measurement */
    test_assert(timer_validate_measurement(100000) == 1,
                "Valid measurement should pass validation");

    /* Zero measurement */
    test_assert(timer_validate_measurement(0) == 0,
                "Zero measurement should fail validation");

    /* Overflow (>10 seconds) */
    test_assert(timer_validate_measurement(15000000) == 0,
                "Overflow measurement should fail validation");

    timer_shutdown();
    return 0;
}

/* Test: Sleep functionality */
static int test_timer_sleep(void) {
    timer_init();

    uint64_t start = timer_get_time_us();
    test_assert(timer_sleep_ms(100) == 0, "Sleep 100ms should succeed");
    uint64_t end = timer_get_time_us();

    uint64_t elapsed = end - start;

    /* 100ms = 100,000 microseconds */
    test_assert(elapsed > 90000 && elapsed < 110000,
                "Sleep duration should be accurate");

    timer_shutdown();
    return 0;
}

/* Test: Timer calibration */
static int test_timer_calibration(void) {
    timer_init();

    /* Calibration should succeed */
    test_assert(timer_calibrate() == 0, "Timer calibration should succeed");

    /* Get calibration info */
    const TimerCalibration *calib = timer_get_calibration();
    test_assert(calib != NULL, "Calibration info should be available");
    test_assert(calib->frequency > 0, "Calibration frequency should be positive");

    timer_shutdown();
    return 0;
}

/* Test: Timer resolution reporting */
static int test_timer_resolution(void) {
    timer_init();

    uint64_t resolution = timer_get_resolution();

    /* Resolution should be reasonable (< 1 second) */
    test_assert(resolution > 0 && resolution < 1000000,
                "Timer resolution should be between 0 and 1 second");

    timer_shutdown();
    return 0;
}

/* Test: Microsecond precision */
static int test_timer_precision(void) {
    timer_init();

    /* Take multiple measurements and check consistency */
    uint64_t measurements[5];
    for (int i = 0; i < 5; i++) {
        uint64_t start = timer_get_time_us();
        usleep(10000);  /* 10ms */
        measurements[i] = timer_elapsed_us(start, timer_get_time_us());
    }

    /* All measurements should be similar (within 20% tolerance) */
    uint64_t avg = 0;
    for (int i = 0; i < 5; i++) {
        avg += measurements[i];
    }
    avg /= 5;

    for (int i = 0; i < 5; i++) {
        uint64_t diff = (measurements[i] > avg) ?
                       measurements[i] - avg :
                       avg - measurements[i];
        test_assert(diff < (avg / 5),
                   "Measurements should be consistent");
    }

    timer_shutdown();
    return 0;
}

/* Test: Monotonic time (never goes backwards) */
static int test_timer_monotonic(void) {
    timer_init();

    uint64_t prev = timer_get_time_us();
    int monotonic_failures = 0;

    /* Take 100 consecutive measurements */
    for (int i = 0; i < 100; i++) {
        uint64_t curr = timer_get_time_us();
        if (curr < prev) {
            monotonic_failures++;
        }
        prev = curr;
    }

    /* Should never go backwards */
    test_assert(monotonic_failures == 0,
                "Timer should be monotonically increasing");

    timer_shutdown();
    return 0;
}

/* Test: Large duration measurements */
static int test_timer_large_duration(void) {
    timer_init();

    uint64_t start = timer_get_time_us();
    sleep(2);  /* Sleep 2 seconds */
    uint64_t elapsed = timer_elapsed_us(start, timer_get_time_us());

    /* Should be approximately 2 seconds = 2,000,000 microseconds */
    test_assert(elapsed > 1900000 && elapsed < 2100000,
                "2 second sleep should measure ~2 seconds");

    timer_shutdown();
    return 0;
}

/* Test: Negative elapsed time handling */
static int test_timer_negative_elapsed(void) {
    timer_init();

    uint64_t end = timer_get_time_us();
    usleep(100000);
    uint64_t start = timer_get_time_us();

    /* elapsed should be negative (returned as unsigned, will wrap) */
    uint64_t elapsed = timer_elapsed_us(start, end);

    /* When end < start, elapsed should be close to UINT64_MAX */
    /* This is a wraparound, not a real failure */
    test_assert(elapsed < 100 || elapsed > 18446744073700000000ULL,
                "Negative elapsed should wrap correctly");

    timer_shutdown();
    return 0;
}

/**
 * Register all timer tests with the test suite.
 */
void test_timer_register_all(void) {
    test_register("timer_init_shutdown", test_timer_init_shutdown, "timer");
    test_register("timer_basic_measurement", test_timer_basic_measurement, "timer");
    test_register("timer_milliseconds", test_timer_milliseconds, "timer");
    test_register("timer_validate", test_timer_validate, "timer");
    test_register("timer_sleep", test_timer_sleep, "timer");
    test_register("timer_calibration", test_timer_calibration, "timer");
    test_register("timer_resolution", test_timer_resolution, "timer");
    test_register("timer_precision", test_timer_precision, "timer");
    test_register("timer_monotonic", test_timer_monotonic, "timer");
    test_register("timer_large_duration", test_timer_large_duration, "timer");
    test_register("timer_negative_elapsed", test_timer_negative_elapsed, "timer");
}
