#ifndef AI_H
#define AI_H

#include "game.h"

/*
 * AI OPPONENT SYSTEM
 * Multiple difficulty levels with different strategies
 */

/* AI initialization */
AIOpponent ai_create(AIStrategy strategy, int range_min, int range_max, int secret);

/* AI guess generation */
int ai_make_guess(AIOpponent *ai);
void ai_provide_feedback(AIOpponent *ai, int guess, Feedback feedback);

/* AI strategy implementations */
int ai_random_guess(AIOpponent *ai);
int ai_binary_search_guess(AIOpponent *ai);
int ai_probabilistic_guess(AIOpponent *ai);
int ai_machine_learning_guess(AIOpponent *ai);

/* AI analysis */
int ai_get_guess_count(AIOpponent *ai);
float ai_get_average_guesses(AIOpponent *ai);
int ai_is_converged(AIOpponent *ai);
const char* ai_get_strategy_name(AIStrategy strategy);

#endif
