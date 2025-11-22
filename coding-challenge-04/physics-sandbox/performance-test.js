/**
 * Performance Test for Physics Engine
 * Tests FPS with various object counts
 */

const { PhysicsEngine, Circle, Box } = require('./physics-engine.js');

class PerformanceTest {
    constructor() {
        this.results = [];
    }

    testPhysicsPerformance(objectCount, iterations = 100) {
        console.log(`\nTesting with ${objectCount} objects...`);

        const engine = new PhysicsEngine(1920, 1080);

        // Spawn objects
        for (let i = 0; i < objectCount; i++) {
            const x = Math.random() * 1920;
            const y = Math.random() * 1080;
            const radius = 20 + Math.random() * 20;

            if (Math.random() > 0.5) {
                const circle = new Circle(x, y, radius, 1);
                circle.velocity.x = (Math.random() - 0.5) * 200;
                circle.velocity.y = (Math.random() - 0.5) * 200;
                engine.addObject(circle);
            } else {
                const box = new Box(x, y, radius * 2, radius * 2, 1.5);
                box.velocity.x = (Math.random() - 0.5) * 200;
                box.velocity.y = (Math.random() - 0.5) * 200;
                engine.addObject(box);
            }
        }

        // Run physics updates and measure performance
        const deltaTime = 1/60; // 60 FPS target
        const startTime = Date.now();

        for (let i = 0; i < iterations; i++) {
            engine.update(deltaTime);
        }

        const endTime = Date.now();
        const totalTime = endTime - startTime;
        const avgTimePerFrame = totalTime / iterations;
        const estimatedFPS = Math.floor(1000 / avgTimePerFrame);

        const result = {
            objectCount,
            iterations,
            totalTime,
            avgTimePerFrame: avgTimePerFrame.toFixed(2),
            estimatedFPS
        };

        this.results.push(result);

        console.log(`  Total time: ${totalTime}ms`);
        console.log(`  Avg time per frame: ${avgTimePerFrame.toFixed(2)}ms`);
        console.log(`  Estimated FPS: ${estimatedFPS}`);
        console.log(`  Status: ${estimatedFPS >= 60 ? '✓ PASS' : estimatedFPS >= 30 ? '⚠ ACCEPTABLE' : '✗ NEEDS OPTIMIZATION'}`);

        return result;
    }

    testCollisionAccuracy() {
        console.log('\nTesting collision accuracy...');

        const engine = new PhysicsEngine(1000, 1000);

        // Create two circles that should collide
        const circle1 = new Circle(500, 500, 30, 1);
        const circle2 = new Circle(550, 500, 30, 1);

        circle1.velocity.x = 100;
        circle2.velocity.x = -100;

        engine.addObject(circle1);
        engine.addObject(circle2);

        // Update physics
        for (let i = 0; i < 10; i++) {
            engine.update(1/60);
        }

        // Check if circles separated after collision
        const distance = circle1.position.distance(circle2.position);
        const minDistance = circle1.radius + circle2.radius;

        if (distance >= minDistance * 0.99) {
            console.log('  ✓ Collision detection and separation working correctly');
            return true;
        } else {
            console.log(`  ✗ Circles overlapping (distance: ${distance.toFixed(2)}, min: ${minDistance})`);
            return false;
        }
    }

    testMemoryLimit() {
        console.log('\nTesting memory limit protection...');

        const engine = new PhysicsEngine(1000, 1000);

        // Try to add more than max objects
        for (let i = 0; i < 600; i++) {
            const circle = new Circle(Math.random() * 1000, Math.random() * 1000, 20, 1);
            engine.addObject(circle);
        }

        if (engine.objects.length === engine.maxObjects) {
            console.log(`  ✓ Memory limit working (${engine.objects.length}/${engine.maxObjects} objects)`);
            return true;
        } else {
            console.log(`  ✗ Memory limit not enforced (${engine.objects.length}/${engine.maxObjects} objects)`);
            return false;
        }
    }

    testBoundaryCollisions() {
        console.log('\nTesting boundary collisions...');

        const engine = new PhysicsEngine(1000, 1000);
        const circle = new Circle(50, 500, 30, 1);
        circle.velocity.x = -500; // Moving fast toward left wall

        engine.addObject(circle);
        engine.update(1/60);
        engine.handleWallBoundaries();

        if (circle.position.x >= circle.radius) {
            console.log('  ✓ Wall boundary working correctly');
            return true;
        } else {
            console.log(`  ✗ Object escaped boundary (x: ${circle.position.x}, min: ${circle.radius})`);
            return false;
        }
    }

    runAllTests() {
        console.log('='.repeat(60));
        console.log('PHYSICS ENGINE PERFORMANCE TEST');
        console.log('='.repeat(60));

        // Performance tests with different object counts
        this.testPhysicsPerformance(10, 100);
        this.testPhysicsPerformance(50, 100);
        this.testPhysicsPerformance(100, 100);
        this.testPhysicsPerformance(200, 100);
        this.testPhysicsPerformance(300, 50);
        this.testPhysicsPerformance(500, 50);

        // Functional tests
        const collisionPass = this.testCollisionAccuracy();
        const memoryPass = this.testMemoryLimit();
        const boundaryPass = this.testBoundaryCollisions();

        // Summary
        console.log('\n' + '='.repeat(60));
        console.log('PERFORMANCE SUMMARY');
        console.log('='.repeat(60));

        this.results.forEach(result => {
            const status = result.estimatedFPS >= 60 ? '✓' : result.estimatedFPS >= 30 ? '⚠' : '✗';
            console.log(`${status} ${result.objectCount.toString().padStart(3)} objects: ${result.estimatedFPS} FPS (${result.avgTimePerFrame}ms/frame)`);
        });

        console.log('\nFUNCTIONAL TESTS');
        console.log('='.repeat(60));
        console.log(`${collisionPass ? '✓' : '✗'} Collision Detection`);
        console.log(`${memoryPass ? '✓' : '✗'} Memory Limit Protection`);
        console.log(`${boundaryPass ? '✓' : '✗'} Boundary Collisions`);

        // Overall assessment
        const allFunctionalPass = collisionPass && memoryPass && boundaryPass;
        const performanceGood = this.results.every(r => r.estimatedFPS >= 30);

        console.log('\n' + '='.repeat(60));
        if (allFunctionalPass && performanceGood) {
            console.log('OVERALL: ✓ READY FOR COMPETITION');
        } else if (allFunctionalPass) {
            console.log('OVERALL: ⚠ FUNCTIONAL BUT NEEDS PERFORMANCE TUNING');
        } else {
            console.log('OVERALL: ✗ NEEDS FIXES');
        }
        console.log('='.repeat(60));
    }
}

// Run tests
const tester = new PerformanceTest();
tester.runAllTests();
