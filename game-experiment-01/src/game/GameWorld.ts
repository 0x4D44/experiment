/**
 * Game world with obstacles, objectives, and collision detection
 */

export interface Position {
  x: number;
  y: number;
}

export interface GameObject {
  position: Position;
  type: 'obstacle' | 'objective' | 'wall';
  radius: number;
}

export interface WorldBounds {
  minX: number;
  maxX: number;
  minY: number;
  maxY: number;
}

export class GameWorld {
  private width: number;
  private height: number;
  private obstacles: GameObject[] = [];
  private objectives: GameObject[] = [];
  private walls: GameObject[] = [];
  private playerPosition: Position;
  private playerRadius: number = 0.5;
  private objectiveRadius: number = 1.0;
  private obstacleRadius: number = 1.5;

  constructor(width: number = 50, height: number = 50) {
    this.width = width;
    this.height = height;
    this.playerPosition = { x: width / 2, y: height / 2 };
    this.initializeWorld();
  }

  /**
   * Initialize the game world with obstacles and objectives
   */
  private initializeWorld(): void {
    // Create boundary walls
    this.createWalls();

    // Create obstacles
    this.createObstacles(10);

    // Create objectives
    this.createObjectives(5);
  }

  /**
   * Create boundary walls
   */
  private createWalls(): void {
    const wallThickness = 1.0;

    // Top wall
    for (let x = 0; x < this.width; x += wallThickness) {
      this.walls.push({
        position: { x, y: 0 },
        type: 'wall',
        radius: wallThickness,
      });
    }

    // Bottom wall
    for (let x = 0; x < this.width; x += wallThickness) {
      this.walls.push({
        position: { x, y: this.height },
        type: 'wall',
        radius: wallThickness,
      });
    }

    // Left wall
    for (let y = 0; y < this.height; y += wallThickness) {
      this.walls.push({
        position: { x: 0, y },
        type: 'wall',
        radius: wallThickness,
      });
    }

    // Right wall
    for (let y = 0; y < this.height; y += wallThickness) {
      this.walls.push({
        position: { x: this.width, y },
        type: 'wall',
        radius: wallThickness,
      });
    }
  }

  /**
   * Create random obstacles
   */
  private createObstacles(count: number): void {
    for (let i = 0; i < count; i++) {
      let position: Position;
      let attempts = 0;
      const maxAttempts = 100;

      do {
        position = {
          x: Math.random() * (this.width - 10) + 5,
          y: Math.random() * (this.height - 10) + 5,
        };
        attempts++;
      } while (
        this.isPositionOccupied(position, this.obstacleRadius) &&
        attempts < maxAttempts
      );

      if (attempts < maxAttempts) {
        this.obstacles.push({
          position,
          type: 'obstacle',
          radius: this.obstacleRadius,
        });
      }
    }
  }

  /**
   * Create random objectives
   */
  private createObjectives(count: number): void {
    for (let i = 0; i < count; i++) {
      let position: Position;
      let attempts = 0;
      const maxAttempts = 100;

      do {
        position = {
          x: Math.random() * (this.width - 10) + 5,
          y: Math.random() * (this.height - 10) + 5,
        };
        attempts++;
      } while (
        this.isPositionOccupied(position, this.objectiveRadius) &&
        attempts < maxAttempts
      );

      if (attempts < maxAttempts) {
        this.objectives.push({
          position,
          type: 'objective',
          radius: this.objectiveRadius,
        });
      }
    }
  }

  /**
   * Check if a position is occupied
   */
  private isPositionOccupied(position: Position, radius: number): boolean {
    // Check distance from player
    const distToPlayer = this.calculateDistance(position, this.playerPosition);
    if (distToPlayer < radius + this.playerRadius + 5) {
      return true;
    }

    // Check distance from obstacles
    for (const obstacle of this.obstacles) {
      const dist = this.calculateDistance(position, obstacle.position);
      if (dist < radius + obstacle.radius + 2) {
        return true;
      }
    }

    // Check distance from objectives
    for (const objective of this.objectives) {
      const dist = this.calculateDistance(position, objective.position);
      if (dist < radius + objective.radius + 2) {
        return true;
      }
    }

    return false;
  }

  /**
   * Calculate distance between two positions
   */
  private calculateDistance(pos1: Position, pos2: Position): number {
    const dx = pos1.x - pos2.x;
    const dy = pos1.y - pos2.y;
    return Math.sqrt(dx * dx + dy * dy);
  }

  /**
   * Calculate angle between two positions
   */
  private calculateAngle(from: Position, to: Position): number {
    return Math.atan2(to.y - from.y, to.x - from.x);
  }

  /**
   * Move player in a direction
   */
  movePlayer(direction: { dx: number; dy: number }, distance: number): boolean {
    const newPosition = {
      x: this.playerPosition.x + direction.dx * distance,
      y: this.playerPosition.y + direction.dy * distance,
    };

    // Check collisions
    if (this.checkCollision(newPosition)) {
      return false; // Collision detected
    }

    this.playerPosition = newPosition;
    return true; // Move successful
  }

  /**
   * Check if position would cause collision
   */
  private checkCollision(position: Position): boolean {
    // Check walls
    for (const wall of this.walls) {
      const dist = this.calculateDistance(position, wall.position);
      if (dist < this.playerRadius + wall.radius) {
        return true;
      }
    }

    // Check obstacles
    for (const obstacle of this.obstacles) {
      const dist = this.calculateDistance(position, obstacle.position);
      if (dist < this.playerRadius + obstacle.radius) {
        return true;
      }
    }

    return false;
  }

  /**
   * Detect objects within sonar range
   */
  detectObjects(range: number): GameObject[] {
    const detected: GameObject[] = [];

    // Check obstacles
    for (const obstacle of this.obstacles) {
      const dist = this.calculateDistance(this.playerPosition, obstacle.position);
      if (dist <= range) {
        detected.push({
          ...obstacle,
          position: { ...obstacle.position },
        });
      }
    }

    // Check objectives
    for (const objective of this.objectives) {
      const dist = this.calculateDistance(this.playerPosition, objective.position);
      if (dist <= range) {
        detected.push({
          ...objective,
          position: { ...objective.position },
        });
      }
    }

    // Check walls
    for (const wall of this.walls) {
      const dist = this.calculateDistance(this.playerPosition, wall.position);
      if (dist <= range) {
        detected.push({
          ...wall,
          position: { ...wall.position },
        });
      }
    }

    return detected;
  }

  /**
   * Get echoes for detected objects
   */
  getEchoes(range: number): Array<{ distance: number; angle: number; type: GameObject['type'] }> {
    const objects = this.detectObjects(range);

    return objects.map(obj => ({
      distance: this.calculateDistance(this.playerPosition, obj.position),
      angle: this.calculateAngle(this.playerPosition, obj.position),
      type: obj.type,
    }));
  }

  /**
   * Check if player reached an objective
   */
  checkObjectiveReached(): boolean {
    for (let i = 0; i < this.objectives.length; i++) {
      const objective = this.objectives[i];
      const dist = this.calculateDistance(this.playerPosition, objective.position);

      if (dist < this.playerRadius + objective.radius) {
        // Remove reached objective
        this.objectives.splice(i, 1);
        return true;
      }
    }

    return false;
  }

  /**
   * Get player position
   */
  getPlayerPosition(): Position {
    return { ...this.playerPosition };
  }

  /**
   * Get number of remaining objectives
   */
  getRemainingObjectives(): number {
    return this.objectives.length;
  }

  /**
   * Get world bounds
   */
  getWorldBounds(): WorldBounds {
    return {
      minX: 0,
      maxX: this.width,
      minY: 0,
      maxY: this.height,
    };
  }

  /**
   * Get all objects (for debugging/visualization)
   */
  getAllObjects(): {
    obstacles: GameObject[];
    objectives: GameObject[];
    walls: GameObject[];
  } {
    return {
      obstacles: this.obstacles.map(o => ({ ...o, position: { ...o.position } })),
      objectives: this.objectives.map(o => ({ ...o, position: { ...o.position } })),
      walls: this.walls.map(w => ({ ...w, position: { ...w.position } })),
    };
  }

  /**
   * Reset player position
   */
  resetPlayerPosition(): void {
    this.playerPosition = { x: this.width / 2, y: this.height / 2 };
  }
}
