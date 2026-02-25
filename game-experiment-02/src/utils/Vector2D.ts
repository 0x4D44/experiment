/**
 * Vector2D - 2D vector mathematics for physics calculations
 */
export class Vector2D {
  constructor(public x: number = 0, public y: number = 0) {}

  /**
   * Add another vector to this vector (mutates)
   */
  add(other: Vector2D): Vector2D {
    this.x += other.x;
    this.y += other.y;
    return this;
  }

  /**
   * Add two vectors and return new vector (immutable)
   */
  static add(a: Vector2D, b: Vector2D): Vector2D {
    return new Vector2D(a.x + b.x, a.y + b.y);
  }

  /**
   * Subtract another vector (mutates)
   */
  subtract(other: Vector2D): Vector2D {
    this.x -= other.x;
    this.y -= other.y;
    return this;
  }

  /**
   * Subtract two vectors and return new vector (immutable)
   */
  static subtract(a: Vector2D, b: Vector2D): Vector2D {
    return new Vector2D(a.x - b.x, a.y - b.y);
  }

  /**
   * Scale vector by scalar (mutates)
   */
  scale(scalar: number): Vector2D {
    this.x *= scalar;
    this.y *= scalar;
    return this;
  }

  /**
   * Scale vector by scalar (immutable)
   */
  static scale(v: Vector2D, scalar: number): Vector2D {
    return new Vector2D(v.x * scalar, v.y * scalar);
  }

  /**
   * Get magnitude of vector
   */
  magnitude(): number {
    return Math.sqrt(this.x * this.x + this.y * this.y);
  }

  /**
   * Get squared magnitude (faster for comparisons)
   */
  magnitudeSquared(): number {
    return this.x * this.x + this.y * this.y;
  }

  /**
   * Normalize vector to unit length (mutates)
   */
  normalize(): Vector2D {
    const mag = this.magnitude();
    if (mag === 0) return this;
    return this.scale(1 / mag);
  }

  /**
   * Get normalized vector (immutable)
   */
  static normalize(v: Vector2D): Vector2D {
    const copy = v.clone();
    return copy.normalize();
  }

  /**
   * Dot product
   */
  dot(other: Vector2D): number {
    return this.x * other.x + this.y * other.y;
  }

  /**
   * Static dot product
   */
  static dot(a: Vector2D, b: Vector2D): number {
    return a.x * b.x + a.y * b.y;
  }

  /**
   * Distance to another vector
   */
  distance(other: Vector2D): number {
    const dx = this.x - other.x;
    const dy = this.y - other.y;
    return Math.sqrt(dx * dx + dy * dy);
  }

  /**
   * Distance squared (faster for comparisons)
   */
  distanceSquared(other: Vector2D): number {
    const dx = this.x - other.x;
    const dy = this.y - other.y;
    return dx * dx + dy * dy;
  }

  /**
   * Clone this vector
   */
  clone(): Vector2D {
    return new Vector2D(this.x, this.y);
  }

  /**
   * Check equality
   */
  equals(other: Vector2D, epsilon: number = 0.0001): boolean {
    return Math.abs(this.x - other.x) < epsilon && Math.abs(this.y - other.y) < epsilon;
  }

  /**
   * Get angle in radians
   */
  angle(): number {
    return Math.atan2(this.y, this.x);
  }

  /**
   * Clamp magnitude to max value (mutates)
   */
  clampMagnitude(maxMagnitude: number): Vector2D {
    const mag = this.magnitude();
    if (mag > maxMagnitude) {
      return this.scale(maxMagnitude / mag);
    }
    return this;
  }

  /**
   * Create from angle and magnitude
   */
  static fromAngle(angle: number, magnitude: number = 1): Vector2D {
    return new Vector2D(Math.cos(angle) * magnitude, Math.sin(angle) * magnitude);
  }

  /**
   * Rotate vector by angle in radians (mutates)
   */
  rotate(angle: number): Vector2D {
    const cos = Math.cos(angle);
    const sin = Math.sin(angle);
    const x = this.x * cos - this.y * sin;
    const y = this.x * sin + this.y * cos;
    this.x = x;
    this.y = y;
    return this;
  }

  /**
   * String representation
   */
  toString(): string {
    return `(${this.x.toFixed(2)}, ${this.y.toFixed(2)})`;
  }
}
