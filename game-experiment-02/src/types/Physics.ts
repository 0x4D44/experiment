import { Vector2D } from '../utils/Vector2D';

/**
 * Gravity well - source of gravitational force
 */
export interface GravityWell {
  position: Vector2D;
  strength: number; // Gravitational constant * mass, positive = attractive, negative = repulsive
  radius: number; // Radius of effect (can be used to limit influence)
}

/**
 * Obstacle in the level
 */
export interface Obstacle {
  position: Vector2D;
  radius: number;
  type: 'wall' | 'blackhole' | 'wormhole' | 'asteroid';
}

/**
 * Wormhole pair for teleportation
 */
export interface Wormhole {
  entrance: Vector2D;
  exit: Vector2D;
  radius: number;
}

/**
 * Ball state
 */
export interface BallState {
  position: Vector2D;
  velocity: Vector2D;
  mass: number;
  radius: number;
}

/**
 * Game state for a hole
 */
export interface GameState {
  ball: BallState;
  gravityWells: GravityWell[];
  obstacles: Obstacle[];
  wormholes: Wormhole[];
  hole: Vector2D;
  holeRadius: number;
  groundFriction: number;
  damping: number;
  ballStopped: boolean;
  strokes: number;
  gravityModifiersUsed: number;
  maxGravityModifiers: number;
  inHole: boolean;
  ballInMotion: boolean;
}

/**
 * Hole configuration
 */
export interface HoleConfig {
  id: number;
  name: string;
  par: number;
  ballStartPos: Vector2D;
  holePos: Vector2D;
  bounds: {
    minX: number;
    maxX: number;
    minY: number;
    maxY: number;
  };
  initialGravityWells: GravityWell[];
  obstacles: Obstacle[];
  wormholes: Wormhole[];
  maxGravityModifiers: number;
  description: string;
}

/**
 * Physics simulation parameters
 */
export interface PhysicsParams {
  timeStep: number;
  damping: number;
  groundFriction: number;
  gravityConstant: number;
  maxVelocity: number;
  stopThreshold: number;
}

/**
 * Collision result
 */
export interface CollisionResult {
  collided: boolean;
  normal?: Vector2D;
  penetration?: number;
  obstacle?: Obstacle;
  wormhole?: Wormhole;
}

/**
 * Trajectory point for prediction
 */
export interface TrajectoryPoint {
  position: Vector2D;
  velocity: Vector2D;
}
