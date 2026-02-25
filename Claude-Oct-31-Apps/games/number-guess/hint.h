#ifndef HINT_H
#define HINT_H

/*
 * HINT SYSTEM
 * Provides intelligent hints without spoiling the answer
 */

/* Mathematical analysis of numbers */
int is_prime(int num);
int is_perfect_square(int num);
int is_fibonacci(int num);
int is_even(int num);
int digit_sum(int num);
int count_divisors(int num);
int largest_prime_factor(int num);

/* Hint generation */
const char* hint_get_mathematical(int secret);
const char* hint_get_range(int guess, int secret, int min, int max);
const char* hint_get_proximity(int guess, int secret, int range_size);
const char* hint_get_pattern(int secret);

/* Hint utility functions */
int hint_is_even_or_odd(int num);
int hint_get_digit_range(int num);
const char* hint_describe_size(int num, int range_max);

#endif
