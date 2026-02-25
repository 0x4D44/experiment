/**
 * display.h - Visual feedback and UI rendering
 *
 * Provides terminal UI with colors, animations, graphs,
 * and progress indication using ANSI codes and Unicode.
 */

#ifndef DISPLAY_H
#define DISPLAY_H

#include <stdint.h>
#include "test_engine.h"
#include "statistics.h"

/* Color codes for terminal output */
enum ColorCode {
    COLOR_RESET = 0,
    COLOR_RED = 1,
    COLOR_GREEN = 2,
    COLOR_YELLOW = 3,
    COLOR_BLUE = 4,
    COLOR_MAGENTA = 5,
    COLOR_CYAN = 6,
    COLOR_WHITE = 7,
    COLOR_GRAY = 8
};

/* Display feature flags */
#define DISPLAY_SUPPORTS_COLOR 0x01
#define DISPLAY_SUPPORTS_UNICODE 0x02
#define DISPLAY_SUPPORTS_256COLOR 0x04
#define DISPLAY_SUPPORTS_TRUECOLOR 0x08

/**
 * Initialize display system.
 * Detects terminal capabilities and caches them.
 * Should be called before any display operations.
 *
 * Returns: 0 on success, -1 on failure
 */
int display_init(void);

/**
 * Shutdown display system.
 * Restores terminal to clean state.
 *
 * Returns: 0 on success, -1 on failure
 */
int display_shutdown(void);

/**
 * Clear the entire terminal screen.
 *
 * Returns: 0 on success
 */
int display_clear(void);

/**
 * Print colored text to terminal.
 *
 * color: Color code from enum
 * text: Text to print
 *
 * Returns: Number of characters printed
 */
int display_print_colored(enum ColorCode color, const char *text);

/**
 * Set text color for subsequent output.
 *
 * color: Color code from enum
 *
 * Returns: 0 on success
 */
int display_set_color(enum ColorCode color);

/**
 * Reset text color to default.
 *
 * Returns: 0 on success
 */
int display_reset_color(void);

/**
 * Draw a box around text.
 * Uses Unicode box-drawing characters if available.
 *
 * title: Title for top of box
 * content: Multi-line content to box
 * width: Box width (0 = auto)
 *
 * Returns: 0 on success
 */
int display_draw_box(const char *title, const char *content, int width);

/**
 * Show ready state animation.
 * Pulsing "GET READY..." indicator.
 *
 * Returns: 0 when complete
 */
int display_show_ready(void);

/**
 * Show waiting state animation.
 * Animated dots during random delay.
 *
 * duration_ms: How long to show waiting
 *
 * Returns: 0 when complete
 */
int display_show_waiting(uint32_t duration_ms);

/**
 * Display stimulus prompt.
 * Shows test-type-specific stimulus.
 *
 * test_type: Type of test running
 * stimulus: The stimulus to display
 * duration_ms: How long to show (0 = until keypress)
 *
 * Returns: 0 on success
 */
int display_show_stimulus(int test_type, char stimulus, uint32_t duration_ms);

/**
 * Show result feedback for single trial.
 * Color-coded performance message.
 *
 * reaction_time_ms: Time of reaction
 * is_correct: Was response correct (for choice tests)
 * is_false_start: Was it too early
 *
 * Returns: 0 on success
 */
int display_show_result(double reaction_time_ms, int is_correct,
                         int is_false_start);

/**
 * Draw performance graph for session.
 * ASCII art chart of reaction times over trials.
 *
 * results: TrialResult array
 * count: Number of results
 * width: Width of graph in characters
 * height: Height of graph in lines
 *
 * Returns: 0 on success
 */
int display_draw_graph(const TrialResult *results, int count,
                        int width, int height);

/**
 * Draw percentile ranking chart.
 * Shows where player stands vs population.
 *
 * player_average_ms: Player's average time
 * percentile: Player's percentile (0-100)
 *
 * Returns: 0 on success
 */
int display_draw_percentile_chart(double player_average_ms, int percentile);

/**
 * Display comprehensive session statistics.
 * Shows mean, median, std dev, best, worst, etc.
 *
 * stats: Statistics structure
 *
 * Returns: 0 on success
 */
int display_show_statistics(const Statistics *stats);

/**
 * Display progress bar.
 * Shows trials completed vs total.
 *
 * current: Current trial number (1-based)
 * total: Total trials in session
 * width: Width of bar in characters
 *
 * Returns: 0 on success
 */
int display_show_progress_bar(int current, int total, int width);

/**
 * Display session summary.
 * Quick overview of key metrics and ranking.
 *
 * session: Completed test session
 *
 * Returns: 0 on success
 */
int display_show_session_summary(const TestSession *session);

/**
 * Show celebration animation.
 * For personal records and achievements.
 *
 * achievement_type: What was achieved
 * Returns: 0 when complete
 */
int display_show_celebration(const char *achievement_type);

/**
 * Display error message.
 * Highlighted in red with context.
 *
 * error_msg: Message to display
 *
 * Returns: 0 on success
 */
int display_show_error(const char *error_msg);

/**
 * Display warning message.
 * Highlighted in yellow with context.
 *
 * warning_msg: Message to display
 *
 * Returns: 0 on success
 */
int display_show_warning(const char *warning_msg);

/**
 * Display info message.
 * For general information in blue.
 *
 * info_msg: Message to display
 *
 * Returns: 0 on success
 */
int display_show_info(const char *info_msg);

/**
 * Get terminal capability flags.
 * Indicates what display features are available.
 *
 * Returns: Bitmask of DISPLAY_SUPPORTS_* flags
 */
uint32_t display_get_capabilities(void);

/**
 * Get terminal dimensions.
 *
 * width: Output terminal width
 * height: Output terminal height
 *
 * Returns: 0 on success
 */
int display_get_dimensions(int *width, int *height);

/**
 * Pause and wait for user input.
 * Shows "Press ENTER to continue" message.
 *
 * Returns: 0 on success
 */
int display_pause(void);

/**
 * Draw horizontal separator line.
 * Useful for visual organization.
 *
 * width: Width of line (0 = terminal width)
 * char_code: Character to use (usually '-')
 *
 * Returns: 0 on success
 */
int display_draw_separator(int width, char char_code);

#endif /* DISPLAY_H */
