#ifndef RANDOM_H
#define RANDOM_H

/*
 * RANDOM NUMBER GENERATION
 * Using MT19937 (Mersenne Twister) for high-quality randomness
 */

/* Initialize random number generator with seed */
void seed_random(unsigned int seed);

/* Generate random integer in range [min, max] inclusive */
int generate_random_number(int min, int max);

/* Generate random float in range [0.0, 1.0] */
float generate_random_float(void);

/* Seed from system entropy if available */
void seed_random_from_entropy(void);

#endif
