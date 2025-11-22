/**
 * Physics Engine Tests
 * Tests for vector math, collision detection, and physics calculations
 */

// Simple test framework
class TestRunner {
    constructor() {
        this.tests = [];
        this.passed = 0;
        this.failed = 0;
    }

    test(name, fn) {
        this.tests.push({ name, fn });
    }

    assertEquals(actual, expected, message = '') {
        if (actual !== expected) {
            throw new Error(`Expected ${expected} but got ${actual}. ${message}`);
        }
    }

    assertApproxEquals(actual, expected, epsilon = 0.001, message = '') {
        if (Math.abs(actual - expected) > epsilon) {
            throw new Error(`Expected ${expected} but got ${actual} (epsilon: ${epsilon}). ${message}`);
        }
    }

    assertTrue(condition, message = '') {
        if (!condition) {
            throw new Error(`Expected true but got false. ${message}`);
        }
    }

    assertFalse(condition, message = '') {
        if (condition) {
            throw new Error(`Expected false but got true. ${message}`);
        }
    }

    run() {
        console.log('Running Physics Engine Tests...\n');

        this.tests.forEach(test => {
            try {
                test.fn();
                this.passed++;
                console.log(`✓ ${test.name}`);
            } catch (error) {
                this.failed++;
                console.error(`✗ ${test.name}`);
                console.error(`  ${error.message}\n`);
            }
        });

        console.log(`\n${'='.repeat(50)}`);
        console.log(`Tests passed: ${this.passed}`);
        console.log(`Tests failed: ${this.failed}`);
        console.log(`Total tests: ${this.tests.length}`);
        console.log('='.repeat(50));

        return this.failed === 0;
    }
}

// Load the physics engine
const { PhysicsEngine, Circle, Box, Vector2, PhysicsObject } = require('./physics-engine.js');

// Create test runner
const runner = new TestRunner();

// Vector2 Tests
runner.test('Vector2: Constructor creates vector with correct x and y', () => {
    const v = new Vector2(3, 4);
    runner.assertEquals(v.x, 3);
    runner.assertEquals(v.y, 4);
});

runner.test('Vector2: Default constructor creates zero vector', () => {
    const v = new Vector2();
    runner.assertEquals(v.x, 0);
    runner.assertEquals(v.y, 0);
});

runner.test('Vector2: Add returns correct sum', () => {
    const v1 = new Vector2(3, 4);
    const v2 = new Vector2(1, 2);
    const result = v1.add(v2);
    runner.assertEquals(result.x, 4);
    runner.assertEquals(result.y, 6);
});

runner.test('Vector2: Subtract returns correct difference', () => {
    const v1 = new Vector2(3, 4);
    const v2 = new Vector2(1, 2);
    const result = v1.subtract(v2);
    runner.assertEquals(result.x, 2);
    runner.assertEquals(result.y, 2);
});

runner.test('Vector2: Multiply returns correct scaled vector', () => {
    const v = new Vector2(3, 4);
    const result = v.multiply(2);
    runner.assertEquals(result.x, 6);
    runner.assertEquals(result.y, 8);
});

runner.test('Vector2: Divide returns correct scaled vector', () => {
    const v = new Vector2(6, 8);
    const result = v.divide(2);
    runner.assertEquals(result.x, 3);
    runner.assertEquals(result.y, 4);
});

runner.test('Vector2: Magnitude returns correct length', () => {
    const v = new Vector2(3, 4);
    const magnitude = v.magnitude();
    runner.assertEquals(magnitude, 5); // 3-4-5 triangle
});

runner.test('Vector2: Normalize returns unit vector', () => {
    const v = new Vector2(3, 4);
    const normalized = v.normalize();
    runner.assertApproxEquals(normalized.magnitude(), 1.0);
});

runner.test('Vector2: Dot product returns correct value', () => {
    const v1 = new Vector2(3, 4);
    const v2 = new Vector2(2, 1);
    const dot = v1.dot(v2);
    runner.assertEquals(dot, 10); // 3*2 + 4*1 = 10
});

runner.test('Vector2: Distance returns correct distance', () => {
    const v1 = new Vector2(0, 0);
    const v2 = new Vector2(3, 4);
    const distance = v1.distance(v2);
    runner.assertEquals(distance, 5);
});

// PhysicsObject Tests
runner.test('PhysicsObject: Constructor initializes correctly', () => {
    const obj = new PhysicsObject(100, 200, 5, 'circle');
    runner.assertEquals(obj.position.x, 100);
    runner.assertEquals(obj.position.y, 200);
    runner.assertEquals(obj.mass, 5);
    runner.assertEquals(obj.type, 'circle');
});

runner.test('PhysicsObject: Apply force updates acceleration', () => {
    const obj = new PhysicsObject(0, 0, 2, 'circle');
    const force = new Vector2(10, 20);
    obj.applyForce(force);
    runner.assertEquals(obj.acceleration.x, 5); // F=ma, a=F/m = 10/2 = 5
    runner.assertEquals(obj.acceleration.y, 10); // 20/2 = 10
});

runner.test('PhysicsObject: Update applies velocity and acceleration', () => {
    const obj = new PhysicsObject(0, 0, 1, 'circle');
    obj.velocity = new Vector2(10, 20);
    obj.update(1); // 1 second delta
    runner.assertApproxEquals(obj.position.x, 10 * 0.98, 0.1); // With friction
    runner.assertApproxEquals(obj.position.y, 20 * 0.98, 0.1);
});

runner.test('PhysicsObject: Grabbed objects do not move', () => {
    const obj = new PhysicsObject(100, 100, 1, 'circle');
    obj.velocity = new Vector2(50, 50);
    obj.isGrabbed = true;
    obj.update(1);
    runner.assertEquals(obj.position.x, 100);
    runner.assertEquals(obj.position.y, 100);
});

// Circle Tests
runner.test('Circle: Constructor sets radius correctly', () => {
    const circle = new Circle(100, 100, 25, 1);
    runner.assertEquals(circle.radius, 25);
    runner.assertEquals(circle.type, 'circle');
});

runner.test('Circle: ContainsPoint detects point inside circle', () => {
    const circle = new Circle(100, 100, 25, 1);
    runner.assertTrue(circle.containsPoint(100, 100));
    runner.assertTrue(circle.containsPoint(110, 110));
});

runner.test('Circle: ContainsPoint detects point outside circle', () => {
    const circle = new Circle(100, 100, 25, 1);
    runner.assertFalse(circle.containsPoint(200, 200));
});

// Box Tests
runner.test('Box: Constructor sets dimensions correctly', () => {
    const box = new Box(100, 100, 50, 40, 1);
    runner.assertEquals(box.width, 50);
    runner.assertEquals(box.height, 40);
    runner.assertEquals(box.type, 'box');
});

runner.test('Box: ContainsPoint detects point inside box', () => {
    const box = new Box(100, 100, 50, 40, 1);
    runner.assertTrue(box.containsPoint(100, 100));
    runner.assertTrue(box.containsPoint(110, 110));
});

runner.test('Box: ContainsPoint detects point outside box', () => {
    const box = new Box(100, 100, 50, 40, 1);
    runner.assertFalse(box.containsPoint(200, 200));
});

runner.test('Box: Update applies angular velocity', () => {
    const box = new Box(100, 100, 50, 40, 1);
    box.angularVelocity = 1;
    const initialAngle = box.angle;
    box.update(1);
    runner.assertTrue(box.angle !== initialAngle);
});

// PhysicsEngine Tests
runner.test('PhysicsEngine: Constructor initializes with correct dimensions', () => {
    const engine = new PhysicsEngine(800, 600);
    runner.assertEquals(engine.width, 800);
    runner.assertEquals(engine.height, 600);
    runner.assertEquals(engine.objects.length, 0);
});

runner.test('PhysicsEngine: AddObject adds object to list', () => {
    const engine = new PhysicsEngine(800, 600);
    const circle = new Circle(100, 100, 25, 1);
    engine.addObject(circle);
    runner.assertEquals(engine.objects.length, 1);
    runner.assertEquals(engine.objects[0], circle);
});

runner.test('PhysicsEngine: RemoveObject removes object from list', () => {
    const engine = new PhysicsEngine(800, 600);
    const circle = new Circle(100, 100, 25, 1);
    engine.addObject(circle);
    engine.removeObject(circle);
    runner.assertEquals(engine.objects.length, 0);
});

runner.test('PhysicsEngine: Clear removes all objects', () => {
    const engine = new PhysicsEngine(800, 600);
    engine.addObject(new Circle(100, 100, 25, 1));
    engine.addObject(new Circle(200, 200, 25, 1));
    engine.clear();
    runner.assertEquals(engine.objects.length, 0);
});

runner.test('PhysicsEngine: ToggleGravity changes gravity state', () => {
    const engine = new PhysicsEngine(800, 600);
    const initialState = engine.enableGravity;
    engine.toggleGravity();
    runner.assertEquals(engine.enableGravity, !initialState);
});

runner.test('PhysicsEngine: Gravity applies force to objects', () => {
    const engine = new PhysicsEngine(800, 600);
    const circle = new Circle(100, 100, 25, 1);
    engine.addObject(circle);

    const initialY = circle.position.y;

    // Run several update cycles
    for (let i = 0; i < 10; i++) {
        engine.update(0.016); // ~60fps
    }

    // Object should have moved down due to gravity
    runner.assertTrue(circle.position.y > initialY);
});

runner.test('PhysicsEngine: Wall boundaries prevent objects from leaving canvas (bottom)', () => {
    const engine = new PhysicsEngine(800, 600);
    const circle = new Circle(400, 590, 25, 1);
    circle.velocity = new Vector2(0, 100); // Moving down fast

    engine.addObject(circle);
    engine.update(1); // Large time step

    // Circle should be constrained within canvas
    runner.assertTrue(circle.position.y + circle.radius <= 600);
});

runner.test('PhysicsEngine: Wall boundaries prevent objects from leaving canvas (right)', () => {
    const engine = new PhysicsEngine(800, 600);
    const circle = new Circle(790, 300, 25, 1);
    circle.velocity = new Vector2(100, 0); // Moving right fast

    engine.addObject(circle);
    engine.update(1);

    // Circle should be constrained within canvas
    runner.assertTrue(circle.position.x + circle.radius <= 800);
});

runner.test('PhysicsEngine: GetObjectAtPoint returns correct object', () => {
    const engine = new PhysicsEngine(800, 600);
    const circle1 = new Circle(100, 100, 25, 1);
    const circle2 = new Circle(200, 200, 25, 1);

    engine.addObject(circle1);
    engine.addObject(circle2);

    const found = engine.getObjectAtPoint(105, 105);
    runner.assertEquals(found, circle1);
});

runner.test('PhysicsEngine: GetObjectAtPoint returns null when no object', () => {
    const engine = new PhysicsEngine(800, 600);
    const circle = new Circle(100, 100, 25, 1);
    engine.addObject(circle);

    const found = engine.getObjectAtPoint(500, 500);
    runner.assertEquals(found, null);
});

runner.test('PhysicsEngine: Collision detection for two circles', () => {
    const engine = new PhysicsEngine(800, 600);

    const circle1 = new Circle(100, 100, 25, 1);
    const circle2 = new Circle(140, 100, 25, 1); // Overlapping

    circle1.velocity = new Vector2(10, 0);
    circle2.velocity = new Vector2(-10, 0);

    engine.addObject(circle1);
    engine.addObject(circle2);

    engine.handleCollisions();

    // After collision, circles should separate
    const distance = circle1.position.distance(circle2.position);
    runner.assertTrue(distance >= circle1.radius + circle2.radius - 1); // Allow small epsilon
});

runner.test('PhysicsEngine: Velocity reversal after wall collision', () => {
    const engine = new PhysicsEngine(800, 600);
    const circle = new Circle(25, 300, 25, 1);
    circle.velocity = new Vector2(-100, 0); // Moving left toward wall

    engine.addObject(circle);
    engine.update(0.1); // Small time step to trigger wall collision

    // Velocity should reverse (with restitution)
    runner.assertTrue(circle.velocity.x > 0);
});

runner.test('PhysicsEngine: Paused objects (grabbed) are not affected by gravity', () => {
    const engine = new PhysicsEngine(800, 600);
    const circle = new Circle(100, 100, 25, 1);
    circle.isGrabbed = true;

    engine.addObject(circle);

    const initialY = circle.position.y;

    // Run several update cycles
    for (let i = 0; i < 10; i++) {
        engine.update(0.016);
    }

    // Grabbed object should not move
    runner.assertEquals(circle.position.y, initialY);
});

runner.test('PhysicsEngine: Conservation of momentum in collision', () => {
    const engine = new PhysicsEngine(800, 600);

    const circle1 = new Circle(100, 100, 30, 1);
    const circle2 = new Circle(170, 100, 30, 1);

    circle1.velocity = new Vector2(100, 0);
    circle2.velocity = new Vector2(0, 0);

    engine.addObject(circle1);
    engine.addObject(circle2);

    // Calculate initial momentum
    const initialMomentum = circle1.velocity.x * circle1.mass + circle2.velocity.x * circle2.mass;

    engine.handleCollisions();

    // Calculate final momentum
    const finalMomentum = circle1.velocity.x * circle1.mass + circle2.velocity.x * circle2.mass;

    // Momentum should be approximately conserved (allowing for restitution)
    runner.assertApproxEquals(Math.abs(initialMomentum), Math.abs(finalMomentum), 50);
});

runner.test('PhysicsEngine: Objects with no gravity enabled float', () => {
    const engine = new PhysicsEngine(800, 600);
    engine.enableGravity = false;

    const circle = new Circle(100, 100, 25, 1);
    engine.addObject(circle);

    const initialY = circle.position.y;

    // Run several update cycles
    for (let i = 0; i < 10; i++) {
        engine.update(0.016);
    }

    // Without velocity and gravity, object should stay in place (accounting for friction)
    runner.assertApproxEquals(circle.position.y, initialY, 1);
});

// Run all tests
const success = runner.run();

// Exit with appropriate code for CI/CD
if (typeof process !== 'undefined') {
    process.exit(success ? 0 : 1);
}
