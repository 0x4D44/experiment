/**
 * Pattern Library for Conway's Game of Life
 * Contains famous patterns including oscillators, spaceships, still lifes, and guns
 */

const PATTERNS = {
    // OSCILLATORS - Patterns that repeat after N generations

    blinker: {
        name: "Blinker",
        description: "Period 2 oscillator - the simplest oscillator",
        width: 3,
        height: 1,
        cells: [
            [0, 0], [1, 0], [2, 0]
        ]
    },

    toad: {
        name: "Toad",
        description: "Period 2 oscillator",
        width: 4,
        height: 2,
        cells: [
            [1, 0], [2, 0], [3, 0],
            [0, 1], [1, 1], [2, 1]
        ]
    },

    beacon: {
        name: "Beacon",
        description: "Period 2 oscillator",
        width: 4,
        height: 4,
        cells: [
            [0, 0], [1, 0],
            [0, 1],
            [3, 2],
            [2, 3], [3, 3]
        ]
    },

    pulsar: {
        name: "Pulsar",
        description: "Period 3 oscillator - one of the most beautiful patterns",
        width: 13,
        height: 13,
        cells: [
            // Top section
            [2, 0], [3, 0], [4, 0], [8, 0], [9, 0], [10, 0],

            // Upper middle
            [0, 2], [5, 2], [7, 2], [12, 2],
            [0, 3], [5, 3], [7, 3], [12, 3],
            [0, 4], [5, 4], [7, 4], [12, 4],

            [2, 5], [3, 5], [4, 5], [8, 5], [9, 5], [10, 5],

            // Lower middle
            [2, 7], [3, 7], [4, 7], [8, 7], [9, 7], [10, 7],

            [0, 8], [5, 8], [7, 8], [12, 8],
            [0, 9], [5, 9], [7, 9], [12, 9],
            [0, 10], [5, 10], [7, 10], [12, 10],

            // Bottom section
            [2, 12], [3, 12], [4, 12], [8, 12], [9, 12], [10, 12]
        ]
    },

    pentadecathlon: {
        name: "Pentadecathlon",
        description: "Period 15 oscillator",
        width: 10,
        height: 3,
        cells: [
            [1, 0], [2, 0], [3, 0], [4, 0], [5, 0], [6, 0], [7, 0], [8, 0],
            [0, 1], [3, 1], [4, 1], [5, 1], [6, 1], [9, 1],
            [1, 2], [2, 2], [3, 2], [4, 2], [5, 2], [6, 2], [7, 2], [8, 2]
        ]
    },

    // SPACESHIPS - Patterns that translate themselves across the grid

    glider: {
        name: "Glider",
        description: "The smallest spaceship - moves diagonally",
        width: 3,
        height: 3,
        cells: [
            [1, 0],
            [2, 1],
            [0, 2], [1, 2], [2, 2]
        ]
    },

    lwss: {
        name: "Lightweight Spaceship (LWSS)",
        description: "Moves horizontally at c/2 speed",
        width: 5,
        height: 4,
        cells: [
            [1, 0], [4, 0],
            [0, 1],
            [0, 2], [4, 2],
            [0, 3], [1, 3], [2, 3], [3, 3]
        ]
    },

    mwss: {
        name: "Middleweight Spaceship (MWSS)",
        description: "Larger spaceship moving horizontally",
        width: 6,
        height: 5,
        cells: [
            [2, 0],
            [0, 1], [4, 1],
            [5, 2],
            [0, 3], [5, 3],
            [0, 4], [1, 4], [2, 4], [3, 4], [4, 4]
        ]
    },

    hwss: {
        name: "Heavyweight Spaceship (HWSS)",
        description: "The largest standard spaceship",
        width: 7,
        height: 5,
        cells: [
            [2, 0], [3, 0],
            [0, 1], [5, 1],
            [6, 2],
            [0, 3], [6, 3],
            [0, 4], [1, 4], [2, 4], [3, 4], [4, 4], [5, 4]
        ]
    },

    // STILL LIFES - Stable patterns that never change

    block: {
        name: "Block",
        description: "The simplest still life - 2×2 square",
        width: 2,
        height: 2,
        cells: [
            [0, 0], [1, 0],
            [0, 1], [1, 1]
        ]
    },

    beehive: {
        name: "Beehive",
        description: "Hexagonal still life",
        width: 4,
        height: 3,
        cells: [
            [1, 0], [2, 0],
            [0, 1], [3, 1],
            [1, 2], [2, 2]
        ]
    },

    loaf: {
        name: "Loaf",
        description: "Asymmetric still life",
        width: 4,
        height: 4,
        cells: [
            [1, 0], [2, 0],
            [0, 1], [3, 1],
            [1, 2], [3, 2],
            [2, 3]
        ]
    },

    boat: {
        name: "Boat",
        description: "Small still life",
        width: 3,
        height: 3,
        cells: [
            [0, 0], [1, 0],
            [0, 1], [2, 1],
            [1, 2]
        ]
    },

    // GUNS - Patterns that generate spaceships

    gosperGun: {
        name: "Gosper Glider Gun",
        description: "First discovered gun - shoots gliders every 30 generations",
        width: 36,
        height: 9,
        cells: [
            // Left square
            [0, 4], [1, 4],
            [0, 5], [1, 5],

            // Left blob
            [10, 4], [11, 4], [12, 4],
            [10, 5], [11, 5], [12, 5],
            [9, 6], [13, 6],
            [8, 7], [14, 7],
            [8, 8], [14, 8],
            [11, 9],
            [9, 10], [13, 10],
            [10, 11], [11, 11], [12, 11],
            [11, 12],

            // Right section
            [20, 2], [21, 2],
            [20, 3], [21, 3],
            [20, 4], [21, 4],
            [22, 1], [22, 5],
            [24, 0], [24, 1], [24, 5], [24, 6],

            // Right square
            [34, 2], [35, 2],
            [34, 3], [35, 3]
        ]
    },

    simkinGun: {
        name: "Simkin Glider Gun",
        description: "Smaller glider gun with period 120",
        width: 33,
        height: 21,
        cells: [
            // Top left block
            [0, 0], [1, 0],
            [0, 1], [1, 1],

            // Left section
            [7, 2], [8, 2],
            [7, 3], [8, 3],

            [4, 4], [5, 4],
            [4, 5], [5, 5],

            // Bottom left
            [21, 10], [22, 10],
            [21, 11], [22, 11],

            [24, 12], [25, 12],
            [24, 13], [25, 13],

            [31, 14], [32, 14],
            [31, 15], [32, 15],

            // Right section
            [19, 17], [20, 17],
            [19, 18], [20, 18],

            [27, 19], [28, 19],
            [27, 20], [28, 20]
        ]
    }
};

// Export patterns
if (typeof module !== 'undefined' && module.exports) {
    module.exports = PATTERNS;
}
