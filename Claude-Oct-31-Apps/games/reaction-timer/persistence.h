/**
 * persistence.h - Data storage and leaderboard management
 *
 * Provides session persistence, statistics storage, and
 * leaderboard management with JSON serialization.
 */

#ifndef PERSISTENCE_H
#define PERSISTENCE_H

#include <stdint.h>
#include <time.h>
#include "test_engine.h"
#include "statistics.h"

/* Data directory paths */
#define DATA_DIR ".reaction-timer"
#define STATS_DIR "stats"
#define LEADERBOARD_DIR "leaderboards"

/* Leaderboard entry */
typedef struct {
    int rank;                   /* Leaderboard position */
    char player_name[64];       /* Player name */
    time_t date;                /* When achieved */
    double average_ms;          /* Average reaction time */
    int test_mode;              /* Mode played */
    int test_type;              /* Test type */
    int sample_count;           /* Number of trials */
    char device[32];            /* Device name (local/cloud) */
} LeaderboardEntry;

/**
 * Initialize persistence system.
 * Creates data directories if they don't exist.
 *
 * Returns: 0 on success, -1 on failure
 */
int persistence_init(void);

/**
 * Save a test session to disk.
 * Creates JSON file with session ID timestamp.
 *
 * session: Completed test session to save
 *
 * Returns: 0 on success, -1 on failure
 */
int persistence_save_session(const TestSession *session);

/**
 * Load a previous session from disk.
 * Reconstructs TestSession from JSON file.
 *
 * session_id: Timestamp or filename of session
 *
 * Returns: Pointer to loaded TestSession, NULL on failure
 */
TestSession* persistence_load_session(const char *session_id);

/**
 * Get list of recent sessions.
 * Useful for history viewing.
 *
 * limit: Maximum sessions to return
 * Returns: Array of session IDs (newest first), NULL-terminated
 */
char** persistence_get_recent_sessions(int limit);

/**
 * Save result to leaderboard.
 * Maintains top 100 scores per test type.
 *
 * session: Test session with results
 *
 * Returns: 0 on success, -1 on failure
 */
int persistence_update_leaderboard(const TestSession *session);

/**
 * Load leaderboard for a specific test type.
 * Returns sorted list of top scores.
 *
 * test_type: From TestType enum (1-7)
 * limit: Maximum entries to return
 * entries: Output array (must be allocated by caller)
 *
 * Returns: Number of entries loaded, -1 on failure
 */
int persistence_load_leaderboard(int test_type, int limit,
                                  LeaderboardEntry *entries);

/**
 * Get player's position and stats on leaderboard.
 * For "you are faster than X% of players" message.
 *
 * player_name: Player to look up
 * test_type: Type of test
 *
 * Returns: Percentile (0-100), -1 if not found
 */
int persistence_get_percentile_rank(const char *player_name, int test_type);

/**
 * Get aggregated daily statistics.
 * Averages all sessions for each day.
 *
 * num_days: How many days of history to retrieve
 * Returns: Pointer to array of daily stats, NULL on failure
 */
double* persistence_get_daily_averages(int num_days);

/**
 * Get all-time statistics for a player.
 * Aggregates all sessions ever played.
 *
 * player_name: Player to look up
 * stats: Output statistics structure
 *
 * Returns: 0 on success, -1 if player not found
 */
int persistence_get_all_time_stats(const char *player_name, Statistics *stats);

/**
 * Export session data to CSV format.
 * Creates CSV file suitable for spreadsheet import.
 *
 * session: Session to export
 * filename: Output filename
 *
 * Returns: 0 on success, -1 on failure
 */
int persistence_export_csv(const TestSession *session, const char *filename);

/**
 * Export session data to JSON format.
 * Creates JSON file for cloud sync or analysis.
 *
 * session: Session to export
 * filename: Output filename
 *
 * Returns: 0 on success, -1 on failure
 */
int persistence_export_json(const TestSession *session, const char *filename);

/**
 * Import and merge leaderboard data from JSON.
 * Used for cloud sync functionality.
 *
 * filename: JSON file to import
 *
 * Returns: 0 on success, -1 on failure
 */
int persistence_import_json(const char *filename);

/**
 * Clear all local data (careful!).
 * Used for reset/cleanup operations.
 *
 * Returns: 0 on success, -1 on failure
 */
int persistence_clear_all(void);

/**
 * Delete a specific session from history.
 *
 * session_id: Session to delete
 *
 * Returns: 0 on success, -1 on failure
 */
int persistence_delete_session(const char *session_id);

/**
 * Get storage usage statistics.
 *
 * total_bytes: Output total data size
 * session_count: Output number of stored sessions
 *
 * Returns: 0 on success, -1 on failure
 */
int persistence_get_storage_stats(uint64_t *total_bytes, int *session_count);

/**
 * Vacuum/cleanup storage by removing old sessions.
 * Keeps only recent history.
 *
 * max_age_days: Delete sessions older than this many days
 *
 * Returns: Number of deleted sessions, -1 on failure
 */
int persistence_cleanup_old_sessions(int max_age_days);

#endif /* PERSISTENCE_H */
