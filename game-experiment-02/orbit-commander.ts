// Orbit Commander - Space Navigation Game with Orbital Mechanics

interface Vector2D { x: number; y: number; }

function createVector(x: number, y: number): Vector2D { return { x, y }; }
function vectorAdd(a: Vector2D, b: Vector2D): Vector2D { return { x: a.x + b.x, y: a.y + b.y }; }
function vectorSubtract(a: Vector2D, b: Vector2D): Vector2D { return { x: a.x - b.x, y: a.y - b.y }; }
function vectorScale(v: Vector2D, s: number): Vector2D { return { x: v.x * s, y: v.y * s }; }
function vectorMagnitude(v: Vector2D): number { return Math.sqrt(v.x * v.x + v.y * v.y); }
function vectorNormalize(v: Vector2D): Vector2D { const m = vectorMagnitude(v); return m === 0 ? { x: 0, y: 0 } : { x: v.x / m, y: v.y / m }; }
function vectorDistance(a: Vector2D, b: Vector2D): number { return vectorMagnitude(vectorSubtract(a, b)); }

interface CelestialBody { id: string; name: string; mass: number; radius: number; position: Vector2D; velocity: Vector2D; color: string; orbitParent?: string; orbitDistance?: number; orbitSpeed?: number; orbitAngle?: number; }
interface Spacecraft { position: Vector2D; velocity: Vector2D; fuel: number; maxFuel: number; radius: number; thrustActive: boolean; thrustDirection: Vector2D; thrustPower: number; }
interface Mission { id: number; name: string; objective: string; targetBody: string; fuelLimit: number; difficultyMultiplier: number; completed: boolean; }
interface GameState { spacecraft: Spacecraft; bodies: Map<string, CelestialBody>; currentMission: Mission; score: number; totalScore: number; missionIndex: number; time: number; timeScale: number; trajectoryPoints: Vector2D[]; gameRunning: boolean; launchPhase: boolean; launchAngle: number; launchVelocity: number; }

const DT = 1 / 60;
const TRAJECTORY_STEPS = 100;
const TRAJECTORY_INTERVAL = 5;
const GRAVITATIONAL_CONSTANT = 5.0;
const VISUAL_SCALE = 0.1;

class PhysicsEngine {
  calculateGravitationalForce(m1: number, m2: number, d: number): number {
    if (d < 10) return 0;
    return (GRAVITATIONAL_CONSTANT * m1 * m2) / (d * d);
  }

  calculateAcceleration(bodies: Map<string, CelestialBody>, sc: Spacecraft): Vector2D {
    let a = createVector(0, 0);
    for (const b of bodies.values()) {
      const dir = vectorSubtract(b.position, sc.position);
      const dist = vectorMagnitude(dir);
      if (dist > 5) {
        const f = this.calculateGravitationalForce(b.mass, 1, dist);
        a = vectorAdd(a, vectorScale(vectorNormalize(dir), f));
      }
    }
    if (sc.thrustActive && sc.fuel > 0) {
      a = vectorAdd(a, vectorScale(sc.thrustDirection, sc.thrustPower));
    }
    return a;
  }

  updatePhysics(s: GameState, dt: number): void {
    dt = dt * s.timeScale;
    const sc = s.spacecraft;
    const a = this.calculateAcceleration(s.bodies, sc);
    sc.velocity = vectorAdd(sc.velocity, vectorScale(a, dt));
    sc.position = vectorAdd(sc.position, vectorScale(sc.velocity, dt));
    if (sc.thrustActive && sc.fuel > 0) {
      sc.fuel -= sc.thrustPower * dt * 2;
      if (sc.fuel < 0) sc.fuel = 0;
    }
    for (const b of s.bodies.values()) {
      if (b.orbitParent && s.bodies.has(b.orbitParent)) {
        const p = s.bodies.get(b.orbitParent)!;
        if (b.orbitAngle !== undefined && b.orbitSpeed !== undefined && b.orbitDistance !== undefined) {
          b.orbitAngle += b.orbitSpeed * dt;
          b.position = vectorAdd(p.position, createVector(Math.cos(b.orbitAngle) * b.orbitDistance, Math.sin(b.orbitAngle) * b.orbitDistance));
        }
      }
    }
    s.time += dt;
  }

  predictTrajectory(bodies: Map<string, CelestialBody>, sc: Spacecraft, steps: number, interval: number): Vector2D[] {
    const pts: Vector2D[] = [];
    let p = { ...sc.position };
    let v = { ...sc.velocity };
    for (let i = 0; i < steps; i++) {
      if (i % interval === 0) pts.push({ ...p });
      let a = createVector(0, 0);
      for (const b of bodies.values()) {
        const dir = vectorSubtract(b.position, p);
        const dist = vectorMagnitude(dir);
        if (dist > 5) {
          const f = this.calculateGravitationalForce(b.mass, 1, dist);
          a = vectorAdd(a, vectorScale(vectorNormalize(dir), f));
        }
      }
      v = vectorAdd(v, vectorScale(a, DT));
      p = vectorAdd(p, vectorScale(v, DT));
    }
    return pts;
  }
}

class OrbitCommander {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private gameState: GameState;
  private engine: PhysicsEngine;
  private missions: Mission[];
  public gameRunning: boolean = false;

  constructor(id: string) {
    this.canvas = document.getElementById(id) as HTMLCanvasElement;
    this.ctx = this.canvas.getContext('2d')!;
    this.engine = new PhysicsEngine();
    this.missions = this.createMissions();
    this.gameState = this.initState();
  }

  private createMissions(): Mission[] {
    return [
      { id: 1, name: "Mars Bound", objective: "Get within 200 units of Mars", targetBody: "mars", fuelLimit: 500, difficultyMultiplier: 1, completed: false },
      { id: 2, name: "Venus Run", objective: "Get within 200 units of Venus", targetBody: "venus", fuelLimit: 400, difficultyMultiplier: 1.2, completed: false },
      { id: 3, name: "Lunar Deployment", objective: "Achieve stable orbit around Moon", targetBody: "moon", fuelLimit: 350, difficultyMultiplier: 1.3, completed: false },
      { id: 4, name: "Gravity Assist", objective: "Reach Jupiter using gravity assists", targetBody: "jupiter", fuelLimit: 600, difficultyMultiplier: 1.5, completed: false },
      { id: 5, name: "Mercury Challenge", objective: "Get within 150 units of Mercury", targetBody: "mercury", fuelLimit: 450, difficultyMultiplier: 1.6, completed: false },
      { id: 6, name: "Fuel Conservation", objective: "Reach any planet with 200+ fuel left", targetBody: "any", fuelLimit: 300, difficultyMultiplier: 1.8, completed: false },
      { id: 7, name: "Asteroid Strike", objective: "Get within 100 units of asteroid", targetBody: "asteroid", fuelLimit: 400, difficultyMultiplier: 2.0, completed: false },
      { id: 8, name: "Dual Rendezvous", objective: "Visit Mars then Venus", targetBody: "mars_venus", fuelLimit: 550, difficultyMultiplier: 2.2, completed: false },
      { id: 9, name: "Grand Tour", objective: "Visit Mercury, Venus, Mars, return", targetBody: "grand_tour", fuelLimit: 800, difficultyMultiplier: 2.5, completed: false },
      { id: 10, name: "Impossible Challenge", objective: "Traverse asteroid belt safely", targetBody: "asteroid_field", fuelLimit: 250, difficultyMultiplier: 3.0, completed: false },
    ];
  }

  private initState(): GameState {
    const bodies = new Map<string, CelestialBody>();
    bodies.set("sun", { id: "sun", name: "Sun", mass: 1000, radius: 20, position: { x: 0, y: 0 }, velocity: { x: 0, y: 0 }, color: "#FFD700" });
    bodies.set("mercury", { id: "mercury", name: "Mercury", mass: 50, radius: 8, position: { x: 300, y: 0 }, velocity: { x: 0, y: -8 }, color: "#8C7853" });
    bodies.set("venus", { id: "venus", name: "Venus", mass: 100, radius: 12, position: { x: 500, y: 0 }, velocity: { x: 0, y: -6 }, color: "#FFC649" });
    bodies.set("earth", { id: "earth", name: "Earth", mass: 150, radius: 13, position: { x: 700, y: 0 }, velocity: { x: 0, y: -4.5 }, color: "#4169E1" });
    bodies.set("moon", { id: "moon", name: "Moon", mass: 10, radius: 5, position: { x: 730, y: 0 }, velocity: { x: 0, y: -4.5 }, color: "#A9A9A9", orbitParent: "earth", orbitDistance: 30, orbitSpeed: 0.05, orbitAngle: 0 });
    bodies.set("mars", { id: "mars", name: "Mars", mass: 75, radius: 10, position: { x: 1000, y: 0 }, velocity: { x: 0, y: -3.5 }, color: "#CD5C5C" });
    bodies.set("jupiter", { id: "jupiter", name: "Jupiter", mass: 300, radius: 25, position: { x: 1600, y: 200 }, velocity: { x: 0, y: -2 }, color: "#DAA520" });
    bodies.set("asteroid", { id: "asteroid", name: "Asteroid", mass: 5, radius: 3, position: { x: 800, y: 400 }, velocity: { x: -1, y: -2 }, color: "#696969" });

    return {
      spacecraft: { position: { x: 700, y: 0 }, velocity: { x: 0, y: -4.5 }, fuel: 500, maxFuel: 500, radius: 3, thrustActive: false, thrustDirection: { x: 0, y: -1 }, thrustPower: 0 },
      bodies,
      currentMission: this.missions[0],
      score: 0, totalScore: 0, missionIndex: 0, time: 0, timeScale: 1,
      trajectoryPoints: [], gameRunning: false, launchPhase: true,
      launchAngle: 270, launchVelocity: 0,
    };
  }

  start(): void {
    this.gameRunning = true;
    window.addEventListener("keydown", (e) => this.keyDown(e));
    window.addEventListener("keyup", (e) => this.keyUp(e));
    this.loop();
  }

  reset(): void {
    this.gameRunning = false;
    this.gameState = this.initState();
  }

  private keyDown(e: KeyboardEvent): void {
    const sc = this.gameState.spacecraft;
    if (this.gameState.launchPhase) {
      if (e.key === "ArrowUp" || e.key === "w") this.gameState.launchVelocity = Math.min(this.gameState.launchVelocity + 2, 100);
      else if (e.key === "ArrowDown" || e.key === "s") this.gameState.launchVelocity = Math.max(this.gameState.launchVelocity - 2, 0);
      else if (e.key === "ArrowLeft" || e.key === "a") this.gameState.launchAngle = (this.gameState.launchAngle - 5) % 360;
      else if (e.key === "ArrowRight" || e.key === "d") this.gameState.launchAngle = (this.gameState.launchAngle + 5) % 360;
      else if (e.key === " ") { this.launch(); e.preventDefault(); }
    } else {
      if (e.key === "ArrowUp" || e.key === "w") { sc.thrustActive = true; sc.thrustPower = 15; }
      else if (e.key === "ArrowLeft" || e.key === "a") { const ang = Math.atan2(sc.velocity.y, sc.velocity.x); sc.thrustDirection = { x: Math.cos(ang + Math.PI / 2), y: Math.sin(ang + Math.PI / 2) }; }
      else if (e.key === "ArrowRight" || e.key === "d") { const ang = Math.atan2(sc.velocity.y, sc.velocity.x); sc.thrustDirection = { x: Math.cos(ang - Math.PI / 2), y: Math.sin(ang - Math.PI / 2) }; }
      if (e.key === "1") this.gameState.timeScale = 1;
      else if (e.key === "2") this.gameState.timeScale = 2;
      else if (e.key === "5") this.gameState.timeScale = 5;
      else if (e.key === "0") this.gameState.timeScale = 10;
    }
  }

  private keyUp(e: KeyboardEvent): void {
    if (e.key === "ArrowUp" || e.key === "w") this.gameState.spacecraft.thrustActive = false;
  }

  private launch(): void {
    const ang = (this.gameState.launchAngle * Math.PI) / 180;
    const vel = this.gameState.launchVelocity;
    this.gameState.spacecraft.velocity = vectorAdd(this.gameState.spacecraft.velocity, { x: Math.cos(ang) * vel, y: Math.sin(ang) * vel });
    this.gameState.launchPhase = false;
    this.gameState.time = 0;
  }

  private checkMission(): boolean {
    const m = this.gameState.currentMission;
    const sc = this.gameState.spacecraft;
    if (m.targetBody === "any") {
      for (const b of this.gameState.bodies.values()) {
        if (b.id !== "sun" && b.id !== "asteroid" && vectorDistance(sc.position, b.position) < 200) return true;
      }
      return false;
    }
    const t = this.gameState.bodies.get(m.targetBody);
    return t ? vectorDistance(sc.position, t.position) < (m.targetBody === "moon" ? 80 : 200) : false;
  }

  private checkOver(): boolean {
    const sun = this.gameState.bodies.get("sun")!;
    if (vectorDistance(this.gameState.spacecraft.position, sun.position) < 50) return true;
    const pos = this.gameState.spacecraft.position;
    return Math.abs(pos.x) > 3000 || Math.abs(pos.y) > 3000;
  }

  private updateTraj(): void {
    this.gameState.trajectoryPoints = this.engine.predictTrajectory(
      this.gameState.bodies,
      this.gameState.spacecraft,
      TRAJECTORY_STEPS,
      TRAJECTORY_INTERVAL
    );
  }

  private render(): void {
    const ctx = this.ctx;
    const cx = 400, cy = 300;

    ctx.fillStyle = "#000011";
    ctx.fillRect(0, 0, 800, 600);

    ctx.fillStyle = "#FFFFFF";
    for (let i = 0; i < 200; i++) {
      const x = (i * 73) % 800, y = (i * 137) % 600;
      ctx.fillRect(x, y, 0.5, 0.5);
    }

    for (const body of this.gameState.bodies.values()) {
      const sx = cx + body.position.x * VISUAL_SCALE;
      const sy = cy + body.position.y * VISUAL_SCALE;
      if (sx < -100 || sx > 900 || sy < -100 || sy > 700) continue;

      ctx.fillStyle = body.color;
      ctx.beginPath();
      ctx.arc(sx, sy, body.radius, 0, Math.PI * 2);
      ctx.fill();
      ctx.fillStyle = "#FFF";
      ctx.font = "10px Arial";
      ctx.fillText(body.name, sx + 10, sy - 10);
    }

    if (!this.gameState.launchPhase) {
      ctx.strokeStyle = "#FFF";
      ctx.lineWidth = 1;
      ctx.setLineDash([5, 5]);
      ctx.globalAlpha = 0.5;
      ctx.beginPath();
      for (let i = 0; i < this.gameState.trajectoryPoints.length; i++) {
        const pt = this.gameState.trajectoryPoints[i];
        const px = cx + pt.x * VISUAL_SCALE, py = cy + pt.y * VISUAL_SCALE;
        if (i === 0) ctx.moveTo(px, py);
        else ctx.lineTo(px, py);
      }
      ctx.stroke();
      ctx.setLineDash([]);
      ctx.globalAlpha = 1;

      const sc = this.gameState.spacecraft;
      const sx = cx + sc.position.x * VISUAL_SCALE, sy = cy + sc.position.y * VISUAL_SCALE;
      ctx.fillStyle = "#0F0";
      ctx.beginPath();
      ctx.arc(sx, sy, 3, 0, Math.PI * 2);
      ctx.fill();

      if (sc.thrustActive && sc.fuel > 0) {
        ctx.fillStyle = "#F60";
        ctx.beginPath();
        ctx.arc(sx, sy, 5, 0, Math.PI * 2);
        ctx.fill();
      }
    }

    ctx.fillStyle = "#FFF";
    ctx.font = "12px Courier";
    ctx.fillText(`Mission ${this.gameState.missionIndex + 1}/10: ${this.gameState.currentMission.name}`, 10, 20);
    ctx.fillText(`Fuel: ${this.gameState.spacecraft.fuel.toFixed(1)}/${this.gameState.spacecraft.maxFuel}`, 10, 40);
    ctx.fillText(`Velocity: ${vectorMagnitude(this.gameState.spacecraft.velocity).toFixed(2)} u/s`, 10, 60);
    ctx.fillText(`Score: ${this.gameState.totalScore}`, 10, 80);

    if (this.gameState.launchPhase) {
      const a = (this.gameState.launchAngle * Math.PI) / 180;
      ctx.strokeStyle = "#0F0";
      ctx.beginPath();
      ctx.arc(400, 300, 50, 0, Math.PI * 2);
      ctx.stroke();
      ctx.strokeStyle = "#F00";
      ctx.beginPath();
      ctx.moveTo(400, 300);
      ctx.lineTo(400 + Math.cos(a) * 100, 300 + Math.sin(a) * 100);
      ctx.stroke();
      ctx.fillStyle = "#0FF";
      const bw = (this.gameState.launchVelocity / 100) * 200;
      ctx.fillRect(300, 450, bw, 20);
      ctx.strokeStyle = "#FFF";
      ctx.strokeRect(300, 450, 200, 20);
      ctx.fillStyle = "#FFF";
      ctx.font = "14px Arial";
      ctx.fillText(`Angle: ${this.gameState.launchAngle.toFixed(0)}°`, 10, 60);
      ctx.fillText(`Velocity: ${this.gameState.launchVelocity.toFixed(1)}`, 10, 85);
    }
  }

  private loop = (): void => {
    this.engine.updatePhysics(this.gameState, DT);

    if (!this.gameState.launchPhase) {
      this.updateTraj();
      if (this.checkMission()) {
        this.complete();
        return;
      }
      if (this.checkOver()) {
        this.gameOver();
        return;
      }
    }

    this.render();

    if (this.gameRunning) {
      requestAnimationFrame(this.loop);
    }
  };

  private complete(): void {
    const m = this.gameState.currentMission;
    m.completed = true;

    const bonus = (this.gameState.spacecraft.fuel / this.gameState.spacecraft.maxFuel) * 500;
    const score = (1000 + bonus) * m.difficultyMultiplier;
    this.gameState.totalScore += score;

    this.gameState.missionIndex++;
    if (this.gameState.missionIndex < this.missions.length) {
      const nm = this.missions[this.gameState.missionIndex];
      this.gameState.currentMission = nm;
      this.gameState.launchPhase = true;
      this.gameState.time = 0;
      const earth = this.gameState.bodies.get("earth")!;
      this.gameState.spacecraft.position = { ...earth.position };
      this.gameState.spacecraft.velocity = { ...earth.velocity };
      this.gameState.spacecraft.fuel = nm.fuelLimit;
      this.gameState.spacecraft.maxFuel = nm.fuelLimit;
    } else {
      this.gameRunning = false;
      alert(`Victory! Final Score: ${this.gameState.totalScore}`);
    }
  }

  private gameOver(): void {
    this.gameRunning = false;
    alert(`Mission Failed at ${this.gameState.missionIndex + 1}/10\nScore: ${this.gameState.totalScore}`);
  }
}

// Global
if (typeof window !== "undefined") {
  (window as any).OrbitCommanderGame = OrbitCommander;
}

export { OrbitCommander, PhysicsEngine, vectorAdd, vectorSubtract, vectorScale, vectorMagnitude, vectorDistance, vectorNormalize, createVector };
