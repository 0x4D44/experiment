#include "game.h"
#include "random.h"
#include "ai.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <time.h>

/*
 * GAME SESSION MANAGEMENT
 */

GameSession game_create_session(GameMode mode, DifficultyLevel difficulty)
{
    GameSession session;
    memset(&session, 0, sizeof(session));

    session.game_mode = mode;
    session.difficulty = difficulty;
    session.state = STATE_PLAYING;
    session.start_time = time(NULL);

    /* Initialize range and guess limits based on difficulty */
    session.range_max = difficulty_get_range_max(difficulty);
    session.range_min = 1;
    session.max_guesses = difficulty_get_max_guesses(difficulty);
    session.hints_available = difficulty_get_max_hints(difficulty);
    session.hints_used = 0;

    /* Generate secret number */
    session.secret_number = generate_random_number(session.range_min, session.range_max);

    return session;
}

void game_initialize_session(GameSession *session, GameMode mode, DifficultyLevel difficulty)
{
    if (session == NULL) return;

    GameSession new_session = game_create_session(mode, difficulty);
    *session = new_session;
}

/*
 * DIFFICULTY LEVEL FUNCTIONS
 */

int difficulty_get_range_max(DifficultyLevel diff)
{
    /* Range = 10 * (2 ^ (difficulty_level - 1)) */
    switch (diff) {
        case DIFFICULTY_EASY:
            return 10;
        case DIFFICULTY_MEDIUM:
            return 100;
        case DIFFICULTY_HARD:
            return 1000;
        case DIFFICULTY_EXPERT:
            return 10000;
        case DIFFICULTY_MASTER:
            return 100000;
        default:
            return 100;
    }
}

int difficulty_get_max_guesses(DifficultyLevel diff)
{
    /* Max Guesses = 5 + (difficulty_level * 2.5) rounded up */
    switch (diff) {
        case DIFFICULTY_EASY:
            return 999;  /* Unlimited */
        case DIFFICULTY_MEDIUM:
            return 10;
        case DIFFICULTY_HARD:
            return 15;
        case DIFFICULTY_EXPERT:
            return 20;
        case DIFFICULTY_MASTER:
            return 25;
        default:
            return 10;
    }
}

int difficulty_get_max_hints(DifficultyLevel diff)
{
    switch (diff) {
        case DIFFICULTY_EASY:
            return 999;  /* Unlimited hints */
        case DIFFICULTY_MEDIUM:
            return 5;
        case DIFFICULTY_HARD:
            return 3;
        case DIFFICULTY_EXPERT:
            return 2;
        case DIFFICULTY_MASTER:
            return 1;
        default:
            return 5;
    }
}

float difficulty_get_hint_cost(DifficultyLevel diff)
{
    switch (diff) {
        case DIFFICULTY_EASY:
            return 1.0f;  /* No cost */
        case DIFFICULTY_MEDIUM:
            return 2.0f;  /* Can use 1 per 2 guesses */
        case DIFFICULTY_HARD:
            return 3.0f;
        case DIFFICULTY_EXPERT:
            return 4.0f;
        case DIFFICULTY_MASTER:
            return 5.0f;
        default:
            return 2.0f;
    }
}

/*
 * GAME FEEDBACK AND CHECKING
 */

Feedback game_get_feedback(int guess, int secret)
{
    if (guess == secret) {
        return FEEDBACK_CORRECT;
    } else if (guess > secret) {
        return FEEDBACK_TOO_HIGH;
    } else {
        return FEEDBACK_TOO_LOW;
    }
}

int game_process_guess(GameSession *session, int guess)
{
    if (session == NULL || session->state != STATE_PLAYING) {
        return -1;
    }

    /* Validate guess */
    if (guess < session->range_min || guess > session->range_max) {
        return -1;  /* Invalid guess */
    }

    /* Record guess */
    session->guesses_history[session->guess_count] = guess;
    session->guess_count++;

    /* Check if correct */
    Feedback feedback = game_get_feedback(guess, session->secret_number);

    if (feedback == FEEDBACK_CORRECT) {
        session->state = STATE_WON;
        session->correct_guess = guess;
        session->end_time = time(NULL);
        return 1;  /* Win */
    }

    /* Check if out of guesses */
    if (session->guess_count >= session->max_guesses) {
        session->state = STATE_LOST;
        session->end_time = time(NULL);
        return -2;  /* Lose */
    }

    /* Narrow range based on feedback */
    if (feedback == FEEDBACK_TOO_HIGH) {
        session->range_max = guess - 1;
    } else if (feedback == FEEDBACK_TOO_LOW) {
        session->range_min = guess + 1;
    }

    return 0;  /* Continue playing */
}

int game_check_win(GameSession *session)
{
    return session && session->state == STATE_WON;
}

int game_check_lose(GameSession *session)
{
    return session && session->state == STATE_LOST;
}

/*
 * SCORING SYSTEM
 */

int game_calculate_score(GameSession *session, float time_seconds)
{
    if (session == NULL || session->state != STATE_WON) {
        return 0;
    }

    /* Base points: 100 - (guesses * 2) */
    int base_points = 100 - (session->guess_count * 2);
    if (base_points < 0) base_points = 0;

    /* Time bonus: 5 points per second saved (capped at 30) */
    int time_bonus = 0;
    if (time_seconds < 60.0f) {
        time_bonus = (int)((60.0f - time_seconds) / 12.0f);  /* Max 5 points per second
 */
        if (time_bonus > 30) time_bonus = 30;
    }

    /* Difficulty multiplier */
    int difficulty_multiplier = 1;
    switch (session->difficulty) {
        case DIFFICULTY_EASY:
            difficulty_multiplier = 1;
            break;
        case DIFFICULTY_MEDIUM:
            difficulty_multiplier = 2;
            break;
        case DIFFICULTY_HARD:
            difficulty_multiplier = 4;
            break;
        case DIFFICULTY_EXPERT:
            difficulty_multiplier = 8;
            break;
        case DIFFICULTY_MASTER:
            difficulty_multiplier = 16;
            break;
        default:
            difficulty_multiplier = 1;
    }

    /* Hint penalty: -10 per hint */
    int hint_penalty = session->hints_used * 10;

    /* Calculate final score */
    int final_score = (base_points + time_bonus) * difficulty_multiplier - hint_penalty;
    if (final_score < 0) final_score = 0;

    return final_score;
}

/*
 * GAME STATE FUNCTIONS
 */

void game_print_current_state(GameSession *session)
{
    if (session == NULL) return;

    printf("\n");
    printf("╔════════════════════════════════════════╗\n");
    printf("║ GAME STATUS                            ║\n");
    printf("╠════════════════════════════════════════╣\n");
    printf("║ Mode:      %s\n", session->game_mode == MODE_CLASSIC ? "Classic" : "Other");
    printf("║ Difficulty: %d (Range: %d-%d)\n",
           session->difficulty, session->range_min, session->range_max);
    printf("║ Guesses: %d/%d\n", session->guess_count, session->max_guesses);
    printf("║ Hints:    %d/%d\n", session->hints_used, session->hints_available);
    printf("╚════════════════════════════════════════╝\n");
    printf("\n");
}

void game_print_guess_history(GameSession *session)
{
    if (session == NULL || session->guess_count == 0) return;

    printf("History: ");
    for (int i = 0; i < session->guess_count; i++) {
        printf("%d ", session->guesses_history[i]);
        if (i > 0) {
            if (session->guesses_history[i] > session->guesses_history[i - 1]) {
                printf("(↑) ");
            } else {
                printf("(↓) ");
            }
        }
    }
    printf("\n\n");
}

void game_print_range_bar(GameSession *session)
{
    if (session == NULL) return;

    int bar_width = 40;
    int range_size = session->range_max - session->range_min + 1;
    int used = ((session->range_max - session->range_min + 1) * bar_width) / range_size;

    printf("Range: [%d", session->range_min);
    for (int i = 0; i < bar_width; i++) {
        if (i < used) {
            printf("=");
        } else {
            printf("-");
        }
    }
    printf(" %d]\n\n", session->range_max);
}

/*
 * UTILITY FUNCTIONS FOR TESTING AND GAME LOGIC
 */

int is_valid_guess(int guess, int min, int max)
{
    return guess >= min && guess <= max;
}

int is_valid_range(int min, int max)
{
    return min >= 0 && max > min && (max - min) >= 1;
}

int calculate_classic_score(int guesses, int max_guesses, int difficulty, int hints_used, float time_seconds)
{
    (void)max_guesses;  /* Unused in simple calculation */

    /* Base points: 100 - (guesses * 2) */
    int base_points = 100 - (guesses * 2);
    if (base_points < 0) base_points = 0;

    /* Time bonus: 5 points per second saved (capped at 30) */
    int time_bonus = 0;
    if (time_seconds < 60.0f) {
        time_bonus = (int)((60.0f - time_seconds) / 12.0f);
        if (time_bonus > 30) time_bonus = 30;
    }

    /* Difficulty multiplier */
    int multiplier = 1 + difficulty;  /* 1-5 based on difficulty */

    /* Hint penalty: -10 per hint */
    int hint_penalty = hints_used * 10;

    int final_score = (base_points + time_bonus) * multiplier - hint_penalty;
    if (final_score < 0) final_score = 0;

    return final_score;
}

int calculate_challenge_score(int guesses, int max_guesses, int difficulty, int streak)
{
    /* Points = (Max_Guesses - Guesses_Used) * Difficulty_Multiplier * Streak_Bonus */
    int remaining = max_guesses - guesses;
    int base_points = remaining * (5 * difficulty);

    /* Streak multiplier: 1x (no streak) ... 3x (6+ wins) */
    float streak_multiplier = 1.0f + (streak * 0.5f);
    if (streak_multiplier > 3.0f) streak_multiplier = 3.0f;

    int final_score = (int)(base_points * streak_multiplier);
    if (final_score < 0) final_score = 0;

    return final_score;
}

float calculate_win_rate(int wins, int total)
{
    if (total == 0) return 0.0f;
    return (float)wins / total;
}

float calculate_average_guesses(int total_guesses, int games_played)
{
    if (games_played == 0) return 0.0f;
    return (float)total_guesses / games_played;
}
