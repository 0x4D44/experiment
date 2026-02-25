#ifndef STATS_H
#define STATS_H

#include "game.h"

/*
 * STATISTICS AND ACHIEVEMENT SYSTEM
 * Tracks player progress and unlocks achievements
 */

/* Statistics file operations */
int stats_load_from_file(const char *filename, PlayerStats *stats);
int stats_save_to_file(const char *filename, const PlayerStats *stats);

/* Statistics calculation */
float stats_calculate_win_rate(PlayerStats *stats);
float stats_calculate_average_guesses(PlayerStats *stats);
float stats_calculate_average_time(PlayerStats *stats);

/* Statistics update */
void stats_update_after_game(PlayerStats *stats, GameSession *session, int score);
void stats_record_guess(PlayerStats *stats, int guess_count);
void stats_increment_win_streak(PlayerStats *stats);
void stats_reset_win_streak(PlayerStats *stats);

/* Achievement system */
typedef enum {
    ACHIEVEMENT_FIRST_WIN = 0,
    ACHIEVEMENT_STREAK_5 = 1,
    ACHIEVEMENT_PERFECT_ROUND = 2,
    ACHIEVEMENT_SPEED_DEMON = 3,
    ACHIEVEMENT_MATHEMATICIAN = 4,
    ACHIEVEMENT_EXPERT_SOLVER = 5,
    ACHIEVEMENT_AI_SLAYER = 6,
    ACHIEVEMENT_MEMORY_CHAMPION = 7,
    ACHIEVEMENT_NO_HINTS = 8,
    ACHIEVEMENT_COMEBACK = 9,
    ACHIEVEMENT_DIVERSITY = 10,
    ACHIEVEMENT_COLLECTOR = 11
} Achievement;

int achievement_is_unlocked(PlayerStats *stats, Achievement achievement);
void achievement_unlock(PlayerStats *stats, Achievement achievement);
const char* achievement_get_name(Achievement achievement);
const char* achievement_get_description(Achievement achievement);

/* Leaderboard system */
typedef struct {
    char name[MAX_PLAYER_NAME];
    int score;
    int difficulty;
    int guesses;
    time_t timestamp;
} LeaderboardEntry;

int leaderboard_load(const char *filename, LeaderboardEntry *entries, int max_entries);
int leaderboard_save(const char *filename, const LeaderboardEntry *entries, int count);
int leaderboard_is_high_score(const LeaderboardEntry *entries, int count, int score);
void leaderboard_insert_score(LeaderboardEntry *entries, int *count, int max_entries,
                             const char *name, int score, int difficulty, int guesses);
void leaderboard_print(const LeaderboardEntry *entries, int count);

/* Statistics reporting */
void stats_print_summary(const PlayerStats *stats);
void stats_print_difficulty_stats(const PlayerStats *stats);
void stats_print_achievement_list(const PlayerStats *stats);

#endif
