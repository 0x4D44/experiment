"use strict";
/**
 * Maze grid management and generation
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
exports.Maze = void 0;
var maze_types_1 = require("./maze-types");
var Maze = /** @class */ (function () {
    function Maze(width, height) {
        this.width = width;
        this.height = height;
        this.grid = this.initializeGrid();
        this.startPos = { x: 0, y: 0 };
        this.goalPos = { x: width - 1, y: height - 1 };
    }
    Maze.prototype.initializeGrid = function () {
        var grid = [];
        for (var y = 0; y < this.height; y++) {
            var row = [];
            for (var x = 0; x < this.width; x++) {
                row.push({ type: maze_types_1.CellType.Empty });
            }
            grid.push(row);
        }
        return grid;
    };
    /**
     * Set cell type at position
     */
    Maze.prototype.setCell = function (pos, type) {
        if (this.isValidPosition(pos)) {
            var cell = this.grid[pos.y][pos.x];
            cell.type |= type; // Add type to bitmask
        }
    };
    /**
     * Clear cell type at position
     */
    Maze.prototype.clearCell = function (pos, type) {
        if (this.isValidPosition(pos)) {
            var cell = this.grid[pos.y][pos.x];
            cell.type &= ~type; // Remove type from bitmask
        }
    };
    /**
     * Check if cell has type
     */
    Maze.prototype.hasType = function (pos, type) {
        if (!this.isValidPosition(pos))
            return false;
        return (this.grid[pos.y][pos.x].type & type) !== 0;
    };
    /**
     * Check if position is valid
     */
    Maze.prototype.isValidPosition = function (pos) {
        return pos.x >= 0 && pos.x < this.width && pos.y >= 0 && pos.y < this.height;
    };
    /**
     * Check if cell is walkable
     */
    Maze.prototype.isWalkable = function (pos) {
        if (!this.isValidPosition(pos))
            return false;
        var cell = this.grid[pos.y][pos.x];
        // Walkable if not a wall
        return !this.hasType(pos, maze_types_1.CellType.Wall);
    };
    /**
     * Set cell as wall
     */
    Maze.prototype.setWall = function (pos) {
        this.setCell(pos, maze_types_1.CellType.Wall);
    };
    /**
     * Check if cell is a wall
     */
    Maze.prototype.isWall = function (pos) {
        return this.hasType(pos, maze_types_1.CellType.Wall);
    };
    /**
     * Set goal position
     */
    Maze.prototype.setGoal = function (pos) {
        this.goalPos = __assign({}, pos);
        this.setCell(pos, maze_types_1.CellType.Goal);
    };
    /**
     * Set start position
     */
    Maze.prototype.setStart = function (pos) {
        this.startPos = __assign({}, pos);
        this.setCell(pos, maze_types_1.CellType.StartPosition);
    };
    /**
     * Add a key to the maze
     */
    Maze.prototype.addKey = function (pos, keyId) {
        if (this.isValidPosition(pos)) {
            var cell = this.grid[pos.y][pos.x];
            cell.type |= maze_types_1.CellType.Key;
            cell.keyId = keyId;
        }
    };
    /**
     * Add a door to the maze
     */
    Maze.prototype.addDoor = function (pos, keyId) {
        if (this.isValidPosition(pos)) {
            var cell = this.grid[pos.y][pos.x];
            cell.type |= maze_types_1.CellType.Door;
            cell.keyId = keyId;
        }
    };
    /**
     * Add a teleporter
     */
    Maze.prototype.addTeleporter = function (pos, target) {
        if (this.isValidPosition(pos) && this.isValidPosition(target)) {
            var cell = this.grid[pos.y][pos.x];
            cell.type |= maze_types_1.CellType.Teleporter;
            cell.teleportTarget = __assign({}, target);
        }
    };
    /**
     * Get cell at position
     */
    Maze.prototype.getCell = function (pos) {
        if (!this.isValidPosition(pos))
            return null;
        return this.grid[pos.y][pos.x];
    };
    /**
     * Get maze as exportable structure
     */
    Maze.prototype.getGrid = function () {
        return {
            width: this.width,
            height: this.height,
            cells: this.grid,
            startPos: __assign({}, this.startPos),
            goalPos: __assign({}, this.goalPos),
        };
    };
    /**
     * Get width
     */
    Maze.prototype.getWidth = function () {
        return this.width;
    };
    /**
     * Get height
     */
    Maze.prototype.getHeight = function () {
        return this.height;
    };
    /**
     * Get start position
     */
    Maze.prototype.getStart = function () {
        return __assign({}, this.startPos);
    };
    /**
     * Get goal position
     */
    Maze.prototype.getGoal = function () {
        return __assign({}, this.goalPos);
    };
    return Maze;
}());
exports.Maze = Maze;
