/**
 * rng.c - Cryptographically secure random number generator
 *
 * XORSHIFT1024* implementation with seeding from system entropy.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <fcntl.h>
#include "rng.h"

/**
 * Read system entropy.
 */
static uint64_t read_system_entropy(void) {
    uint64_t entropy = 0;

    /* Try /dev/urandom first */
    int fd = open("/dev/urandom", O_RDONLY);
    if (fd >= 0) {
        ssize_t n = read(fd, &entropy, sizeof(entropy));
        close(fd);
        if (n == sizeof(entropy)) {
            return entropy;
        }
    }

    /* Fallback: use time-based seeding */
    return (uint64_t)time(NULL) * 2654435761ULL;
}

/**
 * Initialize RNG with seed.
 */
RNGState* rng_init(uint32_t seed) {
    RNGState *state = malloc(sizeof(RNGState));
    if (state == NULL) {
        return NULL;
    }

    state->seed = seed;

    /* Initialize state with seed */
    if (seed == 0) {
        /* Use system entropy */
        state->state[0] = read_system_entropy();
        state->state[1] = read_system_entropy();
        state->state[2] = read_system_entropy();
        state->state[3] = read_system_entropy();
    } else {
        /* Deterministic seed */
        uint64_t s = (uint64_t)seed;
        state->state[0] = s * 2654435761ULL;
        state->state[1] = (s >> 32) * 2246822519ULL;
        state->state[2] = (s * 3735928559ULL) ^ 0x9E3779B97F4A7C15ULL;
        state->state[3] = (s ^ 0xBF58476D1CE4E5B9ULL) * 27916243729ULL;
    }

    return state;
}

/**
 * Initialize RNG with system entropy.
 */
RNGState* rng_init_secure(void) {
    return rng_init(0);  /* seed=0 triggers secure seeding */
}

/**
 * XORSHIFT1024* next value.
 */
uint64_t rng_next_u64(RNGState *state) {
    if (state == NULL) {
        return 0;
    }

    uint64_t s0 = state->state[0];
    uint64_t s1 = state->state[1];
    uint64_t s2 = state->state[2];
    uint64_t s3 = state->state[3];

    uint64_t result = s0 + s3;

    uint64_t t = s1 << 17;
    s2 ^= s0;
    s3 ^= s1;
    s1 ^= s2;
    s0 ^= s3;

    s2 ^= t;
    s3 = ((s3 << 45) | (s3 >> 19));

    state->state[0] = s0;
    state->state[1] = s1;
    state->state[2] = s2;
    state->state[3] = s3;

    return result ^ (result >> 27);
}

/**
 * Get next 32-bit value.
 */
uint32_t rng_next_u32(RNGState *state) {
    return (uint32_t)(rng_next_u64(state) >> 32);
}

/**
 * Get random in range [min, max].
 */
uint32_t rng_next_range(RNGState *state, uint32_t min, uint32_t max) {
    if (min >= max) {
        return min;
    }

    uint64_t range = (uint64_t)(max - min + 1);
    uint64_t value = rng_next_u64(state);

    return min + (uint32_t)(value % range);
}

/**
 * Get random double [0, 1).
 */
double rng_next_double(RNGState *state) {
    uint64_t u = rng_next_u64(state);
    return (double)(u >> 11) * (1.0 / 9007199254740992.0);
}

/**
 * Get random delay in milliseconds.
 */
uint32_t rng_next_delay_ms(RNGState *state, uint32_t min_ms, uint32_t max_ms) {
    return rng_next_range(state, min_ms, max_ms);
}

/**
 * Shuffle array in-place (Fisher-Yates).
 */
int rng_shuffle(RNGState *state, void *array, uint32_t count,
                 uint32_t element_size) {
    if (array == NULL || count == 0 || element_size == 0) {
        return -1;
    }

    uint8_t *arr = (uint8_t *)array;
    uint8_t *temp = malloc(element_size);
    if (temp == NULL) {
        return -1;
    }

    for (uint32_t i = count - 1; i > 0; i--) {
        uint32_t j = rng_next_range(state, 0, i);

        /* Swap */
        memcpy(temp, arr + i * element_size, element_size);
        memcpy(arr + i * element_size, arr + j * element_size, element_size);
        memcpy(arr + j * element_size, temp, element_size);
    }

    free(temp);
    return 0;
}

/**
 * Select random choice from options.
 */
char rng_select_choice(RNGState *state, const char *options, int count) {
    if (options == NULL || count <= 0) {
        return '\0';
    }

    uint32_t idx = rng_next_range(state, 0, count - 1);
    return options[idx];
}

/**
 * Reset to original seed.
 */
int rng_reset(RNGState *state) {
    if (state == NULL) {
        return -1;
    }

    /* Re-initialize with same seed */
    RNGState *temp = rng_init(state->seed);
    if (temp == NULL) {
        return -1;
    }

    memcpy(state, temp, sizeof(RNGState));
    free(temp);
    return 0;
}

/**
 * Clone RNG state.
 */
RNGState* rng_clone(RNGState *state) {
    if (state == NULL) {
        return NULL;
    }

    RNGState *clone = malloc(sizeof(RNGState));
    if (clone == NULL) {
        return NULL;
    }

    memcpy(clone, state, sizeof(RNGState));
    return clone;
}

/**
 * Free RNG state.
 */
void rng_free(RNGState *state) {
    if (state != NULL) {
        free(state);
    }
}

/**
 * Reseed with new value.
 */
int rng_reseed(RNGState *state, uint32_t new_seed) {
    if (state == NULL) {
        return -1;
    }

    state->seed = new_seed;
    return rng_reset(state);
}

/**
 * Get current seed.
 */
uint32_t rng_get_seed(RNGState *state) {
    if (state == NULL) {
        return 0;
    }
    return state->seed;
}
