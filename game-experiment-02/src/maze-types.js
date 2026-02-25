"use strict";
/**
 * Core type definitions for Maze Runner AI game
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.AICommand = exports.CellType = void 0;
var CellType;
(function (CellType) {
    CellType[CellType["Empty"] = 0] = "Empty";
    CellType[CellType["Wall"] = 1] = "Wall";
    CellType[CellType["Goal"] = 2] = "Goal";
    CellType[CellType["Door"] = 4] = "Door";
    CellType[CellType["Key"] = 8] = "Key";
    CellType[CellType["Teleporter"] = 16] = "Teleporter";
    CellType[CellType["StartPosition"] = 32] = "StartPosition";
})(CellType || (exports.CellType = CellType = {}));
var AICommand;
(function (AICommand) {
    AICommand["Forward"] = "FORWARD";
    AICommand["TurnLeft"] = "TURN_LEFT";
    AICommand["TurnRight"] = "TURN_RIGHT";
    AICommand["SenseWall"] = "SENSE_WALL";
    AICommand["MarkPath"] = "MARK_PATH";
    AICommand["PickupKey"] = "PICKUP_KEY";
    AICommand["UseDoor"] = "USE_DOOR";
    AICommand["Wait"] = "WAIT";
})(AICommand || (exports.AICommand = AICommand = {}));
