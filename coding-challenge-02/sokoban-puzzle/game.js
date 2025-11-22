/**
 * SOKOBAN PUZZLE MASTER - CHAMPIONSHIP EDITION
 * Full-featured Sokoban game with 30+ levels, themes, achievements, and more
 */

// ========================================
// GAME CONSTANTS
// ========================================

const TILE_SIZE = 48;
const TILE_TYPES = {
    EMPTY: 0,
    WALL: 1,
    FLOOR: 2,
    BOX: 3,
    TARGET: 4,
    PLAYER: 5,
    BOX_ON_TARGET: 6
};

const DIRECTIONS = {
    UP: { x: 0, y: -1 },
    DOWN: { x: 0, y: 1 },
    LEFT: { x: -1, y: 0 },
    RIGHT: { x: 1, y: 0 }
};

// ========================================
// LEVEL DATABASE
// ========================================

const LEVEL_PACKS = {
    beginner: {
        name: "Beginner Pack",
        icon: "🌱",
        levels: [
            // Level 1 - Tutorial
            {
                name: "First Steps",
                difficulty: "easy",
                optimal: 8,
                grid: [
                    "########",
                    "#      #",
                    "# .$ @ #",
                    "#      #",
                    "########"
                ]
            },
            // Level 2
            {
                name: "Simple Push",
                difficulty: "easy",
                optimal: 12,
                grid: [
                    "########",
                    "#   .  #",
                    "#  $   #",
                    "#  @   #",
                    "#   .  #",
                    "#  $   #",
                    "########"
                ]
            },
            // Level 3
            {
                name: "Corner Turn",
                difficulty: "easy",
                optimal: 15,
                grid: [
                    "#########",
                    "#       #",
                    "# $  .  #",
                    "#   $#  #",
                    "#@  .   #",
                    "#       #",
                    "#########"
                ]
            },
            // Level 4
            {
                name: "Three Boxes",
                difficulty: "easy",
                optimal: 20,
                grid: [
                    "##########",
                    "#        #",
                    "#  ...   #",
                    "#        #",
                    "#  $$$   #",
                    "#    @   #",
                    "#        #",
                    "##########"
                ]
            },
            // Level 5
            {
                name: "Square Dance",
                difficulty: "easy",
                optimal: 25,
                grid: [
                    "##########",
                    "#   ##   #",
                    "# $    $ #",
                    "#  .  .  #",
                    "#   @    #",
                    "#  .  .  #",
                    "# $    $ #",
                    "#   ##   #",
                    "##########"
                ]
            },
            // Level 6
            {
                name: "Maze Start",
                difficulty: "medium",
                optimal: 30,
                grid: [
                    "###########",
                    "#    #    #",
                    "# $  .  $ #",
                    "### ### ###",
                    "#  $  .  $#",
                    "#    @   .#",
                    "###########"
                ]
            },
            // Level 7
            {
                name: "Box Shuffle",
                difficulty: "medium",
                optimal: 35,
                grid: [
                    "############",
                    "#     #    #",
                    "# $$  .    #",
                    "#  #  . #  #",
                    "#  #  . #  #",
                    "#  # $$@.  #",
                    "#          #",
                    "############"
                ]
            },
            // Level 8
            {
                name: "T-Junction",
                difficulty: "medium",
                optimal: 40,
                grid: [
                    "#############",
                    "#           #",
                    "# ... $$$ @ #",
                    "#     #     #",
                    "#     $     #",
                    "#     .     #",
                    "#           #",
                    "#############"
                ]
            }
        ]
    },
    intermediate: {
        name: "Intermediate Pack",
        icon: "⚡",
        levels: [
            // Level 9
            {
                name: "Spiral",
                difficulty: "medium",
                optimal: 45,
                grid: [
                    "##############",
                    "#            #",
                    "# ########## #",
                    "# #.....   # #",
                    "# # #### $ # #",
                    "# #    # $ # #",
                    "# #### # $ # #",
                    "# @  $ $   # #",
                    "# ########## #",
                    "#            #",
                    "##############"
                ]
            },
            // Level 10
            {
                name: "Corridors",
                difficulty: "medium",
                optimal: 50,
                grid: [
                    "###############",
                    "#   #     #   #",
                    "# $ # .$. # $ #",
                    "#   #  .  #   #",
                    "#####  @  #####",
                    "#   #  .  #   #",
                    "# $ # .$. # $ #",
                    "#   #     #   #",
                    "###############"
                ]
            },
            // Level 11
            {
                name: "Diamond",
                difficulty: "medium",
                optimal: 55,
                grid: [
                    "###############",
                    "#      #      #",
                    "#  $   .   $  #",
                    "#    # . #    #",
                    "###  #   #  ###",
                    "#  $   .   $  #",
                    "#      @      #",
                    "###############"
                ]
            },
            // Level 12
            {
                name: "Warehouse",
                difficulty: "hard",
                optimal: 60,
                grid: [
                    "################",
                    "#   #      #   #",
                    "# $ # $  $ # $ #",
                    "#   #  ##  #   #",
                    "### ........ ###",
                    "#   #  ##  #   #",
                    "# $ #  @   # $ #",
                    "#   #      #   #",
                    "################"
                ]
            },
            // Level 13
            {
                name: "Puzzle Box",
                difficulty: "hard",
                optimal: 65,
                grid: [
                    "#################",
                    "#         #     #",
                    "# $$$ ### # ... #",
                    "#     #   # ... #",
                    "##### #   # ... #",
                    "#   # ##### #####",
                    "# $ #     # $   #",
                    "#   # $$  @     #",
                    "#   #     #     #",
                    "#################"
                ]
            },
            // Level 14
            {
                name: "Symmetry",
                difficulty: "hard",
                optimal: 70,
                grid: [
                    "##################",
                    "#                #",
                    "#  $.$   @   $.$  #",
                    "#  $ $       $ $  #",
                    "#  ...       ...  #",
                    "#                #",
                    "##################"
                ]
            },
            // Level 15
            {
                name: "Chambers",
                difficulty: "hard",
                optimal: 75,
                grid: [
                    "##################",
                    "#   #   ##   #   #",
                    "# $ . $ ## $ . $ #",
                    "#   #   ##   #   #",
                    "######    ########",
                    "#   #   .  #   # #",
                    "# $ . $  $ . $ # #",
                    "#   #   @  #   # #",
                    "##################"
                ]
            }
        ]
    },
    advanced: {
        name: "Advanced Pack",
        icon: "🔥",
        levels: [
            // Level 16
            {
                name: "Cross Pattern",
                difficulty: "hard",
                optimal: 80,
                grid: [
                    "###################",
                    "#        #        #",
                    "#  ....  #  $$$$  #",
                    "#        #        #",
                    "####  ########  ###",
                    "#                 #",
                    "#  $$$$  @  ....  #",
                    "#                 #",
                    "###################"
                ]
            },
            // Level 17
            {
                name: "Labyrinth",
                difficulty: "expert",
                optimal: 90,
                grid: [
                    "####################",
                    "#    #   #    #    #",
                    "# $  . $ . $  . $  #",
                    "#  # # # # # # # # #",
                    "# .   .   @   .  . #",
                    "#  # # # # # # # # #",
                    "# $  . $ . $  . $  #",
                    "#    #   #    #    #",
                    "####################"
                ]
            },
            // Level 18
            {
                name: "Tower",
                difficulty: "expert",
                optimal: 85,
                grid: [
                    "##############",
                    "#      #     #",
                    "# $$$  .     #",
                    "# ...  #  ####",
                    "# $$$  # $   #",
                    "# ...  #   # #",
                    "# @$$      # #",
                    "#########    #",
                    "#        $$$ #",
                    "#        ... #",
                    "##############"
                ]
            },
            // Level 19
            {
                name: "Castle",
                difficulty: "expert",
                optimal: 95,
                grid: [
                    "#####################",
                    "#   #       #       #",
                    "# $ .   $   . $ #   #",
                    "#   # $   $ #   #   #",
                    "##### . # . ### . ###",
                    "#     $ # $     $   #",
                    "# ### . @ . ### .   #",
                    "#     $ # $     $   #",
                    "##### . # . ### . ###",
                    "#   # $   $ #   #   #",
                    "# $ .   $   . $ #   #",
                    "#   #       #       #",
                    "#####################"
                ]
            },
            // Level 20
            {
                name: "Fortress",
                difficulty: "expert",
                optimal: 100,
                grid: [
                    "######################",
                    "#         #          #",
                    "# $$$#### ### ####   #",
                    "# .......       # $$ #",
                    "# $$$#### ###   #    #",
                    "#         # # ### ## #",
                    "######    # # #   ## #",
                    "#   @  $$ . . .   ## #",
                    "#   #### ## ## ## ## #",
                    "#        #  #  #     #",
                    "######################"
                ]
            }
        ]
    },
    expert: {
        name: "Expert Pack",
        icon: "💀",
        levels: [
            // Level 21
            {
                name: "Mind Bender",
                difficulty: "expert",
                optimal: 110,
                grid: [
                    "#######################",
                    "#     #   #   #   #   #",
                    "# $ . . $ . $ . $ . $ #",
                    "#   # # # # # # # # # #",
                    "### . $ . $ @ $ . $ . #",
                    "#   # # # # # # # # # #",
                    "# $ . . $ . $ . $ . $ #",
                    "#     #   #   #   #   #",
                    "#######################"
                ]
            },
            // Level 22
            {
                name: "The Gauntlet",
                difficulty: "expert",
                optimal: 120,
                grid: [
                    "########################",
                    "#                      #",
                    "# ########  ########   #",
                    "# #....  #  #  $$$$#   #",
                    "# #....  #  #      #   #",
                    "# ######## ## ######   #",
                    "#          ##          #",
                    "# ######## ## ######   #",
                    "# #  $$$$#  #      #   #",
                    "# #      #  #....  #   #",
                    "# ######## @########   #",
                    "#                      #",
                    "########################"
                ]
            },
            // Level 23
            {
                name: "Nightmare",
                difficulty: "expert",
                optimal: 130,
                grid: [
                    "#########################",
                    "#    #    #      #      #",
                    "#  $ . $  #  $.  .  .$  #",
                    "# .  #  . # $  $ # $  $ #",
                    "#  $ # $  #  ..  #  ..  #",
                    "## . # . ####### # ######",
                    "#  $ @ $          #     #",
                    "## . # . ####### # ######",
                    "#  $ # $  #  ..  #  ..  #",
                    "# .  #  . # $  $ # $  $ #",
                    "#  $ . $  #  $.  .  .$  #",
                    "#    #    #      #      #",
                    "#########################"
                ]
            },
            // Level 24
            {
                name: "Chaos Theory",
                difficulty: "expert",
                optimal: 115,
                grid: [
                    "##########################",
                    "#                        #",
                    "# $ # $ # $ # $ # $ # $  #",
                    "#   .   .   .   .   . .  #",
                    "# # # # # # # # # # # # ##",
                    "# . # . # . # @ # . # .  #",
                    "# # # # # # # # # # # # ##",
                    "#   .   .   .   .   . .  #",
                    "# $ # $ # $ # $ # $ # $  #",
                    "#                        #",
                    "##########################"
                ]
            },
            // Level 25
            {
                name: "Infinity",
                difficulty: "expert",
                optimal: 125,
                grid: [
                    "###########################",
                    "#                         #",
                    "#  ####  #######  ####    #",
                    "#  #.##  #     #  ##.#    #",
                    "#  #.##  #  $  #  ##.#    #",
                    "#  ####  # $ $ #  ####    #",
                    "#        #  $  #          #",
                    "#  ####  # $ $ #  ####    #",
                    "#  #.##  #  $  #  ##.#    #",
                    "#  #.##  #  @  #  ##.#    #",
                    "#  ####  #######  ####    #",
                    "#                         #",
                    "###########################"
                ]
            }
        ]
    },
    master: {
        name: "Master Pack",
        icon: "👑",
        levels: [
            // Level 26
            {
                name: "Grand Master",
                difficulty: "expert",
                optimal: 140,
                grid: [
                    "############################",
                    "#         #                #",
                    "# $$$ ### # ### $$$  ###   #",
                    "# ... #   # # # ...  # #   #",
                    "# ... ### # # # ...  # #   #",
                    "#     #   # # #      # #   #",
                    "##### ##### # ######## #   #",
                    "#                          #",
                    "# ### ####### # ######## # #",
                    "#   # #     # # #        # #",
                    "# $ $ $  @  $ $ $  $$$   # #",
                    "#   # #     # # #        # #",
                    "# ### ####### # ######## # #",
                    "#                          #",
                    "############################"
                ]
            },
            // Level 27
            {
                name: "Ultimate Challenge",
                difficulty: "expert",
                optimal: 150,
                grid: [
                    "#############################",
                    "#                           #",
                    "# $$$$$ ########### .....   #",
                    "#       #         #         #",
                    "# #######  #####  #         #",
                    "#          #   #  #         #",
                    "# #######  #   #  #         #",
                    "#       #  #   #  ###########",
                    "# $$$$$ #  # @ #            #",
                    "#       #  #   #  ###########",
                    "# #######  #   #  #         #",
                    "#          #   #  #         #",
                    "# #######  #####  #         #",
                    "#       #         #         #",
                    "# $$$$$ ########### .....   #",
                    "#                           #",
                    "#############################"
                ]
            },
            // Level 28
            {
                name: "Perfect Storm",
                difficulty: "expert",
                optimal: 135,
                grid: [
                    "##############################",
                    "#   #    #   #    #    #     #",
                    "# $ . $  # $ . $  # $  . $ # #",
                    "#   # #  #   # #  #  # #   # #",
                    "### $ ## ### $ ## ## $ ## ## #",
                    "#   # #  #   # #  #  # # @ # #",
                    "# . . .  # . . .  # .  . . # #",
                    "#   #    #   #    #    #     #",
                    "##############################"
                ]
            },
            // Level 29
            {
                name: "Legend",
                difficulty: "expert",
                optimal: 160,
                grid: [
                    "################################",
                    "#                              #",
                    "#  ##########################  #",
                    "#  #                        #  #",
                    "#  # $$$$$  .....           #  #",
                    "#  # $$$$$  .....       ##  #  #",
                    "#  #                    ##  #  #",
                    "#  # ####################   #  #",
                    "#  #                        #  #",
                    "#  # ###################### #  #",
                    "#  # #                    # #  #",
                    "#  # #  .....       $$$$$ # #  #",
                    "#  # #  .....       $$$$$ # #  #",
                    "#  # #                  @ # #  #",
                    "#  # ###################### #  #",
                    "#  #                        #  #",
                    "#  ##########################  #",
                    "#                              #",
                    "################################"
                ]
            },
            // Level 30
            {
                name: "Eternal Glory",
                difficulty: "expert",
                optimal: 180,
                grid: [
                    "##################################",
                    "#                                #",
                    "# ##### ##### ##### ##### #####  #",
                    "# #...# #...# #...# #...# #...#  #",
                    "# #...# #...# #...# #...# #...#  #",
                    "# ##### ##### ##### ##### #####  #",
                    "#                                #",
                    "# ##### ##### ##### ##### #####  #",
                    "# #$$$# #$$$# #$$$# #$$$# #$$$#  #",
                    "# #$$$# #$$$# #$$$# #$$$# #$$$#  #",
                    "# ##### ##### ##### ##### #####  #",
                    "#                                #",
                    "# ################# #############",
                    "#                 @              #",
                    "##################################"
                ]
            }
        ]
    }
};

// ========================================
// ACHIEVEMENTS DATABASE
// ========================================

const ACHIEVEMENTS = [
    {
        id: "first_steps",
        name: "First Steps",
        description: "Complete your first level",
        icon: "🎯",
        condition: (stats) => stats.levelsCompleted >= 1
    },
    {
        id: "getting_started",
        name: "Getting Started",
        description: "Complete 5 levels",
        icon: "⭐",
        condition: (stats) => stats.levelsCompleted >= 5
    },
    {
        id: "puzzle_solver",
        name: "Puzzle Solver",
        description: "Complete 10 levels",
        icon: "🧩",
        condition: (stats) => stats.levelsCompleted >= 10
    },
    {
        id: "box_master",
        name: "Box Master",
        description: "Complete 20 levels",
        icon: "📦",
        condition: (stats) => stats.levelsCompleted >= 20
    },
    {
        id: "completionist",
        name: "Completionist",
        description: "Complete all 30 levels",
        icon: "💯",
        condition: (stats) => stats.levelsCompleted >= 30
    },
    {
        id: "perfectionist",
        name: "Perfectionist",
        description: "Get 3 stars on any level",
        icon: "⭐",
        condition: (stats) => stats.perfectClears >= 1
    },
    {
        id: "golden_touch",
        name: "Golden Touch",
        description: "Get 3 stars on 10 levels",
        icon: "🌟",
        condition: (stats) => stats.perfectClears >= 10
    },
    {
        id: "efficiency_expert",
        name: "Efficiency Expert",
        description: "Complete a level in optimal moves",
        icon: "🎯",
        condition: (stats) => stats.optimalSolutions >= 1
    },
    {
        id: "speed_demon",
        name: "Speed Demon",
        description: "Complete a level in under 30 seconds",
        icon: "⚡",
        condition: (stats) => stats.fastestLevel <= 30
    },
    {
        id: "thinking_ahead",
        name: "Thinking Ahead",
        description: "Complete a level without using undo",
        icon: "🧠",
        condition: (stats) => stats.noUndoClears >= 1
    },
    {
        id: "persistent",
        name: "Persistent",
        description: "Use undo 100 times",
        icon: "↶",
        condition: (stats) => stats.totalUndos >= 100
    },
    {
        id: "marathoner",
        name: "Marathoner",
        description: "Play for 1 hour total",
        icon: "🏃",
        condition: (stats) => stats.totalTime >= 3600
    }
];

// ========================================
// GAME STATE
// ========================================

class GameState {
    constructor() {
        this.currentScreen = 'mainMenu';
        this.currentPack = 'beginner';
        this.currentLevel = 0;
        this.theme = 'classic';

        // Game state
        this.grid = [];
        this.playerPos = { x: 0, y: 0 };
        this.boxes = [];
        this.targets = [];
        this.moves = 0;
        this.startTime = null;
        this.elapsedTime = 0;
        this.history = [];
        this.redoStack = [];
        this.isAnimating = false;
        this.usedUndoThisLevel = false;

        // Settings
        this.soundEnabled = true;
        this.volume = 0.7;
        this.hintsEnabled = true;
        this.animationsEnabled = true;
        this.mobileControlsVisible = false;

        // Statistics
        this.stats = {
            levelsCompleted: 0,
            totalMoves: 0,
            totalTime: 0,
            totalUndos: 0,
            totalStars: 0,
            perfectClears: 0,
            optimalSolutions: 0,
            fastestLevel: Infinity,
            noUndoClears: 0
        };

        // Progress tracking
        this.progress = {};

        // Load saved data
        this.loadGame();

        // Timer interval
        this.timerInterval = null;
    }

    loadGame() {
        try {
            const saved = localStorage.getItem('sokoban_save');
            if (saved) {
                const data = JSON.parse(saved);
                this.stats = { ...this.stats, ...data.stats };
                this.progress = data.progress || {};
                this.theme = data.theme || 'classic';
                this.soundEnabled = data.soundEnabled !== false;
                this.volume = data.volume || 0.7;
                this.hintsEnabled = data.hintsEnabled !== false;
                this.animationsEnabled = data.animationsEnabled !== false;
                this.mobileControlsVisible = data.mobileControlsVisible || false;
            }
        } catch (e) {
            console.error('Failed to load game:', e);
        }
    }

    saveGame() {
        try {
            const data = {
                stats: this.stats,
                progress: this.progress,
                theme: this.theme,
                soundEnabled: this.soundEnabled,
                volume: this.volume,
                hintsEnabled: this.hintsEnabled,
                animationsEnabled: this.animationsEnabled,
                mobileControlsVisible: this.mobileControlsVisible
            };
            localStorage.setItem('sokoban_save', JSON.stringify(data));
        } catch (e) {
            console.error('Failed to save game:', e);
        }
    }

    getLevelProgress(packName, levelIndex) {
        const key = `${packName}_${levelIndex}`;
        return this.progress[key] || {
            completed: false,
            stars: 0,
            bestMoves: Infinity,
            bestTime: Infinity
        };
    }

    setLevelProgress(packName, levelIndex, data) {
        const key = `${packName}_${levelIndex}`;
        this.progress[key] = { ...this.getLevelProgress(packName, levelIndex), ...data };
        this.saveGame();
    }
}

// ========================================
// SOUND SYSTEM
// ========================================

class SoundSystem {
    constructor() {
        this.sounds = {};
        this.audioContext = null;
        this.initSounds();
    }

    initSounds() {
        // Simple procedural sound generation
        if (typeof AudioContext !== 'undefined' || typeof webkitAudioContext !== 'undefined') {
            this.audioContext = new (AudioContext || webkitAudioContext)();
        }
    }

    play(type) {
        if (!game.soundEnabled || !this.audioContext) return;

        const ctx = this.audioContext;
        const now = ctx.currentTime;
        const oscillator = ctx.createOscillator();
        const gain = ctx.createGain();

        oscillator.connect(gain);
        gain.connect(ctx.destination);

        gain.gain.value = game.volume * 0.1;

        switch (type) {
            case 'move':
                oscillator.frequency.value = 200;
                oscillator.type = 'sine';
                gain.gain.setValueAtTime(game.volume * 0.05, now);
                gain.gain.exponentialRampToValueAtTime(0.01, now + 0.1);
                oscillator.start(now);
                oscillator.stop(now + 0.1);
                break;

            case 'push':
                oscillator.frequency.value = 150;
                oscillator.type = 'square';
                gain.gain.setValueAtTime(game.volume * 0.08, now);
                gain.gain.exponentialRampToValueAtTime(0.01, now + 0.15);
                oscillator.start(now);
                oscillator.stop(now + 0.15);
                break;

            case 'target':
                oscillator.frequency.value = 400;
                oscillator.type = 'sine';
                gain.gain.setValueAtTime(game.volume * 0.1, now);
                gain.gain.exponentialRampToValueAtTime(0.01, now + 0.2);
                oscillator.start(now);
                oscillator.stop(now + 0.2);
                break;

            case 'complete':
                // Victory fanfare
                [440, 554, 659, 880].forEach((freq, i) => {
                    const osc = ctx.createOscillator();
                    const g = ctx.createGain();
                    osc.connect(g);
                    g.connect(ctx.destination);
                    osc.frequency.value = freq;
                    osc.type = 'sine';
                    g.gain.setValueAtTime(game.volume * 0.1, now + i * 0.1);
                    g.gain.exponentialRampToValueAtTime(0.01, now + i * 0.1 + 0.3);
                    osc.start(now + i * 0.1);
                    osc.stop(now + i * 0.1 + 0.3);
                });
                break;

            case 'error':
                oscillator.frequency.value = 100;
                oscillator.type = 'sawtooth';
                gain.gain.setValueAtTime(game.volume * 0.05, now);
                gain.gain.exponentialRampToValueAtTime(0.01, now + 0.1);
                oscillator.start(now);
                oscillator.stop(now + 0.1);
                break;
        }
    }
}

// ========================================
// RENDERER
// ========================================

class Renderer {
    constructor(canvas) {
        this.canvas = canvas;
        this.ctx = canvas.getContext('2d');
        this.animationOffset = { x: 0, y: 0 };
    }

    drawLevel(grid, playerPos, boxes, targets, theme) {
        const width = grid[0].length;
        const height = grid.length;

        this.canvas.width = width * TILE_SIZE;
        this.canvas.height = height * TILE_SIZE;

        // Clear canvas
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

        // Draw grid
        for (let y = 0; y < height; y++) {
            for (let x = 0; x < width; x++) {
                const tile = grid[y][x];
                this.drawTile(x, y, tile, theme);
            }
        }

        // Draw targets
        targets.forEach(target => {
            this.drawTarget(target.x, target.y, theme);
        });

        // Draw boxes
        boxes.forEach(box => {
            const onTarget = targets.some(t => t.x === box.x && t.y === box.y);
            this.drawBox(box.x, box.y, onTarget, theme);
        });

        // Draw player
        this.drawPlayer(playerPos.x + this.animationOffset.x, playerPos.y + this.animationOffset.y, theme);
    }

    drawTile(x, y, type, theme) {
        const px = x * TILE_SIZE;
        const py = y * TILE_SIZE;

        this.ctx.save();

        switch (theme) {
            case 'classic':
                this.drawClassicTile(px, py, type);
                break;
            case 'modern':
                this.drawModernTile(px, py, type);
                break;
            case 'pixel':
                this.drawPixelTile(px, py, type);
                break;
        }

        this.ctx.restore();
    }

    drawClassicTile(px, py, type) {
        switch (type) {
            case TILE_TYPES.WALL:
                this.ctx.fillStyle = '#64748b';
                this.ctx.fillRect(px, py, TILE_SIZE, TILE_SIZE);
                this.ctx.strokeStyle = '#475569';
                this.ctx.lineWidth = 2;
                this.ctx.strokeRect(px, py, TILE_SIZE, TILE_SIZE);
                break;
            case TILE_TYPES.FLOOR:
                this.ctx.fillStyle = '#cbd5e1';
                this.ctx.fillRect(px, py, TILE_SIZE, TILE_SIZE);
                this.ctx.fillStyle = '#e2e8f0';
                this.ctx.fillRect(px + 2, py + 2, TILE_SIZE - 4, TILE_SIZE - 4);
                break;
            case TILE_TYPES.EMPTY:
                this.ctx.fillStyle = '#1e293b';
                this.ctx.fillRect(px, py, TILE_SIZE, TILE_SIZE);
                break;
        }
    }

    drawModernTile(px, py, type) {
        switch (type) {
            case TILE_TYPES.WALL:
                const gradient = this.ctx.createLinearGradient(px, py, px, py + TILE_SIZE);
                gradient.addColorStop(0, '#6366f1');
                gradient.addColorStop(1, '#4338ca');
                this.ctx.fillStyle = gradient;
                this.ctx.fillRect(px, py, TILE_SIZE, TILE_SIZE);
                // Highlight
                this.ctx.fillStyle = 'rgba(255, 255, 255, 0.2)';
                this.ctx.fillRect(px, py, TILE_SIZE, TILE_SIZE / 3);
                break;
            case TILE_TYPES.FLOOR:
                this.ctx.fillStyle = '#e0e7ff';
                this.ctx.fillRect(px, py, TILE_SIZE, TILE_SIZE);
                // Grid pattern
                this.ctx.strokeStyle = '#c7d2fe';
                this.ctx.lineWidth = 1;
                this.ctx.strokeRect(px, py, TILE_SIZE, TILE_SIZE);
                break;
            case TILE_TYPES.EMPTY:
                this.ctx.fillStyle = '#0f172a';
                this.ctx.fillRect(px, py, TILE_SIZE, TILE_SIZE);
                break;
        }
    }

    drawPixelTile(px, py, type) {
        const pixelSize = 4;

        switch (type) {
            case TILE_TYPES.WALL:
                this.ctx.fillStyle = '#64748b';
                this.ctx.fillRect(px, py, TILE_SIZE, TILE_SIZE);
                // Pixel pattern
                for (let i = 0; i < TILE_SIZE / pixelSize; i++) {
                    for (let j = 0; j < TILE_SIZE / pixelSize; j++) {
                        if ((i + j) % 2 === 0) {
                            this.ctx.fillStyle = '#475569';
                            this.ctx.fillRect(px + i * pixelSize, py + j * pixelSize, pixelSize, pixelSize);
                        }
                    }
                }
                break;
            case TILE_TYPES.FLOOR:
                this.ctx.fillStyle = '#cbd5e1';
                this.ctx.fillRect(px, py, TILE_SIZE, TILE_SIZE);
                break;
            case TILE_TYPES.EMPTY:
                this.ctx.fillStyle = '#1e293b';
                this.ctx.fillRect(px, py, TILE_SIZE, TILE_SIZE);
                break;
        }
    }

    drawTarget(x, y, theme) {
        const px = x * TILE_SIZE + TILE_SIZE / 2;
        const py = y * TILE_SIZE + TILE_SIZE / 2;
        const radius = TILE_SIZE / 3;

        this.ctx.save();

        // Draw target circle
        this.ctx.strokeStyle = '#10b981';
        this.ctx.lineWidth = 3;
        this.ctx.beginPath();
        this.ctx.arc(px, py, radius, 0, Math.PI * 2);
        this.ctx.stroke();

        this.ctx.beginPath();
        this.ctx.arc(px, py, radius / 2, 0, Math.PI * 2);
        this.ctx.stroke();

        this.ctx.restore();
    }

    drawBox(x, y, onTarget, theme) {
        const px = x * TILE_SIZE;
        const py = y * TILE_SIZE;
        const size = TILE_SIZE * 0.8;
        const offset = (TILE_SIZE - size) / 2;

        this.ctx.save();

        if (onTarget) {
            // Box on target - glowing effect
            this.ctx.shadowColor = '#10b981';
            this.ctx.shadowBlur = 10;
        }

        switch (theme) {
            case 'classic':
                this.ctx.fillStyle = onTarget ? '#059669' : '#d97706';
                this.ctx.fillRect(px + offset, py + offset, size, size);
                this.ctx.strokeStyle = onTarget ? '#047857' : '#b45309';
                this.ctx.lineWidth = 2;
                this.ctx.strokeRect(px + offset, py + offset, size, size);
                // Diagonal lines
                this.ctx.beginPath();
                this.ctx.moveTo(px + offset, py + offset);
                this.ctx.lineTo(px + offset + size, py + offset + size);
                this.ctx.moveTo(px + offset + size, py + offset);
                this.ctx.lineTo(px + offset, py + offset + size);
                this.ctx.stroke();
                break;

            case 'modern':
                const gradient = this.ctx.createLinearGradient(px + offset, py + offset, px + offset + size, py + offset + size);
                gradient.addColorStop(0, onTarget ? '#10b981' : '#f59e0b');
                gradient.addColorStop(1, onTarget ? '#059669' : '#d97706');
                this.ctx.fillStyle = gradient;
                this.ctx.fillRect(px + offset, py + offset, size, size);
                this.ctx.fillStyle = 'rgba(255, 255, 255, 0.3)';
                this.ctx.fillRect(px + offset, py + offset, size / 2, size / 2);
                break;

            case 'pixel':
                this.ctx.fillStyle = onTarget ? '#10b981' : '#d97706';
                this.ctx.fillRect(px + offset, py + offset, size, size);
                this.ctx.fillStyle = onTarget ? '#059669' : '#b45309';
                const pixelSize = 6;
                for (let i = 0; i < size / pixelSize; i++) {
                    for (let j = 0; j < size / pixelSize; j++) {
                        if ((i + j) % 2 === 0) {
                            this.ctx.fillRect(px + offset + i * pixelSize, py + offset + j * pixelSize, pixelSize, pixelSize);
                        }
                    }
                }
                break;
        }

        this.ctx.restore();
    }

    drawPlayer(x, y, theme) {
        const px = x * TILE_SIZE + TILE_SIZE / 2;
        const py = y * TILE_SIZE + TILE_SIZE / 2;
        const radius = TILE_SIZE / 3;

        this.ctx.save();

        switch (theme) {
            case 'classic':
                // Simple circle
                this.ctx.fillStyle = '#3b82f6';
                this.ctx.beginPath();
                this.ctx.arc(px, py, radius, 0, Math.PI * 2);
                this.ctx.fill();
                this.ctx.strokeStyle = '#1e40af';
                this.ctx.lineWidth = 3;
                this.ctx.stroke();
                // Eyes
                this.ctx.fillStyle = '#1e40af';
                this.ctx.beginPath();
                this.ctx.arc(px - radius / 3, py - radius / 3, radius / 6, 0, Math.PI * 2);
                this.ctx.arc(px + radius / 3, py - radius / 3, radius / 6, 0, Math.PI * 2);
                this.ctx.fill();
                break;

            case 'modern':
                // Gradient circle
                const gradient = this.ctx.createRadialGradient(px, py, 0, px, py, radius);
                gradient.addColorStop(0, '#60a5fa');
                gradient.addColorStop(1, '#3b82f6');
                this.ctx.fillStyle = gradient;
                this.ctx.beginPath();
                this.ctx.arc(px, py, radius, 0, Math.PI * 2);
                this.ctx.fill();
                // Highlight
                this.ctx.fillStyle = 'rgba(255, 255, 255, 0.5)';
                this.ctx.beginPath();
                this.ctx.arc(px - radius / 3, py - radius / 3, radius / 3, 0, Math.PI * 2);
                this.ctx.fill();
                break;

            case 'pixel':
                // Pixelated circle
                this.ctx.fillStyle = '#3b82f6';
                const pixelSize = 4;
                for (let i = -radius; i < radius; i += pixelSize) {
                    for (let j = -radius; j < radius; j += pixelSize) {
                        if (i * i + j * j < radius * radius) {
                            this.ctx.fillRect(px + i, py + j, pixelSize, pixelSize);
                        }
                    }
                }
                break;
        }

        this.ctx.restore();
    }
}

// ========================================
// GAME CLASS
// ========================================

class Game {
    constructor() {
        this.state = new GameState();
        this.sounds = new SoundSystem();
        this.renderer = null;
        this.canvas = null;

        this.init();
    }

    init() {
        // Setup canvas
        this.canvas = document.getElementById('gameCanvas');
        this.renderer = new Renderer(this.canvas);

        // Setup event listeners
        this.setupEventListeners();

        // Initialize UI
        this.updateMainMenuStats();
        this.applyTheme(this.state.theme);
        this.updateSettings();

        // Show main menu
        this.showMainMenu();
    }

    setupEventListeners() {
        // Keyboard controls
        document.addEventListener('keydown', (e) => {
            if (this.state.currentScreen !== 'gameScreen') return;

            const key = e.key.toLowerCase();

            if (key === 'arrowup' || key === 'w') {
                e.preventDefault();
                this.movePlayer(DIRECTIONS.UP);
            } else if (key === 'arrowdown' || key === 's') {
                e.preventDefault();
                this.movePlayer(DIRECTIONS.DOWN);
            } else if (key === 'arrowleft' || key === 'a') {
                e.preventDefault();
                this.movePlayer(DIRECTIONS.LEFT);
            } else if (key === 'arrowright' || key === 'd') {
                e.preventDefault();
                this.movePlayer(DIRECTIONS.RIGHT);
            } else if (key === 'u') {
                e.preventDefault();
                this.undo();
            } else if (key === 'r' && e.shiftKey) {
                e.preventDefault();
                this.redo();
            } else if (key === 'r' && !e.shiftKey) {
                e.preventDefault();
                this.resetLevel();
            } else if (key === 'h') {
                e.preventDefault();
                this.showHint();
            }
        });

        // Mobile controls
        document.querySelectorAll('.dpad-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                const dir = e.target.dataset.dir;
                if (dir) {
                    this.movePlayer(DIRECTIONS[dir.toUpperCase()]);
                }
            });
        });

        // Touch controls for swipe
        let touchStartX = 0;
        let touchStartY = 0;

        this.canvas.addEventListener('touchstart', (e) => {
            touchStartX = e.touches[0].clientX;
            touchStartY = e.touches[0].clientY;
        });

        this.canvas.addEventListener('touchend', (e) => {
            const touchEndX = e.changedTouches[0].clientX;
            const touchEndY = e.changedTouches[0].clientY;

            const dx = touchEndX - touchStartX;
            const dy = touchEndY - touchStartY;

            const minSwipe = 30;

            if (Math.abs(dx) > Math.abs(dy) && Math.abs(dx) > minSwipe) {
                this.movePlayer(dx > 0 ? DIRECTIONS.RIGHT : DIRECTIONS.LEFT);
            } else if (Math.abs(dy) > minSwipe) {
                this.movePlayer(dy > 0 ? DIRECTIONS.DOWN : DIRECTIONS.UP);
            }
        });
    }

    // ========================================
    // SCREEN MANAGEMENT
    // ========================================

    showScreen(screenName) {
        document.querySelectorAll('.screen').forEach(screen => {
            screen.classList.remove('active');
        });
        document.getElementById(screenName).classList.add('active');
        this.state.currentScreen = screenName;
    }

    showMainMenu() {
        this.showScreen('mainMenu');
        this.updateMainMenuStats();
        this.stopTimer();
    }

    showLevelSelect() {
        this.showScreen('levelSelect');
        this.renderLevelPacks();
        this.renderLevelGrid(this.state.currentPack);
    }

    showLevelEditor() {
        this.showScreen('levelEditor');
        if (window.editor) {
            window.editor.init();
        }
    }

    showStatistics() {
        this.showScreen('statistics');
        this.updateStatisticsScreen();
    }

    showAchievements() {
        this.showScreen('achievements');
        this.updateAchievementsScreen();
    }

    showSettings() {
        this.showScreen('settings');
        this.updateSettings();
    }

    showHelp() {
        this.showScreen('help');
    }

    // ========================================
    // LEVEL MANAGEMENT
    // ========================================

    renderLevelPacks() {
        const container = document.getElementById('levelPacks');
        container.innerHTML = '';

        Object.keys(LEVEL_PACKS).forEach(packKey => {
            const pack = LEVEL_PACKS[packKey];
            const btn = document.createElement('button');
            btn.className = 'pack-btn';
            if (packKey === this.state.currentPack) {
                btn.classList.add('active');
            }
            btn.innerHTML = `${pack.icon} ${pack.name}`;
            btn.onclick = () => {
                this.state.currentPack = packKey;
                this.showLevelSelect();
            };
            container.appendChild(btn);
        });
    }

    renderLevelGrid(packKey) {
        const container = document.getElementById('levelGrid');
        container.innerHTML = '';

        const pack = LEVEL_PACKS[packKey];

        pack.levels.forEach((level, index) => {
            const progress = this.state.getLevelProgress(packKey, index);
            const isLocked = index > 0 && !this.state.getLevelProgress(packKey, index - 1).completed;

            const card = document.createElement('div');
            card.className = 'level-card';
            if (progress.completed) card.classList.add('completed');
            if (isLocked) card.classList.add('locked');

            const stars = [];
            for (let i = 0; i < 3; i++) {
                const star = `<span class="star ${i < progress.stars ? 'earned' : ''}">⭐</span>`;
                stars.push(star);
            }

            card.innerHTML = `
                <div class="level-number">${index + 1}</div>
                <div class="level-stars">${stars.join('')}</div>
                <div class="level-difficulty-badge difficulty-${level.difficulty}">${level.difficulty}</div>
            `;

            if (!isLocked) {
                card.onclick = () => this.startLevel(packKey, index);
            }

            container.appendChild(card);
        });
    }

    startLevel(packKey, levelIndex) {
        this.state.currentPack = packKey;
        this.state.currentLevel = levelIndex;

        const pack = LEVEL_PACKS[packKey];
        const level = pack.levels[levelIndex];

        this.loadLevel(level);
        this.showScreen('gameScreen');
        this.startTimer();

        // Update UI
        document.getElementById('levelTitle').textContent = `${level.name}`;
        document.getElementById('levelDifficulty').textContent = level.difficulty.toUpperCase();
        document.getElementById('optimalMoves').textContent = level.optimal;

        this.updateGameUI();
    }

    loadLevel(level) {
        // Parse level grid
        this.state.grid = [];
        this.state.boxes = [];
        this.state.targets = [];
        this.state.playerPos = { x: 0, y: 0 };
        this.state.moves = 0;
        this.state.elapsedTime = 0;
        this.state.history = [];
        this.state.redoStack = [];
        this.state.usedUndoThisLevel = false;

        level.grid.forEach((row, y) => {
            const gridRow = [];
            for (let x = 0; x < row.length; x++) {
                const char = row[x];

                switch (char) {
                    case '#':
                        gridRow.push(TILE_TYPES.WALL);
                        break;
                    case ' ':
                        gridRow.push(TILE_TYPES.FLOOR);
                        break;
                    case '$':
                        gridRow.push(TILE_TYPES.FLOOR);
                        this.state.boxes.push({ x, y });
                        break;
                    case '.':
                        gridRow.push(TILE_TYPES.FLOOR);
                        this.state.targets.push({ x, y });
                        break;
                    case '@':
                        gridRow.push(TILE_TYPES.FLOOR);
                        this.state.playerPos = { x, y };
                        break;
                    case '*':
                        gridRow.push(TILE_TYPES.FLOOR);
                        this.state.boxes.push({ x, y });
                        this.state.targets.push({ x, y });
                        break;
                    default:
                        gridRow.push(TILE_TYPES.EMPTY);
                }
            }
            this.state.grid.push(gridRow);
        });

        document.getElementById('totalBoxes').textContent = this.state.boxes.length;

        this.render();
    }

    // ========================================
    // GAME LOGIC
    // ========================================

    movePlayer(direction) {
        if (this.state.isAnimating) return;

        const newX = this.state.playerPos.x + direction.x;
        const newY = this.state.playerPos.y + direction.y;

        // Check bounds
        if (!this.isValidPosition(newX, newY)) {
            this.sounds.play('error');
            return;
        }

        // Check for box
        const boxIndex = this.state.boxes.findIndex(b => b.x === newX && b.y === newY);

        if (boxIndex !== -1) {
            // Try to push box
            const boxNewX = newX + direction.x;
            const boxNewY = newY + direction.y;

            if (!this.isValidPosition(boxNewX, boxNewY) ||
                this.state.boxes.some(b => b.x === boxNewX && b.y === boxNewY)) {
                this.sounds.play('error');
                return;
            }

            // Save state for undo
            this.saveState();

            // Move box
            const oldBoxPos = { ...this.state.boxes[boxIndex] };
            this.state.boxes[boxIndex] = { x: boxNewX, y: boxNewY };

            // Move player
            this.state.playerPos = { x: newX, y: newY };
            this.state.moves++;

            // Check if box is on target
            const onTarget = this.state.targets.some(t => t.x === boxNewX && t.y === boxNewY);
            this.sounds.play(onTarget ? 'target' : 'push');

            // Animate
            if (this.state.animationsEnabled) {
                this.animateMove(direction);
            }

        } else {
            // Save state for undo
            this.saveState();

            // Move player
            this.state.playerPos = { x: newX, y: newY };
            this.state.moves++;
            this.sounds.play('move');

            // Animate
            if (this.state.animationsEnabled) {
                this.animateMove(direction);
            }
        }

        this.updateGameUI();
        this.render();

        // Check win condition
        if (this.checkWin()) {
            this.handleLevelComplete();
        }
    }

    isValidPosition(x, y) {
        if (y < 0 || y >= this.state.grid.length || x < 0 || x >= this.state.grid[0].length) {
            return false;
        }
        const tile = this.state.grid[y][x];
        return tile !== TILE_TYPES.WALL && tile !== TILE_TYPES.EMPTY;
    }

    checkWin() {
        return this.state.boxes.every(box =>
            this.state.targets.some(target => target.x === box.x && target.y === box.y)
        );
    }

    animateMove(direction) {
        if (!this.state.animationsEnabled) return;

        this.state.isAnimating = true;
        const steps = 10;
        const stepSize = 1 / steps;
        let currentStep = 0;

        const animate = () => {
            currentStep++;
            const progress = currentStep / steps;

            this.renderer.animationOffset = {
                x: -direction.x * (1 - progress),
                y: -direction.y * (1 - progress)
            };

            this.render();

            if (currentStep < steps) {
                requestAnimationFrame(animate);
            } else {
                this.renderer.animationOffset = { x: 0, y: 0 };
                this.state.isAnimating = false;
                this.render();
            }
        };

        requestAnimationFrame(animate);
    }

    // ========================================
    // UNDO/REDO SYSTEM
    // ========================================

    saveState() {
        this.state.history.push({
            playerPos: { ...this.state.playerPos },
            boxes: this.state.boxes.map(b => ({ ...b })),
            moves: this.state.moves
        });
        this.state.redoStack = [];
    }

    undo() {
        if (this.state.history.length === 0) return;

        const previousState = this.state.history.pop();

        // Save current state to redo stack
        this.state.redoStack.push({
            playerPos: { ...this.state.playerPos },
            boxes: this.state.boxes.map(b => ({ ...b })),
            moves: this.state.moves
        });

        // Restore previous state
        this.state.playerPos = previousState.playerPos;
        this.state.boxes = previousState.boxes;
        this.state.moves = previousState.moves;

        this.state.stats.totalUndos++;
        this.state.usedUndoThisLevel = true;
        this.state.saveGame();

        this.updateGameUI();
        this.render();
        this.sounds.play('move');
    }

    redo() {
        if (this.state.redoStack.length === 0) return;

        const nextState = this.state.redoStack.pop();

        // Save current state to history
        this.state.history.push({
            playerPos: { ...this.state.playerPos },
            boxes: this.state.boxes.map(b => ({ ...b })),
            moves: this.state.moves
        });

        // Restore next state
        this.state.playerPos = nextState.playerPos;
        this.state.boxes = nextState.boxes;
        this.state.moves = nextState.moves;

        this.updateGameUI();
        this.render();
        this.sounds.play('move');
    }

    resetLevel() {
        const pack = LEVEL_PACKS[this.state.currentPack];
        const level = pack.levels[this.state.currentLevel];
        this.loadLevel(level);
        this.state.startTime = Date.now();
        this.updateGameUI();
    }

    // ========================================
    // HINT SYSTEM
    // ========================================

    showHint() {
        if (!this.state.hintsEnabled) {
            alert('Hints are disabled in settings.');
            return;
        }

        // Simple hint: find the nearest box not on target
        const unsolvedBoxes = this.state.boxes.filter(box =>
            !this.state.targets.some(t => t.x === box.x && t.y === box.y)
        );

        if (unsolvedBoxes.length === 0) return;

        const nearestBox = unsolvedBoxes.reduce((nearest, box) => {
            const dist = Math.abs(box.x - this.state.playerPos.x) + Math.abs(box.y - this.state.playerPos.y);
            const nearestDist = Math.abs(nearest.x - this.state.playerPos.x) + Math.abs(nearest.y - this.state.playerPos.y);
            return dist < nearestDist ? box : nearest;
        });

        alert(`Hint: Try moving the box at position (${nearestBox.x}, ${nearestBox.y})`);
    }

    // ========================================
    // LEVEL COMPLETION
    // ========================================

    handleLevelComplete() {
        this.stopTimer();
        this.sounds.play('complete');

        const pack = LEVEL_PACKS[this.state.currentPack];
        const level = pack.levels[this.state.currentLevel];
        const progress = this.state.getLevelProgress(this.state.currentPack, this.state.currentLevel);

        // Calculate stars
        const moveRatio = this.state.moves / level.optimal;
        let stars = 1;
        if (moveRatio <= 1.2) stars = 3;
        else if (moveRatio <= 1.5) stars = 2;

        // Update statistics
        if (!progress.completed) {
            this.state.stats.levelsCompleted++;
        }
        this.state.stats.totalMoves += this.state.moves;
        this.state.stats.totalTime += this.state.elapsedTime;

        if (stars === 3) {
            this.state.stats.perfectClears++;
        }
        if (this.state.moves === level.optimal) {
            this.state.stats.optimalSolutions++;
        }
        if (this.state.elapsedTime < this.state.stats.fastestLevel) {
            this.state.stats.fastestLevel = this.state.elapsedTime;
        }
        if (!this.state.usedUndoThisLevel) {
            this.state.stats.noUndoClears++;
        }

        // Update level progress
        const newBest = this.state.moves < progress.bestMoves;
        this.state.setLevelProgress(this.state.currentPack, this.state.currentLevel, {
            completed: true,
            stars: Math.max(stars, progress.stars),
            bestMoves: Math.min(this.state.moves, progress.bestMoves),
            bestTime: Math.min(this.state.elapsedTime, progress.bestTime)
        });

        // Count total stars
        this.state.stats.totalStars = Object.values(this.state.progress).reduce((sum, p) => sum + (p.stars || 0), 0);

        this.state.saveGame();

        // Check achievements
        const unlockedAchievement = this.checkAchievements();

        // Show completion modal
        this.showCompletionModal(stars, newBest, unlockedAchievement);
    }

    showCompletionModal(stars, newBest, achievement) {
        const modal = document.getElementById('completionModal');
        const starsContainer = document.getElementById('completionStars');
        const movesEl = document.getElementById('completionMoves');
        const timeEl = document.getElementById('completionTime');
        const optimalEl = document.getElementById('completionOptimal');
        const achievementEl = document.getElementById('newAchievement');
        const achievementName = document.getElementById('achievementName');

        // Render stars
        const starHTML = [];
        for (let i = 0; i < 3; i++) {
            if (i < stars) {
                starHTML.push('<span class="star" style="font-size: 3rem;">⭐</span>');
            } else {
                starHTML.push('<span class="star" style="font-size: 3rem; opacity: 0.3;">⭐</span>');
            }
        }
        starsContainer.innerHTML = starHTML.join('');

        // Update stats
        movesEl.textContent = this.state.moves;
        timeEl.textContent = this.formatTime(this.state.elapsedTime);
        const pack = LEVEL_PACKS[this.state.currentPack];
        const level = pack.levels[this.state.currentLevel];
        optimalEl.textContent = level.optimal;

        // Show achievement if unlocked
        if (achievement) {
            achievementEl.style.display = 'block';
            achievementName.textContent = achievement.name;
        } else {
            achievementEl.style.display = 'none';
        }

        // Show confetti
        this.showConfetti();

        modal.classList.add('active');
    }

    showConfetti() {
        const confetti = document.getElementById('confetti');
        confetti.innerHTML = '';

        const colors = ['#3b82f6', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6'];

        for (let i = 0; i < 50; i++) {
            const piece = document.createElement('div');
            piece.style.position = 'absolute';
            piece.style.width = '10px';
            piece.style.height = '10px';
            piece.style.backgroundColor = colors[Math.floor(Math.random() * colors.length)];
            piece.style.left = Math.random() * 100 + '%';
            piece.style.top = '-10px';
            piece.style.opacity = '0';
            piece.style.animation = `confettiFall ${1 + Math.random()}s linear forwards`;
            piece.style.animationDelay = Math.random() * 0.5 + 's';
            confetti.appendChild(piece);
        }

        // Add animation
        const style = document.createElement('style');
        style.textContent = `
            @keyframes confettiFall {
                to {
                    top: 100%;
                    opacity: 1;
                    transform: translateX(${Math.random() * 200 - 100}px) rotate(${Math.random() * 360}deg);
                }
            }
        `;
        document.head.appendChild(style);
    }

    nextLevel() {
        document.getElementById('completionModal').classList.remove('active');

        const pack = LEVEL_PACKS[this.state.currentPack];
        const nextLevelIndex = this.state.currentLevel + 1;

        if (nextLevelIndex < pack.levels.length) {
            this.startLevel(this.state.currentPack, nextLevelIndex);
        } else {
            // Find next pack
            const packKeys = Object.keys(LEVEL_PACKS);
            const currentPackIndex = packKeys.indexOf(this.state.currentPack);
            if (currentPackIndex < packKeys.length - 1) {
                this.state.currentPack = packKeys[currentPackIndex + 1];
                this.startLevel(this.state.currentPack, 0);
            } else {
                // All levels complete!
                alert('Congratulations! You have completed all levels!');
                this.showLevelSelect();
            }
        }
    }

    retryLevel() {
        document.getElementById('completionModal').classList.remove('active');
        this.resetLevel();
    }

    // ========================================
    // ACHIEVEMENTS
    // ========================================

    checkAchievements() {
        for (const achievement of ACHIEVEMENTS) {
            if (achievement.condition(this.state.stats)) {
                const key = `achievement_${achievement.id}`;
                if (!localStorage.getItem(key)) {
                    localStorage.setItem(key, 'true');
                    return achievement;
                }
            }
        }
        return null;
    }

    updateAchievementsScreen() {
        const grid = document.getElementById('achievementsGrid');
        grid.innerHTML = '';

        ACHIEVEMENTS.forEach(achievement => {
            const key = `achievement_${achievement.id}`;
            const unlocked = localStorage.getItem(key) === 'true';

            const card = document.createElement('div');
            card.className = 'achievement-card';
            card.classList.add(unlocked ? 'unlocked' : 'locked');

            card.innerHTML = `
                <div class="achievement-icon">${achievement.icon}</div>
                <div class="achievement-name">${achievement.name}</div>
                <div class="achievement-description">${achievement.description}</div>
                <div class="achievement-progress">${unlocked ? 'Unlocked!' : 'Locked'}</div>
            `;

            grid.appendChild(card);
        });
    }

    // ========================================
    // UI UPDATES
    // ========================================

    updateMainMenuStats() {
        document.getElementById('totalCompleted').textContent = this.state.stats.levelsCompleted;
        document.getElementById('totalMoves').textContent = this.state.stats.totalMoves;
        document.getElementById('totalTime').textContent = this.formatTime(this.state.stats.totalTime);
    }

    updateGameUI() {
        document.getElementById('moveCounter').textContent = this.state.moves;
        document.getElementById('timer').textContent = this.formatTime(this.state.elapsedTime);

        const boxesOnTarget = this.state.boxes.filter(box =>
            this.state.targets.some(t => t.x === box.x && t.y === box.y)
        ).length;
        document.getElementById('boxesOnTarget').textContent = boxesOnTarget;

        // Update stars
        const pack = LEVEL_PACKS[this.state.currentPack];
        const level = pack.levels[this.state.currentLevel];
        const moveRatio = this.state.moves / level.optimal;
        let activeStars = 0;
        if (moveRatio <= 1.2) activeStars = 3;
        else if (moveRatio <= 1.5) activeStars = 2;
        else activeStars = 1;

        document.querySelectorAll('#starDisplay .star').forEach((star, i) => {
            star.classList.toggle('active', i < activeStars);
        });
    }

    updateStatisticsScreen() {
        document.getElementById('statsCompleted').textContent = this.state.stats.levelsCompleted;
        document.getElementById('statsMoves').textContent = this.state.stats.totalMoves;
        document.getElementById('statsTime').textContent = this.formatTime(this.state.stats.totalTime);
        document.getElementById('statsStars').textContent = this.state.stats.totalStars;
        document.getElementById('statsUndos').textContent = this.state.stats.totalUndos;
        document.getElementById('statsPerfect').textContent = this.state.stats.perfectClears;

        // Best times
        const bestTimesContainer = document.getElementById('bestTimes');
        bestTimesContainer.innerHTML = '<h3>Personal Bests</h3>';

        const bests = [];
        Object.keys(this.state.progress).forEach(key => {
            const progress = this.state.progress[key];
            if (progress.completed) {
                const [pack, index] = key.split('_');
                const level = LEVEL_PACKS[pack].levels[parseInt(index)];
                bests.push({
                    name: level.name,
                    moves: progress.bestMoves,
                    time: progress.bestTime,
                    stars: progress.stars
                });
            }
        });

        bests.sort((a, b) => b.stars - a.stars || a.moves - b.moves)
            .slice(0, 10)
            .forEach(best => {
                const item = document.createElement('div');
                item.className = 'best-time-item';
                item.innerHTML = `
                    <span>${best.name}</span>
                    <span>${best.moves} moves - ${this.formatTime(best.time)} - ${'⭐'.repeat(best.stars)}</span>
                `;
                bestTimesContainer.appendChild(item);
            });
    }

    updateSettings() {
        document.getElementById('soundToggle').checked = this.state.soundEnabled;
        document.getElementById('volumeSlider').value = this.state.volume * 100;
        document.getElementById('hintsToggle').checked = this.state.hintsEnabled;
        document.getElementById('animationsToggle').checked = this.state.animationsEnabled;
        document.getElementById('mobileToggle').checked = this.state.mobileControlsVisible;

        // Update theme buttons
        document.querySelectorAll('.theme-btn').forEach(btn => {
            btn.classList.toggle('active', btn.dataset.theme === this.state.theme);
        });
    }

    // ========================================
    // TIMER
    // ========================================

    startTimer() {
        this.state.startTime = Date.now();
        this.state.elapsedTime = 0;

        this.state.timerInterval = setInterval(() => {
            this.state.elapsedTime = Math.floor((Date.now() - this.state.startTime) / 1000);
            this.updateGameUI();
        }, 1000);
    }

    stopTimer() {
        if (this.state.timerInterval) {
            clearInterval(this.state.timerInterval);
            this.state.timerInterval = null;
        }
    }

    formatTime(seconds) {
        const mins = Math.floor(seconds / 60);
        const secs = seconds % 60;
        return `${mins}:${secs.toString().padStart(2, '0')}`;
    }

    // ========================================
    // SETTINGS
    // ========================================

    setTheme(theme) {
        this.state.theme = theme;
        this.applyTheme(theme);
        this.state.saveGame();
        this.render();
        this.updateSettings();
    }

    applyTheme(theme) {
        // Theme is applied during rendering
        document.querySelectorAll('.theme-btn').forEach(btn => {
            btn.classList.toggle('active', btn.dataset.theme === theme);
        });
    }

    toggleSound() {
        this.state.soundEnabled = !this.state.soundEnabled;
        this.state.saveGame();
    }

    setVolume(value) {
        this.state.volume = value / 100;
        this.state.saveGame();
    }

    toggleHints() {
        this.state.hintsEnabled = !this.state.hintsEnabled;
        this.state.saveGame();
    }

    toggleAnimations() {
        this.state.animationsEnabled = !this.state.animationsEnabled;
        this.state.saveGame();
    }

    toggleMobileControls() {
        this.state.mobileControlsVisible = !this.state.mobileControlsVisible;
        document.getElementById('mobileControls').classList.toggle('visible', this.state.mobileControlsVisible);
        this.state.saveGame();
    }

    resetProgress() {
        if (confirm('Are you sure you want to reset all progress? This cannot be undone!')) {
            localStorage.removeItem('sokoban_save');
            ACHIEVEMENTS.forEach(a => localStorage.removeItem(`achievement_${a.id}`));
            location.reload();
        }
    }

    exportProgress() {
        const data = localStorage.getItem('sokoban_save');
        const blob = new Blob([data], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'sokoban_progress.json';
        a.click();
    }

    importProgress() {
        const input = document.createElement('input');
        input.type = 'file';
        input.accept = '.json';
        input.onchange = (e) => {
            const file = e.target.files[0];
            const reader = new FileReader();
            reader.onload = (event) => {
                try {
                    localStorage.setItem('sokoban_save', event.target.result);
                    location.reload();
                } catch (e) {
                    alert('Failed to import progress.');
                }
            };
            reader.readAsText(file);
        };
        input.click();
    }

    // ========================================
    // RENDERING
    // ========================================

    render() {
        if (!this.state.grid.length) return;
        this.renderer.drawLevel(
            this.state.grid,
            this.state.playerPos,
            this.state.boxes,
            this.state.targets,
            this.state.theme
        );
    }
}

// ========================================
// INITIALIZE GAME
// ========================================

let game;

window.addEventListener('DOMContentLoaded', () => {
    game = new Game();

    // Detect mobile
    if (/Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent)) {
        game.state.mobileControlsVisible = true;
        document.getElementById('mobileControls').classList.add('visible');
    }
});
