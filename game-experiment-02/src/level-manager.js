"use strict";
/**
 * Level management and puzzle definitions
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.LevelManager = void 0;
var maze_grid_1 = require("./maze-grid");
var LevelManager = /** @class */ (function () {
    function LevelManager() {
        this.levels = new Map();
        this.initializeLevels();
    }
    LevelManager.prototype.initializeLevels = function () {
        var _this = this;
        // Level 1: Simple path
        this.addLevel(1, 'Simple Path', 'Navigate straight to the goal.', function (maze) {
            maze.setStart({ x: 0, y: 0 });
            maze.setGoal({ x: 4, y: 0 });
        }, 5, 10, 'Easy');
        // Level 2: Turn required
        this.addLevel(2, 'The Corner', 'Turn and navigate to the goal.', function (maze) {
            maze.setStart({ x: 0, y: 0 });
            maze.setGoal({ x: 0, y: 4 });
            // Create an obstacle
            for (var i = 1; i < 4; i++) {
                maze.setWall({ x: 1, y: i });
            }
        }, 8, 15, 'Easy');
        // Level 3: Simple maze
        this.addLevel(3, 'First Maze', 'Navigate through a simple maze.', function (maze) {
            maze.setStart({ x: 0, y: 0 });
            maze.setGoal({ x: 6, y: 6 });
            // Create maze walls
            for (var i = 1; i < 6; i++) {
                maze.setWall({ x: 2, y: i });
            }
            for (var i = 0; i < 5; i++) {
                maze.setWall({ x: 4, y: i });
            }
        }, 15, 20, 'Easy');
        // Level 4: Key and door
        this.addLevel(4, 'Locked Gate', 'Find the key to unlock the door.', function (maze) {
            maze.setStart({ x: 0, y: 0 });
            maze.setGoal({ x: 8, y: 0 });
            maze.addKey({ x: 2, y: 0 }, 1);
            maze.addDoor({ x: 5, y: 0 }, 1);
        }, 10, 20, 'Easy');
        // Level 5: Double doors
        this.addLevel(5, 'Two Keys', 'Collect both keys to reach the goal.', function (maze) {
            maze.setStart({ x: 0, y: 0 });
            maze.setGoal({ x: 10, y: 0 });
            maze.addKey({ x: 2, y: 0 }, 1);
            maze.addDoor({ x: 4, y: 0 }, 1);
            maze.addKey({ x: 6, y: 0 }, 2);
            maze.addDoor({ x: 8, y: 0 }, 2);
        }, 15, 25, 'Easy');
        // Level 6: Teleporter
        this.addLevel(6, 'Warp Zone', 'Use the teleporter to shortcut to the goal.', function (maze) {
            maze.setStart({ x: 0, y: 0 });
            maze.setGoal({ x: 10, y: 10 });
            maze.addTeleporter({ x: 5, y: 5 }, { x: 8, y: 8 });
        }, 25, 30, 'Medium');
        // Level 7: Medium maze
        this.addLevel(7, 'Complex Path', 'Navigate a more complex maze.', function (maze) {
            maze.setStart({ x: 1, y: 1 });
            maze.setGoal({ x: 9, y: 9 });
            _this.createMediumMaze(maze);
        }, 30, 40, 'Medium');
        // Level 8: Multi-key puzzle
        this.addLevel(8, 'Three Doors', 'Unlock three doors in sequence.', function (maze) {
            maze.setStart({ x: 0, y: 5 });
            maze.setGoal({ x: 12, y: 5 });
            maze.addKey({ x: 2, y: 5 }, 1);
            maze.addDoor({ x: 4, y: 5 }, 1);
            maze.addKey({ x: 6, y: 5 }, 2);
            maze.addDoor({ x: 8, y: 5 }, 2);
            maze.addKey({ x: 10, y: 5 }, 3);
            maze.addDoor({ x: 11, y: 5 }, 3);
        }, 20, 30, 'Medium');
        // Level 9: Teleporter maze
        this.addLevel(9, 'Portal Puzzle', 'Use teleporters to navigate the maze.', function (maze) {
            maze.setStart({ x: 0, y: 0 });
            maze.setGoal({ x: 12, y: 12 });
            maze.addTeleporter({ x: 3, y: 3 }, { x: 6, y: 6 });
            maze.addTeleporter({ x: 9, y: 9 }, { x: 11, y: 11 });
            _this.createMediumMaze(maze);
        }, 40, 50, 'Medium');
        // Level 10: Hardened maze
        this.addLevel(10, 'The Labyrinth', 'Escape a complex labyrinth.', function (maze) {
            maze.setStart({ x: 1, y: 1 });
            maze.setGoal({ x: 14, y: 14 });
            _this.createHardMaze(maze);
        }, 50, 60, 'Hard');
        var _loop_1 = function (i) {
            var difficulty = i <= 13 ? 'Medium' : i <= 17 ? 'Hard' : 'Expert';
            var maxSteps = 30 + i * 5;
            var maxTime = 40 + i * 5;
            this_1.addLevel(i, "Challenge ".concat(i), "Advanced puzzle ".concat(i, ". Master your AI programming skills!"), function (maze) {
                _this.generatePuzzleMaze(maze, i);
            }, maxSteps, maxTime, difficulty);
        };
        var this_1 = this;
        // Levels 11-20 with increasing difficulty
        for (var i = 11; i <= 20; i++) {
            _loop_1(i);
        }
    };
    LevelManager.prototype.addLevel = function (id, name, description, mazeBuilder, maxSteps, maxTime, difficulty) {
        var size = 8 + Math.floor(id / 5) * 4; // Grow maze size with level
        var maze = new maze_grid_1.Maze(Math.min(size, 16), Math.min(size, 16));
        mazeBuilder(maze);
        var level = {
            id: id,
            name: name,
            description: description,
            maze: maze.getGrid(),
            maxSteps: maxSteps,
            maxTime: maxTime,
            difficulty: difficulty,
        };
        this.levels.set(id, level);
    };
    LevelManager.prototype.createMediumMaze = function (maze) {
        // Create a winding maze
        for (var i = 2; i < 8; i++) {
            maze.setWall({ x: 2, y: i });
        }
        for (var i = 2; i < 8; i++) {
            maze.setWall({ x: 4, y: Math.abs(i - 4) + 2 });
        }
        for (var i = 2; i < 8; i++) {
            maze.setWall({ x: 6, y: 10 - i });
        }
    };
    LevelManager.prototype.createHardMaze = function (maze) {
        // Create a complex maze with multiple paths
        for (var i = 1; i < 10; i++) {
            if (i % 2 === 0) {
                maze.setWall({ x: 2, y: i });
                maze.setWall({ x: 4, y: 10 - i });
                maze.setWall({ x: 6, y: i });
                maze.setWall({ x: 8, y: 10 - i });
                maze.setWall({ x: 10, y: i });
            }
        }
    };
    LevelManager.prototype.generatePuzzleMaze = function (maze, levelId) {
        var complexity = levelId - 10;
        maze.setStart({ x: 1, y: 1 });
        maze.setGoal({ x: Math.min(14, 6 + complexity), y: Math.min(14, 6 + complexity) });
        // Add walls based on complexity
        for (var i = 0; i < complexity * 2; i++) {
            var x = 2 + (i % 6);
            var y = 2 + Math.floor(i / 6);
            if (x < 14 && y < 14) {
                maze.setWall({ x: x, y: y });
            }
        }
        // Add some keys and doors
        if (complexity > 2) {
            maze.addKey({ x: 3, y: 3 }, 1);
            maze.addDoor({ x: 5, y: 3 }, 1);
        }
        if (complexity > 4) {
            maze.addKey({ x: 7, y: 7 }, 2);
            maze.addDoor({ x: 9, y: 7 }, 2);
        }
        if (complexity > 6) {
            maze.addTeleporter({ x: 4, y: 10 }, { x: 11, y: 11 });
        }
    };
    /**
     * Get level by ID
     */
    LevelManager.prototype.getLevel = function (id) {
        return this.levels.get(id);
    };
    /**
     * Get all levels
     */
    LevelManager.prototype.getAllLevels = function () {
        var levels = [];
        for (var i = 1; i <= 20; i++) {
            var level = this.levels.get(i);
            if (level) {
                levels.push(level);
            }
        }
        return levels;
    };
    /**
     * Get total number of levels
     */
    LevelManager.prototype.getTotalLevels = function () {
        return this.levels.size;
    };
    return LevelManager;
}());
exports.LevelManager = LevelManager;
