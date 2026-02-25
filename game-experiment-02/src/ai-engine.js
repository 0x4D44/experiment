"use strict";
/**
 * AI interpreter and execution engine
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
exports.AIEngine = void 0;
var maze_types_1 = require("./maze-types");
var AIEngine = /** @class */ (function () {
    function AIEngine(maze, startPos, maxSteps) {
        if (maxSteps === void 0) { maxSteps = 500; }
        this.executionLog = [];
        this.maze = maze;
        this.maxSteps = maxSteps;
        this.state = {
            position: __assign({}, startPos),
            direction: 'N', // Facing north
            keysHeld: new Set(),
            stepCount: 0,
            markedCells: new Set(),
            finished: false,
            reachedGoal: false,
        };
    }
    /**
     * Execute a single command
     */
    AIEngine.prototype.executeCommand = function (command) {
        var step = {
            command: command,
            position: __assign({}, this.state.position),
            direction: this.state.direction,
            keysHeld: Array.from(this.state.keysHeld),
            success: false,
        };
        // Check if already finished
        if (this.state.finished) {
            step.error = 'Execution already finished';
            return step;
        }
        // Check step limit
        if (this.state.stepCount >= this.maxSteps) {
            step.error = 'Step limit exceeded';
            this.state.finished = true;
            return step;
        }
        try {
            switch (command) {
                case maze_types_1.AICommand.Forward:
                    this.executeForward();
                    break;
                case maze_types_1.AICommand.TurnLeft:
                    this.executeTurnLeft();
                    break;
                case maze_types_1.AICommand.TurnRight:
                    this.executeTurnRight();
                    break;
                case maze_types_1.AICommand.SenseWall:
                    // This just returns success; actual sensing is done differently
                    break;
                case maze_types_1.AICommand.MarkPath:
                    this.executeMarkPath();
                    break;
                case maze_types_1.AICommand.PickupKey:
                    this.executePickupKey();
                    break;
                case maze_types_1.AICommand.UseDoor:
                    this.executeUseDoor();
                    break;
                case maze_types_1.AICommand.Wait:
                    // Just increment step count
                    break;
            }
            // Check if reached goal
            var goalPos = this.maze.getGoal();
            if (this.state.position.x === goalPos.x && this.state.position.y === goalPos.y) {
                this.state.reachedGoal = true;
                this.state.finished = true;
            }
            this.state.stepCount++;
            step.success = true;
            step.position = __assign({}, this.state.position);
            step.direction = this.state.direction;
            step.keysHeld = Array.from(this.state.keysHeld);
        }
        catch (error) {
            step.error = String(error);
            this.state.finished = true;
        }
        this.executionLog.push(step);
        return step;
    };
    /**
     * Execute forward movement
     */
    AIEngine.prototype.executeForward = function () {
        var nextPos = this.getNextPosition();
        if (!this.maze.isValidPosition(nextPos)) {
            throw new Error('Cannot move outside maze boundaries');
        }
        // Check for walls
        if (this.maze.isWall(nextPos)) {
            throw new Error('Cannot move through wall');
        }
        // Check for doors
        if (this.maze.hasType(nextPos, maze_types_1.CellType.Door)) {
            var cell = this.maze.getCell(nextPos);
            if (cell && cell.keyId !== undefined && !this.state.keysHeld.has(cell.keyId)) {
                throw new Error("Need key ".concat(cell.keyId, " to open door"));
            }
        }
        // Check for teleporter
        if (this.maze.hasType(nextPos, maze_types_1.CellType.Teleporter)) {
            var cell = this.maze.getCell(nextPos);
            if (cell && cell.teleportTarget) {
                this.state.position = __assign({}, cell.teleportTarget);
                return;
            }
        }
        this.state.position = nextPos;
    };
    /**
     * Execute turn left
     */
    AIEngine.prototype.executeTurnLeft = function () {
        var turns = { N: 'W', W: 'S', S: 'E', E: 'N' };
        this.state.direction = turns[this.state.direction];
    };
    /**
     * Execute turn right
     */
    AIEngine.prototype.executeTurnRight = function () {
        var turns = { N: 'E', E: 'S', S: 'W', W: 'N' };
        this.state.direction = turns[this.state.direction];
    };
    /**
     * Mark current cell
     */
    AIEngine.prototype.executeMarkPath = function () {
        var key = "".concat(this.state.position.x, ",").concat(this.state.position.y);
        this.state.markedCells.add(key);
    };
    /**
     * Pickup a key
     */
    AIEngine.prototype.executePickupKey = function () {
        var cell = this.maze.getCell(this.state.position);
        if (cell && this.maze.hasType(this.state.position, maze_types_1.CellType.Key) && cell.keyId !== undefined) {
            this.state.keysHeld.add(cell.keyId);
        }
        else {
            throw new Error('No key at current position');
        }
    };
    /**
     * Use door
     */
    AIEngine.prototype.executeUseDoor = function () {
        // Doors are checked during movement, this is a no-op
        // Could be used for interactive door mechanics in future
    };
    /**
     * Sense wall in current direction
     */
    AIEngine.prototype.senseWallAhead = function () {
        var nextPos = this.getNextPosition();
        return this.maze.isWall(nextPos) || !this.maze.isValidPosition(nextPos);
    };
    /**
     * Get next position based on current direction
     */
    AIEngine.prototype.getNextPosition = function () {
        var _a = this.state.position, x = _a.x, y = _a.y;
        switch (this.state.direction) {
            case 'N':
                return { x: x, y: y - 1 };
            case 'E':
                return { x: x + 1, y: y };
            case 'S':
                return { x: x, y: y + 1 };
            case 'W':
                return { x: x - 1, y: y };
        }
    };
    /**
     * Get current state
     */
    AIEngine.prototype.getState = function () {
        return Object.freeze(__assign({}, this.state));
    };
    /**
     * Get execution log
     */
    AIEngine.prototype.getLog = function () {
        return this.executionLog;
    };
    /**
     * Reset to initial state
     */
    AIEngine.prototype.reset = function () {
        var startPos = this.maze.getStart();
        this.state = {
            position: __assign({}, startPos),
            direction: 'N',
            keysHeld: new Set(),
            stepCount: 0,
            markedCells: new Set(),
            finished: false,
            reachedGoal: false,
        };
        this.executionLog = [];
    };
    /**
     * Check if AI reached the goal
     */
    AIEngine.prototype.isGoalReached = function () {
        return this.state.reachedGoal;
    };
    /**
     * Check if execution is finished
     */
    AIEngine.prototype.isFinished = function () {
        return this.state.finished;
    };
    /**
     * Get steps taken
     */
    AIEngine.prototype.getStepCount = function () {
        return this.state.stepCount;
    };
    return AIEngine;
}());
exports.AIEngine = AIEngine;
