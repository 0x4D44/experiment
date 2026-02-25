/**
 * statistics.c - Statistical analysis implementation
 *
 * Comprehensive statistics with online and batch computation.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include "statistics.h"

/**
 * Initialize online statistics tracker.
 */
OnlineStats* stats_online_init(void) {
    OnlineStats *stats = malloc(sizeof(OnlineStats));
    if (stats == NULL) {
        return NULL;
    }

    memset(stats, 0, sizeof(OnlineStats));
    stats->min_us = UINT64_MAX;
    stats->max_us = 0;

    return stats;
}

/**
 * Add sample to online statistics.
 */
int stats_online_add(OnlineStats *stats, uint64_t microseconds) {
    if (stats == NULL) {
        return -1;
    }

    stats->count++;
    stats->sum_us += microseconds;

    /* Update min/max */
    if (microseconds < stats->min_us) {
        stats->min_us = microseconds;
    }
    if (microseconds > stats->max_us) {
        stats->max_us = microseconds;
    }

    /* Welford's algorithm for running variance */
    double delta = (double)microseconds - stats->mean;
    stats->mean = (stats->mean * (stats->count - 1) + (double)microseconds) /
                  stats->count;
    double delta2 = (double)microseconds - stats->mean;
    stats->m2 += delta * delta2;

    return 0;
}

/**
 * Get running mean.
 */
double stats_online_mean(OnlineStats *stats) {
    if (stats == NULL || stats->count == 0) {
        return 0.0;
    }
    return stats->mean / 1000.0;  /* Convert to milliseconds */
}

/**
 * Get running standard deviation.
 */
double stats_online_stddev(OnlineStats *stats) {
    if (stats == NULL || stats->count < 2) {
        return 0.0;
    }

    double variance = stats->m2 / (stats->count - 1);
    return sqrt(variance) / 1000.0;  /* Convert to milliseconds */
}

/**
 * Compare function for qsort.
 */
static int compare_uint64(const void *a, const void *b) {
    uint64_t val_a = *(const uint64_t *)a;
    uint64_t val_b = *(const uint64_t *)b;

    if (val_a < val_b) return -1;
    if (val_a > val_b) return 1;
    return 0;
}

/**
 * Calculate percentile from sorted array.
 */
double stats_percentile(uint64_t *sorted_samples, int count,
                        double percentile) {
    if (count <= 0 || percentile < 0 || percentile > 100) {
        return 0.0;
    }

    if (count == 1) {
        return (double)sorted_samples[0] / 1000.0;  /* Convert to ms */
    }

    /* Type 7 (R-7) linear interpolation */
    double h = (count - 1) * percentile / 100.0;
    int k = (int)h;
    double g = h - k;

    if (k >= count - 1) {
        return (double)sorted_samples[count - 1] / 1000.0;
    }

    double val = (double)sorted_samples[k] * (1.0 - g) +
                (double)sorted_samples[k + 1] * g;

    return val / 1000.0;  /* Convert to ms */
}

/**
 * Calculate comprehensive statistics.
 */
int stats_calculate(uint64_t *samples, int count, Statistics *stats) {
    if (samples == NULL || count <= 0 || stats == NULL) {
        return -1;
    }

    memset(stats, 0, sizeof(Statistics));

    /* Make a copy for sorting */
    uint64_t *sorted = malloc(count * sizeof(uint64_t));
    if (sorted == NULL) {
        return -1;
    }

    memcpy(sorted, samples, count * sizeof(uint64_t));
    qsort(sorted, count, sizeof(uint64_t), compare_uint64);

    /* Basic statistics */
    stats->sample_count = count;
    stats->valid_sample_count = count;
    stats->min_us = sorted[0];
    stats->max_us = sorted[count - 1];

    /* Calculate mean */
    uint64_t sum = 0;
    for (int i = 0; i < count; i++) {
        sum += sorted[i];
    }
    stats->mean_ms = (double)sum / count / 1000.0;

    /* Calculate standard deviation */
    double sum_sq_diff = 0.0;
    for (int i = 0; i < count; i++) {
        double diff = (double)sorted[i] / 1000.0 - stats->mean_ms;
        sum_sq_diff += diff * diff;
    }
    stats->variance_ms2 = sum_sq_diff / (count - 1);
    stats->std_dev_ms = sqrt(stats->variance_ms2);

    /* Percentiles */
    stats->p5_ms = stats_percentile(sorted, count, 5.0);
    stats->p10_ms = stats_percentile(sorted, count, 10.0);
    stats->p25_ms = stats_percentile(sorted, count, 25.0);
    stats->p50_ms = stats_percentile(sorted, count, 50.0);
    stats->p75_ms = stats_percentile(sorted, count, 75.0);
    stats->p90_ms = stats_percentile(sorted, count, 90.0);
    stats->p95_ms = stats_percentile(sorted, count, 95.0);
    stats->p99_ms = stats_percentile(sorted, count, 99.0);

    stats->median_ms = stats->p50_ms;
    stats->range_ms = (double)(stats->max_us - stats->min_us) / 1000.0;
    stats->iqr_ms = stats->p75_ms - stats->p25_ms;

    /* Consistency score */
    stats->consistency_score = stats_consistency_score(stats->mean_ms,
                                                       stats->std_dev_ms);

    /* Coefficient of variation */
    if (stats->mean_ms > 0) {
        stats->coefficient_of_variation = stats->std_dev_ms / stats->mean_ms;
    }

    /* Standard error */
    stats->standard_error = stats->std_dev_ms / sqrt((double)count);

    /* Confidence interval (95%) */
    stats->confidence_95_lower = stats->mean_ms - 1.96 * stats->standard_error;
    stats->confidence_95_upper = stats->mean_ms + 1.96 * stats->standard_error;

    free(sorted);
    return 0;
}

/**
 * Detect outliers using Tukey fence.
 */
int stats_detect_outliers(uint64_t *samples, int count, uint8_t *outlier_flags) {
    if (samples == NULL || count <= 0 || outlier_flags == NULL) {
        return -1;
    }

    memset(outlier_flags, 0, count);

    /* Make sorted copy */
    uint64_t *sorted = malloc(count * sizeof(uint64_t));
    if (sorted == NULL) {
        return -1;
    }

    memcpy(sorted, samples, count * sizeof(uint64_t));
    qsort(sorted, count, sizeof(uint64_t), compare_uint64);

    /* Calculate quartiles */
    double q1 = stats_percentile(sorted, count, 25.0);
    double q3 = stats_percentile(sorted, count, 75.0);
    double iqr = q3 - q1;

    double lower_fence = q1 - 1.5 * iqr;
    double upper_fence = q3 + 1.5 * iqr;

    /* Mark outliers */
    int outlier_count = 0;
    for (int i = 0; i < count; i++) {
        double val_ms = (double)samples[i] / 1000.0;
        if (val_ms < lower_fence || val_ms > upper_fence) {
            outlier_flags[i] = 1;
            outlier_count++;
        }
    }

    free(sorted);
    return outlier_count;
}

/**
 * Analyze fatigue trend.
 */
double stats_analyze_fatigue(uint64_t *samples, int count, int block_size) {
    if (samples == NULL || count <= 0 || block_size <= 0) {
        return 0.0;
    }

    int num_blocks = count / block_size;
    if (num_blocks <= 1) {
        return 0.0;
    }

    /* Calculate average per block */
    double first_block_avg = 0.0;
    double last_block_avg = 0.0;

    /* First block */
    for (int i = 0; i < block_size && i < count; i++) {
        first_block_avg += (double)samples[i];
    }
    first_block_avg /= block_size;

    /* Last block */
    int start = (num_blocks - 1) * block_size;
    for (int i = start; i < count; i++) {
        last_block_avg += (double)samples[i];
    }
    int last_block_count = count - start;
    last_block_avg /= last_block_count;

    /* Fatigue score: percentage change */
    if (first_block_avg == 0) {
        return 0.0;
    }

    return (last_block_avg - first_block_avg) / first_block_avg * 100.0;
}

/**
 * Calculate consistency score.
 */
double stats_consistency_score(double mean, double stddev) {
    if (mean <= 0) {
        return 0.0;
    }

    double cv = stddev / mean;
    double score = 100.0 * exp(-cv * 2.0);

    if (score < 0) score = 0;
    if (score > 100) score = 100;

    return score;
}

/**
 * Free statistics structure.
 */
void stats_free(Statistics *stats) {
    if (stats == NULL) {
        return;
    }

    if (stats->block_averages != NULL) {
        free(stats->block_averages);
        stats->block_averages = NULL;
    }
}

/**
 * Free online statistics.
 */
void stats_online_free(OnlineStats *stats) {
    if (stats != NULL) {
        free(stats);
    }
}
