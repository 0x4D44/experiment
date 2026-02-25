/**
 * persistence.c - Data storage and leaderboard management
 *
 * JSON-based session storage and leaderboard persistence.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>
#include "persistence.h"

/* Helper to get data directory path */
static void get_data_dir(char *path, size_t size) {
    const char *home = getenv("HOME");
    if (home == NULL) {
        home = ".";
    }
    snprintf(path, size, "%s/%s", home, DATA_DIR);
}

/**
 * Initialize persistence system.
 */
int persistence_init(void) {
    char data_dir[512];
    get_data_dir(data_dir, sizeof(data_dir));

    /* Create directories if they don't exist */
    mkdir(data_dir, 0755);

    char stats_dir[512];
    snprintf(stats_dir, sizeof(stats_dir), "%s/%s", data_dir, STATS_DIR);
    mkdir(stats_dir, 0755);

    char leaderboard_dir[512];
    snprintf(leaderboard_dir, sizeof(leaderboard_dir), "%s/%s", data_dir,
             LEADERBOARD_DIR);
    mkdir(leaderboard_dir, 0755);

    return 0;
}

/**
 * Save a test session to disk.
 */
int persistence_save_session(const TestSession *session) {
    if (session == NULL) {
        return -1;
    }

    char data_dir[512];
    get_data_dir(data_dir, sizeof(data_dir));

    char filename[512];
    time_t now = time(NULL);
    struct tm *timeinfo = localtime(&now);
    char timestamp[32];
    strftime(timestamp, sizeof(timestamp), "%Y%m%d-%H%M%S", timeinfo);

    snprintf(filename, sizeof(filename), "%s/%s/session-%s.json", data_dir,
             STATS_DIR, timestamp);

    FILE *fp = fopen(filename, "w");
    if (fp == NULL) {
        return -1;
    }

    /* Write JSON header */
    fprintf(fp, "{\n");
    fprintf(fp, "  \"session_id\": \"%s\",\n", timestamp);
    fprintf(fp, "  \"test_type\": %d,\n", session->config.test_type);
    fprintf(fp, "  \"test_mode\": %d,\n", session->config.test_mode);
    fprintf(fp, "  \"player_name\": \"%s\",\n", session->config.player_name);
    fprintf(fp, "  \"completed_trials\": %d,\n", session->completed_trials);
    fprintf(fp, "  \"average_ms\": %.1f,\n", session->stats.mean_ms);
    fprintf(fp, "  \"median_ms\": %.1f,\n", session->stats.median_ms);
    fprintf(fp, "  \"std_dev_ms\": %.1f\n", session->stats.std_dev_ms);
    fprintf(fp, "}\n");

    fclose(fp);
    return 0;
}

/**
 * Load a session from disk.
 */
TestSession* persistence_load_session(const char *session_id) {
    if (session_id == NULL) {
        return NULL;
    }

    char data_dir[512];
    get_data_dir(data_dir, sizeof(data_dir));

    char filename[512];
    snprintf(filename, sizeof(filename), "%s/%s/session-%s.json", data_dir,
             STATS_DIR, session_id);

    FILE *fp = fopen(filename, "r");
    if (fp == NULL) {
        return NULL;
    }

    TestSession *session = malloc(sizeof(TestSession));
    if (session == NULL) {
        fclose(fp);
        return NULL;
    }

    memset(session, 0, sizeof(TestSession));

    /* Parse JSON (simplified) */
    char buffer[1024];
    while (fgets(buffer, sizeof(buffer), fp) != NULL) {
        if (strstr(buffer, "\"test_type\"") != NULL) {
            sscanf(buffer, "  \"test_type\": %d", &session->config.test_type);
        }
    }

    fclose(fp);
    return session;
}

/**
 * Get recent sessions.
 */
char** persistence_get_recent_sessions(int limit) {
    char data_dir[512];
    get_data_dir(data_dir, sizeof(data_dir));

    char stats_dir[512];
    snprintf(stats_dir, sizeof(stats_dir), "%s/%s", data_dir, STATS_DIR);

    /* Would read directory and return session IDs */
    /* For now, return empty array */
    char **sessions = malloc(1 * sizeof(char *));
    if (sessions != NULL) {
        sessions[0] = NULL;
    }
    return sessions;
}

/**
 * Update leaderboard.
 */
int persistence_update_leaderboard(const TestSession *session) {
    if (session == NULL) {
        return -1;
    }

    char data_dir[512];
    get_data_dir(data_dir, sizeof(data_dir));

    char leaderboard_file[512];
    snprintf(leaderboard_file, sizeof(leaderboard_file),
             "%s/%s/type-%d.json", data_dir, LEADERBOARD_DIR,
             session->config.test_type);

    FILE *fp = fopen(leaderboard_file, "a");
    if (fp == NULL) {
        return -1;
    }

    /* Append entry */
    fprintf(fp, "{\n");
    fprintf(fp, "  \"player_name\": \"%s\",\n", session->config.player_name);
    fprintf(fp, "  \"average_ms\": %.1f,\n", session->stats.mean_ms);
    fprintf(fp, "  \"test_mode\": %d,\n", session->config.test_mode);
    fprintf(fp, "  \"sample_count\": %d\n", session->completed_trials);
    fprintf(fp, "},\n");

    fclose(fp);
    return 0;
}

/**
 * Load leaderboard.
 */
int persistence_load_leaderboard(int test_type, int limit,
                                  LeaderboardEntry *entries) {
    if (entries == NULL || limit <= 0) {
        return -1;
    }

    char data_dir[512];
    get_data_dir(data_dir, sizeof(data_dir));

    char leaderboard_file[512];
    snprintf(leaderboard_file, sizeof(leaderboard_file),
             "%s/%s/type-%d.json", data_dir, LEADERBOARD_DIR, test_type);

    FILE *fp = fopen(leaderboard_file, "r");
    if (fp == NULL) {
        return 0;  /* No leaderboard yet */
    }

    int count = 0;
    /* Would parse JSON and populate entries */
    /* For now, return 0 */

    fclose(fp);
    return count;
}

/**
 * Get percentile rank.
 */
int persistence_get_percentile_rank(const char *player_name, int test_type) {
    if (player_name == NULL) {
        return -1;
    }

    /* Would calculate percentile from leaderboard */
    return 50;  /* Default to 50th percentile */
}

/**
 * Get daily averages.
 */
double* persistence_get_daily_averages(int num_days) {
    double *averages = malloc(num_days * sizeof(double));
    if (averages == NULL) {
        return NULL;
    }

    memset(averages, 0, num_days * sizeof(double));
    return averages;
}

/**
 * Get all-time stats.
 */
int persistence_get_all_time_stats(const char *player_name,
                                    Statistics *stats) {
    if (player_name == NULL || stats == NULL) {
        return -1;
    }

    memset(stats, 0, sizeof(Statistics));
    /* Would aggregate all sessions for player */
    return 0;
}

/**
 * Export to CSV.
 */
int persistence_export_csv(const TestSession *session, const char *filename) {
    if (session == NULL || filename == NULL) {
        return -1;
    }

    FILE *fp = fopen(filename, "w");
    if (fp == NULL) {
        return -1;
    }

    fprintf(fp, "Trial,ReactionTime(ms),Valid,Outlier\n");

    for (uint32_t i = 0; i < session->completed_trials; i++) {
        fprintf(fp, "%d,%.1f,%d,%d\n",
                i + 1,
                (double)session->results[i].reaction_time_us / 1000.0,
                session->results[i].is_valid,
                session->results[i].is_outlier);
    }

    fclose(fp);
    return 0;
}

/**
 * Export to JSON.
 */
int persistence_export_json(const TestSession *session, const char *filename) {
    if (session == NULL || filename == NULL) {
        return -1;
    }

    FILE *fp = fopen(filename, "w");
    if (fp == NULL) {
        return -1;
    }

    /* Would write full JSON export */
    fprintf(fp, "{\n");
    fprintf(fp, "  \"session\": {\n");
    fprintf(fp, "    \"test_type\": %d,\n", session->config.test_type);
    fprintf(fp, "    \"completed_trials\": %d\n", session->completed_trials);
    fprintf(fp, "  }\n");
    fprintf(fp, "}\n");

    fclose(fp);
    return 0;
}

/**
 * Import JSON.
 */
int persistence_import_json(const char *filename) {
    if (filename == NULL) {
        return -1;
    }

    FILE *fp = fopen(filename, "r");
    if (fp == NULL) {
        return -1;
    }

    /* Would parse JSON and merge */

    fclose(fp);
    return 0;
}

/**
 * Clear all data.
 */
int persistence_clear_all(void) {
    char data_dir[512];
    get_data_dir(data_dir, sizeof(data_dir));

    /* Would recursively delete data directory */
    return 0;
}

/**
 * Delete session.
 */
int persistence_delete_session(const char *session_id) {
    if (session_id == NULL) {
        return -1;
    }

    char data_dir[512];
    get_data_dir(data_dir, sizeof(data_dir));

    char filename[512];
    snprintf(filename, sizeof(filename), "%s/%s/session-%s.json", data_dir,
             STATS_DIR, session_id);

    return unlink(filename);
}

/**
 * Get storage stats.
 */
int persistence_get_storage_stats(uint64_t *total_bytes, int *session_count) {
    if (total_bytes == NULL || session_count == NULL) {
        return -1;
    }

    *total_bytes = 0;
    *session_count = 0;

    return 0;
}

/**
 * Cleanup old sessions.
 */
int persistence_cleanup_old_sessions(int max_age_days) {
    return 0;
}
