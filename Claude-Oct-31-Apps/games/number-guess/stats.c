#include "stats.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

/*
 * STATISTICS FILE OPERATIONS
 */

int stats_load_from_file(const char *filename, PlayerStats *stats)
{
    if (filename == NULL || stats == NULL) return -1;

    FILE *file = fopen(filename, "rb");
    if (file == NULL) {
        /* File doesn't exist, initialize with defaults */
        memset(stats, 0, sizeof(PlayerStats));
        return 0;
    }

    size_t read = fread(stats, sizeof(PlayerStats), 1, file);
    fclose(file);

    return (read == 1) ? 0 : -1;
}

int stats_save_to_file(const char *filename, const PlayerStats *stats)
{
    if (filename == NULL || stats == NULL) return -1;

    FILE *file = fopen(filename, "wb");
    if (file == NULL) return -1;

    size_t written = fwrite(stats, sizeof(PlayerStats), 1, file);
    fclose(file);

    return (written == 1) ? 0 : -1;
}

/*
 * STATISTICS CALCULATION
 */

float stats_calculate_win_rate(PlayerStats *stats)
{
    if (stats == NULL || stats->total_games_played == 0) return 0.0f;

    return (float)stats->total_wins / stats->total_games_played;
}

float stats_calculate_average_guesses(PlayerStats *stats)
{
    if (stats == NULL || stats->total_wins == 0) return 0.0f;

    return (float)stats->total_guesses / stats->total_wins;
}

float stats_calculate_average_time(PlayerStats *stats)
{
    if (stats == NULL || stats->total_games_played == 0) return 0.0f;

    return stats->total_time_seconds / stats->total_games_played;
}

/*
 * STATISTICS UPDATE
 */

void stats_update_after_game(PlayerStats *stats, GameSession *session, int score __attribute__((unused)))
{
    if (stats == NULL || session == NULL) return;

    stats->total_games_played++;
    stats->last_game_time = time(NULL);

    if (session->state == STATE_WON) {
        stats->total_wins++;
        stats->total_guesses += session->guess_count;

        if (session->guess_count < stats->best_game_guesses ||
            stats->best_game_guesses == 0) {
            stats->best_game_guesses = session->guess_count;
        }

        stats_increment_win_streak(stats);
        if (stats->current_streak > stats->longest_streak) {
            stats->longest_streak = stats->current_streak;
        }
    } else {
        stats->total_losses++;
        stats_reset_win_streak(stats);
    }

    if (session->guess_count > stats->worst_game_guesses) {
        stats->worst_game_guesses = session->guess_count;
    }

    /* Update difficulty stats */
    int diff_idx = (int)session->difficulty;
    if (diff_idx >= 0 && diff_idx < 6) {
        stats->games_by_difficulty[diff_idx]++;
        if (session->state == STATE_WON) {
            stats->wins_by_difficulty[diff_idx]++;
        }
    }
}

void stats_record_guess(PlayerStats *stats, int guess_count)
{
    if (stats == NULL) return;
    stats->total_guesses += guess_count;
}

void stats_increment_win_streak(PlayerStats *stats)
{
    if (stats == NULL) return;
    stats->current_streak++;
}

void stats_reset_win_streak(PlayerStats *stats)
{
    if (stats == NULL) return;
    stats->current_streak = 0;
}

/*
 * ACHIEVEMENT SYSTEM
 */

int achievement_is_unlocked(PlayerStats *stats, Achievement achievement)
{
    if (stats == NULL) return 0;

    for (int i = 0; i < stats->achievements_unlocked; i++) {
        if ((Achievement)stats->achievements_list[i] == achievement) {
            return 1;
        }
    }
    return 0;
}

void achievement_unlock(PlayerStats *stats, Achievement achievement)
{
    if (stats == NULL) return;

    if (!achievement_is_unlocked(stats, achievement)) {
        if (stats->achievements_unlocked < MAX_ACHIEVEMENTS) {
            stats->achievements_list[stats->achievements_unlocked] = achievement;
            stats->achievements_unlocked++;
        }
    }
}

const char* achievement_get_name(Achievement achievement)
{
    switch (achievement) {
        case ACHIEVEMENT_FIRST_WIN:
            return "First Victory";
        case ACHIEVEMENT_STREAK_5:
            return "5-Win Streak";
        case ACHIEVEMENT_PERFECT_ROUND:
            return "Perfect Round";
        case ACHIEVEMENT_SPEED_DEMON:
            return "Speed Demon";
        case ACHIEVEMENT_MATHEMATICIAN:
            return "Mathematician";
        case ACHIEVEMENT_EXPERT_SOLVER:
            return "Expert Solver";
        case ACHIEVEMENT_AI_SLAYER:
            return "AI Slayer";
        case ACHIEVEMENT_MEMORY_CHAMPION:
            return "Memory Champion";
        case ACHIEVEMENT_NO_HINTS:
            return "No Hints Run";
        case ACHIEVEMENT_COMEBACK:
            return "Comeback King";
        case ACHIEVEMENT_DIVERSITY:
            return "Diversity Master";
        case ACHIEVEMENT_COLLECTOR:
            return "Achievement Collector";
        default:
            return "Unknown";
    }
}

const char* achievement_get_description(Achievement achievement)
{
    switch (achievement) {
        case ACHIEVEMENT_FIRST_WIN:
            return "Win your first game";
        case ACHIEVEMENT_STREAK_5:
            return "Win 5 consecutive games";
        case ACHIEVEMENT_PERFECT_ROUND:
            return "Guess on first try";
        case ACHIEVEMENT_SPEED_DEMON:
            return "Win in under 20 seconds";
        case ACHIEVEMENT_MATHEMATICIAN:
            return "Win Medium without hints";
        case ACHIEVEMENT_EXPERT_SOLVER:
            return "Win Expert difficulty";
        case ACHIEVEMENT_AI_SLAYER:
            return "Defeat Expert AI";
        case ACHIEVEMENT_MEMORY_CHAMPION:
            return "Complete Memory mode";
        case ACHIEVEMENT_NO_HINTS:
            return "Win without using hints";
        case ACHIEVEMENT_COMEBACK:
            return "Win with only 1 guess left";
        case ACHIEVEMENT_DIVERSITY:
            return "Play all 5 game modes";
        case ACHIEVEMENT_COLLECTOR:
            return "Unlock 10 achievements";
        default:
            return "Unknown achievement";
    }
}

/*
 * LEADERBOARD SYSTEM
 */

int leaderboard_load(const char *filename, LeaderboardEntry *entries, int max_entries)
{
    if (filename == NULL || entries == NULL) return 0;

    FILE *file = fopen(filename, "rb");
    if (file == NULL) return 0;

    int count = 0;
    while (count < max_entries && fread(&entries[count], sizeof(LeaderboardEntry), 1, file)) {
        count++;
    }

    fclose(file);
    return count;
}

int leaderboard_save(const char *filename, const LeaderboardEntry *entries, int count)
{
    if (filename == NULL || entries == NULL) return -1;

    FILE *file = fopen(filename, "wb");
    if (file == NULL) return -1;

    for (int i = 0; i < count; i++) {
        fwrite(&entries[i], sizeof(LeaderboardEntry), 1, file);
    }

    fclose(file);
    return 0;
}

int leaderboard_is_high_score(const LeaderboardEntry *entries, int count, int score)
{
    if (entries == NULL) return 1;

    if (count < MAX_LEADERBOARD_ENTRIES) return 1;

    return score > entries[count - 1].score;
}

void leaderboard_insert_score(LeaderboardEntry *entries, int *count, int max_entries,
                             const char *name, int score, int difficulty, int guesses)
{
    if (entries == NULL || count == NULL || name == NULL) return;

    LeaderboardEntry new_entry;
    strncpy(new_entry.name, name, MAX_PLAYER_NAME - 1);
    new_entry.name[MAX_PLAYER_NAME - 1] = '\0';
    new_entry.score = score;
    new_entry.difficulty = difficulty;
    new_entry.guesses = guesses;
    new_entry.timestamp = time(NULL);

    /* Find insertion point */
    int insert_pos = *count;
    for (int i = 0; i < *count; i++) {
        if (score > entries[i].score) {
            insert_pos = i;
            break;
        }
    }

    /* Shift entries */
    for (int i = *count - 1; i >= insert_pos; i--) {
        if (i + 1 < max_entries) {
            entries[i + 1] = entries[i];
        }
    }

    /* Insert new entry */
    entries[insert_pos] = new_entry;

    if (*count < max_entries) {
        (*count)++;
    }
}

void leaderboard_print(const LeaderboardEntry *entries, int count)
{
    if (entries == NULL) return;

    printf("\n╔═══════════════════════════════════════╗\n");
    printf("║        LEADERBOARD (Top 10)           ║\n");
    printf("╠═══════════════════════════════════════╣\n");

    for (int i = 0; i < count && i < MAX_LEADERBOARD_ENTRIES; i++) {
        printf("║ %2d. %-15s %6d pts (D%d, %d guesses)\n",
               i + 1, entries[i].name, entries[i].score,
               entries[i].difficulty, entries[i].guesses);
    }

    printf("╚═══════════════════════════════════════╝\n\n");
}

/*
 * STATISTICS REPORTING
 */

void stats_print_summary(const PlayerStats *stats)
{
    if (stats == NULL) return;

    printf("\n╔═══════════════════════════════════════╗\n");
    printf("║      PLAYER STATISTICS SUMMARY        ║\n");
    printf("╠═══════════════════════════════════════╣\n");
    printf("║ Total Games:      %d\n", stats->total_games_played);
    printf("║ Total Wins:       %d\n", stats->total_wins);
    printf("║ Total Losses:     %d\n", stats->total_losses);
    printf("║ Win Rate:         %.1f%%\n", stats_calculate_win_rate((PlayerStats *)stats) * 100);
    printf("║ Current Streak:   %d\n", stats->current_streak);
    printf("║ Best Streak:      %d\n", stats->longest_streak);
    printf("║ Avg Guesses:      %.1f\n", stats_calculate_average_guesses((PlayerStats *)stats));
    printf("║ Best Game:        %d guesses\n", stats->best_game_guesses);
    printf("║ Worst Game:       %d guesses\n", stats->worst_game_guesses);
    printf("║ Achievements:     %d\n", stats->achievements_unlocked);
    printf("╚═══════════════════════════════════════╝\n\n");
}

void stats_print_difficulty_stats(const PlayerStats *stats)
{
    if (stats == NULL) return;

    printf("\n╔═══════════════════════════════════════╗\n");
    printf("║     STATISTICS BY DIFFICULTY          ║\n");
    printf("╠═══════════════════════════════════════╣\n");

    const char *difficulty_names[] = {
        "Unknown", "Easy", "Medium", "Hard", "Expert", "Master"
    };

    for (int i = 1; i < 6; i++) {
        int games = stats->games_by_difficulty[i];
        int wins = stats->wins_by_difficulty[i];
        float rate = (games > 0) ? (float)wins / games : 0.0f;

        printf("║ %s:    %d games, %d wins (%.1f%%)\n",
               difficulty_names[i], games, wins, rate * 100);
    }

    printf("╚═══════════════════════════════════════╝\n\n");
}

void stats_print_achievement_list(const PlayerStats *stats)
{
    if (stats == NULL) return;

    printf("\n╔═══════════════════════════════════════╗\n");
    printf("║         ACHIEVEMENTS UNLOCKED         ║\n");
    printf("╠═══════════════════════════════════════╣\n");

    if (stats->achievements_unlocked == 0) {
        printf("║ No achievements unlocked yet!\n");
    } else {
        for (int i = 0; i < stats->achievements_unlocked && i < MAX_ACHIEVEMENTS; i++) {
            printf("║ * %s\n", achievement_get_name((Achievement)stats->achievements_list[i]));
        }
    }

    printf("╚═══════════════════════════════════════╝\n\n");
}
