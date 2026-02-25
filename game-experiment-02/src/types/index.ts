export enum PowerUpType { MULTI_BALL = 'MULTI_BALL', LASER_PADDLE = 'LASER_PADDLE', STICKY_PADDLE = 'STICKY_PADDLE', SLOW_MO = 'SLOW_MO', EXPAND_PADDLE = 'EXPAND_PADDLE', SHIELD = 'SHIELD' }
export interface BlockConfig { x: number; y: number; width: number; height: number; health: number; points: number; color: number }
export interface PowerUpConfig { x: number; y: number; type: PowerUpType; duration?: number }
export interface GameState { score: number; lives: number; level: number; combo: number; isPaused: boolean; gameOver: boolean; levelComplete: boolean }
export interface LevelConfig { number: number; blocks: BlockConfig[]; bossHealth?: number; ballSpeed: number; paddleWidth: number }
