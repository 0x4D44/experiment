import {
  PhysicsEngine,
  vectorAdd,
  vectorSubtract,
  vectorScale,
  vectorMagnitude,
  vectorDistance,
  vectorNormalize,
  createVector,
} from "./orbit-commander";

describe("Vector Operations", () => {
  test("createVector should create correct vector", () => {
    const v = createVector(3, 4);
    expect(v.x).toBe(3);
    expect(v.y).toBe(4);
  });

  test("vectorAdd should add vectors", () => {
    const a = createVector(1, 2);
    const b = createVector(3, 4);
    const result = vectorAdd(a, b);
    expect(result.x).toBe(4);
    expect(result.y).toBe(6);
  });

  test("vectorSubtract should subtract vectors", () => {
    const a = createVector(5, 10);
    const b = createVector(2, 3);
    const result = vectorSubtract(a, b);
    expect(result.x).toBe(3);
    expect(result.y).toBe(7);
  });

  test("vectorScale should scale vectors", () => {
    const v = createVector(2, 3);
    const result = vectorScale(v, 2);
    expect(result.x).toBe(4);
    expect(result.y).toBe(6);
  });

  test("vectorMagnitude of 3-4-5 triangle", () => {
    const v = createVector(3, 4);
    expect(vectorMagnitude(v)).toBe(5);
  });

  test("vectorMagnitude of zero vector", () => {
    const v = createVector(0, 0);
    expect(vectorMagnitude(v)).toBe(0);
  });

  test("vectorNormalize should normalize vectors", () => {
    const v = createVector(3, 4);
    const normalized = vectorNormalize(v);
    const mag = vectorMagnitude(normalized);
    expect(Math.abs(mag - 1) < 0.0001).toBe(true);
  });

  test("vectorNormalize zero vector", () => {
    const v = createVector(0, 0);
    const normalized = vectorNormalize(v);
    expect(normalized.x).toBe(0);
    expect(normalized.y).toBe(0);
  });

  test("vectorDistance between points", () => {
    const a = createVector(0, 0);
    const b = createVector(3, 4);
    expect(vectorDistance(a, b)).toBe(5);
  });

  test("vectorDistance is symmetric", () => {
    const a = createVector(1, 2);
    const b = createVector(4, 6);
    expect(vectorDistance(a, b)).toBe(vectorDistance(b, a));
  });
});

describe("Physics Engine", () => {
  let engine: PhysicsEngine;

  beforeEach(() => {
    engine = new PhysicsEngine();
  });

  test("calculateGravitationalForce should return positive force", () => {
    const force = engine.calculateGravitationalForce(100, 150, 100);
    expect(force).toBeGreaterThan(0);
  });

  test("calculateGravitationalForce follows inverse square law", () => {
    const force1 = engine.calculateGravitationalForce(100, 100, 100);
    const force2 = engine.calculateGravitationalForce(100, 100, 200);
    const ratio = force1 / force2;
    expect(Math.abs(ratio - 4) < 0.01).toBe(true);
  });

  test("calculateGravitationalForce returns zero for distance < 10", () => {
    const force = engine.calculateGravitationalForce(100, 150, 5);
    expect(force).toBe(0);
  });

  test("calculateGravitationalForce increases with mass", () => {
    const force1 = engine.calculateGravitationalForce(100, 100, 100);
    const force2 = engine.calculateGravitationalForce(200, 100, 100);
    expect(force2).toBeGreaterThan(force1);
  });
});

describe("Trajectory Prediction", () => {
  let engine: PhysicsEngine;

  beforeEach(() => {
    engine = new PhysicsEngine();
  });

  test("should predict trajectory with multiple points", () => {
    const sc: any = {
      position: createVector(100, 100),
      velocity: createVector(1, 0),
      fuel: 500,
      maxFuel: 500,
      radius: 3,
      thrustActive: false,
      thrustDirection: createVector(0, -1),
      thrustPower: 0,
    };

    const bodies = new Map();
    bodies.set("sun", {
      id: "sun",
      name: "Sun",
      mass: 1000,
      radius: 20,
      position: createVector(0, 0),
      velocity: createVector(0, 0),
      color: "#FFD700",
    });

    const points = engine.predictTrajectory(bodies, sc, 50, 5);
    expect(points.length).toBeGreaterThan(0);
    expect(points[0]).toBeDefined();
    expect(points[0].x).toBeDefined();
    expect(points[0].y).toBeDefined();
  });
});

describe("Integration Tests", () => {
  let engine: PhysicsEngine;

  beforeEach(() => {
    engine = new PhysicsEngine();
  });

  test("should conserve energy in stable orbit", () => {
    const sc: any = {
      position: createVector(500, 0),
      velocity: createVector(0, 4),
      fuel: 1000,
      maxFuel: 1000,
      radius: 3,
      thrustActive: false,
      thrustDirection: createVector(0, -1),
      thrustPower: 0,
    };

    const bodies = new Map();
    bodies.set("sun", {
      id: "sun",
      name: "Sun",
      mass: 1000,
      radius: 20,
      position: createVector(0, 0),
      velocity: createVector(0, 0),
      color: "#FFD700",
    });

    const state: any = {
      spacecraft: sc,
      bodies,
      timeScale: 1,
    };

    const initialVel = vectorMagnitude(sc.velocity);

    for (let i = 0; i < 500; i++) {
      engine.updatePhysics(state, 1 / 60);
    }

    const finalVel = vectorMagnitude(sc.velocity);
    expect(Math.abs(finalVel - initialVel) < 1).toBe(true);
  });

  test("should apply thrust correctly", () => {
    const sc: any = {
      position: createVector(0, 0),
      velocity: createVector(1, 0),
      fuel: 1000,
      maxFuel: 1000,
      radius: 3,
      thrustActive: true,
      thrustDirection: createVector(1, 0),
      thrustPower: 10,
    };

    const bodies = new Map();
    bodies.set("sun", {
      id: "sun",
      name: "Sun",
      mass: 1000,
      radius: 20,
      position: createVector(500, 0),
      velocity: createVector(0, 0),
      color: "#FFD700",
    });

    const state: any = {
      spacecraft: sc,
      bodies,
      timeScale: 1,
    };

    const initialVel = vectorMagnitude(sc.velocity);

    for (let i = 0; i < 60; i++) {
      engine.updatePhysics(state, 1 / 60);
    }

    const finalVel = vectorMagnitude(sc.velocity);
    expect(finalVel).toBeGreaterThan(initialVel);
  });

  test("should consume fuel when thrusting", () => {
    const sc: any = {
      position: createVector(0, 0),
      velocity: createVector(0, 0),
      fuel: 1000,
      maxFuel: 1000,
      radius: 3,
      thrustActive: true,
      thrustDirection: createVector(1, 0),
      thrustPower: 10,
    };

    const bodies = new Map();
    bodies.set("sun", {
      id: "sun",
      name: "Sun",
      mass: 1000,
      radius: 20,
      position: createVector(500, 0),
      velocity: createVector(0, 0),
      color: "#FFD700",
    });

    const state: any = {
      spacecraft: sc,
      bodies,
      timeScale: 1,
    };

    const initialFuel = sc.fuel;

    for (let i = 0; i < 60; i++) {
      engine.updatePhysics(state, 1 / 60);
    }

    expect(sc.fuel).toBeLessThan(initialFuel);
  });

  test("should not consume fuel when not thrusting", () => {
    const sc: any = {
      position: createVector(0, 0),
      velocity: createVector(1, 0),
      fuel: 1000,
      maxFuel: 1000,
      radius: 3,
      thrustActive: false,
      thrustDirection: createVector(0, 0),
      thrustPower: 0,
    };

    const bodies = new Map();
    bodies.set("sun", {
      id: "sun",
      name: "Sun",
      mass: 1000,
      radius: 20,
      position: createVector(500, 0),
      velocity: createVector(0, 0),
      color: "#FFD700",
    });

    const state: any = {
      spacecraft: sc,
      bodies,
      timeScale: 1,
    };

    const initialFuel = sc.fuel;

    for (let i = 0; i < 60; i++) {
      engine.updatePhysics(state, 1 / 60);
    }

    expect(sc.fuel).toBe(initialFuel);
  });
});
