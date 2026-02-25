"use strict";
/**
 * Orbit Commander - A space navigation game with realistic orbital mechanics
 * TypeScript implementation
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.PhysicsEngine = exports.OrbitCommander = void 0;
exports.vectorAdd = vectorAdd;
exports.vectorSubtract = vectorSubtract;
exports.vectorScale = vectorScale;
exports.vectorMagnitude = vectorMagnitude;
exports.vectorDistance = vectorDistance;
exports.vectorNormalize = vectorNormalize;
exports.createVector = createVector;
// ============================================================================
// Constants
// ============================================================================
const PHYSICS_SCALE = 1.0;
const GRAVITATIONAL_CONSTANT = 5.0;
const VISUAL_SCALE = 0.1; // 1 physics unit = 0.1 pixels for display
const CANVAS_WIDTH = 800;
const CANVAS_HEIGHT = 600;
const FPS = 60;
const DT = 1 / FPS;
const TRAJECTORY_STEPS = 100;
const TRAJECTORY_INTERVAL = 5;
// ============================================================================
// Utility Functions
// ============================================================================
function createVector(x, y) {
    return { x, y };
}
function vectorAdd(a, b) {
    return { x: a.x + b.x, y: a.y + b.y };
}
function vectorSubtract(a, b) {
    return { x: a.x - b.x, y: a.y - b.y };
}
function vectorScale(v, scalar) {
    return { x: v.x * scalar, y: v.y * scalar };
}
function vectorMagnitude(v) {
    return Math.sqrt(v.x * v.x + v.y * v.y);
}
function vectorNormalize(v) {
    const mag = vectorMagnitude(v);
    if (mag === 0)
        return { x: 0, y: 0 };
    return { x: v.x / mag, y: v.y / mag };
}
function vectorDistance(a, b) {
    return vectorMagnitude(vectorSubtract(a, b));
}
// ============================================================================
// Physics Engine
// ============================================================================
class PhysicsEngine {
    constructor() {
        this.gravitationalConstant = GRAVITATIONAL_CONSTANT;
    }
    calculateGravitationalForce(mass1, mass2, distance) {
        if (distance < 10)
            return 0; // Prevent infinite forces
        return (this.gravitationalConstant * mass1 * mass2) / (distance * distance);
    }
    calculateAcceleration(bodies, spacecraft) {
        let acceleration = createVector(0, 0);
        for (const body of bodies.values()) {
            const direction = vectorSubtract(body.position, spacecraft.position);
            const distance = vectorMagnitude(direction);
            if (distance > 5) {
                // Don't calculate if too close
                const force = this.calculateGravitationalForce(body.mass, 1, // spacecraft mass = 1
                distance);
                const directionNorm = vectorNormalize(direction);
                const forceVector = vectorScale(directionNorm, force);
                acceleration = vectorAdd(acceleration, forceVector);
            }
        }
        // Add thrust
        if (spacecraft.thrustActive && spacecraft.fuel > 0) {
            const thrustAccel = vectorScale(spacecraft.thrustDirection, spacecraft.thrustPower);
            acceleration = vectorAdd(acceleration, thrustAccel);
        }
        return acceleration;
    }
    updatePhysics(state, deltaTime) {
        const dt = deltaTime * state.timeScale;
        const spacecraft = state.spacecraft;
        const bodies = state.bodies;
        // Calculate acceleration
        const acceleration = this.calculateAcceleration(bodies, spacecraft);
        // Update velocity
        spacecraft.velocity = vectorAdd(spacecraft.velocity, vectorScale(acceleration, dt));
        // Update position
        spacecraft.position = vectorAdd(spacecraft.position, vectorScale(spacecraft.velocity, dt));
        // Update fuel
        if (spacecraft.thrustActive && spacecraft.fuel > 0) {
            spacecraft.fuel -= spacecraft.thrustPower * dt * 2; // Fuel consumption
            if (spacecraft.fuel < 0)
                spacecraft.fuel = 0;
        }
        // Update orbital bodies
        for (const body of bodies.values()) {
            if (body.orbitParent && bodies.has(body.orbitParent)) {
                const parent = bodies.get(body.orbitParent);
                if (body.orbitAngle !== undefined && body.orbitSpeed !== undefined && body.orbitDistance !== undefined) {
                    body.orbitAngle += body.orbitSpeed * dt;
                    body.position = vectorAdd(parent.position, createVector(Math.cos(body.orbitAngle) * body.orbitDistance, Math.sin(body.orbitAngle) * body.orbitDistance));
                }
            }
        }
        state.time += dt;
    }
    predictTrajectory(state, steps, interval) {
        const points = [];
        let tempSpacecraft = {
            position: { ...state.spacecraft.position },
            velocity: { ...state.spacecraft.velocity },
            fuel: state.spacecraft.fuel,
            maxFuel: state.spacecraft.maxFuel,
            radius: state.spacecraft.radius,
            thrustActive: false,
            thrustDirection: { ...state.spacecraft.thrustDirection },
            thrustPower: state.spacecraft.thrustPower,
        };
        for (let i = 0; i < steps; i++) {
            if (i % interval === 0) {
                points.push({ ...tempSpacecraft.position });
            }
            const acceleration = this.calculateAcceleration(state, tempSpacecraft);
            tempSpacecraft.velocity = vectorAdd(tempSpacecraft.velocity, vectorScale(acceleration, DT));
            tempSpacecraft.position = vectorAdd(tempSpacecraft.position, vectorScale(tempSpacecraft.velocity, DT));
        }
        return points;
    }
}
exports.PhysicsEngine = PhysicsEngine;
// ============================================================================
// Game Logic
// ============================================================================
class OrbitCommander {
    constructor(canvasId) {
        this.gameRunning = false;
        this.gameLoop = () => {
            // Update physics
            this.physicsEngine.updatePhysics(this.gameState, DT);
            // Update trajectory if in flight
            if (!this.gameState.launchPhase) {
                this.updateTrajectory();
            }
            // Check mission complete
            if (!this.gameState.launchPhase && this.checkMissionComplete()) {
                this.completeMission();
            }
            // Check game over
            if (this.checkGameOver()) {
                this.gameOver();
            }
            // Render
            this.render();
            if (this.gameRunning) {
                requestAnimationFrame(this.gameLoop);
            }
        };
        this.canvas = document.getElementById(canvasId);
        this.ctx = this.canvas.getContext("2d");
        this.physicsEngine = new PhysicsEngine();
        this.missions = this.createMissions();
        this.gameState = this.initializeGameState();
    }
    createMissions() {
        return [
            {
                id: 1,
                name: "Mars Bound",
                description: "Reach Mars from Earth orbit",
                objective: "Get within 200 units of Mars",
                targetBody: "mars",
                fuelLimit: 500,
                difficultyMultiplier: 1,
                completed: false,
            },
            {
                id: 2,
                name: "Venus Run",
                description: "Reach Venus with fuel efficiency",
                objective: "Get within 200 units of Venus with minimum fuel loss",
                targetBody: "venus",
                fuelLimit: 400,
                difficultyMultiplier: 1.2,
                completed: false,
            },
            {
                id: 3,
                name: "Lunar Deployment",
                description: "Deploy satellite to lunar orbit",
                objective: "Achieve stable orbit around the Moon",
                targetBody: "moon",
                fuelLimit: 350,
                difficultyMultiplier: 1.3,
                completed: false,
            },
            {
                id: 4,
                name: "Gravity Assist Maneuver",
                description: "Use gravity assist to reach outer system",
                objective: "Reach Jupiter area using gravity assists",
                targetBody: "jupiter",
                fuelLimit: 600,
                difficultyMultiplier: 1.5,
                completed: false,
            },
            {
                id: 5,
                name: "Mercury Challenge",
                description: "Navigate to the closest planet to the Sun",
                objective: "Get within 150 units of Mercury",
                targetBody: "mercury",
                fuelLimit: 450,
                difficultyMultiplier: 1.6,
                completed: false,
            },
            {
                id: 6,
                name: "Fuel Conservation",
                description: "Complete mission with minimal fuel",
                objective: "Reach any target planet with > 200 fuel remaining",
                targetBody: "any",
                fuelLimit: 300,
                difficultyMultiplier: 1.8,
                completed: false,
            },
            {
                id: 7,
                name: "Asteroid Strike",
                description: "Intercept a moving asteroid",
                objective: "Get within 100 units of asteroid",
                targetBody: "asteroid",
                fuelLimit: 400,
                difficultyMultiplier: 2.0,
                completed: false,
            },
            {
                id: 8,
                name: "Dual Rendezvous",
                description: "Visit two planets in one mission",
                objective: "Reach Mars, then Venus",
                targetBody: "mars_venus",
                fuelLimit: 550,
                difficultyMultiplier: 2.2,
                completed: false,
            },
            {
                id: 9,
                name: "Grand Tour",
                description: "Visit multiple planets in sequence",
                objective: "Visit Mercury, Venus, Mars, and return",
                targetBody: "grand_tour",
                fuelLimit: 800,
                difficultyMultiplier: 2.5,
                completed: false,
            },
            {
                id: 10,
                name: "Impossible Challenge",
                description: "Navigate through the asteroid belt",
                objective: "Safely traverse the asteroid belt",
                targetBody: "asteroid_field",
                fuelLimit: 250,
                difficultyMultiplier: 3.0,
                completed: false,
            },
        ];
    }
    initializeGameState() {
        const bodies = new Map();
        // Sun (center)
        bodies.set("sun", {
            id: "sun",
            name: "Sun",
            mass: 1000,
            radius: 20,
            position: { x: 0, y: 0 },
            velocity: { x: 0, y: 0 },
            color: "#FFD700",
        });
        // Mercury
        bodies.set("mercury", {
            id: "mercury",
            name: "Mercury",
            mass: 50,
            radius: 8,
            position: { x: 300, y: 0 },
            velocity: { x: 0, y: -8 },
            color: "#8C7853",
        });
        // Venus
        bodies.set("venus", {
            id: "venus",
            name: "Venus",
            mass: 100,
            radius: 12,
            position: { x: 500, y: 0 },
            velocity: { x: 0, y: -6 },
            color: "#FFC649",
        });
        // Earth (starting position)
        bodies.set("earth", {
            id: "earth",
            name: "Earth",
            mass: 150,
            radius: 13,
            position: { x: 700, y: 0 },
            velocity: { x: 0, y: -4.5 },
            color: "#4169E1",
        });
        // Moon (orbits Earth)
        bodies.set("moon", {
            id: "moon",
            name: "Moon",
            mass: 10,
            radius: 5,
            position: { x: 730, y: 0 },
            velocity: { x: 0, y: -4.5 },
            color: "#A9A9A9",
            orbitParent: "earth",
            orbitDistance: 30,
            orbitSpeed: 0.05,
            orbitAngle: 0,
        });
        // Mars
        bodies.set("mars", {
            id: "mars",
            name: "Mars",
            mass: 75,
            radius: 10,
            position: { x: 1000, y: 0 },
            velocity: { x: 0, y: -3.5 },
            color: "#CD5C5C",
        });
        // Jupiter (far outer system)
        bodies.set("jupiter", {
            id: "jupiter",
            name: "Jupiter",
            mass: 300,
            radius: 25,
            position: { x: 1600, y: 200 },
            velocity: { x: 0, y: -2 },
            color: "#DAA520",
        });
        // Asteroid
        bodies.set("asteroid", {
            id: "asteroid",
            name: "Asteroid",
            mass: 5,
            radius: 3,
            position: { x: 800, y: 400 },
            velocity: { x: -1, y: -2 },
            color: "#696969",
        });
        const spacecraft = {
            position: { x: 700, y: 0 },
            velocity: { x: 0, y: -4.5 },
            fuel: 500,
            maxFuel: 500,
            radius: 3,
            thrustActive: false,
            thrustDirection: { x: 0, y: -1 },
            thrustPower: 0,
        };
        const mission = this.missions[0];
        return {
            spacecraft,
            bodies,
            currentMission: mission,
            score: 0,
            totalScore: 0,
            missionIndex: 0,
            time: 0,
            timeScale: 1,
            trajectoryPoints: [],
            gameRunning: false,
            launchPhase: true,
            launchAngle: 270,
            launchVelocity: 0,
        };
    }
    start() {
        this.gameRunning = true;
        this.gameState.launchPhase = true;
        this.setupEventListeners();
        this.gameLoop();
    }
    setupEventListeners() {
        window.addEventListener("keydown", (e) => this.handleKeyDown(e));
        window.addEventListener("keyup", (e) => this.handleKeyUp(e));
    }
    handleKeyDown(event) {
        const sc = this.gameState.spacecraft;
        const launchPhase = this.gameState.launchPhase;
        if (launchPhase) {
            // Launch phase controls
            if (event.key === "ArrowUp" || event.key === "w") {
                this.gameState.launchVelocity = Math.min(this.gameState.launchVelocity + 2, 100);
            }
            else if (event.key === "ArrowDown" || event.key === "s") {
                this.gameState.launchVelocity = Math.max(this.gameState.launchVelocity - 2, 0);
            }
            else if (event.key === "ArrowLeft" || event.key === "a") {
                this.gameState.launchAngle = (this.gameState.launchAngle - 5) % 360;
            }
            else if (event.key === "ArrowRight" || event.key === "d") {
                this.gameState.launchAngle = (this.gameState.launchAngle + 5) % 360;
            }
            else if (event.key === " ") {
                this.launchSpacecraft();
                event.preventDefault();
            }
        }
        else {
            // Flight phase controls
            if (event.key === "ArrowUp" || event.key === "w") {
                sc.thrustActive = true;
                sc.thrustPower = 15;
            }
            else if (event.key === "ArrowLeft" || event.key === "a") {
                const angle = Math.atan2(sc.velocity.y, sc.velocity.x);
                sc.thrustDirection = {
                    x: Math.cos(angle + Math.PI / 2),
                    y: Math.sin(angle + Math.PI / 2),
                };
            }
            else if (event.key === "ArrowRight" || event.key === "d") {
                const angle = Math.atan2(sc.velocity.y, sc.velocity.x);
                sc.thrustDirection = {
                    x: Math.cos(angle - Math.PI / 2),
                    y: Math.sin(angle - Math.PI / 2),
                };
            }
            // Time controls
            if (event.key === "1") {
                this.gameState.timeScale = 1;
            }
            else if (event.key === "2") {
                this.gameState.timeScale = 2;
            }
            else if (event.key === "5") {
                this.gameState.timeScale = 5;
            }
            else if (event.key === "0") {
                this.gameState.timeScale = 10;
            }
        }
    }
    handleKeyUp(event) {
        if (event.key === "ArrowUp" || event.key === "w") {
            this.gameState.spacecraft.thrustActive = false;
        }
    }
    launchSpacecraft() {
        const angle = (this.gameState.launchAngle * Math.PI) / 180;
        const velocity = this.gameState.launchVelocity;
        const sc = this.gameState.spacecraft;
        sc.velocity = vectorAdd(sc.velocity, createVector(Math.cos(angle) * velocity, Math.sin(angle) * velocity));
        this.gameState.launchPhase = false;
        this.gameState.time = 0;
    }
    checkMissionComplete() {
        const mission = this.gameState.currentMission;
        const sc = this.gameState.spacecraft;
        const bodies = this.gameState.bodies;
        if (mission.targetBody === "any") {
            // Check distance to any planet (except sun)
            for (const body of bodies.values()) {
                if (body.id !== "sun" && body.id !== "asteroid") {
                    const dist = vectorDistance(sc.position, body.position);
                    if (dist < 200) {
                        return true;
                    }
                }
            }
            return false;
        }
        const targetBody = bodies.get(mission.targetBody);
        if (!targetBody)
            return false;
        const distance = vectorDistance(sc.position, targetBody.position);
        const requiredDistance = mission.targetBody === "moon" ? 80 : 200;
        return distance < requiredDistance;
    }
    checkGameOver() {
        // Collision with sun
        const sunDist = vectorDistance(this.gameState.spacecraft.position, this.gameState.bodies.get("sun").position);
        if (sunDist < 50) {
            return true;
        }
        // Out of bounds
        if (Math.abs(this.gameState.spacecraft.position.x) > 3000 ||
            Math.abs(this.gameState.spacecraft.position.y) > 3000) {
            return true;
        }
        return false;
    }
    updateTrajectory() {
        this.gameState.trajectoryPoints = this.physicsEngine.predictTrajectory(this.gameState, TRAJECTORY_STEPS, TRAJECTORY_INTERVAL);
    }
    render() {
        const ctx = this.ctx;
        const centerX = CANVAS_WIDTH / 2;
        const centerY = CANVAS_HEIGHT / 2;
        // Clear canvas
        ctx.fillStyle = "#000011";
        ctx.fillRect(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
        // Draw stars background
        this.drawStarfield();
        // Draw celestial bodies
        for (const body of this.gameState.bodies.values()) {
            this.drawBody(body, centerX, centerY);
        }
        // Draw trajectory prediction
        if (!this.gameState.launchPhase) {
            this.drawTrajectory(centerX, centerY);
        }
        // Draw spacecraft
        if (!this.gameState.launchPhase) {
            this.drawSpacecraft(centerX, centerY);
        }
        // Draw UI
        this.drawUI();
        // Draw launch phase UI
        if (this.gameState.launchPhase) {
            this.drawLaunchUI();
        }
    }
    drawStarfield() {
        const ctx = this.ctx;
        ctx.fillStyle = "#FFFFFF";
        for (let i = 0; i < 200; i++) {
            const x = (i * 73) % CANVAS_WIDTH;
            const y = (i * 137) % CANVAS_HEIGHT;
            const size = (i % 3) * 0.3 + 0.3;
            ctx.fillRect(x, y, size, size);
        }
    }
    drawBody(body, centerX, centerY) {
        const ctx = this.ctx;
        const screenX = centerX + body.position.x * VISUAL_SCALE;
        const screenY = centerY + body.position.y * VISUAL_SCALE;
        // Clipping for bodies far off screen
        if (screenX < -100 ||
            screenX > CANVAS_WIDTH + 100 ||
            screenY < -100 ||
            screenY > CANVAS_HEIGHT + 100) {
            return;
        }
        ctx.fillStyle = body.color;
        ctx.beginPath();
        ctx.arc(screenX, screenY, body.radius, 0, Math.PI * 2);
        ctx.fill();
        // Draw label
        ctx.fillStyle = "#FFFFFF";
        ctx.font = "10px Arial";
        ctx.fillText(body.name, screenX + 10, screenY - 10);
    }
    drawSpacecraft(centerX, centerY) {
        const ctx = this.ctx;
        const sc = this.gameState.spacecraft;
        const screenX = centerX + sc.position.x * VISUAL_SCALE;
        const screenY = centerY + sc.position.y * VISUAL_SCALE;
        // Draw spacecraft
        ctx.fillStyle = "#00FF00";
        ctx.beginPath();
        ctx.arc(screenX, screenY, sc.radius, 0, Math.PI * 2);
        ctx.fill();
        // Draw velocity vector
        const velMag = Math.min(vectorMagnitude(sc.velocity) * VISUAL_SCALE, 50);
        const velDir = vectorNormalize(sc.velocity);
        ctx.strokeStyle = "#00AA00";
        ctx.lineWidth = 1;
        ctx.beginPath();
        ctx.moveTo(screenX, screenY);
        ctx.lineTo(screenX + velDir.x * velMag, screenY + velDir.y * velMag);
        ctx.stroke();
        // Draw thrust indicator
        if (sc.thrustActive && sc.fuel > 0) {
            ctx.fillStyle = "#FF6600";
            ctx.beginPath();
            ctx.arc(screenX, screenY, 5, 0, Math.PI * 2);
            ctx.fill();
        }
    }
    drawTrajectory(centerX, centerY) {
        const ctx = this.ctx;
        ctx.strokeStyle = "#FFFFFF";
        ctx.lineWidth = 1;
        ctx.setLineDash([5, 5]);
        ctx.globalAlpha = 0.5;
        ctx.beginPath();
        for (let i = 0; i < this.gameState.trajectoryPoints.length; i++) {
            const point = this.gameState.trajectoryPoints[i];
            const screenX = centerX + point.x * VISUAL_SCALE;
            const screenY = centerY + point.y * VISUAL_SCALE;
            if (i === 0) {
                ctx.moveTo(screenX, screenY);
            }
            else {
                ctx.lineTo(screenX, screenY);
            }
        }
        ctx.stroke();
        ctx.setLineDash([]);
        ctx.globalAlpha = 1;
    }
    drawUI() {
        const ctx = this.ctx;
        const sc = this.gameState.spacecraft;
        const mission = this.gameState.currentMission;
        ctx.fillStyle = "#FFFFFF";
        ctx.font = "12px Courier";
        let yPos = 20;
        const lineHeight = 18;
        // Mission info
        ctx.fillText(`Mission ${this.gameState.missionIndex + 1}/10: ${mission.name}`, 10, yPos);
        yPos += lineHeight;
        ctx.fillText(`Objective: ${mission.objective}`, 10, yPos);
        yPos += lineHeight;
        // Spacecraft info
        yPos += 5;
        ctx.fillText(`Fuel: ${sc.fuel.toFixed(1)} / ${sc.maxFuel}`, 10, yPos);
        yPos += lineHeight;
        ctx.fillText(`Velocity: ${vectorMagnitude(sc.velocity).toFixed(2)} units/s`, 10, yPos);
        yPos += lineHeight;
        ctx.fillText(`Time: ${this.gameState.time.toFixed(1)}s`, 10, yPos);
        yPos += lineHeight;
        ctx.fillText(`Time Scale: ${this.gameState.timeScale}x`, 10, yPos);
        // Score
        yPos += 5;
        ctx.fillText(`Score: ${this.gameState.totalScore}`, 10, yPos);
        // Controls hint
        yPos = CANVAS_HEIGHT - 40;
        if (this.gameState.launchPhase) {
            ctx.fillText("Launch Phase - Arrow keys: aim | W/S: velocity | SPACE: launch", 10, yPos);
        }
        else {
            ctx.fillText("Flight - Arrow keys: thrust | 1/2/5/0: time scale", 10, yPos);
        }
    }
    drawLaunchUI() {
        const ctx = this.ctx;
        const centerX = CANVAS_WIDTH / 2;
        const centerY = CANVAS_HEIGHT / 2;
        // Draw launch reticle
        ctx.strokeStyle = "#00FF00";
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.arc(centerX, centerY, 50, 0, Math.PI * 2);
        ctx.stroke();
        // Draw angle indicator
        const angle = (this.gameState.launchAngle * Math.PI) / 180;
        ctx.strokeStyle = "#FF0000";
        ctx.beginPath();
        ctx.moveTo(centerX, centerY);
        ctx.lineTo(centerX + Math.cos(angle) * 100, centerY + Math.sin(angle) * 100);
        ctx.stroke();
        // Draw velocity bar
        ctx.fillStyle = "#0099FF";
        const barWidth = (this.gameState.launchVelocity / 100) * 200;
        ctx.fillRect(centerX - 100, centerY + 150, barWidth, 20);
        ctx.strokeStyle = "#FFFFFF";
        ctx.strokeRect(centerX - 100, centerY + 150, 200, 20);
        // Labels
        ctx.fillStyle = "#FFFFFF";
        ctx.font = "14px Arial";
        ctx.fillText(`Angle: ${this.gameState.launchAngle.toFixed(0)}°`, 10, 60);
        ctx.fillText(`Velocity: ${this.gameState.launchVelocity.toFixed(1)} units/s`, 10, 85);
    }
    completeMission() {
        const mission = this.gameState.currentMission;
        mission.completed = true;
        // Calculate score
        const fuelBonus = (this.gameState.spacecraft.fuel / this.gameState.spacecraft.maxFuel) *
            500;
        const missionScore = (1000 + fuelBonus) * mission.difficultyMultiplier;
        this.gameState.score = missionScore;
        this.gameState.totalScore += missionScore;
        // Move to next mission
        this.gameState.missionIndex++;
        if (this.gameState.missionIndex < this.missions.length) {
            this.nextMission();
        }
        else {
            this.victory();
        }
    }
    nextMission() {
        const nextMissionIndex = this.gameState.missionIndex;
        const nextMission = this.missions[nextMissionIndex];
        this.gameState.currentMission = nextMission;
        this.gameState.launchPhase = true;
        this.gameState.time = 0;
        // Reset spacecraft at Earth
        const earth = this.gameState.bodies.get("earth");
        this.gameState.spacecraft.position = { ...earth.position };
        this.gameState.spacecraft.velocity = { ...earth.velocity };
        this.gameState.spacecraft.fuel = nextMission.fuelLimit;
        this.gameState.spacecraft.maxFuel = nextMission.fuelLimit;
        this.gameState.launchAngle = 270;
        this.gameState.launchVelocity = 0;
    }
    gameOver() {
        this.gameRunning = false;
        alert(`Mission Failed! You reached mission ${this.gameState.missionIndex + 1}/10\nTotal Score: ${this.gameState.totalScore}`);
    }
    victory() {
        this.gameRunning = false;
        alert(`Victory! All missions completed!\nFinal Score: ${this.gameState.totalScore}`);
    }
}
exports.OrbitCommander = OrbitCommander;
