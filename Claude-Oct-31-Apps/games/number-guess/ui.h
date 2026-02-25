#ifndef UI_H
#define UI_H

#include "game.h"

/*
 * USER INTERFACE AND DISPLAY SYSTEM
 * ASCII art, menus, and visualization
 */

/* Terminal utilities */
void ui_clear_screen(void);
void ui_print_title(void);
void ui_print_header(const char *title);
void ui_print_footer(void);

/* Main menu */
int ui_show_main_menu(void);
int ui_show_game_mode_menu(void);
int ui_show_difficulty_menu(void);

/* In-game display */
void ui_print_game_header(GameSession *session);
void ui_print_game_status(GameSession *session);
void ui_print_range_visualization(GameSession *session);
void ui_print_guess_history(GameSession *session);
void ui_print_feedback(Feedback feedback, int guess, int secret);

/* Game prompts */
int ui_prompt_for_guess(GameSession *session, int *guess);
int ui_prompt_for_hint(void);
int ui_prompt_play_again(void);

/* Statistics and results display */
void ui_print_win_screen(GameSession *session, int score);
void ui_print_lose_screen(GameSession *session);
void ui_print_ai_vs_player_results(GameSession *player_session, AIOpponent *ai);
void ui_print_statistics_summary(PlayerStats *stats);

/* Visual elements */
void ui_print_progress_bar(int current, int max, int width);
void ui_print_celebration(void);
void ui_print_game_over(void);
void ui_print_separator(int width);

/* Input utilities */
int ui_get_integer_input(int min, int max);
int ui_get_yes_no_input(void);
void ui_wait_for_input(void);

/* Error messages */
void ui_print_error(const char *message);
void ui_print_warning(const char *message);
void ui_print_success(const char *message);
void ui_print_info(const char *message);

/* Settings and info */
void ui_print_help(void);
void ui_print_about(void);
void ui_show_difficulty_info(DifficultyLevel diff);

#endif
