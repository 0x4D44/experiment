/**
 * timer.h - High-precision timing interface
 *
 * Provides microsecond-precision timing abstractions with
 * platform-specific implementations (POSIX and Windows).
 */

#ifndef TIMER_H
#define TIMER_H

#include <stdint.h>
#include <time.h>

/* Timer precision in microseconds (0.001ms) */
#define TIMER_RESOLUTION 1

/* Maximum reaction time measurement (10 seconds) */
#define TIMER_MAX_MEASUREMENT 10000000ULL

/* Warm-up trials before actual measurement */
#define TIMER_WARMUP_TRIALS 3

typedef struct {
    uint64_t microseconds;      /* Time in microseconds */
    uint64_t system_ticks;      /* Raw system ticks */
    int is_valid;               /* Passed sanity checks */
} TimingRecord;

typedef struct {
    uint64_t frequency;         /* System timer frequency */
    uint64_t calibration_us;    /* Calibration offset */
    double clock_drift_factor;  /* Drift compensation */
} TimerCalibration;

/**
 * Initialize the timer system with platform-specific setup.
 * Must be called before any timing operations.
 *
 * Returns: 0 on success, -1 on failure
 */
int timer_init(void);

/**
 * Shutdown the timer system and free resources.
 * Must be called before program exit.
 *
 * Returns: 0 on success, -1 on failure
 */
int timer_shutdown(void);

/**
 * Get current high-precision timestamp in microseconds.
 * Uses CLOCK_MONOTONIC_RAW (POSIX) or QueryPerformanceCounter (Windows).
 *
 * Returns: Microseconds since some arbitrary epoch
 */
uint64_t timer_get_time_us(void);

/**
 * Get current high-precision timestamp in milliseconds.
 * Convenience wrapper around timer_get_time_us().
 *
 * Returns: Milliseconds with microsecond precision (as double)
 */
double timer_get_time_ms(void);

/**
 * Measure elapsed time between two timestamps.
 * Handles wraparound and overflow.
 *
 * Returns: Elapsed microseconds, or 0 if invalid
 */
uint64_t timer_elapsed_us(uint64_t start_us, uint64_t end_us);

/**
 * Sleep for specified microseconds with minimal jitter.
 * Platform-specific implementation for accuracy.
 *
 * Returns: 0 on success, -1 on failure
 */
int timer_sleep_us(uint64_t microseconds);

/**
 * Sleep for specified milliseconds.
 * Convenience wrapper around timer_sleep_us().
 *
 * Returns: 0 on success, -1 on failure
 */
int timer_sleep_ms(uint32_t milliseconds);

/**
 * Calibrate the timer system against known reference.
 * Should be called during initialization and periodically.
 * Detects and compensates for system drift.
 *
 * Returns: 0 if calibration successful (error < 1%), -1 otherwise
 */
int timer_calibrate(void);

/**
 * Validate a timing measurement for sanity.
 * Checks for overflow, underflow, and outliers.
 *
 * Returns: 1 if valid, 0 if invalid
 */
int timer_validate_measurement(uint64_t microseconds);

/**
 * Get current timer calibration state.
 * For diagnostics and accuracy reporting.
 *
 * Returns: Pointer to calibration structure (read-only)
 */
const TimerCalibration* timer_get_calibration(void);

/**
 * Get estimated system timer resolution in microseconds.
 * Used for reporting measurement accuracy.
 *
 * Returns: Resolution in microseconds
 */
uint64_t timer_get_resolution(void);

#endif /* TIMER_H */
