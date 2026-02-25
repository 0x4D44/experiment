/**
 * test_runner.c - Test suite execution framework
 *
 * Main test runner for all unit tests.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "test_suite.h"

/* Maximum number of test cases */
#define MAX_TESTS 256

/* Global test registry */
typedef struct {
    TestCase cases[MAX_TESTS];
    int count;
    TestResult results[MAX_TESTS];
} TestRegistry;

static TestRegistry test_registry = {0};

/**
 * Register a test case for execution.
 */
int test_register(const char *name, TestFunction func, const char *category) {
    if (test_registry.count >= MAX_TESTS) {
        fprintf(stderr, "Error: Too many test cases (max %d)\n", MAX_TESTS);
        return -1;
    }

    test_registry.cases[test_registry.count].name = name;
    test_registry.cases[test_registry.count].func = func;
    test_registry.cases[test_registry.count].category = category;
    test_registry.count++;

    return 0;
}

/**
 * Assert condition is true.
 */
int test_assert(int condition, const char *message) {
    if (!condition) {
        fprintf(stderr, "  FAIL: %s\n", message);
        return -1;
    }
    return 0;
}

/**
 * Assert equality.
 */
int test_assert_equal(int expected, int actual, const char *message) {
    if (expected != actual) {
        fprintf(stderr, "  FAIL: %s (expected %d, got %d)\n",
                message, expected, actual);
        return -1;
    }
    return 0;
}

/**
 * Assert near equality for doubles.
 */
int test_assert_near(double expected, double actual, double tolerance,
                      const char *message) {
    double diff = expected > actual ? expected - actual : actual - expected;
    if (diff > tolerance) {
        fprintf(stderr, "  FAIL: %s (expected %f, got %f, diff %f > %f)\n",
                message, expected, actual, diff, tolerance);
        return -1;
    }
    return 0;
}

/**
 * Assert string equality.
 */
int test_assert_string_equal(const char *expected, const char *actual,
                              const char *message) {
    if (strcmp(expected, actual) != 0) {
        fprintf(stderr, "  FAIL: %s (expected '%s', got '%s')\n",
                message, expected, actual);
        return -1;
    }
    return 0;
}

/**
 * Run all registered tests.
 */
int test_run_all(void) {
    printf("\n=== REACTION TIMER TEST SUITE ===\n");
    printf("Running %d tests...\n\n", test_registry.count);

    int passed = 0;
    int failed = 0;

    for (int i = 0; i < test_registry.count; i++) {
        TestCase *test = &test_registry.cases[i];
        printf("[%s] %s... ", test->category, test->name);
        fflush(stdout);

        clock_t start = clock();
        int result = test->func();
        clock_t end = clock();

        double elapsed_ms = (double)(end - start) / CLOCKS_PER_SEC * 1000.0;

        if (result == 0) {
            printf("PASS (%.2fms)\n", elapsed_ms);
            passed++;
        } else {
            printf("FAIL\n");
            failed++;
        }
    }

    printf("\n=== TEST SUMMARY ===\n");
    printf("Total:  %d\n", test_registry.count);
    printf("Passed: %d\n", passed);
    printf("Failed: %d\n", failed);
    printf("Success Rate: %.1f%%\n", 100.0 * passed / test_registry.count);

    return failed == 0 ? 0 : 1;
}

/**
 * Run tests for specific category.
 */
int test_run_category(const char *category) {
    printf("\n=== TESTS: %s ===\n", category);

    int passed = 0;
    int failed = 0;

    for (int i = 0; i < test_registry.count; i++) {
        TestCase *test = &test_registry.cases[i];
        if (strcmp(test->category, category) != 0) {
            continue;
        }

        printf("[%s] %s... ", test->category, test->name);
        fflush(stdout);

        int result = test->func();

        if (result == 0) {
            printf("PASS\n");
            passed++;
        } else {
            printf("FAIL\n");
            failed++;
        }
    }

    printf("Category Summary: %d passed, %d failed\n", passed, failed);

    return failed == 0 ? 0 : 1;
}

/**
 * Run single test by name.
 */
int test_run_single(const char *test_name) {
    for (int i = 0; i < test_registry.count; i++) {
        TestCase *test = &test_registry.cases[i];
        if (strcmp(test->name, test_name) == 0) {
            printf("\n=== TEST: %s ===\n", test_name);
            int result = test->func();
            printf(result == 0 ? "PASS\n" : "FAIL\n");
            return result;
        }
    }

    fprintf(stderr, "Test not found: %s\n", test_name);
    return -1;
}

/**
 * Get test results summary.
 */
int test_get_summary(int *total, int *passed, int *failed) {
    *total = test_registry.count;
    *passed = 0;
    *failed = 0;

    /* Would need to track results from actual test runs */
    return 0;
}

/**
 * Forward declarations for test registration functions.
 */
extern void test_timer_register_all(void);
extern void test_statistics_register_all(void);
extern void test_rng_register_all(void);

/**
 * Main test runner.
 */
int main(int argc, char *argv[]) {
    /* Register all tests */
    test_timer_register_all();
    test_statistics_register_all();
    test_rng_register_all();

    /* Run tests based on arguments */
    if (argc > 1) {
        if (strcmp(argv[1], "--category") == 0 && argc > 2) {
            return test_run_category(argv[2]);
        } else if (strcmp(argv[1], "--single") == 0 && argc > 2) {
            return test_run_single(argv[2]);
        }
    }

    /* Run all tests by default */
    return test_run_all();
}
