#include "ui.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <time.h>

/*
 * TERMINAL UTILITIES
 */

void ui_clear_screen(void)
{
#ifdef _WIN32
    (void)system("cls");
#else
    (void)system("clear");
#endif
}

void ui_print_title(void)
{
    printf("\n");
    printf("╔══════════════════════════════════════════════════╗\n");
    printf("║                                                  ║\n");
    printf("║          NUMBER GUESS MASTER v1.0               ║\n");
    printf("║                                                  ║\n");
    printf("║        Test your guessing skills!               ║\n");
    printf("║                                                  ║\n");
    printf("╚══════════════════════════════════════════════════╝\n");
    printf("\n");
}

void ui_print_header(const char *title)
{
    if (title == NULL) return;

    int len = strlen(title);
    int padding = (50 - len) / 2;

    printf("\n");
    printf("╔════════════════════════════════════════════════════╗\n");
    for (int i = 0; i < padding; i++) printf("║ ");
    printf("%s\n", title);
    printf("╚════════════════════════════════════════════════════╝\n");
    printf("\n");
}

void ui_print_footer(void)
{
    printf("\n");
    printf("╔════════════════════════════════════════════════════╗\n");
    printf("║ Press [ENTER] to continue...                      ║\n");
    printf("╚════════════════════════════════════════════════════╝\n");
}

void ui_print_separator(int width)
{
    for (int i = 0; i < width; i++) {
        printf("─");
    }
    printf("\n");
}

/*
 * MENU SYSTEMS
 */

int ui_show_main_menu(void)
{
    ui_clear_screen();
    ui_print_title();

    printf("╔════════════════════════════════════════════════════╗\n");
    printf("║                   MAIN MENU                        ║\n");
    printf("╠════════════════════════════════════════════════════╣\n");
    printf("║  1. Play Game                                      ║\n");
    printf("║  2. View Statistics                                ║\n");
    printf("║  3. View Leaderboard                               ║\n");
    printf("║  4. Daily Challenge                                ║\n");
    printf("║  5. Settings                                       ║\n");
    printf("║  6. Help / Tutorial                                ║\n");
    printf("║  7. About                                          ║\n");
    printf("║  8. Exit                                           ║\n");
    printf("╚════════════════════════════════════════════════════╝\n");
    printf("\nSelect option (1-8): ");

    int choice = ui_get_integer_input(1, 8);
    return choice;
}

int ui_show_game_mode_menu(void)
{
    ui_clear_screen();
    ui_print_header("SELECT GAME MODE");

    printf("╔════════════════════════════════════════════════════╗\n");
    printf("║  1. Classic        - Guess the computer's number  ║\n");
    printf("║  2. Reverse        - Computer guesses your number ║\n");
    printf("║  3. Versus AI      - Race against the AI          ║\n");
    printf("║  4. Challenge      - Limited guesses with points  ║\n");
    printf("║  5. Memory         - Guess number sequences       ║\n");
    printf("║  6. Back to Menu                                  ║\n");
    printf("╚════════════════════════════════════════════════════╝\n");
    printf("\nSelect mode (1-6): ");

    int choice = ui_get_integer_input(1, 6);
    return choice;
}

int ui_show_difficulty_menu(void)
{
    ui_clear_screen();
    ui_print_header("SELECT DIFFICULTY");

    printf("╔════════════════════════════════════════════════════╗\n");
    printf("║  1. Easy           - Range 1-10, unlimited guesses║\n");
    printf("║  2. Medium         - Range 1-100, ~10 guesses     ║\n");
    printf("║  3. Hard           - Range 1-1000, ~15 guesses    ║\n");
    printf("║  4. Expert         - Range 1-10000, ~20 guesses   ║\n");
    printf("║  5. Master         - Range 1-100000, ~25 guesses  ║\n");
    printf("║  6. Back to Menu                                  ║\n");
    printf("╚════════════════════════════════════════════════════╝\n");
    printf("\nSelect difficulty (1-6): ");

    int choice = ui_get_integer_input(1, 6);
    return choice;
}

/*
 * IN-GAME DISPLAY
 */

void ui_print_game_header(GameSession *session)
{
    if (session == NULL) return;

    printf("\n");
    printf("╔════════════════════════════════════════════════════╗\n");
    printf("║ Game: Classic  |  Difficulty: Medium (1-100)      ║\n");
    printf("║ Guesses: %d/%-2d  |  Hints: %d/%-2d                        ║\n",
           session->guess_count, session->max_guesses,
           session->hints_used, session->hints_available);
    printf("╚════════════════════════════════════════════════════╝\n");
    printf("\n");
}

void ui_print_game_status(GameSession *session)
{
    if (session == NULL) return;

    time_t elapsed = time(NULL) - session->start_time;
    int minutes = elapsed / 60;
    int seconds = elapsed % 60;

    printf("Time: %02d:%02d  |  Guesses Used: %d/%d  |  Hints: %d/%d\n\n",
           minutes, seconds, session->guess_count, session->max_guesses,
           session->hints_used, session->hints_available);
}

void ui_print_range_visualization(GameSession *session)
{
    if (session == NULL) return;

    int width = 50;
    int range_size = session->range_max - session->range_min + 1;
    int narrowed = ((session->range_max - session->range_min + 1) * width) / range_size;

    printf("Range: [");
    for (int i = 0; i < width; i++) {
        if (i < narrowed) {
            printf("=");
        } else {
            printf("-");
        }
    }
    printf("] %d-%d\n\n", session->range_min, session->range_max);
}

void ui_print_guess_history(GameSession *session)
{
    if (session == NULL || session->guess_count == 0) return;

    printf("Previous guesses: ");
    for (int i = 0; i < session->guess_count - 1 && i < 10; i++) {
        printf("%d ", session->guesses_history[i]);
    }
    printf("\n\n");
}

void ui_print_feedback(Feedback feedback, int guess __attribute__((unused)), int secret __attribute__((unused)))
{
    switch (feedback) {
        case FEEDBACK_TOO_HIGH:
            printf(">>> Too HIGH! Try a lower number.\n\n");
            break;
        case FEEDBACK_TOO_LOW:
            printf(">>> Too LOW! Try a higher number.\n\n");
            break;
        case FEEDBACK_CORRECT:
            printf(">>> CORRECT! You found it!\n\n");
            break;
        case FEEDBACK_INVALID:
            printf(">>> Invalid guess. Try again.\n\n");
            break;
    }
}

/*
 * GAME PROMPTS
 */

int ui_prompt_for_guess(GameSession *session, int *guess)
{
    if (session == NULL || guess == NULL) return -1;

    printf("Your guess (Range: %d-%d): ", session->range_min, session->range_max);
    *guess = ui_get_integer_input(session->range_min, session->range_max);
    return 0;
}

int ui_prompt_for_hint(void)
{
    printf("\nWould you like a hint? (1=Yes, 0=No): ");
    return ui_get_integer_input(0, 1);
}

int ui_prompt_play_again(void)
{
    printf("\nPlay again? (1=Yes, 0=No): ");
    return ui_get_integer_input(0, 1);
}

/*
 * STATISTICS AND RESULTS
 */

void ui_print_win_screen(GameSession *session, int score)
{
    if (session == NULL) return;

    ui_clear_screen();
    printf("\n");
    printf("╔════════════════════════════════════════════════════╗\n");
    printf("║                                                    ║\n");
    printf("║              CONGRATULATIONS!                      ║\n");
    printf("║             YOU FOUND THE NUMBER!                  ║\n");
    printf("║                                                    ║\n");
    printf("╚════════════════════════════════════════════════════╝\n");
    printf("\n");

    time_t elapsed = session->end_time - session->start_time;
    int minutes = elapsed / 60;
    int seconds = elapsed % 60;

    printf("╔════════════════════════════════════════════════════╗\n");
    printf("║                  GAME SUMMARY                      ║\n");
    printf("╠════════════════════════════════════════════════════╣\n");
    printf("║ Secret Number:     %d\n", session->secret_number);
    printf("║ Guesses Used:      %d/%d\n", session->guess_count, session->max_guesses);
    printf("║ Time:              %02d:%02d\n", minutes, seconds);
    printf("║ Hints Used:        %d\n", session->hints_used);
    printf("║ Score:             %d points\n", score);
    printf("╚════════════════════════════════════════════════════╝\n");
    printf("\n");
}

void ui_print_lose_screen(GameSession *session)
{
    if (session == NULL) return;

    ui_clear_screen();
    printf("\n");
    printf("╔════════════════════════════════════════════════════╗\n");
    printf("║                                                    ║\n");
    printf("║              GAME OVER - YOU LOST!                ║\n");
    printf("║                                                    ║\n");
    printf("╚════════════════════════════════════════════════════╝\n");
    printf("\n");

    printf("╔════════════════════════════════════════════════════╗\n");
    printf("║                  GAME SUMMARY                      ║\n");
    printf("╠════════════════════════════════════════════════════╣\n");
    printf("║ Secret Number was: %d\n", session->secret_number);
    printf("║ Your Guesses:      %d/%d\n", session->guess_count, session->max_guesses);
    printf("║ Closest Guess:     %d\n",
           session->guess_count > 0 ? session->guesses_history[session->guess_count - 1] : 0);
    printf("╚════════════════════════════════════════════════════╝\n");
    printf("\n");
}

void ui_print_ai_vs_player_results(GameSession *player_session, AIOpponent *ai)
{
    if (player_session == NULL || ai == NULL) return;

    printf("\n");
    printf("╔════════════════════════════════════════════════════╗\n");
    printf("║           VERSUS AI - RESULTS                      ║\n");
    printf("╠════════════════════════════════════════════════════╣\n");
    printf("║ Player Guesses:    %d\n", player_session->guess_count);
    printf("║ AI Guesses:        %d\n", ai->guess_count);

    if (player_session->guess_count < ai->guess_count) {
        printf("║ WINNER: YOU WIN!\n");
    } else if (player_session->guess_count > ai->guess_count) {
        printf("║ WINNER: AI WINS!\n");
    } else {
        printf("║ RESULT: TIE!\n");
    }

    printf("╚════════════════════════════════════════════════════╝\n");
    printf("\n");
}

void ui_print_statistics_summary(PlayerStats *stats)
{
    if (stats == NULL) return;

    printf("\n");
    printf("╔════════════════════════════════════════════════════╗\n");
    printf("║          YOUR STATISTICS                           ║\n");
    printf("╠════════════════════════════════════════════════════╣\n");
    printf("║ Total Games:    %d\n", stats->total_games_played);
    printf("║ Total Wins:     %d\n", stats->total_wins);
    printf("║ Win Rate:       %.1f%%\n",
           (stats->total_games_played > 0) ?
           ((float)stats->total_wins / stats->total_games_played * 100) : 0.0f);
    printf("║ Best Game:      %d guesses\n", stats->best_game_guesses);
    printf("║ Current Streak: %d\n", stats->current_streak);
    printf("║ Achievements:   %d\n", stats->achievements_unlocked);
    printf("╚════════════════════════════════════════════════════╝\n");
    printf("\n");
}

/*
 * VISUAL ELEMENTS
 */

void ui_print_progress_bar(int current, int max, int width)
{
    if (max <= 0) return;

    int filled = (current * width) / max;
    printf("[");
    for (int i = 0; i < width; i++) {
        printf(i < filled ? "=" : "-");
    }
    printf("] %d/%d\n", current, max);
}

void ui_print_celebration(void)
{
    printf("\n");
    printf("        *   *   *\n");
    printf("       * * * * *\n");
    printf("      * VICTORY *\n");
    printf("       * * * * *\n");
    printf("        *   *   *\n");
    printf("\n");
}

void ui_print_game_over(void)
{
    printf("\n");
    printf("     ═══════════════\n");
    printf("    ║  GAME OVER  ║\n");
    printf("     ═══════════════\n");
    printf("\n");
}

/*
 * INPUT UTILITIES
 */

int ui_get_integer_input(int min, int max)
{
    int value;
    int result;

    while (1) {
        result = scanf("%d", &value);
        while (getchar() != '\n');  /* Clear input buffer */

        if (result != 1 || value < min || value > max) {
            printf("Invalid input. Please enter a number between %d and %d: ", min, max);
            continue;
        }
        return value;
    }
}

int ui_get_yes_no_input(void)
{
    printf("(1=Yes, 0=No): ");
    return ui_get_integer_input(0, 1);
}

void ui_wait_for_input(void)
{
    printf("\nPress [ENTER] to continue...");
    while (getchar() != '\n');
}

/*
 * ERROR MESSAGES
 */

void ui_print_error(const char *message)
{
    if (message == NULL) return;
    printf("\n[ERROR] %s\n\n", message);
}

void ui_print_warning(const char *message)
{
    if (message == NULL) return;
    printf("\n[WARNING] %s\n\n", message);
}

void ui_print_success(const char *message)
{
    if (message == NULL) return;
    printf("\n[SUCCESS] %s\n\n", message);
}

void ui_print_info(const char *message)
{
    if (message == NULL) return;
    printf("\n[INFO] %s\n\n", message);
}

/*
 * SETTINGS AND INFO
 */

void ui_print_help(void)
{
    ui_clear_screen();
    ui_print_header("HELP & TUTORIAL");

    printf("GAME MODES:\n");
    printf("  1. Classic:   Try to guess the computer's secret number.\n");
    printf("  2. Reverse:   Computer tries to guess your number.\n");
    printf("  3. Versus:    Race against the AI to guess fastest.\n");
    printf("  4. Challenge: Limited guesses for bonus points.\n");
    printf("  5. Memory:    Guess number sequences.\n\n");

    printf("DIFFICULTY LEVELS:\n");
    printf("  Easy:     1-10 range, unlimited guesses (learning mode)\n");
    printf("  Medium:   1-100 range, ~10 guesses allowed\n");
    printf("  Hard:     1-1000 range, ~15 guesses allowed\n");
    printf("  Expert:   1-10000 range, ~20 guesses allowed\n");
    printf("  Master:   1-100000 range, ~25 guesses allowed\n\n");

    printf("STRATEGY:\n");
    printf("  Use binary search: Always guess the middle of the range.\n");
    printf("  Use hints wisely to narrow down the possibilities.\n");
    printf("  Pay attention to feedback (high/low/warmer/colder).\n\n");

    ui_wait_for_input();
}

void ui_print_about(void)
{
    ui_clear_screen();
    printf("\n");
    printf("╔════════════════════════════════════════════════════╗\n");
    printf("║         NUMBER GUESS MASTER v1.0                  ║\n");
    printf("║                                                    ║\n");
    printf("║  A comprehensive number guessing game in C         ║\n");
    printf("║  with multiple modes, difficulties, and AI.        ║\n");
    printf("║                                                    ║\n");
    printf("║  Features:                                          ║\n");
    printf("║    - 5 Different game modes                         ║\n");
    printf("║    - 5 Difficulty levels                           ║\n");
    printf("║    - 4 AI opponent strategies                       ║\n");
    printf("║    - Smart hint system                              ║\n");
    printf("║    - Statistics tracking & leaderboards            ║\n");
    printf("║    - Achievement system                            ║\n");
    printf("║                                                    ║\n");
    printf("║  Made in C99 with POSIX compliance                ║\n");
    printf("║                                                    ║\n");
    printf("╚════════════════════════════════════════════════════╝\n");
    printf("\n");

    ui_wait_for_input();
}

void ui_show_difficulty_info(DifficultyLevel diff)
{
    ui_clear_screen();
    ui_print_header("DIFFICULTY INFORMATION");

    const char *names[] = {"Unknown", "Easy", "Medium", "Hard", "Expert", "Master"};
    const char *descriptions[] = {
        "",
        "Perfect for learning the game. No pressure, unlimited guesses.",
        "Good challenge. Tests basic strategy and speed.",
        "Requires optimal guessing strategy (binary search).",
        "Expert level. Fast convergence required.",
        "Master level. Extreme challenge for experienced players."
    };

    printf("Difficulty: %s\n\n", names[(int)diff]);
    printf("%s\n\n", descriptions[(int)diff]);

    ui_wait_for_input();
}
