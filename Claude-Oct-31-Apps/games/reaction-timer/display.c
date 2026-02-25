/**
 * display.c - Terminal UI and visualization
 *
 * ANSI color codes and Unicode box drawing for display.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/ioctl.h>
#include "display.h"

/* Global display state */
static uint32_t display_capabilities = 0;
static int display_initialized = 0;

/**
 * Initialize display system.
 */
int display_init(void) {
    display_capabilities = DISPLAY_SUPPORTS_COLOR | DISPLAY_SUPPORTS_UNICODE;

    /* Check terminal capabilities */
    const char *term = getenv("TERM");
    if (term != NULL) {
        if (strstr(term, "256color") != NULL) {
            display_capabilities |= DISPLAY_SUPPORTS_256COLOR;
        }
        if (strstr(term, "truecolor") != NULL || strstr(term, "24bit") != NULL) {
            display_capabilities |= DISPLAY_SUPPORTS_TRUECOLOR;
        }
    }

    display_initialized = 1;
    return 0;
}

/**
 * Shutdown display system.
 */
int display_shutdown(void) {
    display_clear();
    display_reset_color();
    display_initialized = 0;
    return 0;
}

/**
 * Clear screen.
 */
int display_clear(void) {
    if (system("clear") == 0 || system("cls") == 0) {
        return 0;
    }
    /* Fallback: print escape sequence */
    printf("\033[2J\033[H");
    fflush(stdout);
    return 0;
}

/**
 * Print colored text.
 */
int display_print_colored(enum ColorCode color, const char *text) {
    display_set_color(color);
    int len = printf("%s", text);
    display_reset_color();
    return len;
}

/**
 * Set color.
 */
int display_set_color(enum ColorCode color) {
    if (!(display_capabilities & DISPLAY_SUPPORTS_COLOR)) {
        return 0;
    }

    const char *codes[] = {
        "\033[0m",      /* RESET */
        "\033[31m",     /* RED */
        "\033[32m",     /* GREEN */
        "\033[33m",     /* YELLOW */
        "\033[34m",     /* BLUE */
        "\033[35m",     /* MAGENTA */
        "\033[36m",     /* CYAN */
        "\033[37m",     /* WHITE */
        "\033[90m"      /* GRAY */
    };

    if (color >= 0 && color <= COLOR_GRAY) {
        printf("%s", codes[color]);
    }

    return 0;
}

/**
 * Reset color.
 */
int display_reset_color(void) {
    printf("\033[0m");
    return 0;
}

/**
 * Draw box.
 */
int display_draw_box(const char *title, const char *content, int width) {
    if (width == 0) {
        width = 50;
    }

    printf("\n");
    printf("╔");
    for (int i = 0; i < width - 2; i++) printf("═");
    printf("╗\n");

    if (title != NULL && strlen(title) > 0) {
        int padding = (width - 2 - strlen(title)) / 2;
        printf("║");
        for (int i = 0; i < padding; i++) printf(" ");
        printf("%s", title);
        for (int i = padding + strlen(title); i < width - 1; i++) printf(" ");
        printf("║\n");

        printf("╠");
        for (int i = 0; i < width - 2; i++) printf("═");
        printf("╣\n");
    }

    /* Print content line by line */
    if (content != NULL) {
        char *temp = strdup(content);
        char *line = temp;
        char *next;

        while ((next = strchr(line, '\n')) != NULL) {
            *next = '\0';
            printf("║ %-*s ║\n", width - 4, line);
            line = next + 1;
        }

        if (strlen(line) > 0) {
            printf("║ %-*s ║\n", width - 4, line);
        }

        free(temp);
    }

    printf("╚");
    for (int i = 0; i < width - 2; i++) printf("═");
    printf("╝\n");

    return 0;
}

/**
 * Show ready animation.
 */
int display_show_ready(void) {
    for (int i = 0; i < 6; i++) {
        printf("\r  GET READY...%s   ", i % 2 == 0 ? "" : " ");
        fflush(stdout);
        usleep(200000);
    }
    printf("\n");
    return 0;
}

/**
 * Show waiting animation.
 */
int display_show_waiting(uint32_t duration_ms) {
    const char *frames[] = {".   ", "..  ", "... ", "...."};
    uint32_t elapsed = 0;

    while (elapsed < duration_ms) {
        int frame = (elapsed / 100) % 4;
        printf("\r        %s        ", frames[frame]);
        fflush(stdout);

        usleep(100000);
        elapsed += 100;
    }

    printf("\n");
    return 0;
}

/**
 * Show stimulus.
 */
int display_show_stimulus(int test_type, char stimulus, uint32_t duration_ms) {
    display_set_color(COLOR_GREEN);
    printf("\n              [ GO! ]\n\n");
    display_reset_color();

    if (duration_ms > 0) {
        usleep(duration_ms * 1000);
    }

    return 0;
}

/**
 * Show result.
 */
int display_show_result(double reaction_time_ms, int is_correct,
                         int is_false_start) {
    printf("\nYour time: %.0fms ", reaction_time_ms);

    if (is_false_start) {
        display_set_color(COLOR_RED);
        printf("- Too early!\n");
    } else if (reaction_time_ms < 250) {
        display_set_color(COLOR_GREEN);
        printf("- Excellent!\n");
    } else if (reaction_time_ms < 300) {
        display_set_color(COLOR_GREEN);
        printf("- Good!\n");
    } else if (reaction_time_ms < 350) {
        display_set_color(COLOR_YELLOW);
        printf("- Average\n");
    } else {
        display_set_color(COLOR_YELLOW);
        printf("- Slow...\n");
    }

    display_reset_color();
    return 0;
}

/**
 * Draw graph.
 */
int display_draw_graph(const TrialResult *results, int count,
                        int width, int height) {
    if (width == 0) width = 40;
    if (height == 0) height = 10;

    printf("\nPerformance Graph:\n");

    if (count == 0) {
        return 0;
    }

    /* Find min/max */
    uint64_t min_us = UINT64_MAX;
    uint64_t max_us = 0;

    for (int i = 0; i < count; i++) {
        if (results[i].reaction_time_us < min_us) {
            min_us = results[i].reaction_time_us;
        }
        if (results[i].reaction_time_us > max_us) {
            max_us = results[i].reaction_time_us;
        }
    }

    uint64_t range = max_us - min_us;
    if (range == 0) range = 1;

    /* Draw graph */
    for (int y = height; y > 0; y--) {
        printf("%4d │ ", (int)(min_us / 1000 + (range / 1000) * y / height));

        for (int x = 0; x < count && x < width; x++) {
            uint64_t val = results[x].reaction_time_us;
            int normalized = (int)((val - min_us) * height / range);

            if (normalized >= y - 1) {
                printf("*");
            } else {
                printf(" ");
            }
        }

        printf("\n");
    }

    printf("     └");
    for (int i = 0; i < width; i++) printf("─");
    printf("\n");

    return 0;
}

/**
 * Draw percentile chart.
 */
int display_draw_percentile_chart(double player_average_ms, int percentile) {
    printf("\nPercentile Ranking:\n");
    printf("Your average (%.0fms) is faster than\n", player_average_ms);
    printf("%d%% of all players\n\n", percentile);

    const char *ranks[] = {"Elite", "Fast", "Good", "Average"};
    int ranges[] = {200, 250, 300, 350};

    for (int i = 0; i < 4; i++) {
        printf("%-8s (<%-3dms): [", ranks[i], ranges[i]);
        for (int j = 0; j < 10; j++) {
            if ((double)ranges[i] > player_average_ms) {
                printf("█");
            } else if (j == 4) {
                printf("◀");
            } else {
                printf(" ");
            }
        }
        printf("]\n");
    }

    return 0;
}

/**
 * Show statistics.
 */
int display_show_statistics(const Statistics *stats) {
    printf("\nStatistics:\n");
    printf("├─ Average: %.0fms\n", stats->mean_ms);
    printf("├─ Median:  %.0fms\n", stats->median_ms);
    printf("├─ Std Dev: %.0fms\n", stats->std_dev_ms);
    printf("├─ Best:    %.0fms\n", (double)stats->min_us / 1000.0);
    printf("├─ Worst:   %.0fms\n", (double)stats->max_us / 1000.0);
    printf("└─ Consistency: %.0f%%\n", stats->consistency_score);

    return 0;
}

/**
 * Show progress bar.
 */
int display_show_progress_bar(int current, int total, int width) {
    if (width == 0) width = 20;

    int filled = (current * width) / total;
    printf("Progress: [");

    for (int i = 0; i < width; i++) {
        printf("%c", i < filled ? '█' : '░');
    }

    printf("] %d/%d\n", current, total);
    return 0;
}

/**
 * Show session summary.
 */
int display_show_session_summary(const TestSession *session) {
    if (session == NULL) {
        return -1;
    }

    display_draw_box("SESSION SUMMARY", "", 50);
    display_show_statistics(&session->stats);

    return 0;
}

/**
 * Show celebration.
 */
int display_show_celebration(const char *achievement_type) {
    display_set_color(COLOR_GREEN);
    printf("\n");
    printf("  ★ %s ★\n", achievement_type);
    printf("\n");
    display_reset_color();

    return 0;
}

/**
 * Show error.
 */
int display_show_error(const char *error_msg) {
    display_set_color(COLOR_RED);
    printf("ERROR: %s\n", error_msg);
    display_reset_color();
    return 0;
}

/**
 * Show warning.
 */
int display_show_warning(const char *warning_msg) {
    display_set_color(COLOR_YELLOW);
    printf("WARNING: %s\n", warning_msg);
    display_reset_color();
    return 0;
}

/**
 * Show info.
 */
int display_show_info(const char *info_msg) {
    display_set_color(COLOR_BLUE);
    printf("INFO: %s\n", info_msg);
    display_reset_color();
    return 0;
}

/**
 * Get capabilities.
 */
uint32_t display_get_capabilities(void) {
    return display_capabilities;
}

/**
 * Get dimensions.
 */
int display_get_dimensions(int *width, int *height) {
    if (width == NULL || height == NULL) {
        return -1;
    }

    *width = 80;
    *height = 24;

#ifdef TIOCGWINSZ
    struct winsize ws;
    if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == 0) {
        *width = ws.ws_col;
        *height = ws.ws_row;
    }
#endif

    return 0;
}

/**
 * Pause and wait.
 */
int display_pause(void) {
    printf("\nPress ENTER to continue...");
    fflush(stdout);
    getchar();
    return 0;
}

/**
 * Draw separator.
 */
int display_draw_separator(int width, char char_code) {
    if (width == 0) {
        width = 80;
    }

    for (int i = 0; i < width; i++) {
        printf("%c", char_code);
    }
    printf("\n");

    return 0;
}
