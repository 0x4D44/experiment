/**
 * Custom Physics Engine for Physics Sandbox
 * Implements gravity, collision detection, momentum, and velocity
 */

// Vector2D utility class
class Vector2 {
    constructor(x = 0, y = 0) {
        this.x = x;
        this.y = y;
    }

    add(v) {
        return new Vector2(this.x + v.x, this.y + v.y);
    }

    subtract(v) {
        return new Vector2(this.x - v.x, this.y - v.y);
    }

    multiply(scalar) {
        return new Vector2(this.x * scalar, this.y * scalar);
    }

    divide(scalar) {
        return new Vector2(this.x / scalar, this.y / scalar);
    }

    magnitude() {
        return Math.sqrt(this.x * this.x + this.y * this.y);
    }

    normalize() {
        const mag = this.magnitude();
        return mag > 0 ? this.divide(mag) : new Vector2(0, 0);
    }

    dot(v) {
        return this.x * v.x + this.y * v.y;
    }

    distance(v) {
        return this.subtract(v).magnitude();
    }
}

// Base physics object class
class PhysicsObject {
    constructor(x, y, mass, type = 'circle') {
        this.id = Math.random().toString(36).substr(2, 9);
        this.position = new Vector2(x, y);
        this.velocity = new Vector2(0, 0);
        this.acceleration = new Vector2(0, 0);
        this.mass = mass;
        this.type = type;
        this.restitution = 0.7; // Bounciness (0-1)
        this.friction = 0.98; // Air resistance
        this.isGrabbed = false;
        this.trail = [];
        this.maxTrailLength = 30;
        this.color = this.generateColor();
    }

    generateColor() {
        const hue = Math.floor(Math.random() * 360);
        const saturation = 70 + Math.floor(Math.random() * 30);
        const lightness = 50 + Math.floor(Math.random() * 20);
        return `hsl(${hue}, ${saturation}%, ${lightness}%)`;
    }

    applyForce(force) {
        // F = ma, so a = F/m
        const acceleration = force.divide(this.mass);
        this.acceleration = this.acceleration.add(acceleration);
    }

    update(deltaTime) {
        if (this.isGrabbed) {
            this.velocity = new Vector2(0, 0);
            this.acceleration = new Vector2(0, 0);
            return;
        }

        // Update velocity with acceleration
        this.velocity = this.velocity.add(this.acceleration.multiply(deltaTime));

        // Apply friction
        this.velocity = this.velocity.multiply(this.friction);

        // Update position with velocity
        this.position = this.position.add(this.velocity.multiply(deltaTime));

        // Reset acceleration
        this.acceleration = new Vector2(0, 0);

        // Update trail
        if (this.velocity.magnitude() > 0.5) {
            this.trail.push({
                x: this.position.x,
                y: this.position.y,
                alpha: 1.0
            });
            if (this.trail.length > this.maxTrailLength) {
                this.trail.shift();
            }
        }

        // Fade trail
        this.trail = this.trail.map(point => ({
            ...point,
            alpha: point.alpha * 0.95
        })).filter(point => point.alpha > 0.05);
    }
}

// Circle object
class Circle extends PhysicsObject {
    constructor(x, y, radius, mass) {
        super(x, y, mass, 'circle');
        this.radius = radius;
    }

    containsPoint(x, y) {
        return this.position.distance(new Vector2(x, y)) <= this.radius;
    }
}

// Box object
class Box extends PhysicsObject {
    constructor(x, y, width, height, mass) {
        super(x, y, mass, 'box');
        this.width = width;
        this.height = height;
        this.angle = 0;
        this.angularVelocity = 0;
    }

    containsPoint(x, y) {
        return x >= this.position.x - this.width / 2 &&
               x <= this.position.x + this.width / 2 &&
               y >= this.position.y - this.height / 2 &&
               y <= this.position.y + this.height / 2;
    }

    update(deltaTime) {
        super.update(deltaTime);
        if (!this.isGrabbed) {
            this.angle += this.angularVelocity * deltaTime;
            this.angularVelocity *= 0.98; // Angular friction
        }
    }
}

// Main Physics Engine
class PhysicsEngine {
    constructor(width, height) {
        this.width = width;
        this.height = height;
        this.objects = [];
        this.gravity = new Vector2(0, 980); // pixels per second squared
        this.enableGravity = true;
        this.wallRestitution = 0.7;
        this.maxObjects = 500; // Prevent memory issues
    }

    addObject(obj) {
        // Limit total objects to prevent performance degradation
        if (this.objects.length >= this.maxObjects) {
            // Remove oldest object
            this.objects.shift();
        }
        this.objects.push(obj);
        return obj;
    }

    removeObject(obj) {
        const index = this.objects.indexOf(obj);
        if (index > -1) {
            this.objects.splice(index, 1);
        }
    }

    clear() {
        this.objects = [];
    }

    update(deltaTime) {
        // Apply forces
        if (this.enableGravity) {
            this.objects.forEach(obj => {
                if (!obj.isGrabbed) {
                    obj.applyForce(this.gravity.multiply(obj.mass));
                }
            });
        }

        // Update all objects
        this.objects.forEach(obj => obj.update(deltaTime));

        // Handle collisions
        this.handleCollisions();

        // Handle wall boundaries
        this.handleWallBoundaries();
    }

    handleCollisions() {
        for (let i = 0; i < this.objects.length; i++) {
            for (let j = i + 1; j < this.objects.length; j++) {
                const objA = this.objects[i];
                const objB = this.objects[j];

                if (objA.isGrabbed || objB.isGrabbed) continue;

                if (objA.type === 'circle' && objB.type === 'circle') {
                    this.handleCircleCircleCollision(objA, objB);
                } else if (objA.type === 'circle' && objB.type === 'box') {
                    this.handleCircleBoxCollision(objA, objB);
                } else if (objA.type === 'box' && objB.type === 'circle') {
                    this.handleCircleBoxCollision(objB, objA);
                } else if (objA.type === 'box' && objB.type === 'box') {
                    this.handleBoxBoxCollision(objA, objB);
                }
            }
        }
    }

    handleCircleCircleCollision(circleA, circleB) {
        const distance = circleA.position.distance(circleB.position);
        const minDistance = circleA.radius + circleB.radius;

        if (distance < minDistance) {
            // Collision detected
            // Handle exact overlap case
            const normal = distance > 0.01
                ? circleB.position.subtract(circleA.position).normalize()
                : new Vector2(1, 0); // Default separation direction

            // Separate circles
            const overlap = minDistance - distance;
            const separationA = normal.multiply(-overlap / 2);
            const separationB = normal.multiply(overlap / 2);

            circleA.position = circleA.position.add(separationA);
            circleB.position = circleB.position.add(separationB);

            // Calculate relative velocity
            const relativeVelocity = circleA.velocity.subtract(circleB.velocity);
            const velocityAlongNormal = relativeVelocity.dot(normal);

            // Don't resolve if velocities are separating
            if (velocityAlongNormal > 0) return;

            // Calculate restitution
            const restitution = Math.min(circleA.restitution, circleB.restitution);

            // Calculate impulse scalar
            const impulseScalar = -(1 + restitution) * velocityAlongNormal /
                                  (1 / circleA.mass + 1 / circleB.mass);

            // Apply impulse
            const impulse = normal.multiply(impulseScalar);
            circleA.velocity = circleA.velocity.add(impulse.divide(circleA.mass));
            circleB.velocity = circleB.velocity.subtract(impulse.divide(circleB.mass));
        }
    }

    handleCircleBoxCollision(circle, box) {
        // Simple AABB collision for boxes (no rotation for simplicity)
        const closestX = Math.max(box.position.x - box.width / 2,
                                  Math.min(circle.position.x, box.position.x + box.width / 2));
        const closestY = Math.max(box.position.y - box.height / 2,
                                  Math.min(circle.position.y, box.position.y + box.height / 2));

        const distance = circle.position.distance(new Vector2(closestX, closestY));

        if (distance < circle.radius) {
            // Collision detected
            // Handle exact overlap case
            const normal = distance > 0.01
                ? circle.position.subtract(new Vector2(closestX, closestY)).normalize()
                : new Vector2(0, -1); // Default separation direction (up)

            // Separate objects
            const overlap = circle.radius - distance;
            circle.position = circle.position.add(normal.multiply(overlap));

            // Calculate relative velocity
            const relativeVelocity = circle.velocity.subtract(box.velocity);
            const velocityAlongNormal = relativeVelocity.dot(normal);

            if (velocityAlongNormal > 0) return;

            // Calculate restitution
            const restitution = Math.min(circle.restitution, box.restitution);

            // Calculate impulse
            const impulseScalar = -(1 + restitution) * velocityAlongNormal /
                                  (1 / circle.mass + 1 / box.mass);

            const impulse = normal.multiply(impulseScalar);
            circle.velocity = circle.velocity.add(impulse.divide(circle.mass));
            box.velocity = box.velocity.subtract(impulse.divide(box.mass));

            // Add rotation to box
            box.angularVelocity += impulseScalar * 0.001;
        }
    }

    handleBoxBoxCollision(boxA, boxB) {
        // Simple AABB collision detection
        const aLeft = boxA.position.x - boxA.width / 2;
        const aRight = boxA.position.x + boxA.width / 2;
        const aTop = boxA.position.y - boxA.height / 2;
        const aBottom = boxA.position.y + boxA.height / 2;

        const bLeft = boxB.position.x - boxB.width / 2;
        const bRight = boxB.position.x + boxB.width / 2;
        const bTop = boxB.position.y - boxB.height / 2;
        const bBottom = boxB.position.y + boxB.height / 2;

        if (aLeft < bRight && aRight > bLeft && aTop < bBottom && aBottom > bTop) {
            // Collision detected
            const overlapX = Math.min(aRight - bLeft, bRight - aLeft);
            const overlapY = Math.min(aBottom - bTop, bBottom - aTop);

            let normal;
            if (overlapX < overlapY) {
                normal = new Vector2(boxA.position.x < boxB.position.x ? -1 : 1, 0);
                boxA.position.x += normal.x * overlapX / 2;
                boxB.position.x -= normal.x * overlapX / 2;
            } else {
                normal = new Vector2(0, boxA.position.y < boxB.position.y ? -1 : 1);
                boxA.position.y += normal.y * overlapY / 2;
                boxB.position.y -= normal.y * overlapY / 2;
            }

            // Calculate and apply impulse
            const relativeVelocity = boxA.velocity.subtract(boxB.velocity);
            const velocityAlongNormal = relativeVelocity.dot(normal);

            if (velocityAlongNormal > 0) return;

            const restitution = Math.min(boxA.restitution, boxB.restitution);
            const impulseScalar = -(1 + restitution) * velocityAlongNormal /
                                  (1 / boxA.mass + 1 / boxB.mass);

            const impulse = normal.multiply(impulseScalar);
            boxA.velocity = boxA.velocity.add(impulse.divide(boxA.mass));
            boxB.velocity = boxB.velocity.subtract(impulse.divide(boxB.mass));

            // Add rotation
            boxA.angularVelocity += impulseScalar * 0.001;
            boxB.angularVelocity -= impulseScalar * 0.001;
        }
    }

    handleWallBoundaries() {
        this.objects.forEach(obj => {
            if (obj.isGrabbed) return;

            if (obj.type === 'circle') {
                // Left wall
                if (obj.position.x - obj.radius < 0) {
                    obj.position.x = obj.radius;
                    obj.velocity.x *= -this.wallRestitution;
                }
                // Right wall
                if (obj.position.x + obj.radius > this.width) {
                    obj.position.x = this.width - obj.radius;
                    obj.velocity.x *= -this.wallRestitution;
                }
                // Top wall
                if (obj.position.y - obj.radius < 0) {
                    obj.position.y = obj.radius;
                    obj.velocity.y *= -this.wallRestitution;
                }
                // Bottom wall
                if (obj.position.y + obj.radius > this.height) {
                    obj.position.y = this.height - obj.radius;
                    obj.velocity.y *= -this.wallRestitution;
                    // Stop small bounces
                    if (Math.abs(obj.velocity.y) < 10) {
                        obj.velocity.y = 0;
                    }
                }
            } else if (obj.type === 'box') {
                const halfWidth = obj.width / 2;
                const halfHeight = obj.height / 2;

                // Left wall
                if (obj.position.x - halfWidth < 0) {
                    obj.position.x = halfWidth;
                    obj.velocity.x *= -this.wallRestitution;
                    obj.angularVelocity *= -0.5;
                }
                // Right wall
                if (obj.position.x + halfWidth > this.width) {
                    obj.position.x = this.width - halfWidth;
                    obj.velocity.x *= -this.wallRestitution;
                    obj.angularVelocity *= -0.5;
                }
                // Top wall
                if (obj.position.y - halfHeight < 0) {
                    obj.position.y = halfHeight;
                    obj.velocity.y *= -this.wallRestitution;
                    obj.angularVelocity *= -0.5;
                }
                // Bottom wall
                if (obj.position.y + halfHeight > this.height) {
                    obj.position.y = this.height - halfHeight;
                    obj.velocity.y *= -this.wallRestitution;
                    obj.angularVelocity *= -0.5;
                    if (Math.abs(obj.velocity.y) < 10) {
                        obj.velocity.y = 0;
                    }
                }
            }
        });
    }

    getObjectAtPoint(x, y) {
        // Check in reverse order (top object first)
        for (let i = this.objects.length - 1; i >= 0; i--) {
            if (this.objects[i].containsPoint(x, y)) {
                return this.objects[i];
            }
        }
        return null;
    }

    toggleGravity() {
        this.enableGravity = !this.enableGravity;
    }
}

// Export for use in other files and tests
if (typeof module !== 'undefined' && module.exports) {
    module.exports = { PhysicsEngine, Circle, Box, Vector2, PhysicsObject };
}
