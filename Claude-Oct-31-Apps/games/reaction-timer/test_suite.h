/**
 * test_suite.h - Comprehensive test framework
 *
 * Defines test structure and runner for unit tests covering
 * all modules with >80% coverage target.
 */

#ifndef TEST_SUITE_H
#define TEST_SUITE_H

#include <stdint.h>

/* Test result structure */
typedef struct {
    const char *test_name;      /* Human-readable test name */
    int passed;                 /* 1 if passed, 0 if failed */
    const char *error_msg;      /* Error message if failed */
    uint64_t duration_us;       /* Time taken to run */
} TestResult;

/* Test function pointer type */
typedef int (*TestFunction)(void);

/* Test case registration */
typedef struct {
    const char *name;           /* Test name */
    TestFunction func;          /* Test function */
    const char *category;       /* Test category (module) */
} TestCase;

/**
 * Run all registered tests.
 *
 * Returns: 0 if all passed, 1 if any failed
 */
int test_run_all(void);

/**
 * Run tests for specific category/module.
 *
 * category: Module name (timer, input, statistics, etc.)
 *
 * Returns: 0 if all passed, 1 if any failed
 */
int test_run_category(const char *category);

/**
 * Run single test by name.
 *
 * test_name: Exact test name
 *
 * Returns: 0 if passed, 1 if failed
 */
int test_run_single(const char *test_name);

/**
 * Register test case for execution.
 * Called at startup to register all tests.
 *
 * name: Test name
 * func: Test function
 * category: Module category
 *
 * Returns: 0 on success, -1 on failure
 */
int test_register(const char *name, TestFunction func, const char *category);

/**
 * Get test results summary.
 *
 * total: Output total tests run
 * passed: Output tests passed
 * failed: Output tests failed
 *
 * Returns: 0 on success
 */
int test_get_summary(int *total, int *passed, int *failed);

/**
 * Assert condition is true.
 * Used in test functions.
 *
 * condition: Condition to check
 * message: Error message if false
 *
 * Returns: 0 if true, -1 if false
 */
int test_assert(int condition, const char *message);

/**
 * Assert equality.
 *
 * expected: Expected value
 * actual: Actual value
 * message: Error message
 *
 * Returns: 0 if equal, -1 if different
 */
int test_assert_equal(int expected, int actual, const char *message);

/**
 * Assert near equality for doubles.
 *
 * expected: Expected value
 * actual: Actual value
 * tolerance: Acceptable difference
 * message: Error message
 *
 * Returns: 0 if within tolerance, -1 otherwise
 */
int test_assert_near(double expected, double actual, double tolerance,
                      const char *message);

/**
 * Assert string equality.
 *
 * expected: Expected string
 * actual: Actual string
 * message: Error message
 *
 * Returns: 0 if equal, -1 if different
 */
int test_assert_string_equal(const char *expected, const char *actual,
                              const char *message);

#endif /* TEST_SUITE_H */
