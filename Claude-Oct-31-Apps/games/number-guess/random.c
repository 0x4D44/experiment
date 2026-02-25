#include "random.h"
#include <stdlib.h>
#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
#include <time.h>

/*
 * MERSENNE TWISTER IMPLEMENTATION (MT19937)
 * High quality random number generator
 */

#define MT_N 624
#define MT_M 397
#define MATRIX_A 0x9908b0dfUL
#define UPPER_MASK 0x80000000UL
#define LOWER_MASK 0x7fffffffUL

static unsigned long mt[MT_N];
static int mti = MT_N + 1;

void seed_random(unsigned int seed)
{
    mt[0] = seed & 0xffffffffUL;
    for (mti = 1; mti < MT_N; mti++) {
        unsigned long s = mt[mti - 1] ^ (mt[mti - 1] >> 30);
        mt[mti] = (((((s & 0xffff0000UL) >> 16) * 1812433253UL) << 16) +
                   (s & 0x0000ffffUL) * 1812433253UL) + mti;
        mt[mti] &= 0xffffffffUL;
    }
}

static void twist(void)
{
    unsigned long y;
    int kk;

    for (kk = 0; kk < MT_N - MT_M; kk++) {
        y = (mt[kk] & UPPER_MASK) | (mt[kk + 1] & LOWER_MASK);
        mt[kk] = mt[kk + MT_M] ^ (y >> 1) ^ ((y & 1UL) ? MATRIX_A : 0UL);
    }
    for (; kk < MT_N - 1; kk++) {
        y = (mt[kk] & UPPER_MASK) | (mt[kk + 1] & LOWER_MASK);
        mt[kk] = mt[kk + (MT_M - MT_N)] ^ (y >> 1) ^ ((y & 1UL) ? MATRIX_A : 0UL);
    }
    y = (mt[MT_N - 1] & UPPER_MASK) | (mt[0] & LOWER_MASK);
    mt[MT_N - 1] = mt[MT_M - 1] ^ (y >> 1) ^ ((y & 1UL) ? MATRIX_A : 0UL);

    mti = 0;
}

static unsigned long genrand_int32(void)
{
    unsigned long y;

    if (mti >= MT_N) {
        if (mti > MT_N) {
            seed_random(5489U);
        }
        twist();
    }

    y = mt[mti++];

    /* Tempering */
    y ^= (y >> 11);
    y ^= (y << 7) & 0x9d2c5680UL;
    y ^= (y << 15) & 0xefc60000UL;
    y ^= (y >> 18);

    return y;
}

float generate_random_float(void)
{
    return (float)genrand_int32() * (1.0f / 4294967296.0f);
}

int generate_random_number(int min, int max)
{
    if (min == max) {
        return min;
    }

    if (min > max) {
        int temp = min;
        min = max;
        max = temp;
    }

    unsigned long range = (unsigned long)(max - min + 1);
    unsigned long random_val = genrand_int32() % range;
    return min + (int)random_val;
}

void seed_random_from_entropy(void)
{
    unsigned int seed = 0;

    /* Try to read from /dev/urandom on Unix-like systems */
#ifdef __unix__
    int fd = open("/dev/urandom", O_RDONLY);
    if (fd >= 0) {
        if (read(fd, &seed, sizeof(seed)) == sizeof(seed)) {
            close(fd);
            seed_random(seed);
            return;
        }
        close(fd);
    }
#endif

    /* Fallback: use time and process ID */
    seed = (unsigned int)(time(NULL) ^ getpid());
    seed_random(seed);
}
