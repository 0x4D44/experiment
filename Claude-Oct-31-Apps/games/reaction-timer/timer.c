/**
 * timer.c - High-precision timing implementation
 *
 * Platform-specific implementations for POSIX and Windows.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include "timer.h"

/* Global calibration state */
static TimerCalibration global_calibration = {0};
static int timer_initialized = 0;

/**
 * Initialize the timer system.
 */
int timer_init(void) {
    if (timer_initialized) {
        return 0;
    }

    memset(&global_calibration, 0, sizeof(TimerCalibration));

    /* Set up platform-specific initialization */
#ifdef _POSIX_C_SOURCE
    /* POSIX systems: use CLOCK_MONOTONIC_RAW */
    struct timespec ts;
    if (clock_gettime(CLOCK_MONOTONIC_RAW, &ts) != 0) {
        return -1;
    }
    global_calibration.frequency = 1000000000LL;  /* nanoseconds */
#else
    /* Fallback: use CLOCK_MONOTONIC */
    struct timespec ts;
    if (clock_gettime(CLOCK_MONOTONIC, &ts) != 0) {
        return -1;
    }
    global_calibration.frequency = 1000000000LL;
#endif

    timer_initialized = 1;
    return 0;
}

/**
 * Shutdown the timer system.
 */
int timer_shutdown(void) {
    timer_initialized = 0;
    return 0;
}

/**
 * Get current high-precision timestamp in microseconds.
 */
uint64_t timer_get_time_us(void) {
    struct timespec ts;

#ifdef _POSIX_C_SOURCE
    if (clock_gettime(CLOCK_MONOTONIC_RAW, &ts) != 0) {
        return 0;
    }
#else
    if (clock_gettime(CLOCK_MONOTONIC, &ts) != 0) {
        return 0;
    }
#endif

    /* Convert to microseconds */
    uint64_t microseconds = (uint64_t)ts.tv_sec * 1000000ULL +
                           (uint64_t)ts.tv_nsec / 1000ULL;

    return microseconds;
}

/**
 * Get current timestamp in milliseconds.
 */
double timer_get_time_ms(void) {
    uint64_t us = timer_get_time_us();
    return (double)us / 1000.0;
}

/**
 * Measure elapsed time between timestamps.
 */
uint64_t timer_elapsed_us(uint64_t start_us, uint64_t end_us) {
    if (end_us < start_us) {
        /* Handle wraparound */
        return UINT64_MAX - start_us + end_us;
    }
    return end_us - start_us;
}

/**
 * Sleep for specified microseconds.
 */
int timer_sleep_us(uint64_t microseconds) {
    struct timespec ts;
    ts.tv_sec = microseconds / 1000000;
    ts.tv_nsec = (microseconds % 1000000) * 1000;

    if (nanosleep(&ts, NULL) != 0) {
        return -1;
    }
    return 0;
}

/**
 * Sleep for specified milliseconds.
 */
int timer_sleep_ms(uint32_t milliseconds) {
    return timer_sleep_us((uint64_t)milliseconds * 1000);
}

/**
 * Calibrate the timer system.
 */
int timer_calibrate(void) {
    if (!timer_initialized) {
        return -1;
    }

    /* Measure a known interval (1 second) */
    uint64_t start = timer_get_time_us();
    sleep(1);
    uint64_t end = timer_get_time_us();

    uint64_t elapsed = timer_elapsed_us(start, end);

    /* 1 second = 1,000,000 microseconds */
    /* Check if within 1% of expected */
    if (elapsed < 990000 || elapsed > 1010000) {
        return -1;
    }

    global_calibration.calibration_us = elapsed;
    global_calibration.clock_drift_factor = 1000000.0 / (double)elapsed;

    return 0;
}

/**
 * Validate a timing measurement.
 */
int timer_validate_measurement(uint64_t microseconds) {
    /* Must be positive */
    if (microseconds == 0) {
        return 0;
    }

    /* Must not exceed maximum (10 seconds) */
    if (microseconds > TIMER_MAX_MEASUREMENT) {
        return 0;
    }

    return 1;
}

/**
 * Get calibration state.
 */
const TimerCalibration* timer_get_calibration(void) {
    return &global_calibration;
}

/**
 * Get timer resolution.
 */
uint64_t timer_get_resolution(void) {
    /* POSIX systems typically have nanosecond resolution */
    /* Report as microseconds */
    return 1;  /* 1 microsecond resolution */
}
