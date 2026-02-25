/**
 * Level management and puzzle definitions
 */

import { GameLevel } from './maze-types';
import { Maze } from './maze-grid';

export class LevelManager {
  private levels: Map<number, GameLevel> = new Map();

  constructor() {
    this.initializeLevels();
  }

  private initializeLevels(): void {
    // Level 1: Simple path
    this.addLevel(1, 'Simple Path', 'Navigate straight to the goal.', (maze) => {
      maze.setStart({ x: 0, y: 0 });
      maze.setGoal({ x: 4, y: 0 });
    }, 5, 10, 'Easy');

    // Level 2: Turn required
    this.addLevel(2, 'The Corner', 'Turn and navigate to the goal.', (maze) => {
      maze.setStart({ x: 0, y: 0 });
      maze.setGoal({ x: 0, y: 4 });
      // Create an obstacle
      for (let i = 1; i < 4; i++) {
        maze.setWall({ x: 1, y: i });
      }
    }, 8, 15, 'Easy');

    // Level 3: Simple maze
    this.addLevel(3, 'First Maze', 'Navigate through a simple maze.', (maze) => {
      maze.setStart({ x: 0, y: 0 });
      maze.setGoal({ x: 6, y: 6 });
      // Create maze walls
      for (let i = 1; i < 6; i++) {
        maze.setWall({ x: 2, y: i });
      }
      for (let i = 0; i < 5; i++) {
        maze.setWall({ x: 4, y: i });
      }
    }, 15, 20, 'Easy');

    // Level 4: Key and door
    this.addLevel(4, 'Locked Gate', 'Find the key to unlock the door.', (maze) => {
      maze.setStart({ x: 0, y: 0 });
      maze.setGoal({ x: 8, y: 0 });
      maze.addKey({ x: 2, y: 0 }, 1);
      maze.addDoor({ x: 5, y: 0 }, 1);
    }, 10, 20, 'Easy');

    // Level 5: Double doors
    this.addLevel(5, 'Two Keys', 'Collect both keys to reach the goal.', (maze) => {
      maze.setStart({ x: 0, y: 0 });
      maze.setGoal({ x: 10, y: 0 });
      maze.addKey({ x: 2, y: 0 }, 1);
      maze.addDoor({ x: 4, y: 0 }, 1);
      maze.addKey({ x: 6, y: 0 }, 2);
      maze.addDoor({ x: 8, y: 0 }, 2);
    }, 15, 25, 'Easy');

    // Level 6: Teleporter
    this.addLevel(6, 'Warp Zone', 'Use the teleporter to shortcut to the goal.', (maze) => {
      maze.setStart({ x: 0, y: 0 });
      maze.setGoal({ x: 10, y: 10 });
      maze.addTeleporter({ x: 5, y: 5 }, { x: 8, y: 8 });
    }, 25, 30, 'Medium');

    // Level 7: Medium maze
    this.addLevel(7, 'Complex Path', 'Navigate a more complex maze.', (maze) => {
      maze.setStart({ x: 1, y: 1 });
      maze.setGoal({ x: 9, y: 9 });
      this.createMediumMaze(maze);
    }, 30, 40, 'Medium');

    // Level 8: Multi-key puzzle
    this.addLevel(8, 'Three Doors', 'Unlock three doors in sequence.', (maze) => {
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
    this.addLevel(9, 'Portal Puzzle', 'Use teleporters to navigate the maze.', (maze) => {
      maze.setStart({ x: 0, y: 0 });
      maze.setGoal({ x: 12, y: 12 });
      maze.addTeleporter({ x: 3, y: 3 }, { x: 6, y: 6 });
      maze.addTeleporter({ x: 9, y: 9 }, { x: 11, y: 11 });
      this.createMediumMaze(maze);
    }, 40, 50, 'Medium');

    // Level 10: Hardened maze
    this.addLevel(10, 'The Labyrinth', 'Escape a complex labyrinth.', (maze) => {
      maze.setStart({ x: 1, y: 1 });
      maze.setGoal({ x: 14, y: 14 });
      this.createHardMaze(maze);
    }, 50, 60, 'Hard');

    // Levels 11-20 with increasing difficulty
    for (let i = 11; i <= 20; i++) {
      const difficulty = i <= 13 ? 'Medium' : i <= 17 ? 'Hard' : 'Expert';
      const maxSteps = 30 + i * 5;
      const maxTime = 40 + i * 5;

      this.addLevel(
        i,
        `Challenge ${i}`,
        `Advanced puzzle ${i}. Master your AI programming skills!`,
        (maze) => {
          this.generatePuzzleMaze(maze, i);
        },
        maxSteps,
        maxTime,
        difficulty
      );
    }
  }

  private addLevel(
    id: number,
    name: string,
    description: string,
    mazeBuilder: (maze: Maze) => void,
    maxSteps: number,
    maxTime: number,
    difficulty: 'Easy' | 'Medium' | 'Hard' | 'Expert'
  ): void {
    const size = 8 + Math.floor(id / 5) * 4; // Grow maze size with level
    const maze = new Maze(Math.min(size, 16), Math.min(size, 16));
    mazeBuilder(maze);

    const level: GameLevel = {
      id,
      name,
      description,
      maze: maze.getGrid(),
      maxSteps,
      maxTime,
      difficulty,
    };

    this.levels.set(id, level);
  }

  private createMediumMaze(maze: Maze): void {
    // Create a winding maze
    for (let i = 2; i < 8; i++) {
      maze.setWall({ x: 2, y: i });
    }
    for (let i = 2; i < 8; i++) {
      maze.setWall({ x: 4, y: Math.abs(i - 4) + 2 });
    }
    for (let i = 2; i < 8; i++) {
      maze.setWall({ x: 6, y: 10 - i });
    }
  }

  private createHardMaze(maze: Maze): void {
    // Create a complex maze with multiple paths
    for (let i = 1; i < 10; i++) {
      if (i % 2 === 0) {
        maze.setWall({ x: 2, y: i });
        maze.setWall({ x: 4, y: 10 - i });
        maze.setWall({ x: 6, y: i });
        maze.setWall({ x: 8, y: 10 - i });
        maze.setWall({ x: 10, y: i });
      }
    }
  }

  private generatePuzzleMaze(maze: Maze, levelId: number): void {
    const complexity = levelId - 10;
    maze.setStart({ x: 1, y: 1 });
    maze.setGoal({ x: Math.min(14, 6 + complexity), y: Math.min(14, 6 + complexity) });

    // Add walls based on complexity
    for (let i = 0; i < complexity * 2; i++) {
      const x = 2 + (i % 6);
      const y = 2 + Math.floor(i / 6);
      if (x < 14 && y < 14) {
        maze.setWall({ x, y });
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
  }

  /**
   * Get level by ID
   */
  public getLevel(id: number): GameLevel | undefined {
    return this.levels.get(id);
  }

  /**
   * Get all levels
   */
  public getAllLevels(): GameLevel[] {
    const levels: GameLevel[] = [];
    for (let i = 1; i <= 20; i++) {
      const level = this.levels.get(i);
      if (level) {
        levels.push(level);
      }
    }
    return levels;
  }

  /**
   * Get total number of levels
   */
  public getTotalLevels(): number {
    return this.levels.size;
  }
}
