#ifndef GAME_H
#define GAME_H

#include <time.h>

/*
 * GAME CONSTANTS
 */

#define MAX_GUESSES_HISTORY 100
#define MAX_PLAYER_NAME 50
#define MAX_ACHIEVEMENTS 20
#define MAX_LEADERBOARD_ENTRIES 10

/* Game modes */
typedef enum {
    MODE_CLASSIC = 1,
    MODE_REVERSE = 2,
    MODE_VERSUS = 3,
    MODE_CHALLENGE = 4,
    MODE_MEMORY = 5
} GameMode;

/* Difficulty levels */
typedef enum {
    DIFFICULTY_EASY = 1,
    DIFFICULTY_MEDIUM = 2,
    DIFFICULTY_HARD = 3,
    DIFFICULTY_EXPERT = 4,
    DIFFICULTY_MASTER = 5
} DifficultyLevel;

/* Game states */
typedef enum {
    STATE_MENU = 0,
    STATE_PLAYING = 1,
    STATE_WON = 2,
    STATE_LOST = 3,
    STATE_QUIT = 4
} GameState;

/* Feedback types */
typedef enum {
    FEEDBACK_TOO_HIGH = 1,
    FEEDBACK_TOO_LOW = -1,
    FEEDBACK_CORRECT = 0,
    FEEDBACK_INVALID = 999
} Feedback;

/*
 * STATISTICS STRUCTURE
 */

typedef struct {
    int total_games_played;
    int total_wins;
    int total_losses;
    int current_streak;
    int longest_streak;

    int games_by_difficulty[6];
    int wins_by_difficulty[6];

    int total_guesses;
    int best_game_guesses;
    int worst_game_guesses;

    float total_time_seconds;
    float avg_time_seconds;

    int achievements_unlocked;
    int achievements_list[MAX_ACHIEVEMENTS];

    time_t last_game_time;
    int total_playtime_minutes;
} PlayerStats;

/*
 * GAME SESSION STRUCTURE
 */

typedef struct {
    int secret_number;
    int range_min;
    int range_max;

    int guess_count;
    int max_guesses;
    int guesses_history[MAX_GUESSES_HISTORY];

    int hints_used;
    int hints_available;
    int hints_used_this_game;

    time_t start_time;
    time_t end_time;

    GameMode game_mode;
    DifficultyLevel difficulty;
    GameState state;

    int current_score;
    int streak;

    int correct_guess;  // For stats
} GameSession;

/*
 * AI OPPONENT STRUCTURE
 */

typedef enum {
    AI_RANDOM = 1,
    AI_BINARY_SEARCH = 2,
    AI_PROBABILISTIC = 3,
    AI_MACHINE_LEARNING = 4
} AIStrategy;

typedef struct {
    AIStrategy strategy;
    int guess_count;
    int current_guess;
    int range_min;
    int range_max;
    int guesses_history[MAX_GUESSES_HISTORY];
    int secret_number;  // Known to AI in reverse mode
    time_t start_time;
} AIOpponent;

/*
 * PLAYER PROFILE STRUCTURE
 */

typedef struct {
    char name[MAX_PLAYER_NAME];
    PlayerStats stats;
    GameSession current_session;
} Player;

/*
 * GAME FUNCTIONS
 */

/* Game initialization */
GameSession game_create_session(GameMode mode, DifficultyLevel difficulty);
void game_initialize_session(GameSession *session, GameMode mode, DifficultyLevel difficulty);

/* Game flow */
int game_run_session(GameSession *session, PlayerStats *stats);
int game_process_guess(GameSession *session, int guess);
int game_check_win(GameSession *session);
int game_check_lose(GameSession *session);

/* Difficulty management */
int difficulty_get_range_max(DifficultyLevel diff);
int difficulty_get_max_guesses(DifficultyLevel diff);
int difficulty_get_max_hints(DifficultyLevel diff);
float difficulty_get_hint_cost(DifficultyLevel diff);

/* Feedback and scoring */
Feedback game_get_feedback(int guess, int secret);
int game_calculate_score(GameSession *session, float time_seconds);
void game_update_statistics(GameSession *session, PlayerStats *stats, int score);

/* State management */
void game_print_current_state(GameSession *session);
void game_print_guess_history(GameSession *session);
void game_print_range_bar(GameSession *session);

#endif
