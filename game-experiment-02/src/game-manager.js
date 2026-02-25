"use strict";
/**
 * Main game state and flow management
 */
var __assign = (this && this.__assign) || function () {
    __assign = Object.assign || function(t) {
        for (var s, i = 1, n = arguments.length; i < n; i++) {
            s = arguments[i];
            for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p))
                t[p] = s[p];
        }
        return t;
    };
    return __assign.apply(this, arguments);
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.GameManager = void 0;
var level_manager_1 = require("./level-manager");
var maze_grid_1 = require("./maze-grid");
var ai_engine_1 = require("./ai-engine");
var GameManager = /** @class */ (function () {
    function GameManager() {
        this.currentLevelId = 1;
        this.currentLevel = null;
        this.currentMaze = null;
        this.currentAI = null;
        this.scores = new Map();
        this.startTime = 0;
        this.levelManager = new level_manager_1.LevelManager();
        this.loadLevel(1);
    }
    /**
     * Load a specific level
     */
    GameManager.prototype.loadLevel = function (levelId) {
        var level = this.levelManager.getLevel(levelId);
        if (!level) {
            return false;
        }
        this.currentLevelId = levelId;
        this.currentLevel = level;
        // Reconstruct maze from grid
        var grid = level.maze;
        this.currentMaze = new maze_grid_1.Maze(grid.width, grid.height);
        // Copy all cells
        for (var y = 0; y < grid.height; y++) {
            for (var x = 0; x < grid.width; x++) {
                var srcCell = grid.cells[y][x];
                var pos = { x: x, y: y };
                // Copy cell types
                if (srcCell.type !== 0) {
                    this.currentMaze.getCell(pos).type = srcCell.type;
                    if (srcCell.keyId !== undefined) {
                        this.currentMaze.getCell(pos).keyId = srcCell.keyId;
                    }
                    if (srcCell.teleportTarget) {
                        this.currentMaze.getCell(pos).teleportTarget = __assign({}, srcCell.teleportTarget);
                    }
                }
            }
        }
        // Create AI engine
        this.currentAI = new ai_engine_1.AIEngine(this.currentMaze, grid.startPos, level.maxSteps);
        this.startTime = Date.now();
        return true;
    };
    /**
     * Execute a single command on the current AI
     */
    GameManager.prototype.executeCommand = function (command) {
        if (!this.currentAI || !this.currentLevel) {
            return false;
        }
        var step = this.currentAI.executeCommand(command);
        return step.success;
    };
    /**
     * Execute multiple commands
     */
    GameManager.prototype.executeProgram = function (commands) {
        if (!this.currentAI || !this.currentLevel) {
            return;
        }
        for (var _i = 0, commands_1 = commands; _i < commands_1.length; _i++) {
            var command = commands_1[_i];
            if (this.currentAI.isFinished()) {
                break;
            }
            this.executeCommand(command);
        }
    };
    /**
     * Reset current level
     */
    GameManager.prototype.resetLevel = function () {
        if (this.currentAI) {
            this.currentAI.reset();
            this.startTime = Date.now();
        }
    };
    /**
     * Get current level
     */
    GameManager.prototype.getCurrentLevel = function () {
        return this.currentLevel;
    };
    /**
     * Get current AI state
     */
    GameManager.prototype.getCurrentAIState = function () {
        var _a;
        return (_a = this.currentAI) === null || _a === void 0 ? void 0 : _a.getState();
    };
    /**
     * Get current level ID
     */
    GameManager.prototype.getCurrentLevelId = function () {
        return this.currentLevelId;
    };
    /**
     * Get all levels
     */
    GameManager.prototype.getAllLevels = function () {
        return this.levelManager.getAllLevels();
    };
    /**
     * Check if current level is completed
     */
    GameManager.prototype.isLevelComplete = function () {
        var _a, _b;
        return (_b = (_a = this.currentAI) === null || _a === void 0 ? void 0 : _a.isGoalReached()) !== null && _b !== void 0 ? _b : false;
    };
    /**
     * Get current score
     */
    GameManager.prototype.getCurrentScore = function () {
        if (!this.currentAI || !this.currentLevel) {
            return null;
        }
        var timeTaken = (Date.now() - this.startTime) / 1000;
        var stepsTaken = this.currentAI.getStepCount();
        var reachedGoal = this.currentAI.isGoalReached();
        // Calculate efficiency score (0-100)
        var efficiency = 0;
        if (reachedGoal) {
            var maxSteps = this.currentLevel.maxSteps;
            var maxTime = this.currentLevel.maxTime;
            var stepRatio = Math.min(stepsTaken / maxSteps, 1.0);
            var timeRatio = Math.min(timeTaken / maxTime, 1.0);
            efficiency = Math.max(0, 100 - (stepRatio * 50 + timeRatio * 50));
        }
        return {
            levelId: this.currentLevel.id,
            stepsTaken: stepsTaken,
            timeTaken: timeTaken,
            reachedGoal: reachedGoal,
            efficiency: efficiency,
        };
    };
    /**
     * Save score for level
     */
    GameManager.prototype.saveScore = function (levelId, score) {
        // Only save if better than previous
        var existing = this.scores.get(levelId);
        if (!existing || score.efficiency > existing.efficiency) {
            this.scores.set(levelId, score);
        }
    };
    /**
     * Get score for level
     */
    GameManager.prototype.getScore = function (levelId) {
        return this.scores.get(levelId);
    };
    /**
     * Get all scores
     */
    GameManager.prototype.getAllScores = function () {
        return new Map(this.scores);
    };
    /**
     * Get AI execution log
     */
    GameManager.prototype.getExecutionLog = function () {
        var _a;
        return (_a = this.currentAI) === null || _a === void 0 ? void 0 : _a.getLog();
    };
    /**
     * Sense wall ahead in current AI
     */
    GameManager.prototype.senseWallAhead = function () {
        var _a, _b;
        return (_b = (_a = this.currentAI) === null || _a === void 0 ? void 0 : _a.senseWallAhead()) !== null && _b !== void 0 ? _b : false;
    };
    /**
     * Get total steps taken
     */
    GameManager.prototype.getStepCount = function () {
        var _a, _b;
        return (_b = (_a = this.currentAI) === null || _a === void 0 ? void 0 : _a.getStepCount()) !== null && _b !== void 0 ? _b : 0;
    };
    /**
     * Get total levels
     */
    GameManager.prototype.getTotalLevels = function () {
        return this.levelManager.getTotalLevels();
    };
    return GameManager;
}());
exports.GameManager = GameManager;
