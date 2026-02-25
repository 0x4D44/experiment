/**
 * main.c - Reaction Timer Game Entry Point
 *
 * Main game loop with menu system and session management.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "timer.h"
#include "input.h"
#include "display.h"
#include "test_engine.h"
#include "persistence.h"
#include "rng.h"

/* Game state */
typedef struct {
    TestConfig current_test;
    TestSession *current_session;
    int running;
} GameState;

static GameState game_state = {0};

/**
 * Display main menu and get user selection.
 */
static int show_main_menu(void) {
    display_clear();

    display_draw_box("REACTION TIMER",
                     "\n"
                     "1. Quick Test (5 attempts)\n"
                     "2. Standard Test (10 attempts)\n"
                     "3. Marathon (50 attempts)\n"
                     "4. Endurance (until miss)\n"
                     "5. Daily Challenge\n"
                     "6. Reflex Training\n"
                     "7. View Statistics\n"
                     "8. View Leaderboard\n"
                     "9. Settings\n"
                     "0. Exit\n",
                     50);

    printf("\nSelect option (0-9): ");
    char choice = getchar();
    input_flush();

    return choice - '0';
}

/**
 * Display test type selection menu.
 */
static int show_test_type_menu(void) {
    display_clear();

    display_draw_box("SELECT TEST TYPE",
                     "\n"
                     "1. Simple Reaction (Press any key)\n"
                     "2. Choice Reaction (Press correct key)\n"
                     "3. Color Reaction (React to color)\n"
                     "4. Sequence Memory (Reproduce sequence)\n"
                     "5. Pattern Recognition (Match pattern)\n"
                     "6. Audio Reaction (React to sound)\n"
                     "7. Inhibition Test (Go/No-Go)\n"
                     "0. Back to menu\n",
                     50);

    printf("\nSelect test (0-7): ");
    char choice = getchar();
    input_flush();

    return choice - '0';
}

/**
 * Get player name for leaderboard.
 */
static void get_player_name(char *name, size_t size) {
    printf("Enter your name: ");
    fgets(name, size, stdin);

    /* Remove newline */
    size_t len = strlen(name);
    if (len > 0 && name[len - 1] == '\n') {
        name[len - 1] = '\0';
    }
}

/**
 * Configure and run a test session.
 */
static int run_test_session(void) {
    /* Get test type */
    int test_type = show_test_type_menu();
    if (test_type == 0) {
        return 0;
    }
    if (test_type < 1 || test_type > 7) {
        display_show_error("Invalid test type");
        return -1;
    }

    /* Get test mode */
    printf("\nSelect difficulty (1-10): ");
    int difficulty;
    scanf("%d", &difficulty);
    input_flush();

    if (difficulty < 1) difficulty = 1;
    if (difficulty > 10) difficulty = 10;

    /* Initialize test config */
    TestConfig config = {0};
    config.test_type = test_type;
    config.test_mode = MODE_STANDARD;
    config.num_trials = 10;
    config.min_delay_ms = 1000;
    config.max_delay_ms = 5000;
    config.random_seed = 0;  /* Use system entropy */
    config.difficulty_level = difficulty;
    config.session_start = time(NULL);

    get_player_name(config.player_name, sizeof(config.player_name));

    /* Initialize session */
    TestSession *session = test_engine_init(&config);
    if (session == NULL) {
        display_show_error("Failed to initialize test session");
        return -1;
    }

    /* Run the test */
    int result = test_engine_run(session);

    if (result == 0) {
        /* Finalize and show results */
        test_engine_finalize(session);
        display_show_session_summary(session);

        /* Save session */
        persistence_save_session(session);

        printf("\nPress ENTER to continue...");
        getchar();
    }

    test_engine_free(session);
    return result;
}

/**
 * Show player statistics.
 */
static int show_statistics(void) {
    display_clear();

    char player_name[64];
    get_player_name(player_name, sizeof(player_name));

    Statistics stats;
    if (persistence_get_all_time_stats(player_name, &stats) == 0) {
        display_show_statistics(&stats);
    } else {
        display_show_warning("No data found for this player");
    }

    printf("\nPress ENTER to continue...");
    getchar();
    return 0;
}

/**
 * Show leaderboard.
 */
static int show_leaderboard(void) {
    display_clear();

    int test_type = show_test_type_menu();
    if (test_type == 0) {
        return 0;
    }
    if (test_type < 1 || test_type > 7) {
        return -1;
    }

    LeaderboardEntry entries[100];
    int count = persistence_load_leaderboard(test_type, 100, entries);

    if (count > 0) {
        printf("\n=== LEADERBOARD: %s ===\n\n",
               test_engine_type_string(test_type));

        printf("Rank  Name                 Average    Mode\n");
        printf("────────────────────────────────────────────\n");

        for (int i = 0; i < count && i < 20; i++) {
            printf("%4d  %-20s  %7.1fms  %s\n",
                   entries[i].rank,
                   entries[i].player_name,
                   entries[i].average_ms,
                   test_engine_mode_string(entries[i].test_mode));
        }
    } else {
        display_show_warning("No leaderboard data available");
    }

    printf("\nPress ENTER to continue...");
    getchar();
    return 0;
}

/**
 * Show settings menu.
 */
static int show_settings(void) {
    display_clear();

    display_draw_box("SETTINGS",
                     "\n"
                     "1. Calibrate Timer\n"
                     "2. Export Data\n"
                     "3. Clear Data\n"
                     "4. About\n"
                     "0. Back\n",
                     50);

    printf("\nSelect option (0-4): ");
    int choice = getchar() - '0';
    input_flush();

    switch (choice) {
        case 1:
            printf("Calibrating timer...");
            fflush(stdout);
            if (timer_calibrate() == 0) {
                printf(" OK\n");
            } else {
                printf(" FAILED\n");
            }
            break;

        case 2:
            printf("Export format? (C)SV or (J)SON: ");
            char fmt = getchar();
            input_flush();
            if (fmt == 'C' || fmt == 'c') {
                printf("Exporting to CSV...\n");
                /* persistence_export_csv() */
            } else if (fmt == 'J' || fmt == 'j') {
                printf("Exporting to JSON...\n");
                /* persistence_export_json() */
            }
            break;

        case 3:
            printf("Clear all data? (y/N): ");
            char confirm = getchar();
            input_flush();
            if (confirm == 'y' || confirm == 'Y') {
                persistence_clear_all();
                printf("Data cleared.\n");
            }
            break;

        case 4:
            display_draw_box("ABOUT",
                           "\n"
                           "Reaction Timer v1.0\n"
                           "High-precision reaction time measurement\n"
                           "\n"
                           "Uses CLOCK_MONOTONIC_RAW timing\n"
                           "Microsecond precision\n"
                           "\n",
                           50);
            break;
    }

    printf("\nPress ENTER to continue...");
    getchar();
    return 0;
}

/**
 * Initialize all systems.
 */
static int initialize_systems(void) {
    if (timer_init() != 0) {
        fprintf(stderr, "Error: Failed to initialize timer\n");
        return -1;
    }

    if (input_init() != 0) {
        fprintf(stderr, "Error: Failed to initialize input\n");
        timer_shutdown();
        return -1;
    }

    if (display_init() != 0) {
        fprintf(stderr, "Error: Failed to initialize display\n");
        timer_shutdown();
        input_shutdown();
        return -1;
    }

    if (persistence_init() != 0) {
        fprintf(stderr, "Warning: Could not initialize data persistence\n");
        /* Non-fatal - continue without persistence */
    }

    /* Calibrate timer */
    if (timer_calibrate() != 0) {
        fprintf(stderr, "Warning: Timer calibration failed\n");
    }

    return 0;
}

/**
 * Shutdown all systems.
 */
static void shutdown_systems(void) {
    display_shutdown();
    input_shutdown();
    timer_shutdown();
}

/**
 * Main game loop.
 */
static void game_loop(void) {
    game_state.running = 1;

    while (game_state.running) {
        int choice = show_main_menu();

        switch (choice) {
            case 1:
            case 2:
            case 3:
            case 4:
            case 5:
            case 6:
                /* These would set test mode then run_test_session */
                run_test_session();
                break;

            case 7:
                show_statistics();
                break;

            case 8:
                show_leaderboard();
                break;

            case 9:
                show_settings();
                break;

            case 0:
                game_state.running = 0;
                break;

            default:
                display_show_error("Invalid option");
                break;
        }
    }
}

/**
 * Main entry point.
 */
int main(int argc, char *argv[]) {
    /* Handle command-line arguments */
    if (argc > 1) {
        if (strcmp(argv[1], "--help") == 0) {
            printf("Reaction Timer v1.0\n\n");
            printf("Usage: %s [options]\n\n", argv[0]);
            printf("Options:\n");
            printf("  --help              Show this help\n");
            printf("  --stats             Show statistics\n");
            printf("  --calibrate         Calibrate timer\n");
            printf("  --test <type>       Run specific test\n");
            return 0;
        } else if (strcmp(argv[1], "--calibrate") == 0) {
            if (timer_init() != 0) {
                fprintf(stderr, "Timer init failed\n");
                return 1;
            }
            printf("Calibrating timer...\n");
            if (timer_calibrate() == 0) {
                printf("Calibration successful\n");
            } else {
                printf("Calibration failed\n");
            }
            timer_shutdown();
            return 0;
        }
    }

    /* Initialize game systems */
    if (initialize_systems() != 0) {
        fprintf(stderr, "Failed to initialize game systems\n");
        return 1;
    }

    /* Run main game loop */
    game_loop();

    /* Shutdown */
    shutdown_systems();

    return 0;
}
