/**
 * rng.h - Cryptographically secure random number generator
 *
 * Provides high-quality random number generation for test delays
 * and stimulus generation, preventing predictable patterns.
 */

#ifndef RNG_H
#define RNG_H

#include <stdint.h>

/* RNG state for reproducible sequences */
typedef struct {
    uint64_t state[4];          /* Internal XORSHIFT1024* state */
    uint32_t seed;              /* Original seed for reproducibility */
} RNGState;

/**
 * Initialize RNG with seed.
 * Reproducible sequences for daily challenges.
 *
 * seed: Random seed (0 = system entropy)
 *
 * Returns: Pointer to RNGState, NULL on failure
 */
RNGState* rng_init(uint32_t seed);

/**
 * Initialize RNG with system entropy.
 * Non-reproducible, secure randomness.
 *
 * Returns: Pointer to RNGState, NULL on failure
 */
RNGState* rng_init_secure(void);

/**
 * Get next random 64-bit integer.
 * Uses XORSHIFT1024* for speed and quality.
 *
 * state: RNG state
 *
 * Returns: Random 64-bit unsigned integer
 */
uint64_t rng_next_u64(RNGState *state);

/**
 * Get next random 32-bit integer.
 *
 * state: RNG state
 *
 * Returns: Random 32-bit unsigned integer
 */
uint32_t rng_next_u32(RNGState *state);

/**
 * Get random integer in range [min, max].
 * Inclusive on both ends.
 *
 * state: RNG state
 * min: Minimum value
 * max: Maximum value
 *
 * Returns: Random integer in range
 */
uint32_t rng_next_range(RNGState *state, uint32_t min, uint32_t max);

/**
 * Get random double in range [0.0, 1.0).
 *
 * state: RNG state
 *
 * Returns: Random double
 */
double rng_next_double(RNGState *state);

/**
 * Get random millisecond delay in range.
 * Useful for test delays (e.g., 1000-5000ms).
 *
 * state: RNG state
 * min_ms: Minimum delay
 * max_ms: Maximum delay
 *
 * Returns: Delay in milliseconds
 */
uint32_t rng_next_delay_ms(RNGState *state, uint32_t min_ms, uint32_t max_ms);

/**
 * Shuffle array in-place.
 * Uses Fisher-Yates algorithm.
 *
 * state: RNG state
 * array: Array to shuffle
 * count: Number of elements
 * element_size: Size of each element in bytes
 *
 * Returns: 0 on success
 */
int rng_shuffle(RNGState *state, void *array, uint32_t count,
                 uint32_t element_size);

/**
 * Generate random choice from set.
 * Selects random element from provided options.
 *
 * state: RNG state
 * options: Array of options
 * count: Number of options
 *
 * Returns: Random option from array
 */
char rng_select_choice(RNGState *state, const char *options, int count);

/**
 * Reset RNG to original seed.
 * For reproducible sequences.
 *
 * state: RNG state
 *
 * Returns: 0 on success
 */
int rng_reset(RNGState *state);

/**
 * Clone RNG state.
 * Creates independent copy at current point.
 *
 * state: RNG state to clone
 *
 * Returns: New RNGState, NULL on failure
 */
RNGState* rng_clone(RNGState *state);

/**
 * Free RNG state.
 *
 * state: RNG state to free
 */
void rng_free(RNGState *state);

/**
 * Reseed with new value.
 * Useful for different test variations.
 *
 * state: RNG state
 * new_seed: New seed value
 *
 * Returns: 0 on success
 */
int rng_reseed(RNGState *state, uint32_t new_seed);

/**
 * Get current RNG seed for logging/reproducibility.
 *
 * state: RNG state
 *
 * Returns: Current seed value
 */
uint32_t rng_get_seed(RNGState *state);

#endif /* RNG_H */
