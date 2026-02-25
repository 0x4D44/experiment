#include "hint.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>

/*
 * MATHEMATICAL ANALYSIS FUNCTIONS
 */

int is_even(int num)
{
    return (num % 2 == 0);
}

int is_prime(int num)
{
    if (num < 2) return 0;
    if (num == 2) return 1;
    if (num % 2 == 0) return 0;

    for (int i = 3; i * i <= num; i += 2) {
        if (num % i == 0) return 0;
    }
    return 1;
}

int is_perfect_square(int num)
{
    if (num < 0) return 0;
    int root = (int)sqrt((double)num);
    return (root * root == num);
}

int is_fibonacci(int num)
{
    /* A number is Fibonacci if one of (5*n^2 + 4) or (5*n^2 - 4) is a perfect square */
    if (num < 0) return 0;

    int a = 5 * num * num + 4;
    int b = 5 * num * num - 4;

    return is_perfect_square(a) || is_perfect_square(b);
}

int digit_sum(int num)
{
    int sum = 0;
    if (num < 0) num = -num;

    while (num > 0) {
        sum += num % 10;
        num /= 10;
    }
    return sum;
}

int count_divisors(int num)
{
    if (num <= 0) return 0;

    int count = 0;
    for (int i = 1; i * i <= num; i++) {
        if (num % i == 0) {
            if (i * i == num) {
                count++;
            } else {
                count += 2;
            }
        }
    }
    return count;
}

int largest_prime_factor(int num)
{
    if (num <= 1) return 1;

    int largest = -1;

    /* Remove factor 2 */
    while (num % 2 == 0) {
        largest = 2;
        num /= 2;
    }

    /* Check odd factors */
    for (int i = 3; i * i <= num; i += 2) {
        while (num % i == 0) {
            largest = i;
            num /= i;
        }
    }

    if (num > 2) {
        largest = num;
    }

    return largest;
}

int hint_is_even_or_odd(int num)
{
    return num % 2;  /* Returns 0 for even, 1 for odd */
}

int hint_get_digit_range(int num)
{
    if (num < 10) return 1;
    if (num < 100) return 2;
    if (num < 1000) return 3;
    if (num < 10000) return 4;
    if (num < 100000) return 5;
    return 6;
}

const char* hint_describe_size(int num, int range_max)
{
    float percentage = (float)num / range_max * 100.0f;

    if (percentage < 25) return "in the lower quarter";
    if (percentage < 50) return "in the lower half";
    if (percentage < 75) return "in the upper half";
    return "in the upper quarter";
}

/*
 * HINT GENERATION FUNCTIONS
 */

const char* hint_get_mathematical(int secret)
{
    static char hint_buffer[256];

    /* Check for multiple properties and provide most interesting hint */
    if (is_fibonacci(secret)) {
        snprintf(hint_buffer, sizeof(hint_buffer),
                "The number is a Fibonacci number");
        return hint_buffer;
    }

    if (is_prime(secret)) {
        snprintf(hint_buffer, sizeof(hint_buffer),
                "The number is a prime number");
        return hint_buffer;
    }

    if (is_perfect_square(secret)) {
        snprintf(hint_buffer, sizeof(hint_buffer),
                "The number is a perfect square");
        return hint_buffer;
    }

    if (secret % 10 == 0) {
        snprintf(hint_buffer, sizeof(hint_buffer),
                "The number is divisible by 10");
        return hint_buffer;
    }

    if (secret % 5 == 0) {
        snprintf(hint_buffer, sizeof(hint_buffer),
                "The number is divisible by 5 (ends in 0 or 5)");
        return hint_buffer;
    }

    int dsum = digit_sum(secret);
    if (dsum == 9 || (secret % 9 == 0)) {
        snprintf(hint_buffer, sizeof(hint_buffer),
                "The number is divisible by 9 (digit sum: %d)", dsum);
        return hint_buffer;
    }

    if (is_even(secret)) {
        snprintf(hint_buffer, sizeof(hint_buffer),
                "The number is even");
        return hint_buffer;
    } else {
        snprintf(hint_buffer, sizeof(hint_buffer),
                "The number is odd");
        return hint_buffer;
    }
}

const char* hint_get_range(int guess, int secret, int min, int max)
{
    static char hint_buffer[256];

    if (guess < secret) {
        snprintf(hint_buffer, sizeof(hint_buffer),
                "Too low! Try higher (range: %d-%d)", guess + 1, max);
    } else if (guess > secret) {
        snprintf(hint_buffer, sizeof(hint_buffer),
                "Too high! Try lower (range: %d-%d)", min, guess - 1);
    } else {
        snprintf(hint_buffer, sizeof(hint_buffer), "Correct!");
    }

    return hint_buffer;
}

const char* hint_get_proximity(int guess, int secret, int range_size)
{
    static char hint_buffer[256];

    int distance = abs(secret - guess);
    float percentage = (float)distance / range_size * 100.0f;

    if (distance == 0) {
        snprintf(hint_buffer, sizeof(hint_buffer), "You got it!");
        return hint_buffer;
    }

    if (percentage < 5) {
        snprintf(hint_buffer, sizeof(hint_buffer),
                "You're extremely close! Only %d away", distance);
    } else if (percentage < 10) {
        snprintf(hint_buffer, sizeof(hint_buffer),
                "Very close! About %d away", distance);
    } else if (percentage < 25) {
        snprintf(hint_buffer, sizeof(hint_buffer),
                "Getting warmer... about %d away", distance);
    } else if (percentage < 50) {
        snprintf(hint_buffer, sizeof(hint_buffer),
                "Warm, but still far. About %d away", distance);
    } else {
        snprintf(hint_buffer, sizeof(hint_buffer),
                "Cold... very far away. About %d away", distance);
    }

    return hint_buffer;
}

const char* hint_get_pattern(int secret)
{
    static char hint_buffer[256];

    /* Provide pattern-based hints */
    int dsum = digit_sum(secret);

    if (dsum % 2 == 0) {
        snprintf(hint_buffer, sizeof(hint_buffer),
                "The sum of its digits is even (%d)", dsum);
    } else {
        snprintf(hint_buffer, sizeof(hint_buffer),
                "The sum of its digits is odd (%d)", dsum);
    }

    return hint_buffer;
}

/*
 * TEST COMPATIBILITY FUNCTIONS
 * Alias functions for test suite compatibility
 */

const char* get_range_hint(int guess, int secret, int min, int max)
{
    return hint_get_range(guess, secret, min, max);
}

const char* get_mathematical_hint(int secret)
{
    return hint_get_mathematical(secret);
}

const char* get_proximity_hint(int guess, int secret, int range_size)
{
    return hint_get_proximity(guess, secret, range_size);
}
