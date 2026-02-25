import { Vector2D } from './Vector2D';

describe('Vector2D', () => {
  describe('construction', () => {
    test('default constructor creates zero vector', () => {
      const v = new Vector2D();
      expect(v.x).toBe(0);
      expect(v.y).toBe(0);
    });

    test('constructor with values', () => {
      const v = new Vector2D(3, 4);
      expect(v.x).toBe(3);
      expect(v.y).toBe(4);
    });
  });

  describe('addition', () => {
    test('add mutates vector', () => {
      const v1 = new Vector2D(1, 2);
      const v2 = new Vector2D(3, 4);
      const result = v1.add(v2);
      expect(result).toBe(v1); // Same object
      expect(v1.x).toBe(4);
      expect(v1.y).toBe(6);
    });

    test('static add returns new vector', () => {
      const v1 = new Vector2D(1, 2);
      const v2 = new Vector2D(3, 4);
      const result = Vector2D.add(v1, v2);
      expect(result).not.toBe(v1);
      expect(result.x).toBe(4);
      expect(result.y).toBe(6);
      expect(v1.x).toBe(1); // Original unchanged
    });
  });

  describe('subtraction', () => {
    test('subtract mutates vector', () => {
      const v1 = new Vector2D(5, 6);
      const v2 = new Vector2D(2, 1);
      v1.subtract(v2);
      expect(v1.x).toBe(3);
      expect(v1.y).toBe(5);
    });

    test('static subtract returns new vector', () => {
      const v1 = new Vector2D(5, 6);
      const v2 = new Vector2D(2, 1);
      const result = Vector2D.subtract(v1, v2);
      expect(result.x).toBe(3);
      expect(result.y).toBe(5);
      expect(v1.x).toBe(5); // Original unchanged
    });
  });

  describe('scaling', () => {
    test('scale mutates vector', () => {
      const v = new Vector2D(2, 3);
      v.scale(2);
      expect(v.x).toBe(4);
      expect(v.y).toBe(6);
    });

    test('static scale returns new vector', () => {
      const v = new Vector2D(2, 3);
      const result = Vector2D.scale(v, 2);
      expect(result.x).toBe(4);
      expect(result.y).toBe(6);
      expect(v.x).toBe(2); // Original unchanged
    });
  });

  describe('magnitude', () => {
    test('magnitude of 3-4-5 triangle', () => {
      const v = new Vector2D(3, 4);
      expect(v.magnitude()).toBe(5);
    });

    test('magnitude squared is faster', () => {
      const v = new Vector2D(3, 4);
      expect(v.magnitudeSquared()).toBe(25);
    });

    test('zero vector magnitude', () => {
      const v = new Vector2D(0, 0);
      expect(v.magnitude()).toBe(0);
    });
  });

  describe('normalization', () => {
    test('normalize mutates vector to unit length', () => {
      const v = new Vector2D(3, 4);
      v.normalize();
      expect(v.magnitude()).toBeCloseTo(1, 5);
      expect(v.x).toBeCloseTo(0.6, 5);
      expect(v.y).toBeCloseTo(0.8, 5);
    });

    test('static normalize returns new unit vector', () => {
      const v = new Vector2D(3, 4);
      const result = Vector2D.normalize(v);
      expect(result.magnitude()).toBeCloseTo(1, 5);
      expect(v.magnitude()).toBe(5); // Original unchanged
    });

    test('normalize zero vector returns zero', () => {
      const v = new Vector2D(0, 0);
      v.normalize();
      expect(v.x).toBe(0);
      expect(v.y).toBe(0);
    });
  });

  describe('dot product', () => {
    test('dot product of perpendicular vectors is zero', () => {
      const v1 = new Vector2D(1, 0);
      const v2 = new Vector2D(0, 1);
      expect(v1.dot(v2)).toBe(0);
    });

    test('dot product of parallel vectors', () => {
      const v1 = new Vector2D(1, 2);
      const v2 = new Vector2D(2, 4);
      expect(v1.dot(v2)).toBe(10); // 1*2 + 2*4
    });

    test('static dot product', () => {
      const v1 = new Vector2D(1, 2);
      const v2 = new Vector2D(3, 4);
      expect(Vector2D.dot(v1, v2)).toBe(11); // 1*3 + 2*4
    });
  });

  describe('distance', () => {
    test('distance between two points', () => {
      const v1 = new Vector2D(0, 0);
      const v2 = new Vector2D(3, 4);
      expect(v1.distance(v2)).toBe(5);
    });

    test('distance squared', () => {
      const v1 = new Vector2D(0, 0);
      const v2 = new Vector2D(3, 4);
      expect(v1.distanceSquared(v2)).toBe(25);
    });

    test('distance to self is zero', () => {
      const v = new Vector2D(5, 5);
      expect(v.distance(v)).toBe(0);
    });
  });

  describe('clone', () => {
    test('clone creates independent copy', () => {
      const v1 = new Vector2D(3, 4);
      const v2 = v1.clone();
      expect(v2).not.toBe(v1);
      expect(v2.x).toBe(3);
      expect(v2.y).toBe(4);
    });

    test('modifying clone does not affect original', () => {
      const v1 = new Vector2D(3, 4);
      const v2 = v1.clone();
      v2.x = 10;
      expect(v1.x).toBe(3);
      expect(v2.x).toBe(10);
    });
  });

  describe('equality', () => {
    test('equal vectors return true', () => {
      const v1 = new Vector2D(1, 2);
      const v2 = new Vector2D(1, 2);
      expect(v1.equals(v2)).toBe(true);
    });

    test('different vectors return false', () => {
      const v1 = new Vector2D(1, 2);
      const v2 = new Vector2D(1, 3);
      expect(v1.equals(v2)).toBe(false);
    });

    test('epsilon comparison allows small differences', () => {
      const v1 = new Vector2D(1, 2);
      const v2 = new Vector2D(1.00005, 2.00005);
      expect(v1.equals(v2, 0.0001)).toBe(true);
    });
  });

  describe('angle', () => {
    test('angle of (1, 0) is 0', () => {
      const v = new Vector2D(1, 0);
      expect(v.angle()).toBeCloseTo(0, 5);
    });

    test('angle of (0, 1) is pi/2', () => {
      const v = new Vector2D(0, 1);
      expect(v.angle()).toBeCloseTo(Math.PI / 2, 5);
    });

    test('angle of (-1, 0) is pi', () => {
      const v = new Vector2D(-1, 0);
      expect(Math.abs(v.angle())).toBeCloseTo(Math.PI, 5);
    });
  });

  describe('clampMagnitude', () => {
    test('clamp reduces magnitude', () => {
      const v = new Vector2D(10, 0);
      v.clampMagnitude(5);
      expect(v.magnitude()).toBe(5);
      expect(v.x).toBe(5);
      expect(v.y).toBe(0);
    });

    test('clamp does not increase magnitude', () => {
      const v = new Vector2D(3, 4);
      v.clampMagnitude(10);
      expect(v.magnitude()).toBe(5);
    });
  });

  describe('fromAngle', () => {
    test('create vector from angle and magnitude', () => {
      const v = Vector2D.fromAngle(0, 1);
      expect(v.x).toBeCloseTo(1, 5);
      expect(v.y).toBeCloseTo(0, 5);
    });

    test('create vector with custom magnitude', () => {
      const v = Vector2D.fromAngle(0, 5);
      expect(v.magnitude()).toBeCloseTo(5, 5);
    });
  });

  describe('rotation', () => {
    test('rotate vector', () => {
      const v = new Vector2D(1, 0);
      v.rotate(Math.PI / 2);
      expect(v.x).toBeCloseTo(0, 5);
      expect(v.y).toBeCloseTo(1, 5);
    });

    test('rotate 360 degrees returns same vector', () => {
      const v = new Vector2D(3, 4);
      v.rotate(2 * Math.PI);
      expect(v.x).toBeCloseTo(3, 5);
      expect(v.y).toBeCloseTo(4, 5);
    });
  });
});
