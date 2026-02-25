#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include "game.h"
#include "random.h"
#include "hint.h"
#include "ai.h"
#include "stats.h"
#include "ui.h"

/*
 * GAME SESSION RUNNER
 * Controls the flow of a single game
 */

int run_classic_game(GameSession *session, PlayerStats *stats)
{
    if (session == NULL || stats == NULL) return -1;

    ui_clear_screen();
    ui_print_game_header(session);

    while (session->state == STATE_PLAYING) {
        ui_print_game_status(session);
        ui_print_range_visualization(session);

        if (session->guess_count > 0) {
            ui_print_guess_history(session);
        }

        /* Get player guess */
        int guess;
        ui_prompt_for_guess(session, &guess);

        /* Process guess */
        int result = game_process_guess(session, guess);

        if (result == 1) {
            /* Win */
            break;
        } else if (result == -2) {
            /* Lose */
            break;
        } else if (result == 0) {
            /* Continue */
            Feedback feedback = game_get_feedback(guess, session->secret_number);
            ui_print_feedback(feedback, guess, session->secret_number);

            /* Offer hint */
            if (session->hints_available > 0) {
                if (ui_prompt_for_hint()) {
                    const char *hint = hint_get_mathematical(session->secret_number);
                    printf("Hint: %s\n\n", hint);
                    session->hints_used++;
                    session->hints_used_this_game++;
                    session->hints_available--;
                }
            }
        }

        ui_wait_for_input();
        ui_clear_screen();
        ui_print_game_header(session);
    }

    /* Calculate score and update statistics */
    time_t elapsed = session->end_time - session->start_time;
    float seconds = (float)elapsed;
    int score = game_calculate_score(session, seconds);

    /* Display results */
    if (session->state == STATE_WON) {
        ui_print_win_screen(session, score);
        stats_update_after_game(stats, session, score);

        /* Check for achievements */
        if (session->guess_count == 1) {
            achievement_unlock(stats, ACHIEVEMENT_PERFECT_ROUND);
            printf("[ACHIEVEMENT] Perfect Round unlocked!\n");
        }
        if (stats->current_streak == 5) {
            achievement_unlock(stats, ACHIEVEMENT_STREAK_5);
            printf("[ACHIEVEMENT] 5-Win Streak unlocked!\n");
        }
        if (session->hints_used == 0) {
            achievement_unlock(stats, ACHIEVEMENT_NO_HINTS);
            printf("[ACHIEVEMENT] No Hints Run unlocked!\n");
        }
    } else {
        ui_print_lose_screen(session);
        stats_update_after_game(stats, session, 0);
    }

    ui_wait_for_input();
    return 0;
}

int run_reverse_game(GameSession *session, PlayerStats *stats)
{
    if (session == NULL || stats == NULL) return -1;

    ui_clear_screen();
    printf("\n╔════════════════════════════════════════════════════╗\n");
    printf("║ REVERSE MODE - Computer Guesses Your Number       ║\n");
    printf("╠════════════════════════════════════════════════════╣\n");
    printf("║ Think of a number between %d and %d               ║\n", 1, session->range_max);
    printf("║ I will try to guess it.                           ║\n");
    printf("║ When I guess, tell me if it's:                    ║\n");
    printf("║  - Too High (type: 1)                             ║\n");
    printf("║  - Too Low (type: 2)                              ║\n");
    printf("║  - Correct (type: 3)                              ║\n");
    printf("╚════════════════════════════════════════════════════╝\n\n");

    printf("Got your number in mind? Press ENTER to continue...");
    ui_wait_for_input();

    /* Create AI opponent */
    AIOpponent ai = ai_create(AI_BINARY_SEARCH, 1, session->range_max, 0);

    while (session->state == STATE_PLAYING) {
        int ai_guess = ai_make_guess(&ai);
        printf("\nI guess: %d\n", ai_guess);

        printf("Is it:\n");
        printf("  1. Too High\n");
        printf("  2. Too Low\n");
        printf("  3. Correct\n");
        printf("Your response: ");

        int response = ui_get_integer_input(1, 3);

        if (response == 3) {
            /* AI won */
            printf("\nGreat! I found your number in %d guesses!\n", ai.guess_count);
            session->state = STATE_LOST;
            ui_print_ai_vs_player_results(session, &ai);
            session->guess_count = ai.guess_count;  /* For stats */
            break;
        } else if (response == 1) {
            ai_provide_feedback(&ai, ai_guess, FEEDBACK_TOO_HIGH);
        } else {
            ai_provide_feedback(&ai, ai_guess, FEEDBACK_TOO_LOW);
        }

        if (ai_get_guess_count(&ai) >= session->max_guesses) {
            printf("\nI give up! I couldn't find your number.\n");
            session->state = STATE_WON;
            break;
        }
    }

    stats_update_after_game(stats, session, 0);
    ui_wait_for_input();
    return 0;
}

int run_challenge_game(GameSession *session, PlayerStats *stats)
{
    if (session == NULL || stats == NULL) return -1;

    /* Challenge mode: limited guesses with scoring */
    session->max_guesses = 5;  /* Reduced for challenge */

    int score = 0;
    while (session->state == STATE_PLAYING) {
        ui_print_game_status(session);
        ui_print_range_visualization(session);

        int guess;
        ui_prompt_for_guess(session, &guess);

        int result = game_process_guess(session, guess);

        if (result == 1) {
            /* Win */
            time_t elapsed = session->end_time - session->start_time;
            float seconds = (float)elapsed;
            score = game_calculate_score(session, seconds);
            ui_print_win_screen(session, score);
            break;
        } else if (result == -2) {
            /* Lose */
            ui_print_lose_screen(session);
            score = 0;
            break;
        }

        ui_wait_for_input();
        ui_clear_screen();
    }

    stats_update_after_game(stats, session, score);
    ui_wait_for_input();
    return 0;
}

int run_memory_game(GameSession *session, PlayerStats *stats)
{
    if (session == NULL || stats == NULL) return -1;

    ui_clear_screen();
    ui_print_header("MEMORY MODE");

    printf("I will show you a sequence of numbers.\n");
    printf("Try to remember them and guess the next one!\n\n");

    int sequence[10];
    int round = 0;
    int score = 0;

    while (round < 10) {
        printf("Round %d\n", round + 1);
        printf("═══════════════\n");

        /* Generate sequence for this round */
        for (int i = 0; i <= round; i++) {
            sequence[i] = generate_random_number(1, 100);
        }

        /* Show sequence */
        printf("Sequence: ");
        for (int i = 0; i <= round; i++) {
            printf("%d ", sequence[i]);
            fflush(stdout);
            sleep(1);  /* Show each number for 1 second */
        }
        printf("\n\n");

        /* Clear screen and ask for next number */
        printf("What is the next number? ");
        int guess = ui_get_integer_input(1, 100);

        int correct_next = generate_random_number(1, 100);

        if (guess == correct_next) {
            printf("Correct!\n\n");
            score += (round + 1) * 10;
            round++;
        } else {
            printf("Wrong! The number was: %d\n\n", correct_next);
            break;
        }
    }

    printf("\nFinal Score: %d\n", score);
    printf("Rounds Completed: %d\n\n", round);

    session->state = STATE_WON;
    session->current_score = score;

    ui_wait_for_input();
    return 0;
}

/*
 * MAIN GAME LOOP
 */

int main(void)
{
    /* Initialize random number generator */
    seed_random_from_entropy();

    /* Create data directory if needed */
#ifdef __unix__
    (void)system("mkdir -p DATA");
#else
    (void)system("mkdir DATA");
#endif

    /* Initialize player stats */
    PlayerStats stats;
    stats_load_from_file("DATA/stats.dat", &stats);

    int running = 1;

    while (running) {
        int main_choice = ui_show_main_menu();

        switch (main_choice) {
            case 1: {
                /* Play Game */
                int mode_choice = ui_show_game_mode_menu();

                if (mode_choice >= 1 && mode_choice <= 5) {
                    int diff_choice = ui_show_difficulty_menu();

                    if (diff_choice >= 1 && diff_choice <= 5) {
                        /* Create game session */
                        GameSession session = game_create_session(
                            (GameMode)mode_choice,
                            (DifficultyLevel)diff_choice
                        );

                        /* Run appropriate game mode */
                        switch (mode_choice) {
                            case MODE_CLASSIC:
                                run_classic_game(&session, &stats);
                                break;
                            case MODE_REVERSE:
                                run_reverse_game(&session, &stats);
                                break;
                            case MODE_VERSUS:
                                run_challenge_game(&session, &stats);
                                break;
                            case MODE_CHALLENGE:
                                run_challenge_game(&session, &stats);
                                break;
                            case MODE_MEMORY:
                                run_memory_game(&session, &stats);
                                break;
                        }

                        /* Save stats after game */
                        stats_save_to_file("DATA/stats.dat", &stats);
                    }
                }
                break;
            }

            case 2:
                /* View Statistics */
                ui_clear_screen();
                ui_print_header("YOUR STATISTICS");
                stats_print_summary(&stats);
                stats_print_difficulty_stats(&stats);
                stats_print_achievement_list(&stats);
                ui_wait_for_input();
                break;

            case 3:
                /* View Leaderboard */
                ui_clear_screen();
                ui_print_header("LEADERBOARD");
                printf("Leaderboard system coming soon!\n");
                ui_wait_for_input();
                break;

            case 4:
                /* Daily Challenge */
                ui_clear_screen();
                ui_print_header("DAILY CHALLENGE");
                printf("Daily challenge system coming soon!\n");
                ui_wait_for_input();
                break;

            case 5:
                /* Settings */
                ui_clear_screen();
                ui_print_header("SETTINGS");
                printf("Settings coming soon!\n");
                ui_wait_for_input();
                break;

            case 6:
                /* Help */
                ui_print_help();
                break;

            case 7:
                /* About */
                ui_print_about();
                break;

            case 8:
                /* Exit */
                running = 0;
                ui_clear_screen();
                printf("\nThanks for playing Number Guess Master!\n");
                printf("Your statistics have been saved.\n\n");
                break;
        }
    }

    return 0;
}
