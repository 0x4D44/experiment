/**
 * Particle Playground - Physics Tests
 * Unit tests for the physics engine and particle system
 */

import {
  Vector2D,
  Particle,
  ParticleType,
  Attractor,
  GoalZone,
  Barrier,
  Portal,
  PhysicsEngine,
} from './particle-physics';

describe('Vector2D', () => {
  test('should create vector with coordinates', () => {
    const v = new Vector2D(3, 4);
    expect(v.x).toBe(3);
    expect(v.y).toBe(4);
  });

  test('should create zero vector by default', () => {
    const v = new Vector2D();
    expect(v.x).toBe(0);
    expect(v.y).toBe(0);
  });

  test('should clone vector', () => {
    const v1 = new Vector2D(5, 6);
    const v2 = v1.clone();
    expect(v2.x).toBe(5);
    expect(v2.y).toBe(6);
    v2.x = 10;
    expect(v1.x).toBe(5); // Original should not change
  });

  test('should add vectors', () => {
    const v1 = new Vector2D(1, 2);
    const v2 = new Vector2D(3, 4);
    v1.add(v2);
    expect(v1.x).toBe(4);
    expect(v1.y).toBe(6);
  });

  test('should subtract vectors', () => {
    const v1 = new Vector2D(5, 7);
    const v2 = new Vector2D(3, 2);
    v1.subtract(v2);
    expect(v1.x).toBe(2);
    expect(v1.y).toBe(5);
  });

  test('should multiply by scalar', () => {
    const v = new Vector2D(2, 3);
    v.multiply(2);
    expect(v.x).toBe(4);
    expect(v.y).toBe(6);
  });

  test('should divide by scalar', () => {
    const v = new Vector2D(4, 6);
    v.divide(2);
    expect(v.x).toBe(2);
    expect(v.y).toBe(3);
  });

  test('should calculate distance between vectors', () => {
    const v1 = new Vector2D(0, 0);
    const v2 = new Vector2D(3, 4);
    expect(v1.distance(v2)).toBe(5);
  });

  test('should calculate magnitude', () => {
    const v = new Vector2D(3, 4);
    expect(v.magnitude()).toBe(5);
  });

  test('should normalize vector', () => {
    const v = new Vector2D(3, 4);
    v.normalize();
    expect(v.magnitude()).toBeCloseTo(1, 5);
  });

  test('should create vector from angle', () => {
    const v = Vector2D.fromAngle(0, 1);
    expect(v.x).toBeCloseTo(1, 5);
    expect(v.y).toBeCloseTo(0, 5);
  });

  test('should have static add method', () => {
    const v1 = new Vector2D(1, 2);
    const v2 = new Vector2D(3, 4);
    const v3 = Vector2D.add(v1, v2);
    expect(v3.x).toBe(4);
    expect(v3.y).toBe(6);
  });
});

describe('Particle', () => {
  test('should create particle with default values', () => {
    const pos = new Vector2D(100, 200);
    const p = new Particle(pos);
    expect(p.position.x).toBe(100);
    expect(p.position.y).toBe(200);
    expect(p.type).toBe(ParticleType.NEUTRAL);
    expect(p.mass).toBe(1);
    expect(p.charge).toBe(0);
  });

  test('should create positive particle with charge', () => {
    const pos = new Vector2D(0, 0);
    const p = new Particle(pos, Vector2D.zero(), ParticleType.POSITIVE);
    expect(p.type).toBe(ParticleType.POSITIVE);
    expect(p.charge).toBe(1);
  });

  test('should create negative particle with charge', () => {
    const pos = new Vector2D(0, 0);
    const p = new Particle(pos, Vector2D.zero(), ParticleType.NEGATIVE);
    expect(p.type).toBe(ParticleType.NEGATIVE);
    expect(p.charge).toBe(-1);
  });

  test('should apply force and update velocity', () => {
    const p = new Particle(new Vector2D(0, 0), Vector2D.zero(), ParticleType.NEUTRAL, 1);
    const force = new Vector2D(1, 0);
    p.applyForce(force);
    expect(p.acceleration.x).toBeCloseTo(1, 5);
  });

  test('should update position when velocity is applied', () => {
    const p = new Particle(new Vector2D(0, 0), new Vector2D(1, 0), ParticleType.NEUTRAL);
    p.update();
    expect(p.position.x).toBeGreaterThan(0);
  });

  test('should maintain trail', () => {
    const p = new Particle(new Vector2D(0, 0));
    const initialTrailLength = p.trail.length;
    p.update();
    expect(p.trail.length).toBe(initialTrailLength + 1);
  });

  test('should limit trail length', () => {
    const p = new Particle(new Vector2D(0, 0));
    p.maxTrailLength = 5;
    for (let i = 0; i < 20; i++) {
      p.position.add(new Vector2D(1, 0));
      p.update();
    }
    expect(p.trail.length).toBeLessThanOrEqual(p.maxTrailLength + 1);
  });

  test('should check bounds', () => {
    const p = new Particle(new Vector2D(100, 100));
    expect(p.isInBounds(800, 600)).toBe(true);

    const p2 = new Particle(new Vector2D(-100, 100));
    expect(p2.isInBounds(800, 600, 50)).toBe(false);
  });

  test('should bounce off walls', () => {
    const p = new Particle(new Vector2D(5, 300), new Vector2D(-5, 0));
    p.bounceOffWalls(800, 600);
    expect(p.velocity.x).toBeGreaterThan(0); // Should reverse
  });
});

describe('Attractor', () => {
  test('should create attractor', () => {
    const pos = new Vector2D(100, 100);
    const a = new Attractor(pos, 2, true, 150);
    expect(a.position.x).toBe(100);
    expect(a.position.y).toBe(100);
    expect(a.strength).toBe(2);
    expect(a.isAttractor).toBe(true);
    expect(a.radius).toBe(150);
  });

  test('should calculate attractive force', () => {
    const a = new Attractor(new Vector2D(100, 0), 1, true, 200);
    const p = new Particle(new Vector2D(0, 0), Vector2D.zero(), ParticleType.NEUTRAL);
    const force = a.calculateForce(p);
    expect(force.x).toBeGreaterThan(0); // Should point toward attractor
  });

  test('should calculate repulsive force', () => {
    const a = new Attractor(new Vector2D(100, 0), 1, false, 200);
    const p = new Particle(new Vector2D(0, 0), Vector2D.zero(), ParticleType.NEUTRAL);
    const force = a.calculateForce(p);
    expect(force.x).toBeLessThan(0); // Should point away from attractor
  });

  test('should not apply force outside radius', () => {
    const a = new Attractor(new Vector2D(0, 0), 1, true, 100);
    const p = new Particle(new Vector2D(500, 0));
    const force = a.calculateForce(p);
    expect(force.magnitude()).toBe(0);
  });
});

describe('GoalZone', () => {
  test('should create goal zone', () => {
    const pos = new Vector2D(400, 300);
    const g = new GoalZone(pos, 40, 2);
    expect(g.position.x).toBe(400);
    expect(g.position.y).toBe(300);
    expect(g.radius).toBe(40);
    expect(g.requiredParticles).toBe(2);
  });

  test('should check if particle is inside', () => {
    const g = new GoalZone(new Vector2D(100, 100), 50);
    const p1 = new Particle(new Vector2D(110, 110));
    const p2 = new Particle(new Vector2D(200, 200));

    expect(g.contains(p1)).toBe(true);
    expect(g.contains(p2)).toBe(false);
  });

  test('should update particles in zone', () => {
    const g = new GoalZone(new Vector2D(100, 100), 50);
    const p1 = new Particle(new Vector2D(110, 110));
    const p2 = new Particle(new Vector2D(200, 200));

    g.update([p1, p2]);
    expect(g.particlesInZone.length).toBe(1);
    expect(g.particlesInZone[0]).toBe(p1);
  });

  test('should detect goal completion', () => {
    const g = new GoalZone(new Vector2D(100, 100), 50, 2);
    const p1 = new Particle(new Vector2D(110, 110));
    const p2 = new Particle(new Vector2D(120, 120));
    const p3 = new Particle(new Vector2D(200, 200));

    g.update([p1, p2, p3]);
    expect(g.isComplete()).toBe(true);
  });
});

describe('Barrier', () => {
  test('should create barrier', () => {
    const start = new Vector2D(0, 100);
    const end = new Vector2D(200, 100);
    const b = new Barrier(start, end, 10);
    expect(b.start.x).toBe(0);
    expect(b.end.x).toBe(200);
    expect(b.thickness).toBe(10);
  });

  test('should detect collision with particle', () => {
    const b = new Barrier(new Vector2D(0, 100), new Vector2D(200, 100), 20);
    const p1 = new Particle(new Vector2D(100, 105)); // Close to barrier
    const p2 = new Particle(new Vector2D(100, 200)); // Far from barrier

    expect(b.collidesWith(p1)).toBe(true);
    expect(b.collidesWith(p2)).toBe(false);
  });

  test('should calculate distance to point', () => {
    const b = new Barrier(new Vector2D(0, 0), new Vector2D(100, 0), 10);
    const dist = b.distanceToPoint(new Vector2D(50, 10));
    expect(dist).toBeCloseTo(10, 1);
  });
});

describe('Portal', () => {
  test('should create portal', () => {
    const pos = new Vector2D(100, 100);
    const dest = new Vector2D(500, 500);
    const p = new Portal(pos, dest, 20, '#00FF00');
    expect(p.position.x).toBe(100);
    expect(p.destinationPosition.x).toBe(500);
    expect(p.radius).toBe(20);
  });

  test('should detect particle in portal', () => {
    const portal = new Portal(new Vector2D(100, 100), new Vector2D(500, 500), 20);
    const p1 = new Particle(new Vector2D(110, 110));
    const p2 = new Particle(new Vector2D(200, 200));

    expect(portal.contains(p1)).toBe(true);
    expect(portal.contains(p2)).toBe(false);
  });

  test('should teleport particle', () => {
    const portal = new Portal(new Vector2D(100, 100), new Vector2D(500, 500));
    const p = new Particle(new Vector2D(100, 100));
    portal.teleport(p);

    expect(p.position.x).toBe(500);
    expect(p.position.y).toBe(500);
  });
});

describe('PhysicsEngine', () => {
  test('should create physics engine', () => {
    const e = new PhysicsEngine(800, 600);
    expect(e.width).toBe(800);
    expect(e.height).toBe(600);
    expect(e.particles.length).toBe(0);
  });

  test('should add particle', () => {
    const e = new PhysicsEngine(800, 600);
    const p = new Particle(new Vector2D(100, 100));
    e.addParticle(p);
    expect(e.particles.length).toBe(1);
    expect(e.particles[0]).toBe(p);
  });

  test('should add attractor', () => {
    const e = new PhysicsEngine(800, 600);
    const a = new Attractor(new Vector2D(100, 100));
    e.addAttractor(a);
    expect(e.attractors.length).toBe(1);
  });

  test('should clear particles', () => {
    const e = new PhysicsEngine(800, 600);
    e.addParticle(new Particle(new Vector2D(100, 100)));
    e.addParticle(new Particle(new Vector2D(200, 200)));
    expect(e.particles.length).toBe(2);

    e.clearParticles();
    expect(e.particles.length).toBe(0);
  });

  test('should set gravity', () => {
    const e = new PhysicsEngine(800, 600);
    e.setGravity(0, 0.1);
    expect(e.gravity.y).toBe(0.1);
  });

  test('should toggle pause', () => {
    const e = new PhysicsEngine(800, 600);
    expect(e.isPaused).toBe(false);
    e.togglePause();
    expect(e.isPaused).toBe(true);
  });

  test('should not update when paused', () => {
    const e = new PhysicsEngine(800, 600);
    const p = new Particle(new Vector2D(100, 100), new Vector2D(5, 0));
    e.addParticle(p);
    e.togglePause();

    const initialX = p.position.x;
    e.update();
    expect(p.position.x).toBe(initialX); // Should not move
  });

});
