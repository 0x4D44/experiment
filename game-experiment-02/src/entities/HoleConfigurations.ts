import { Vector2D } from '../utils/Vector2D';
import { HoleConfig } from '../types/Physics';

export const HOLE_CONFIGURATIONS: HoleConfig[] = [
  {
    id: 1,
    name: 'The Gentle Start',
    par: 2,
    ballStartPos: new Vector2D(100, 500),
    holePos: new Vector2D(700, 500),
    bounds: { minX: 0, maxX: 800, minY: 0, maxY: 600 },
    initialGravityWells: [],
    obstacles: [],
    wormholes: [],
    maxGravityModifiers: 0,
    description: 'Simple straight shot. No obstacles, no gravity wells. Get a feel for the physics.',
  },
  {
    id: 2,
    name: 'Single Attractor',
    par: 2,
    ballStartPos: new Vector2D(100, 100),
    holePos: new Vector2D(700, 500),
    bounds: { minX: 0, maxX: 800, minY: 0, maxY: 600 },
    initialGravityWells: [
      {
        position: new Vector2D(400, 300),
        strength: 300, // Attractive
        radius: 250,
      },
    ],
    obstacles: [],
    wormholes: [],
    maxGravityModifiers: 0,
    description: 'Learn to navigate around a gravity attractor. Time your shot to curve around the well.',
  },
  {
    id: 3,
    name: 'Dual Wells',
    par: 3,
    ballStartPos: new Vector2D(50, 300),
    holePos: new Vector2D(750, 300),
    bounds: { minX: 0, maxX: 800, minY: 0, maxY: 600 },
    initialGravityWells: [
      {
        position: new Vector2D(300, 150),
        strength: 250,
        radius: 200,
      },
      {
        position: new Vector2D(500, 450),
        strength: 250,
        radius: 200,
      },
    ],
    obstacles: [],
    wormholes: [],
    maxGravityModifiers: 0,
    description: 'Navigate between two gravity wells. The path curves dramatically.',
  },
  {
    id: 4,
    name: 'Wall Maze',
    par: 3,
    ballStartPos: new Vector2D(100, 300),
    holePos: new Vector2D(700, 300),
    bounds: { minX: 0, maxX: 800, minY: 0, maxY: 600 },
    initialGravityWells: [],
    obstacles: [
      { position: new Vector2D(300, 200), radius: 30, type: 'wall' },
      { position: new Vector2D(400, 400), radius: 30, type: 'wall' },
      { position: new Vector2D(500, 200), radius: 30, type: 'wall' },
      { position: new Vector2D(600, 400), radius: 30, type: 'wall' },
    ],
    wormholes: [],
    maxGravityModifiers: 2,
    description: 'Dodge through walls using gravity modifiers to guide your path.',
  },
  {
    id: 5,
    name: 'Black Hole Danger',
    par: 3,
    ballStartPos: new Vector2D(100, 500),
    holePos: new Vector2D(700, 100),
    bounds: { minX: 0, maxX: 800, minY: 0, maxY: 600 },
    initialGravityWells: [
      {
        position: new Vector2D(400, 300),
        strength: 600, // Strong attractor
        radius: 300,
      },
    ],
    obstacles: [
      { position: new Vector2D(400, 300), radius: 40, type: 'blackhole' },
    ],
    wormholes: [],
    maxGravityModifiers: 2,
    description: 'The black hole pulls everything in! Use repelling gravity wells to avoid it.',
  },
  {
    id: 6,
    name: 'Wormhole Portal',
    par: 2,
    ballStartPos: new Vector2D(100, 300),
    holePos: new Vector2D(700, 300),
    bounds: { minX: 0, maxX: 800, minY: 0, maxY: 600 },
    initialGravityWells: [
      {
        position: new Vector2D(350, 300),
        strength: 250,
        radius: 200,
      },
    ],
    obstacles: [
      { position: new Vector2D(300, 300), radius: 20, type: 'wormhole' },
    ],
    wormholes: [
      {
        entrance: new Vector2D(300, 300),
        exit: new Vector2D(600, 300),
        radius: 20,
      },
    ],
    maxGravityModifiers: 0,
    description: 'Use the wormhole to teleport across the map. Time it perfectly!',
  },
  {
    id: 7,
    name: 'Asteroid Field',
    par: 4,
    ballStartPos: new Vector2D(100, 100),
    holePos: new Vector2D(700, 500),
    bounds: { minX: 0, maxX: 800, minY: 0, maxY: 600 },
    initialGravityWells: [
      {
        position: new Vector2D(250, 250),
        strength: 200,
        radius: 180,
      },
      {
        position: new Vector2D(550, 350),
        strength: 200,
        radius: 180,
      },
    ],
    obstacles: [
      { position: new Vector2D(300, 300), radius: 15, type: 'asteroid' },
      { position: new Vector2D(350, 250), radius: 15, type: 'asteroid' },
      { position: new Vector2D(400, 350), radius: 15, type: 'asteroid' },
      { position: new Vector2D(450, 280), radius: 15, type: 'asteroid' },
      { position: new Vector2D(500, 400), radius: 15, type: 'asteroid' },
    ],
    wormholes: [],
    maxGravityModifiers: 3,
    description: 'Dodge asteroids while navigating gravity wells. This one requires precision.',
  },
  {
    id: 8,
    name: 'The Gravity Gauntlet',
    par: 4,
    ballStartPos: new Vector2D(50, 300),
    holePos: new Vector2D(750, 300),
    bounds: { minX: 0, maxX: 800, minY: 0, maxY: 600 },
    initialGravityWells: [
      {
        position: new Vector2D(200, 150),
        strength: -300, // Repulsive!
        radius: 200,
      },
      {
        position: new Vector2D(400, 300),
        strength: 400,
        radius: 250,
      },
      {
        position: new Vector2D(600, 450),
        strength: -250, // Repulsive
        radius: 200,
      },
    ],
    obstacles: [
      { position: new Vector2D(300, 300), radius: 25, type: 'asteroid' },
      { position: new Vector2D(500, 300), radius: 25, type: 'asteroid' },
    ],
    wormholes: [],
    maxGravityModifiers: 3,
    description: 'Master both attractive and repulsive gravity wells. Complex geometry ahead!',
  },
  {
    id: 9,
    name: 'Cosmic Challenge',
    par: 5,
    ballStartPos: new Vector2D(100, 100),
    holePos: new Vector2D(700, 500),
    bounds: { minX: 0, maxX: 800, minY: 0, maxY: 600 },
    initialGravityWells: [
      {
        position: new Vector2D(250, 200),
        strength: 300,
        radius: 200,
      },
      {
        position: new Vector2D(400, 400),
        strength: -350, // Repulsive
        radius: 220,
      },
      {
        position: new Vector2D(550, 250),
        strength: 250,
        radius: 200,
      },
    ],
    obstacles: [
      { position: new Vector2D(400, 300), radius: 50, type: 'blackhole' },
      { position: new Vector2D(200, 400), radius: 15, type: 'asteroid' },
      { position: new Vector2D(600, 350), radius: 15, type: 'asteroid' },
      { position: new Vector2D(300, 150), radius: 15, type: 'asteroid' },
      { position: new Vector2D(550, 500), radius: 15, type: 'asteroid' },
    ],
    wormholes: [
      {
        entrance: new Vector2D(150, 500),
        exit: new Vector2D(700, 150),
        radius: 20,
      },
    ],
    maxGravityModifiers: 4,
    description: 'The ultimate challenge! Black holes, wormholes, asteroids, and complex gravity. Can you par this?',
  },
];
