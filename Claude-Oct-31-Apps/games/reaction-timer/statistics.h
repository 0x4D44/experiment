/**
 * statistics.h - Statistical analysis system
 *
 * Provides comprehensive reaction time statistics including
 * mean, median, percentiles, outlier detection, and trend analysis.
 */

#ifndef STATISTICS_H
#define STATISTICS_H

#include <stdint.h>
#include <stddef.h>

/* Maximum reaction time samples per session */
#define STATS_MAX_SAMPLES 1000

/* Welford's online algorithm for running variance */
typedef struct {
    uint64_t count;             /* Number of samples */
    double mean;                /* Running mean */
    double m2;                  /* Sum of squared differences */
    uint64_t min_us;            /* Minimum value */
    uint64_t max_us;            /* Maximum value */
    uint64_t sum_us;            /* Sum for mean calculation */
} OnlineStats;

/* Comprehensive statistics record */
typedef struct {
    /* Central tendency */
    double mean_ms;             /* Arithmetic mean in milliseconds */
    double median_ms;           /* Median (50th percentile) */
    double mode_ms;             /* Most frequent value */

    /* Dispersion */
    double std_dev_ms;          /* Standard deviation */
    double variance_ms2;        /* Variance */
    double range_ms;            /* Max - Min */
    double iqr_ms;              /* Interquartile range */

    /* Extremes */
    uint64_t min_us;            /* Minimum in microseconds */
    uint64_t max_us;            /* Maximum in microseconds */

    /* Percentiles */
    double p5_ms;               /* 5th percentile */
    double p10_ms;              /* 10th percentile */
    double p25_ms;              /* 25th percentile (Q1) */
    double p50_ms;              /* 50th percentile (median) */
    double p75_ms;              /* 75th percentile (Q3) */
    double p90_ms;              /* 90th percentile */
    double p95_ms;              /* 95th percentile */
    double p99_ms;              /* 99th percentile */

    /* Outliers & Anomalies */
    int outlier_count;          /* Count of statistical outliers */
    int false_start_count;      /* Count of too-early presses */
    double outlier_percentage;  /* Outliers as % of total */

    /* Fatigue Analysis */
    double fatigue_score;       /* Trend analysis: -100 (improving) to +100 (deteriorating) */
    int block_count;            /* Number of blocks for fatigue analysis */
    double *block_averages;     /* Average per block */

    /* Consistency & Reliability */
    double consistency_score;   /* 0-100, higher is more consistent */
    double coefficient_of_variation; /* CV = StdDev / Mean */
    double skewness;            /* Measure of asymmetry */
    double kurtosis;            /* Measure of tail heaviness */

    /* Confidence */
    double confidence_95_lower; /* 95% CI lower bound */
    double confidence_95_upper; /* 95% CI upper bound */
    double standard_error;      /* SE = StdDev / sqrt(n) */

    /* Metadata */
    int sample_count;           /* Total samples analyzed */
    int valid_sample_count;     /* Samples after outlier removal */
    int outlier_method;         /* 0=Tukey, 1=z-score, 2=modified z-score */
} Statistics;

/**
 * Initialize empty online statistics tracker.
 * Uses Welford's algorithm for numerical stability.
 *
 * Returns: Pointer to initialized OnlineStats, NULL on failure
 */
OnlineStats* stats_online_init(void);

/**
 * Add a single measurement to running statistics.
 * Can be called in real-time as data arrives.
 *
 * stats: Online statistics object
 * microseconds: Reaction time in microseconds
 *
 * Returns: 0 on success, -1 on error
 */
int stats_online_add(OnlineStats *stats, uint64_t microseconds);

/**
 * Get running mean from online statistics.
 *
 * stats: Online statistics object
 * Returns: Current running mean in milliseconds
 */
double stats_online_mean(OnlineStats *stats);

/**
 * Get running standard deviation.
 * Uses Welford's algorithm for stability.
 *
 * stats: Online statistics object
 * Returns: Current running std dev in milliseconds
 */
double stats_online_stddev(OnlineStats *stats);

/**
 * Calculate comprehensive statistics from sample array.
 * Performs all calculations: mean, median, percentiles, etc.
 * Modifies input array (sorts for percentile calculation).
 *
 * samples: Array of reaction times in microseconds
 * count: Number of samples
 * stats: Output statistics structure (will be filled)
 *
 * Returns: 0 on success, -1 on invalid input
 */
int stats_calculate(uint64_t *samples, int count, Statistics *stats);

/**
 * Calculate percentile from sorted sample array.
 * Uses linear interpolation (Type 7 / R-7).
 *
 * sorted_samples: Pre-sorted array of samples
 * count: Number of samples
 * percentile: Desired percentile (0-100)
 *
 * Returns: Percentile value in milliseconds
 */
double stats_percentile(uint64_t *sorted_samples, int count,
                        double percentile);

/**
 * Detect statistical outliers using Tukey fence method.
 * Modifies outlier flags in sample array.
 *
 * samples: Array of reaction times
 * count: Number of samples
 * outlier_flags: Output array (count elements), set to 1 for outliers
 *
 * Returns: Number of outliers detected
 */
int stats_detect_outliers(uint64_t *samples, int count, uint8_t *outlier_flags);

/**
 * Analyze fatigue trend over session.
 * Divides samples into blocks and calculates trend.
 *
 * samples: Array of reaction times in order
 * count: Number of samples
 * block_size: Trials per block (typically 10)
 *
 * Returns: Fatigue score (-100 to +100), 0 = no fatigue
 */
double stats_analyze_fatigue(uint64_t *samples, int count, int block_size);

/**
 * Calculate consistency score (inverse of variation).
 * Higher score = more consistent (0-100).
 *
 * mean: Average reaction time
 * stddev: Standard deviation
 *
 * Returns: Consistency score 0-100
 */
double stats_consistency_score(double mean, double stddev);

/**
 * Free allocated statistics memory.
 *
 * stats: Statistics structure
 */
void stats_free(Statistics *stats);

/**
 * Free online statistics tracker.
 *
 * stats: OnlineStats object to free
 */
void stats_online_free(OnlineStats *stats);

#endif /* STATISTICS_H */
