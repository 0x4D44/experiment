/**
 * tests_statistics.c - Unit tests for statistics module
 *
 * Tests statistical calculations, percentiles, outlier detection,
 * and fatigue analysis.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include "statistics.h"
#include "test_suite.h"

/* Test: Online statistics initialization */
static int test_stats_online_init(void) {
    OnlineStats *stats = stats_online_init();
    test_assert(stats != NULL, "Online stats should initialize");
    test_assert(stats->count == 0, "Initial count should be 0");
    test_assert(stats->mean == 0.0, "Initial mean should be 0");
    stats_online_free(stats);
    return 0;
}

/* Test: Adding samples to online stats */
static int test_stats_online_add(void) {
    OnlineStats *stats = stats_online_init();

    test_assert(stats_online_add(stats, 100000) == 0, "Add first sample");
    test_assert(stats->count == 1, "Count should be 1 after first add");

    test_assert(stats_online_add(stats, 200000) == 0, "Add second sample");
    test_assert(stats->count == 2, "Count should be 2 after second add");

    stats_online_free(stats);
    return 0;
}

/* Test: Online mean calculation */
static int test_stats_online_mean(void) {
    OnlineStats *stats = stats_online_init();

    stats_online_add(stats, 100000);  /* 100ms */
    stats_online_add(stats, 200000);  /* 200ms */
    stats_online_add(stats, 300000);  /* 300ms */

    double mean = stats_online_mean(stats);
    /* Mean should be 200000 microseconds = 200ms */
    test_assert_near(200.0, mean, 0.1, "Mean should be 200ms");

    stats_online_free(stats);
    return 0;
}

/* Test: Online standard deviation */
static int test_stats_online_stddev(void) {
    OnlineStats *stats = stats_online_init();

    /* Add samples: 100, 200, 300 (microseconds) */
    stats_online_add(stats, 100);
    stats_online_add(stats, 200);
    stats_online_add(stats, 300);

    double stddev = stats_online_stddev(stats);

    /* Sample std dev should be positive */
    test_assert(stddev > 0, "Std dev should be positive");

    stats_online_free(stats);
    return 0;
}

/* Test: Comprehensive statistics */
static int test_stats_calculate(void) {
    uint64_t samples[] = {
        100000, 150000, 200000, 250000, 300000,
        350000, 400000, 450000, 500000, 550000
    };
    int count = 10;

    Statistics stats;
    memset(&stats, 0, sizeof(Statistics));

    test_assert(stats_calculate(samples, count, &stats) == 0,
                "Statistics calculation should succeed");

    test_assert(stats.sample_count == count, "Sample count should be 10");
    test_assert(stats.mean_ms > 0, "Mean should be positive");
    test_assert(stats.median_ms > 0, "Median should be positive");
    test_assert(stats.std_dev_ms >= 0, "Std dev should be non-negative");

    return 0;
}

/* Test: Percentile calculation */
static int test_stats_percentile(void) {
    uint64_t samples[] = {
        100, 200, 300, 400, 500,
        600, 700, 800, 900, 1000
    };
    int count = 10;

    /* Must be sorted */
    double p50 = stats_percentile(samples, count, 50.0);

    /* 50th percentile of 100-1000 should be around 550 microseconds */
    test_assert(p50 > 500 && p50 < 600,
                "P50 should be around 550us for 100-1000 range");

    return 0;
}

/* Test: Outlier detection */
static int test_stats_outlier_detection(void) {
    uint64_t samples[] = {
        100000, 105000, 110000, 108000, 112000,
        900000  /* Clear outlier - 9x other values */
    };
    int count = 6;
    uint8_t outliers[6];

    int outlier_count = stats_detect_outliers(samples, count, outliers);

    test_assert(outlier_count > 0, "Should detect outliers");
    test_assert(outliers[5] == 1, "Last sample should be flagged as outlier");

    return 0;
}

/* Test: Fatigue analysis */
static int test_stats_fatigue_analysis(void) {
    /* Create samples showing improvement (decreasing times) */
    uint64_t improving[] = {
        300000, 290000, 280000, 270000, 260000,
        250000, 240000, 230000, 220000, 210000
    };
    int count = 10;

    double fatigue = stats_analyze_fatigue(improving, count, 5);

    /* Should show improvement (negative fatigue score) */
    test_assert(fatigue < 0, "Improving reaction times should show negative fatigue");

    return 0;
}

/* Test: Fatigue analysis with deterioration */
static int test_stats_fatigue_deterioration(void) {
    /* Create samples showing deterioration (increasing times) */
    uint64_t deteriorating[] = {
        200000, 210000, 220000, 230000, 240000,
        250000, 260000, 270000, 280000, 290000
    };
    int count = 10;

    double fatigue = stats_analyze_fatigue(deteriorating, count, 5);

    /* Should show fatigue (positive score) */
    test_assert(fatigue > 0, "Deteriorating reaction times should show positive fatigue");

    return 0;
}

/* Test: Consistency score */
static int test_stats_consistency(void) {
    /* Highly consistent (low variation) */
    double consistency = stats_consistency_score(250.0, 10.0);
    test_assert(consistency > 80, "Consistent results should have high score");

    /* Less consistent (high variation) */
    consistency = stats_consistency_score(250.0, 100.0);
    test_assert(consistency < 50, "Inconsistent results should have low score");

    return 0;
}

/* Test: Min/max tracking */
static int test_stats_minmax(void) {
    uint64_t samples[] = {
        150000, 250000, 200000, 300000, 100000,
        350000, 275000
    };
    int count = 7;

    Statistics stats;
    memset(&stats, 0, sizeof(Statistics));

    stats_calculate(samples, count, &stats);

    test_assert(stats.min_us == 100000, "Min should be 100000");
    test_assert(stats.max_us == 350000, "Max should be 350000");

    return 0;
}

/* Test: IQR calculation */
static int test_stats_iqr(void) {
    uint64_t samples[] = {
        100000, 110000, 120000, 130000, 140000,
        150000, 160000, 170000, 180000, 190000
    };
    int count = 10;

    Statistics stats;
    memset(&stats, 0, sizeof(Statistics));

    stats_calculate(samples, count, &stats);

    test_assert(stats.iqr_ms > 0, "IQR should be positive");
    test_assert(stats.iqr_ms < stats.range_ms,
                "IQR should be less than range");

    return 0;
}

/* Test: Edge case - single sample */
static int test_stats_single_sample(void) {
    uint64_t samples[] = {250000};

    Statistics stats;
    memset(&stats, 0, sizeof(Statistics));

    test_assert(stats_calculate(samples, 1, &stats) == 0,
                "Should handle single sample");

    test_assert(stats.mean_ms > 0, "Mean should be calculated");
    test_assert(stats.std_dev_ms == 0, "Std dev of single sample is 0");

    return 0;
}

/* Test: Edge case - identical samples */
static int test_stats_identical_samples(void) {
    uint64_t samples[] = {250000, 250000, 250000, 250000};

    Statistics stats;
    memset(&stats, 0, sizeof(Statistics));

    stats_calculate(samples, 4, &stats);

    test_assert_near(250.0, stats.mean_ms, 0.1,
                    "Mean of identical samples");
    test_assert_near(0.0, stats.std_dev_ms, 0.01,
                    "Std dev of identical samples should be 0");

    return 0;
}

/**
 * Register all statistics tests with the test suite.
 */
void test_statistics_register_all(void) {
    test_register("stats_online_init", test_stats_online_init, "statistics");
    test_register("stats_online_add", test_stats_online_add, "statistics");
    test_register("stats_online_mean", test_stats_online_mean, "statistics");
    test_register("stats_online_stddev", test_stats_online_stddev, "statistics");
    test_register("stats_calculate", test_stats_calculate, "statistics");
    test_register("stats_percentile", test_stats_percentile, "statistics");
    test_register("stats_outlier_detection", test_stats_outlier_detection, "statistics");
    test_register("stats_fatigue_analysis", test_stats_fatigue_analysis, "statistics");
    test_register("stats_fatigue_deterioration", test_stats_fatigue_deterioration, "statistics");
    test_register("stats_consistency", test_stats_consistency, "statistics");
    test_register("stats_minmax", test_stats_minmax, "statistics");
    test_register("stats_iqr", test_stats_iqr, "statistics");
    test_register("stats_single_sample", test_stats_single_sample, "statistics");
    test_register("stats_identical_samples", test_stats_identical_samples, "statistics");
}
