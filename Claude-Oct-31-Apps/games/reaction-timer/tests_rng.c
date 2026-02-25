/**
 * tests_rng.c - Unit tests for random number generator
 *
 * Tests randomness quality, reproducibility, and range functions.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "rng.h"
#include "test_suite.h"

/* Test: RNG initialization with seed */
static int test_rng_init_seed(void) {
    RNGState *rng = rng_init(12345);
    test_assert(rng != NULL, "RNG should initialize with seed");
    test_assert(rng_get_seed(rng) == 12345, "Seed should be stored");
    rng_free(rng);
    return 0;
}

/* Test: RNG secure initialization */
static int test_rng_init_secure(void) {
    RNGState *rng = rng_init_secure();
    test_assert(rng != NULL, "RNG should initialize securely");
    rng_free(rng);
    return 0;
}

/* Test: RNG reproducibility */
static int test_rng_reproducibility(void) {
    RNGState *rng1 = rng_init(42);
    RNGState *rng2 = rng_init(42);

    /* Both should produce identical sequence with same seed */
    uint64_t val1 = rng_next_u64(rng1);
    uint64_t val2 = rng_next_u64(rng2);
    test_assert(val1 == val2, "Same seed should produce same sequence");

    val1 = rng_next_u64(rng1);
    val2 = rng_next_u64(rng2);
    test_assert(val1 == val2, "Sequences should remain identical");

    rng_free(rng1);
    rng_free(rng2);
    return 0;
}

/* Test: Different seeds produce different sequences */
static int test_rng_different_seeds(void) {
    RNGState *rng1 = rng_init(11);
    RNGState *rng2 = rng_init(22);

    uint64_t val1 = rng_next_u64(rng1);
    uint64_t val2 = rng_next_u64(rng2);

    test_assert(val1 != val2, "Different seeds should produce different values");

    rng_free(rng1);
    rng_free(rng2);
    return 0;
}

/* Test: U32 generation */
static int test_rng_next_u32(void) {
    RNGState *rng = rng_init(999);

    uint32_t val = rng_next_u32(rng);
    test_assert(val >= 0, "U32 should be valid");

    rng_free(rng);
    return 0;
}

/* Test: Range generation */
static int test_rng_next_range(void) {
    RNGState *rng = rng_init(777);

    /* Generate 100 values in range [1000, 5000] */
    int all_valid = 1;
    for (int i = 0; i < 100; i++) {
        uint32_t val = rng_next_range(rng, 1000, 5000);
        if (val < 1000 || val > 5000) {
            all_valid = 0;
            break;
        }
    }
    test_assert(all_valid, "All values should be in range");

    rng_free(rng);
    return 0;
}

/* Test: Double generation [0, 1) */
static int test_rng_next_double(void) {
    RNGState *rng = rng_init(555);

    int all_valid = 1;
    for (int i = 0; i < 100; i++) {
        double val = rng_next_double(rng);
        if (val < 0.0 || val >= 1.0) {
            all_valid = 0;
            break;
        }
    }
    test_assert(all_valid, "All doubles should be in [0, 1)");

    rng_free(rng);
    return 0;
}

/* Test: Delay generation */
static int test_rng_delay_ms(void) {
    RNGState *rng = rng_init(333);

    int all_valid = 1;
    for (int i = 0; i < 50; i++) {
        uint32_t delay = rng_next_delay_ms(rng, 1000, 5000);
        if (delay < 1000 || delay > 5000) {
            all_valid = 0;
            break;
        }
    }
    test_assert(all_valid, "All delays should be in range");

    rng_free(rng);
    return 0;
}

/* Test: Shuffle operation */
static int test_rng_shuffle(void) {
    RNGState *rng = rng_init(111);

    int array[] = {1, 2, 3, 4, 5};
    int original[] = {1, 2, 3, 4, 5};

    test_assert(rng_shuffle(rng, array, 5, sizeof(int)) == 0,
                "Shuffle should succeed");

    /* At least one element should be out of order (probably) */
    int different = 0;
    for (int i = 0; i < 5; i++) {
        if (array[i] != original[i]) {
            different = 1;
            break;
        }
    }
    test_assert(different, "Shuffle should reorder elements");

    rng_free(rng);
    return 0;
}

/* Test: Choice selection */
static int test_rng_select_choice(void) {
    RNGState *rng = rng_init(222);

    const char *options = "ASDF";

    /* Select 50 times and verify all are valid */
    int all_valid = 1;
    for (int i = 0; i < 50; i++) {
        char choice = rng_select_choice(rng, options, 4);
        if (choice != 'A' && choice != 'S' && choice != 'D' && choice != 'F') {
            all_valid = 0;
            break;
        }
    }
    test_assert(all_valid, "All choices should be from options");

    rng_free(rng);
    return 0;
}

/* Test: Seeding */
static int test_rng_reseed(void) {
    RNGState *rng = rng_init(100);

    uint64_t val1 = rng_next_u64(rng);

    rng_reseed(rng, 100);
    uint64_t val2 = rng_next_u64(rng);

    test_assert(val1 == val2, "Reseed should restart sequence");

    rng_free(rng);
    return 0;
}

/* Test: Clone state */
static int test_rng_clone(void) {
    RNGState *rng1 = rng_init(456);

    uint64_t val1 = rng_next_u64(rng1);

    RNGState *rng2 = rng_clone(rng1);
    test_assert(rng2 != NULL, "Clone should succeed");

    uint64_t val2 = rng_next_u64(rng1);
    uint64_t val2_clone = rng_next_u64(rng2);

    test_assert(val2 == val2_clone, "Cloned RNG should produce same sequence");

    rng_free(rng1);
    rng_free(rng2);
    return 0;
}

/* Test: Reset to seed */
static int test_rng_reset(void) {
    RNGState *rng = rng_init(789);

    uint64_t val1 = rng_next_u64(rng);

    rng_reset(rng);
    uint64_t val2 = rng_next_u64(rng);

    test_assert(val1 == val2, "Reset should return to initial state");

    rng_free(rng);
    return 0;
}

/* Test: Distribution uniformity (basic) */
static int test_rng_distribution(void) {
    RNGState *rng = rng_init(1000);

    /* Count occurrences of each choice */
    int counts[4] = {0, 0, 0, 0};
    const char *options = "ABCD";

    for (int i = 0; i < 1000; i++) {
        char choice = rng_select_choice(rng, options, 4);
        for (int j = 0; j < 4; j++) {
            if (choice == options[j]) {
                counts[j]++;
            }
        }
    }

    /* Each should have roughly 250 counts (within 50% deviation) */
    int good_distribution = 1;
    for (int i = 0; i < 4; i++) {
        if (counts[i] < 150 || counts[i] > 350) {
            good_distribution = 0;
        }
    }
    test_assert(good_distribution, "Distribution should be roughly uniform");

    rng_free(rng);
    return 0;
}

/**
 * Register all RNG tests with the test suite.
 */
void test_rng_register_all(void) {
    test_register("rng_init_seed", test_rng_init_seed, "rng");
    test_register("rng_init_secure", test_rng_init_secure, "rng");
    test_register("rng_reproducibility", test_rng_reproducibility, "rng");
    test_register("rng_different_seeds", test_rng_different_seeds, "rng");
    test_register("rng_next_u32", test_rng_next_u32, "rng");
    test_register("rng_next_range", test_rng_next_range, "rng");
    test_register("rng_next_double", test_rng_next_double, "rng");
    test_register("rng_delay_ms", test_rng_delay_ms, "rng");
    test_register("rng_shuffle", test_rng_shuffle, "rng");
    test_register("rng_select_choice", test_rng_select_choice, "rng");
    test_register("rng_reseed", test_rng_reseed, "rng");
    test_register("rng_clone", test_rng_clone, "rng");
    test_register("rng_reset", test_rng_reset, "rng");
    test_register("rng_distribution", test_rng_distribution, "rng");
}
