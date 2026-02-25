/**
 * Simple AI for Hex Commander
 * Uses basic heuristics to make tactical decisions
 */

import { Game, Unit, PlayerSide, coordDistance, coordNeighbors, UnitType } from './hex-commander';

export class SimpleAI {
  private game: Game;

  constructor(game: Game) {
    this.game = game;
  }

  takeTurn(): void {
    // Get all AI units
    const units = this.game.state.map.getUnitsForPlayer(this.game.state.currentPlayer);

    // Process each unit
    for (const unit of units) {
      // Try to attack first
      if (!unit.attacked && this.tryAttack(unit)) {
        continue;
      }

      // Then move towards enemies
      if (!unit.moved) {
        this.moveTowardEnemy(unit);
      }
    }

    // End turn
    this.game.endTurn();
  }

  private tryAttack(unit: Unit): boolean {
    const enemyPlayers = unit.owner === PlayerSide.Player1 ? [PlayerSide.Player2] : [PlayerSide.Player1];

    for (const enemyPlayer of enemyPlayers) {
      const enemies = this.game.state.map.getUnitsForPlayer(enemyPlayer);

      // Find closest enemy within attack range
      for (const enemy of enemies) {
        if (this.game.canAttackUnit(unit, enemy)) {
          this.game.attackUnit(unit, enemy);
          return true;
        }
      }
    }

    return false;
  }

  private moveTowardEnemy(unit: Unit): void {
    const enemyPlayers = unit.owner === PlayerSide.Player1 ? [PlayerSide.Player2] : [PlayerSide.Player1];
    let closestEnemy: Unit | null = null;
    let closestDistance = Infinity;

    // Find the closest enemy
    for (const enemyPlayer of enemyPlayers) {
      const enemies = this.game.state.map.getUnitsForPlayer(enemyPlayer);
      for (const enemy of enemies) {
        const distance = coordDistance(unit.position, enemy.position);
        if (distance < closestDistance) {
          closestDistance = distance;
          closestEnemy = enemy;
        }
      }
    }

    if (!closestEnemy) return;

    // Move toward closest enemy
    const neighbors = coordNeighbors(unit.position);
    let bestMove = null;
    let bestDistance = closestDistance;

    for (const neighbor of neighbors) {
      if (this.game.canMoveUnit(unit, neighbor)) {
        const newDistance = coordDistance(neighbor, closestEnemy.position);
        if (newDistance < bestDistance) {
          bestDistance = newDistance;
          bestMove = neighbor;
        }
      }
    }

    if (bestMove) {
      this.game.moveUnit(unit, bestMove);
    }
  }
}

export default SimpleAI;
