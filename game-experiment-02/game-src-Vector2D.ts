/**
 * 2D Vector for position, velocity, and direction
 */
export interface Vector2D {
  x: number;
  y: number;
}

/**
 * Vector utility functions
 */
export class Vector {
  static zero(): Vector2D {
    return { x: 0, y: 0 };
  }

  static create(x: number, y: number): Vector2D {
    return { x, y };
  }

  static add(a: Vector2D, b: Vector2D): Vector2D {
    return { x: a.x + b.x, y: a.y + b.y };
  }

  static subtract(a: Vector2D, b: Vector2D): Vector2D {
    return { x: a.x - b.x, y: a.y - b.y };
  }

  static scale(v: Vector2D, scalar: number): Vector2D {
    return { x: v.x * scalar, y: v.y * scalar };
  }

  static magnitude(v: Vector2D): number {
    return Math.sqrt(v.x * v.x + v.y * v.y);
  }

  static normalize(v: Vector2D): Vector2D {
    const mag = Vector.magnitude(v);
    if (mag === 0) return { x: 0, y: 0 };
    return { x: v.x / mag, y: v.y / mag };
  }

  static distance(a: Vector2D, b: Vector2D): number {
    return Vector.magnitude(Vector.subtract(b, a));
  }

  static dot(a: Vector2D, b: Vector2D): number {
    return a.x * b.x + a.y * b.y;
  }

  static fromAngle(angleRad: number, length: number = 1): Vector2D {
    return { x: Math.cos(angleRad) * length, y: Math.sin(angleRad) * length };
  }
}
