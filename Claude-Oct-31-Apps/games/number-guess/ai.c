#include "ai.h"
#include "random.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

/*
 * AI OPPONENT IMPLEMENTATION
 * Multiple strategies for different difficulty levels
 */

AIOpponent ai_create(AIStrategy strategy, int range_min, int range_max, int secret)
{
    AIOpponent ai;

    ai.strategy = strategy;
    ai.guess_count = 0;
    ai.current_guess = (range_min + range_max) / 2;
    ai.range_min = range_min;
    ai.range_max = range_max;
    ai.secret_number = secret;
    ai.start_time = time(NULL);

    memset(ai.guesses_history, 0, sizeof(ai.guesses_history));

    return ai;
}

int ai_make_guess(AIOpponent *ai)
{
    if (ai == NULL) return -1;

    int guess = -1;

    switch (ai->strategy) {
        case AI_RANDOM:
            guess = ai_random_guess(ai);
            break;
        case AI_BINARY_SEARCH:
            guess = ai_binary_search_guess(ai);
            break;
        case AI_PROBABILISTIC:
            guess = ai_probabilistic_guess(ai);
            break;
        case AI_MACHINE_LEARNING:
            guess = ai_machine_learning_guess(ai);
            break;
        default:
            guess = ai_binary_search_guess(ai);
    }

    ai->current_guess = guess;
    ai->guesses_history[ai->guess_count] = guess;
    ai->guess_count++;

    return guess;
}

void ai_provide_feedback(AIOpponent *ai, int guess, Feedback feedback)
{
    if (ai == NULL) return;

    switch (feedback) {
        case FEEDBACK_TOO_HIGH:
            ai->range_max = guess - 1;
            break;
        case FEEDBACK_TOO_LOW:
            ai->range_min = guess + 1;
            break;
        case FEEDBACK_CORRECT:
            /* Game ends */
            break;
        default:
            break;
    }
}

/*
 * AI STRATEGY: RANDOM GUESSER
 * Suitable for Easy difficulty
 */
int ai_random_guess(AIOpponent *ai)
{
    return generate_random_number(ai->range_min, ai->range_max);
}

/*
 * AI STRATEGY: BINARY SEARCH
 * Suitable for Medium/Hard difficulty
 * Optimal information-theoretic approach
 */
int ai_binary_search_guess(AIOpponent *ai)
{
    return (ai->range_min + ai->range_max) / 2;
}

/*
 * AI STRATEGY: PROBABILISTIC
 * Suitable for Hard/Expert difficulty
 * More human-like, slightly suboptimal but realistic
 */
int ai_probabilistic_guess(AIOpponent *ai)
{
    /* 80% chance to use binary search, 20% chance to add random deviation */
    float rand_val = generate_random_float();

    if (rand_val < 0.8f) {
        return ai_binary_search_guess(ai);
    } else {
        /* Add random deviation from midpoint */
        int midpoint = (ai->range_min + ai->range_max) / 2;
        int range = ai->range_max - ai->range_min;
        int deviation = (range / 4) - generate_random_number(0, range / 2);
        int guess = midpoint + deviation;

        if (guess < ai->range_min) guess = ai->range_min;
        if (guess > ai->range_max) guess = ai->range_max;

        return guess;
    }
}

/*
 * AI STRATEGY: MACHINE LEARNING
 * Suitable for Expert difficulty
 * Adapts based on history and patterns
 */
int ai_machine_learning_guess(AIOpponent *ai)
{
    /* Analyze pattern of previous guesses and responses */

    /* If we have enough history, analyze the pattern */
    if (ai->guess_count >= 3) {
        /* Check if secret seems to be concentrated in certain area */
        int lower_third = ai->range_min + (ai->range_max - ai->range_min) / 3;

        /* Count if we're converging faster from top or bottom */
        int low_count = 0;
        for (int i = 0; i < ai->guess_count; i++) {
            if (ai->guesses_history[i] < lower_third) low_count++;
        }

        /* Bias guess towards area with more feedback */
        if (low_count > ai->guess_count / 2) {
            /* Bias lower */
            return ai->range_min + (ai->range_max - ai->range_min) / 3;
        } else {
            /* Bias upper */
            return ai->range_max - (ai->range_max - ai->range_min) / 3;
        }
    }

    /* Default to binary search if not enough history */
    return ai_binary_search_guess(ai);
}

/*
 * AI ANALYSIS FUNCTIONS
 */

int ai_get_guess_count(AIOpponent *ai)
{
    return ai->guess_count;
}

float ai_get_average_guesses(AIOpponent *ai)
{
    if (ai->guess_count == 0) return 0.0f;
    return (float)ai->guess_count / (ai->guess_count ? ai->guess_count : 1);
}

int ai_is_converged(AIOpponent *ai)
{
    /* AI has converged if range is very small (1-3 possible values) */
    return (ai->range_max - ai->range_min) <= 2;
}

const char* ai_get_strategy_name(AIStrategy strategy)
{
    switch (strategy) {
        case AI_RANDOM:
            return "Random Guesser";
        case AI_BINARY_SEARCH:
            return "Binary Search";
        case AI_PROBABILISTIC:
            return "Probabilistic";
        case AI_MACHINE_LEARNING:
            return "Machine Learning";
        default:
            return "Unknown";
    }
}
