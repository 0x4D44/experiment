/**
 * Chain Reaction - Physics Puzzle Game
 * A creative physics-based puzzle game using Matter.js
 */

// Matter.js module aliases
const Engine = Matter.Engine;
const Render = Matter.Render;
const Runner = Matter.Runner;
const Bodies = Matter.Bodies;
const World = Matter.World;
const Events = Matter.Events;
const Mouse = Matter.Mouse;
const MouseConstraint = Matter.MouseConstraint;
const Body = Matter.Body;
const Constraint = Matter.Constraint;
const Composite = Matter.Composite;
const Vector = Matter.Vector;

// Constants
const MAX_PARTICLES = 200;

/**
 * Main Game Class
 * Manages all game state, physics, rendering, and user interactions
 */
class PhysicsGame {
    constructor() {
        this.engine = null;
        this.render = null;
        this.runner = null;
        this.currentLevel = 1;
        this.maxUnlockedLevel = 1;
        this.levelStars = {}; // Store stars earned per level
        this.moveCount = 0;
        this.startTime = 0;
        this.elapsedTime = 0;
        this.timerInterval = null;
        this.isLevelStarted = false;
        this.isLevelComplete = false;
        this.history = []; // For undo functionality
        this.selectedObject = null;
        this.placedObjects = [];
        this.goldenBall = null;
        this.targetStar = null;
        this.particles = [];
        this.keyboardHandler = null; // Store keyboard listener reference

        // Load saved progress
        this.loadProgress();

        // Initialize the game
        this.init();
    }

    /**
     * Initialize game systems
     */
    init() {
        // Set up keyboard controls
        this.setupKeyboardControls();

        // Update stars display
        this.updateStarsDisplay();
    }

    /**
     * Setup keyboard event listeners
     */
    setupKeyboardControls() {
        // Remove previous listener if it exists
        if (this.keyboardHandler) {
            document.removeEventListener('keydown', this.keyboardHandler);
        }

        // Create and store new listener
        this.keyboardHandler = (e) => {
            if (!document.getElementById('gameScreen').classList.contains('active')) return;

            switch(e.key.toLowerCase()) {
                case ' ':
                    e.preventDefault();
                    if (!this.isLevelStarted && !this.isLevelComplete) {
                        this.startLevel();
                    }
                    break;
                case 'r':
                    this.resetLevel();
                    break;
                case 'u':
                    this.undo();
                    break;
            }
        };

        document.addEventListener('keydown', this.keyboardHandler);
    }

    /**
     * Show main menu
     */
    showMenu() {
        this.hideAllScreens();
        document.getElementById('mainMenu').classList.add('active');
        this.updateStarsDisplay();

        // Clear timer if running
        if (this.timerInterval) {
            clearInterval(this.timerInterval);
            this.timerInterval = null;
        }

        // Clean up physics engine if running
        if (this.engine) {
            this.destroyPhysicsEngine();
        }

        // Clean up keyboard listener
        if (this.keyboardHandler) {
            document.removeEventListener('keydown', this.keyboardHandler);
        }
    }

    /**
     * Show help screen
     */
    showHelp() {
        this.hideAllScreens();
        document.getElementById('helpScreen').classList.add('active');
    }

    /**
     * Show level select screen
     */
    showLevelSelect() {
        this.hideAllScreens();
        document.getElementById('levelSelect').classList.add('active');
        this.generateLevelGrid();
    }

    /**
     * Generate level selection grid
     */
    generateLevelGrid() {
        const levelGrid = document.getElementById('levelGrid');
        levelGrid.innerHTML = '';

        const totalLevels = this.getLevels().length;

        for (let i = 1; i <= totalLevels; i++) {
            const btn = document.createElement('button');
            btn.className = 'level-btn';
            btn.textContent = i;

            // Check if level is unlocked
            if (i <= this.maxUnlockedLevel) {
                btn.onclick = () => this.loadLevel(i);

                // Show stars earned
                const stars = this.levelStars[i] || 0;
                if (stars > 0) {
                    btn.classList.add('completed');
                    const starDiv = document.createElement('div');
                    starDiv.className = 'level-stars';
                    starDiv.textContent = '★'.repeat(stars) + '☆'.repeat(3 - stars);
                    btn.appendChild(starDiv);
                }
            } else {
                btn.disabled = true;
            }

            levelGrid.appendChild(btn);
        }
    }

    /**
     * Hide all screens
     */
    hideAllScreens() {
        document.querySelectorAll('.screen').forEach(screen => {
            screen.classList.remove('active');
        });
        document.getElementById('winOverlay').classList.remove('active');
    }

    /**
     * Load and start a specific level
     */
    loadLevel(levelNum) {
        this.currentLevel = levelNum;
        this.hideAllScreens();
        document.getElementById('gameScreen').classList.add('active');

        // Reset level state
        this.moveCount = 0;
        this.elapsedTime = 0;
        this.isLevelStarted = false;
        this.isLevelComplete = false;
        this.history = [];
        this.placedObjects = [];
        this.selectedObject = null;

        // Update UI
        document.getElementById('currentLevel').textContent = levelNum;
        document.getElementById('moveCount').textContent = '0';
        document.getElementById('timeCount').textContent = '0';
        this.updateStarsEarned(0);

        // Stop timer if running
        if (this.timerInterval) {
            clearInterval(this.timerInterval);
            this.timerInterval = null;
        }

        // Initialize physics engine
        this.initPhysicsEngine();

        // Load level configuration
        const levelConfig = this.getLevelConfig(levelNum);
        this.buildLevel(levelConfig);

        // Update instructions
        document.getElementById('levelInstructions').textContent =
            levelConfig.instructions || 'Press SPACE to start';
    }

    /**
     * Initialize Matter.js physics engine
     */
    initPhysicsEngine() {
        // Destroy existing engine if any
        if (this.engine) {
            this.destroyPhysicsEngine();
        }

        // Create engine
        this.engine = Engine.create();
        this.engine.gravity.y = 1;

        // Create renderer
        const canvas = document.getElementById('gameCanvas');
        this.render = Render.create({
            element: canvas,
            engine: this.engine,
            options: {
                width: canvas.clientWidth,
                height: canvas.clientHeight,
                wireframes: false,
                background: '#1a1a2e'
            }
        });

        // Add mouse control
        const mouse = Mouse.create(this.render.canvas);
        this.mouseConstraint = MouseConstraint.create(this.engine, {
            mouse: mouse,
            constraint: {
                stiffness: 0.2,
                render: { visible: false }
            }
        });

        World.add(this.engine.world, this.mouseConstraint);

        // Handle mouse events for custom interactions
        Events.on(this.mouseConstraint, 'mousedown', (event) => {
            this.handleMouseDown(event);
        });

        // Start renderer
        Render.run(this.render);

        // Create runner
        this.runner = Runner.create();
        Runner.run(this.runner, this.engine);

        // Add collision detection
        Events.on(this.engine, 'collisionStart', (event) => {
            this.handleCollision(event);
        });

        // Custom update loop for particles
        Events.on(this.engine, 'afterUpdate', () => {
            this.updateParticles();
        });
    }

    /**
     * Destroy physics engine and clean up
     */
    destroyPhysicsEngine() {
        if (this.runner) {
            Runner.stop(this.runner);
        }
        if (this.render) {
            Render.stop(this.render);
            this.render.canvas.remove();
            this.render.canvas = null;
            this.render.context = null;
            this.render.textures = {};
        }
        if (this.engine) {
            World.clear(this.engine.world);
            Engine.clear(this.engine);
        }
        this.engine = null;
        this.render = null;
        this.runner = null;
    }

    /**
     * Build level from configuration
     */
    buildLevel(config) {
        const world = this.engine.world;

        // Add walls
        const wallThickness = 50;
        const width = this.render.options.width;
        const height = this.render.options.height;

        const walls = [
            Bodies.rectangle(width / 2, -wallThickness / 2, width, wallThickness, {
                isStatic: true,
                render: { fillStyle: '#16213e' }
            }),
            Bodies.rectangle(width / 2, height + wallThickness / 2, width, wallThickness, {
                isStatic: true,
                render: { fillStyle: '#16213e' }
            }),
            Bodies.rectangle(-wallThickness / 2, height / 2, wallThickness, height, {
                isStatic: true,
                render: { fillStyle: '#16213e' }
            }),
            Bodies.rectangle(width + wallThickness / 2, height / 2, wallThickness, height, {
                isStatic: true,
                render: { fillStyle: '#16213e' }
            })
        ];
        World.add(world, walls);

        // Add golden ball (the main object to guide)
        this.goldenBall = Bodies.circle(
            config.ballStart.x,
            config.ballStart.y,
            20,
            {
                restitution: 0.6,
                friction: 0.05,
                density: 0.01,
                render: {
                    fillStyle: '#ffd700',
                    strokeStyle: '#ffed4e',
                    lineWidth: 3
                },
                isStatic: true, // Start frozen
                label: 'goldenBall'
            }
        );
        World.add(world, this.goldenBall);

        // Add target star
        this.targetStar = Bodies.circle(
            config.target.x,
            config.target.y,
            25,
            {
                isStatic: true,
                isSensor: true,
                render: {
                    fillStyle: '#00ff88',
                    strokeStyle: '#00ffaa',
                    lineWidth: 3
                },
                label: 'targetStar'
            }
        );
        World.add(world, this.targetStar);

        // Add static objects from level config
        if (config.staticObjects) {
            config.staticObjects.forEach(obj => {
                let body;
                switch(obj.type) {
                    case 'rectangle':
                        body = Bodies.rectangle(obj.x, obj.y, obj.width, obj.height, {
                            isStatic: true,
                            angle: obj.angle || 0,
                            render: {
                                fillStyle: obj.color || '#533483'
                            },
                            label: obj.label || 'static'
                        });
                        break;
                    case 'circle':
                        body = Bodies.circle(obj.x, obj.y, obj.radius, {
                            isStatic: true,
                            render: {
                                fillStyle: obj.color || '#533483'
                            },
                            label: obj.label || 'static'
                        });
                        break;
                }
                if (body) World.add(world, body);
            });
        }

        // Add interactive objects from level config
        if (config.interactiveObjects) {
            config.interactiveObjects.forEach(obj => {
                this.createInteractiveObject(obj);
            });
        }

        // Setup object palette
        this.setupObjectPalette(config.availableObjects || []);
    }

    /**
     * Create interactive objects (ropes, bombs, dominoes, etc.)
     */
    createInteractiveObject(config) {
        const world = this.engine.world;

        switch(config.type) {
            case 'rope':
                this.createRope(config);
                break;
            case 'bomb':
                this.createBomb(config);
                break;
            case 'domino':
                this.createDomino(config);
                break;
            case 'seesaw':
                this.createSeesaw(config);
                break;
            case 'pendulum':
                this.createPendulum(config);
                break;
        }
    }

    /**
     * Create a rope with hanging object
     */
    createRope(config) {
        const world = this.engine.world;
        const segments = config.segments || 8;
        const segmentLength = config.length / segments;

        let prevBody = Bodies.circle(config.x, config.y, 5, {
            isStatic: true,
            render: { fillStyle: '#666' }
        });
        World.add(world, prevBody);

        const ropeSegments = [];

        for (let i = 0; i < segments; i++) {
            const segment = Bodies.rectangle(
                config.x,
                config.y + segmentLength * (i + 0.5),
                4,
                segmentLength,
                {
                    density: 0.001,
                    render: { fillStyle: '#8b4513' }
                }
            );
            World.add(world, segment);
            ropeSegments.push(segment);

            const constraint = Constraint.create({
                bodyA: prevBody,
                bodyB: segment,
                length: segmentLength / 2,
                stiffness: 0.9
            });
            World.add(world, constraint);

            prevBody = segment;
        }

        // Add hanging object
        let hangingObj;
        if (config.hanging === 'ball') {
            hangingObj = Bodies.circle(
                config.x,
                config.y + config.length + 15,
                15,
                {
                    restitution: 0.8,
                    render: { fillStyle: '#ff6b6b' }
                }
            );
        } else {
            hangingObj = Bodies.rectangle(
                config.x,
                config.y + config.length + 20,
                30,
                30,
                {
                    restitution: 0.3,
                    render: { fillStyle: '#ff6b6b' }
                }
            );
        }
        World.add(world, hangingObj);

        const finalConstraint = Constraint.create({
            bodyA: prevBody,
            bodyB: hangingObj,
            length: segmentLength,
            stiffness: 0.9,
            label: 'rope_' + Date.now()
        });
        World.add(world, finalConstraint);

        // Store rope reference for cutting
        ropeSegments.forEach(seg => {
            seg.ropeConstraint = finalConstraint;
            seg.label = 'rope';
        });
    }

    /**
     * Create a clickable bomb
     */
    createBomb(config) {
        const world = this.engine.world;

        const bomb = Bodies.circle(config.x, config.y, 20, {
            isStatic: config.isStatic || false,
            render: {
                fillStyle: '#333',
                strokeStyle: '#ff0000',
                lineWidth: 3
            },
            label: 'bomb'
        });

        bomb.bombRadius = config.explosionRadius || 150;
        bomb.bombForce = config.explosionForce || 0.05;

        World.add(world, bomb);
    }

    /**
     * Create a domino piece
     */
    createDomino(config) {
        const world = this.engine.world;

        const domino = Bodies.rectangle(
            config.x,
            config.y,
            10,
            50,
            {
                density: 0.002,
                friction: 0.8,
                render: {
                    fillStyle: '#4a90e2'
                },
                label: 'domino'
            }
        );

        World.add(world, domino);
    }

    /**
     * Create a seesaw (balanced platform)
     */
    createSeesaw(config) {
        const world = this.engine.world;

        // Pivot point
        const pivot = Bodies.circle(config.x, config.y, 10, {
            isStatic: true,
            render: { fillStyle: '#666' }
        });

        // Plank
        const plank = Bodies.rectangle(
            config.x,
            config.y - 5,
            config.width || 200,
            15,
            {
                density: 0.005,
                render: { fillStyle: '#8b4513' }
            }
        );

        const constraint = Constraint.create({
            bodyA: pivot,
            bodyB: plank,
            pointB: { x: 0, y: 0 },
            stiffness: 1,
            length: 0
        });

        World.add(world, [pivot, plank, constraint]);
    }

    /**
     * Create a pendulum
     */
    createPendulum(config) {
        const world = this.engine.world;

        const anchor = Bodies.circle(config.x, config.y, 5, {
            isStatic: true,
            render: { fillStyle: '#666' }
        });

        const bob = Bodies.circle(
            config.x,
            config.y + config.length,
            config.radius || 20,
            {
                density: 0.01,
                restitution: 0.9,
                render: { fillStyle: '#9b59b6' }
            }
        );

        const constraint = Constraint.create({
            bodyA: anchor,
            bodyB: bob,
            length: config.length,
            stiffness: 1
        });

        // Give initial velocity if specified
        if (config.initialVelocity) {
            Body.setVelocity(bob, config.initialVelocity);
        }

        World.add(world, [anchor, bob, constraint]);
    }

    /**
     * Setup object palette for player to place objects
     */
    setupObjectPalette(availableObjects) {
        const palette = document.getElementById('objectPalette');
        palette.innerHTML = '';

        availableObjects.forEach(obj => {
            const btn = document.createElement('button');
            btn.className = 'palette-item';
            btn.innerHTML = `${obj.name} <span class="count">(${obj.count})</span>`;
            btn.dataset.type = obj.type;
            btn.dataset.remaining = obj.count;

            btn.onclick = () => {
                if (btn.dataset.remaining > 0) {
                    this.selectPaletteItem(obj.type, btn);
                }
            };

            palette.appendChild(btn);
        });
    }

    /**
     * Select object from palette
     */
    selectPaletteItem(type, btn) {
        // Remove previous selection
        document.querySelectorAll('.palette-item').forEach(item => {
            item.classList.remove('selected');
        });

        btn.classList.add('selected');
        this.selectedObject = {
            type: type,
            button: btn
        };

        document.getElementById('levelInstructions').textContent =
            'Click on canvas to place object';
    }

    /**
     * Handle mouse down events
     */
    handleMouseDown(event) {
        const mousePosition = event.mouse.position;

        // Check if clicking on a rope
        const bodies = Matter.Query.point(Composite.allBodies(this.engine.world), mousePosition);
        for (let body of bodies) {
            if (body.label === 'rope' && body.ropeConstraint) {
                this.cutRope(body.ropeConstraint);
                this.incrementMoves();
                return;
            }

            if (body.label === 'bomb') {
                this.explodeBomb(body);
                this.incrementMoves();
                return;
            }
        }

        // Place selected object
        if (this.selectedObject && this.isLevelStarted) {
            this.placeObject(mousePosition);
        }
    }

    /**
     * Place object on canvas
     */
    placeObject(position) {
        const world = this.engine.world;
        let body;

        switch(this.selectedObject.type) {
            case 'platform':
                body = Bodies.rectangle(position.x, position.y, 100, 15, {
                    isStatic: true,
                    render: { fillStyle: '#533483' }
                });
                break;
            case 'ramp':
                body = Bodies.rectangle(position.x, position.y, 120, 15, {
                    isStatic: true,
                    angle: Math.PI / 6,
                    render: { fillStyle: '#533483' }
                });
                break;
            case 'box':
                body = Bodies.rectangle(position.x, position.y, 40, 40, {
                    density: 0.001,
                    render: { fillStyle: '#e74c3c' }
                });
                break;
            case 'circle':
                body = Bodies.circle(position.x, position.y, 20, {
                    density: 0.001,
                    restitution: 0.8,
                    render: { fillStyle: '#3498db' }
                });
                break;
        }

        if (body) {
            World.add(world, body);
            this.placedObjects.push(body);

            // Update count
            const btn = this.selectedObject.button;
            btn.dataset.remaining--;
            btn.innerHTML = btn.innerHTML.replace(/\d+/, btn.dataset.remaining);

            if (btn.dataset.remaining <= 0) {
                btn.disabled = true;
                btn.classList.remove('selected');
                this.selectedObject = null;
                document.getElementById('levelInstructions').textContent =
                    'All objects placed';
            }

            this.incrementMoves();

            // Save to history for undo
            this.history.push({
                type: 'place',
                body: body,
                button: btn
            });
        }
    }

    /**
     * Cut a rope constraint
     */
    cutRope(constraint) {
        World.remove(this.engine.world, constraint);

        // Visual feedback
        this.createParticles(constraint.bodyB.position, '#8b4513', 5);

        // Save to history
        this.history.push({
            type: 'cut',
            constraint: constraint
        });
    }

    /**
     * Explode a bomb
     */
    explodeBomb(bomb) {
        const bodies = Composite.allBodies(this.engine.world);
        const explosionCenter = bomb.position;

        // Apply forces to nearby objects
        bodies.forEach(body => {
            if (body === bomb || body.isStatic) return;

            const distance = Vector.magnitude(
                Vector.sub(body.position, explosionCenter)
            );

            if (distance < bomb.bombRadius) {
                const force = Vector.mult(
                    Vector.normalise(Vector.sub(body.position, explosionCenter)),
                    bomb.bombForce * (1 - distance / bomb.bombRadius)
                );
                Body.applyForce(body, body.position, force);
            }
        });

        // Visual effects
        this.createParticles(explosionCenter, '#ff6b00', 20);

        // Remove bomb
        World.remove(this.engine.world, bomb);

        // Save to history
        this.history.push({
            type: 'explode',
            bomb: bomb
        });
    }

    /**
     * Create particle effects
     */
    createParticles(position, color, count) {
        for (let i = 0; i < count; i++) {
            // Enforce particle limit
            if (this.particles.length >= MAX_PARTICLES) {
                break;
            }

            const angle = (Math.PI * 2 * i) / count;
            const velocity = {
                x: Math.cos(angle) * (2 + Math.random() * 3),
                y: Math.sin(angle) * (2 + Math.random() * 3)
            };

            this.particles.push({
                x: position.x,
                y: position.y,
                vx: velocity.x,
                vy: velocity.y,
                life: 1,
                color: color,
                size: 3 + Math.random() * 3
            });
        }
    }

    /**
     * Update particles animation
     */
    updateParticles() {
        const ctx = this.render.context;

        this.particles = this.particles.filter(particle => {
            particle.x += particle.vx;
            particle.y += particle.vy;
            particle.vy += 0.2; // Gravity
            particle.life -= 0.02;

            if (particle.life > 0) {
                ctx.globalAlpha = particle.life;
                ctx.fillStyle = particle.color;
                ctx.beginPath();
                ctx.arc(particle.x, particle.y, particle.size, 0, Math.PI * 2);
                ctx.fill();
                ctx.globalAlpha = 1;
                return true;
            }
            return false;
        });
    }

    /**
     * Start level (release golden ball)
     */
    startLevel() {
        if (this.isLevelStarted) return;

        this.isLevelStarted = true;
        Body.setStatic(this.goldenBall, false);

        // Start timer
        this.startTime = Date.now();
        this.timerInterval = setInterval(() => {
            this.elapsedTime = Math.floor((Date.now() - this.startTime) / 1000);
            document.getElementById('timeCount').textContent = this.elapsedTime;
        }, 1000);

        document.getElementById('levelInstructions').textContent =
            'Guide the golden ball to the star!';
    }

    /**
     * Handle collisions
     */
    handleCollision(event) {
        const pairs = event.pairs;

        for (let pair of pairs) {
            const bodyA = pair.bodyA;
            const bodyB = pair.bodyB;

            // Check if golden ball touches target
            if ((bodyA.label === 'goldenBall' && bodyB.label === 'targetStar') ||
                (bodyB.label === 'goldenBall' && bodyA.label === 'targetStar')) {
                this.completeLevel();
            }
        }
    }

    /**
     * Complete the level
     */
    completeLevel() {
        if (this.isLevelComplete) return;

        this.isLevelComplete = true;

        // Stop timer
        if (this.timerInterval) {
            clearInterval(this.timerInterval);
        }

        // Calculate stars based on time and moves
        const stars = this.calculateStars(this.elapsedTime, this.moveCount);

        // Save progress
        this.levelStars[this.currentLevel] = Math.max(
            this.levelStars[this.currentLevel] || 0,
            stars
        );

        if (this.currentLevel >= this.maxUnlockedLevel) {
            this.maxUnlockedLevel = this.currentLevel + 1;
        }

        this.saveProgress();

        // Show win screen
        this.showWinScreen(stars);

        // Create celebration particles
        this.createParticles(this.targetStar.position, '#ffd700', 30);
    }

    /**
     * Calculate star rating
     */
    calculateStars(time, moves) {
        const levelConfig = this.getLevelConfig(this.currentLevel);
        const thresholds = levelConfig.starThresholds || {
            threeStarTime: 10,
            threeStarMoves: 3,
            twoStarTime: 20,
            twoStarMoves: 6
        };

        if (time <= thresholds.threeStarTime && moves <= thresholds.threeStarMoves) {
            return 3;
        } else if (time <= thresholds.twoStarTime && moves <= thresholds.twoStarMoves) {
            return 2;
        } else {
            return 1;
        }
    }

    /**
     * Show win screen overlay
     */
    showWinScreen(stars) {
        const overlay = document.getElementById('winOverlay');
        overlay.classList.add('active');

        document.getElementById('winTime').textContent = this.elapsedTime;
        document.getElementById('winMoves').textContent = this.moveCount;

        // Animate stars
        const starElements = document.querySelectorAll('#winStars .star-big');
        starElements.forEach((star, index) => {
            star.classList.remove('earned');
            if (index < stars) {
                setTimeout(() => {
                    star.classList.add('earned');
                }, index * 300);
            }
        });
    }

    /**
     * Update stars earned display during gameplay
     */
    updateStarsEarned(count) {
        const stars = document.querySelectorAll('#starsEarned .star');
        stars.forEach((star, index) => {
            star.classList.remove('earned');
            if (index < count) {
                star.classList.add('earned');
            }
        });
    }

    /**
     * Increment move counter
     */
    incrementMoves() {
        this.moveCount++;
        document.getElementById('moveCount').textContent = this.moveCount;

        // Update star preview
        const currentStars = this.calculateStars(this.elapsedTime, this.moveCount);
        this.updateStarsEarned(currentStars);
    }

    /**
     * Reset current level
     */
    resetLevel() {
        document.getElementById('winOverlay').classList.remove('active');
        this.loadLevel(this.currentLevel);
    }

    /**
     * Load next level
     */
    nextLevel() {
        const totalLevels = this.getLevels().length;
        if (this.currentLevel < totalLevels) {
            this.loadLevel(this.currentLevel + 1);
        } else {
            this.showLevelSelect();
        }
    }

    /**
     * Undo last action
     */
    undo() {
        if (this.history.length === 0 || !this.isLevelStarted) return;

        const lastAction = this.history.pop();

        switch(lastAction.type) {
            case 'place':
                World.remove(this.engine.world, lastAction.body);
                const index = this.placedObjects.indexOf(lastAction.body);
                if (index > -1) {
                    this.placedObjects.splice(index, 1);
                }

                // Restore object count using stored button reference
                if (lastAction.button) {
                    const btn = lastAction.button;
                    btn.dataset.remaining++;
                    btn.innerHTML = btn.innerHTML.replace(/\d+/, btn.dataset.remaining);
                    btn.disabled = false;
                }
                break;

            case 'cut':
            case 'explode':
                // These actions cannot be undone in this implementation
                // Could be enhanced to store full state
                break;
        }

        this.moveCount = Math.max(0, this.moveCount - 1);
        document.getElementById('moveCount').textContent = this.moveCount;
    }

    /**
     * Update total stars display
     */
    updateStarsDisplay() {
        const totalStars = Object.values(this.levelStars).reduce((sum, stars) => sum + stars, 0);
        const maxStars = this.getLevels().length * 3;

        document.getElementById('totalStars').textContent = totalStars;
        document.getElementById('maxStars').textContent = maxStars;
    }

    /**
     * Save progress to localStorage
     */
    saveProgress() {
        try {
            const progress = {
                maxUnlockedLevel: this.maxUnlockedLevel,
                levelStars: this.levelStars
            };
            localStorage.setItem('chainReactionProgress', JSON.stringify(progress));
        } catch (error) {
            // Handle private browsing mode or quota exceeded gracefully
            console.warn('Failed to save progress to localStorage:', error);
        }
    }

    /**
     * Load progress from localStorage
     */
    loadProgress() {
        try {
            const saved = localStorage.getItem('chainReactionProgress');
            if (saved) {
                const progress = JSON.parse(saved);
                this.maxUnlockedLevel = progress.maxUnlockedLevel || 1;
                this.levelStars = progress.levelStars || {};
            }
        } catch (error) {
            // Handle corrupted data or private browsing mode gracefully
            console.warn('Failed to load progress from localStorage:', error);
            this.maxUnlockedLevel = 1;
            this.levelStars = {};
        }
    }

    /**
     * Get all level configurations
     */
    getLevels() {
        return [
            this.getLevel1(),
            this.getLevel2(),
            this.getLevel3(),
            this.getLevel4(),
            this.getLevel5(),
            this.getLevel6(),
            this.getLevel7(),
            this.getLevel8(),
            this.getLevel9(),
            this.getLevel10(),
            this.getLevel11(),
            this.getLevel12(),
            this.getLevel13(),
            this.getLevel14(),
            this.getLevel15()
        ];
    }

    /**
     * Get specific level configuration
     */
    getLevelConfig(levelNum) {
        const levels = this.getLevels();
        return levels[levelNum - 1] || levels[0];
    }

    // ====== LEVEL DEFINITIONS ======

    /**
     * Level 1: Tutorial - Simple ramp
     */
    getLevel1() {
        return {
            name: "Getting Started",
            instructions: "Press SPACE to release the ball!",
            ballStart: { x: 100, y: 100 },
            target: { x: 700, y: 400 },
            staticObjects: [
                { type: 'rectangle', x: 400, y: 300, width: 400, height: 20, angle: -0.2 }
            ],
            interactiveObjects: [],
            availableObjects: [],
            starThresholds: {
                threeStarTime: 5,
                threeStarMoves: 0,
                twoStarTime: 10,
                twoStarMoves: 0
            }
        };
    }

    /**
     * Level 2: Cut the rope
     */
    getLevel2() {
        return {
            name: "Cut the Rope",
            instructions: "Click the rope to release the weight!",
            ballStart: { x: 100, y: 100 },
            target: { x: 600, y: 450 },
            staticObjects: [
                { type: 'rectangle', x: 200, y: 200, width: 150, height: 20 },
                { type: 'rectangle', x: 500, y: 350, width: 200, height: 20, angle: -0.3 }
            ],
            interactiveObjects: [
                { type: 'rope', x: 250, y: 100, length: 150, segments: 6, hanging: 'ball' }
            ],
            availableObjects: [],
            starThresholds: {
                threeStarTime: 8,
                threeStarMoves: 1,
                twoStarTime: 15,
                twoStarMoves: 2
            }
        };
    }

    /**
     * Level 3: Place a platform
     */
    getLevel3() {
        return {
            name: "Bridge Builder",
            instructions: "Place a platform to bridge the gap!",
            ballStart: { x: 100, y: 100 },
            target: { x: 700, y: 450 },
            staticObjects: [
                { type: 'rectangle', x: 150, y: 200, width: 100, height: 20 },
                { type: 'rectangle', x: 650, y: 350, width: 100, height: 20 }
            ],
            interactiveObjects: [],
            availableObjects: [
                { type: 'platform', name: '📏 Platform', count: 2 }
            ],
            starThresholds: {
                threeStarTime: 10,
                threeStarMoves: 1,
                twoStarTime: 20,
                twoStarMoves: 3
            }
        };
    }

    /**
     * Level 4: Seesaw challenge
     */
    getLevel4() {
        return {
            name: "Balance Act",
            instructions: "Use the seesaw to launch the ball!",
            ballStart: { x: 100, y: 100 },
            target: { x: 700, y: 150 },
            staticObjects: [
                { type: 'rectangle', x: 200, y: 200, width: 100, height: 20 }
            ],
            interactiveObjects: [
                { type: 'seesaw', x: 400, y: 400, width: 300 },
                { type: 'rope', x: 600, y: 100, length: 200, segments: 8, hanging: 'box' }
            ],
            availableObjects: [],
            starThresholds: {
                threeStarTime: 12,
                threeStarMoves: 1,
                twoStarTime: 20,
                twoStarMoves: 2
            }
        };
    }

    /**
     * Level 5: Bomb blast
     */
    getLevel5() {
        return {
            name: "Explosive Solution",
            instructions: "Click the bomb to clear the path!",
            ballStart: { x: 100, y: 100 },
            target: { x: 700, y: 450 },
            staticObjects: [
                { type: 'rectangle', x: 200, y: 200, width: 100, height: 20 },
                { type: 'rectangle', x: 600, y: 380, width: 200, height: 20, angle: 0.3 }
            ],
            interactiveObjects: [
                { type: 'bomb', x: 400, y: 300, explosionRadius: 200, explosionForce: 0.08 },
                { type: 'domino', x: 350, y: 275 },
                { type: 'domino', x: 370, y: 275 },
                { type: 'domino', x: 390, y: 275 },
                { type: 'domino', x: 410, y: 275 },
                { type: 'domino', x: 430, y: 275 },
                { type: 'domino', x: 450, y: 275 }
            ],
            availableObjects: [],
            starThresholds: {
                threeStarTime: 10,
                threeStarMoves: 1,
                twoStarTime: 18,
                twoStarMoves: 2
            }
        };
    }

    /**
     * Level 6: Domino chain
     */
    getLevel6() {
        return {
            name: "Chain Reaction",
            instructions: "Start the domino chain!",
            ballStart: { x: 80, y: 100 },
            target: { x: 750, y: 450 },
            staticObjects: [
                { type: 'rectangle', x: 150, y: 200, width: 120, height: 20 },
                { type: 'rectangle', x: 250, y: 300, width: 20, height: 100 },
                { type: 'rectangle', x: 350, y: 300, width: 200, height: 20 },
                { type: 'rectangle', x: 550, y: 300, width: 20, height: 100 },
                { type: 'rectangle', x: 650, y: 400, width: 200, height: 20 }
            ],
            interactiveObjects: [
                { type: 'domino', x: 200, y: 175 },
                { type: 'domino', x: 220, y: 175 },
                { type: 'domino', x: 240, y: 175 },
                { type: 'domino', x: 300, y: 275 },
                { type: 'domino', x: 320, y: 275 },
                { type: 'domino', x: 340, y: 275 },
                { type: 'domino', x: 400, y: 275 },
                { type: 'domino', x: 420, y: 275 },
                { type: 'domino', x: 440, y: 275 },
                { type: 'domino', x: 600, y: 375 },
                { type: 'domino', x: 620, y: 375 },
                { type: 'domino', x: 640, y: 375 }
            ],
            availableObjects: [],
            starThresholds: {
                threeStarTime: 15,
                threeStarMoves: 0,
                twoStarTime: 25,
                twoStarMoves: 1
            }
        };
    }

    /**
     * Level 7: Pendulum push
     */
    getLevel7() {
        return {
            name: "Pendulum Push",
            instructions: "Time the pendulum swing!",
            ballStart: { x: 100, y: 100 },
            target: { x: 700, y: 200 },
            staticObjects: [
                { type: 'rectangle', x: 150, y: 200, width: 100, height: 20 },
                { type: 'rectangle', x: 500, y: 300, width: 150, height: 20 }
            ],
            interactiveObjects: [
                {
                    type: 'pendulum',
                    x: 350,
                    y: 100,
                    length: 180,
                    radius: 25,
                    initialVelocity: { x: -5, y: 0 }
                }
            ],
            availableObjects: [
                { type: 'ramp', name: '📐 Ramp', count: 1 }
            ],
            starThresholds: {
                threeStarTime: 12,
                threeStarMoves: 1,
                twoStarTime: 20,
                twoStarMoves: 2
            }
        };
    }

    /**
     * Level 8: Multi-path puzzle
     */
    getLevel8() {
        return {
            name: "Choose Your Path",
            instructions: "Create the right path!",
            ballStart: { x: 100, y: 100 },
            target: { x: 700, y: 450 },
            staticObjects: [
                { type: 'rectangle', x: 150, y: 200, width: 100, height: 20 },
                { type: 'rectangle', x: 300, y: 280, width: 20, height: 150 },
                { type: 'rectangle', x: 500, y: 280, width: 20, height: 150 },
                { type: 'rectangle', x: 650, y: 350, width: 100, height: 20 }
            ],
            interactiveObjects: [
                { type: 'bomb', x: 400, y: 200, explosionRadius: 120, explosionForce: 0.06 }
            ],
            availableObjects: [
                { type: 'platform', name: '📏 Platform', count: 2 },
                { type: 'ramp', name: '📐 Ramp', count: 1 }
            ],
            starThresholds: {
                threeStarTime: 15,
                threeStarMoves: 2,
                twoStarTime: 25,
                twoStarMoves: 4
            }
        };
    }

    /**
     * Level 9: Rope maze
     */
    getLevel9() {
        return {
            name: "Rope Maze",
            instructions: "Cut the right ropes!",
            ballStart: { x: 100, y: 100 },
            target: { x: 700, y: 450 },
            staticObjects: [
                { type: 'rectangle', x: 150, y: 200, width: 100, height: 20 },
                { type: 'rectangle', x: 400, y: 350, width: 150, height: 20 },
                { type: 'rectangle', x: 650, y: 400, width: 100, height: 20 }
            ],
            interactiveObjects: [
                { type: 'rope', x: 200, y: 100, length: 120, segments: 6, hanging: 'box' },
                { type: 'rope', x: 300, y: 100, length: 180, segments: 8, hanging: 'ball' },
                { type: 'rope', x: 450, y: 200, length: 100, segments: 5, hanging: 'box' },
                { type: 'rope', x: 550, y: 200, length: 150, segments: 7, hanging: 'ball' }
            ],
            availableObjects: [],
            starThresholds: {
                threeStarTime: 12,
                threeStarMoves: 2,
                twoStarTime: 20,
                twoStarMoves: 4
            }
        };
    }

    /**
     * Level 10: Complex machine
     */
    getLevel10() {
        return {
            name: "Rube Goldberg",
            instructions: "Set off the chain reaction!",
            ballStart: { x: 80, y: 100 },
            target: { x: 750, y: 450 },
            staticObjects: [
                { type: 'rectangle', x: 120, y: 180, width: 80, height: 20, angle: -0.3 },
                { type: 'rectangle', x: 350, y: 300, width: 20, height: 120 },
                { type: 'rectangle', x: 550, y: 300, width: 20, height: 120 }
            ],
            interactiveObjects: [
                { type: 'seesaw', x: 220, y: 280, width: 160 },
                { type: 'rope', x: 160, y: 100, length: 140, segments: 7, hanging: 'box' },
                { type: 'domino', x: 300, y: 255 },
                { type: 'domino', x: 320, y: 255 },
                { type: 'domino', x: 340, y: 255 },
                { type: 'bomb', x: 450, y: 250, explosionRadius: 150, explosionForce: 0.07 },
                { type: 'domino', x: 500, y: 255 },
                { type: 'domino', x: 520, y: 255 },
                { type: 'domino', x: 540, y: 255 },
                { type: 'pendulum', x: 650, y: 200, length: 150, radius: 20, initialVelocity: { x: 0, y: 0 } }
            ],
            availableObjects: [
                { type: 'platform', name: '📏 Platform', count: 1 }
            ],
            starThresholds: {
                threeStarTime: 20,
                threeStarMoves: 2,
                twoStarTime: 35,
                twoStarMoves: 4
            }
        };
    }

    /**
     * Level 11: Bounce house
     */
    getLevel11() {
        return {
            name: "Bounce House",
            instructions: "Use bouncy circles to reach the target!",
            ballStart: { x: 100, y: 100 },
            target: { x: 700, y: 100 },
            staticObjects: [
                { type: 'rectangle', x: 150, y: 200, width: 100, height: 20 },
                { type: 'rectangle', x: 400, y: 450, width: 200, height: 20 }
            ],
            interactiveObjects: [],
            availableObjects: [
                { type: 'circle', name: '⚽ Bouncy Ball', count: 3 },
                { type: 'platform', name: '📏 Platform', count: 1 }
            ],
            starThresholds: {
                threeStarTime: 12,
                threeStarMoves: 3,
                twoStarTime: 20,
                twoStarMoves: 5
            }
        };
    }

    /**
     * Level 12: Stairway to heaven
     */
    getLevel12() {
        return {
            name: "Stairway Challenge",
            instructions: "Build stairs to climb up!",
            ballStart: { x: 100, y: 450 },
            target: { x: 700, y: 100 },
            staticObjects: [
                { type: 'rectangle', x: 150, y: 480, width: 100, height: 20 }
            ],
            interactiveObjects: [],
            availableObjects: [
                { type: 'platform', name: '📏 Platform', count: 5 },
                { type: 'box', name: '📦 Box', count: 2 }
            ],
            starThresholds: {
                threeStarTime: 18,
                threeStarMoves: 5,
                twoStarTime: 30,
                twoStarMoves: 8
            }
        };
    }

    /**
     * Level 13: Timing is everything
     */
    getLevel13() {
        return {
            name: "Perfect Timing",
            instructions: "Time your moves perfectly!",
            ballStart: { x: 100, y: 100 },
            target: { x: 700, y: 450 },
            staticObjects: [
                { type: 'rectangle', x: 150, y: 200, width: 100, height: 20 },
                { type: 'rectangle', x: 650, y: 400, width: 100, height: 20 }
            ],
            interactiveObjects: [
                { type: 'pendulum', x: 300, y: 100, length: 150, radius: 20, initialVelocity: { x: 0, y: 5 } },
                { type: 'pendulum', x: 450, y: 100, length: 180, radius: 20, initialVelocity: { x: 0, y: -5 } },
                { type: 'seesaw', x: 550, y: 320, width: 180 },
                { type: 'rope', x: 500, y: 100, length: 150, segments: 7, hanging: 'box' }
            ],
            availableObjects: [],
            starThresholds: {
                threeStarTime: 15,
                threeStarMoves: 1,
                twoStarTime: 25,
                twoStarMoves: 2
            }
        };
    }

    /**
     * Level 14: Chaos theory
     */
    getLevel14() {
        return {
            name: "Controlled Chaos",
            instructions: "Master the chaos!",
            ballStart: { x: 100, y: 100 },
            target: { x: 700, y: 450 },
            staticObjects: [
                { type: 'rectangle', x: 150, y: 200, width: 100, height: 20 }
            ],
            interactiveObjects: [
                { type: 'domino', x: 200, y: 175 },
                { type: 'domino', x: 220, y: 175 },
                { type: 'bomb', x: 270, y: 250, explosionRadius: 140, explosionForce: 0.06 },
                { type: 'rope', x: 320, y: 100, length: 160, segments: 8, hanging: 'ball' },
                { type: 'seesaw', x: 450, y: 350, width: 200 },
                { type: 'rope', x: 550, y: 150, length: 130, segments: 6, hanging: 'box' },
                { type: 'pendulum', x: 600, y: 100, length: 120, radius: 18, initialVelocity: { x: -4, y: 0 } }
            ],
            availableObjects: [
                { type: 'platform', name: '📏 Platform', count: 2 },
                { type: 'ramp', name: '📐 Ramp', count: 1 }
            ],
            starThresholds: {
                threeStarTime: 18,
                threeStarMoves: 3,
                twoStarTime: 30,
                twoStarMoves: 5
            }
        };
    }

    /**
     * Level 15: Final challenge
     */
    getLevel15() {
        return {
            name: "Ultimate Challenge",
            instructions: "The final test! Good luck!",
            ballStart: { x: 100, y: 100 },
            target: { x: 750, y: 100 },
            staticObjects: [
                { type: 'rectangle', x: 150, y: 200, width: 100, height: 20 },
                { type: 'rectangle', x: 300, y: 350, width: 20, height: 180 },
                { type: 'rectangle', x: 500, y: 350, width: 20, height: 180 }
            ],
            interactiveObjects: [
                { type: 'rope', x: 200, y: 100, length: 140, segments: 7, hanging: 'box' },
                { type: 'domino', x: 260, y: 325 },
                { type: 'domino', x: 280, y: 325 },
                { type: 'bomb', x: 400, y: 300, explosionRadius: 160, explosionForce: 0.08 },
                { type: 'domino', x: 460, y: 325 },
                { type: 'domino', x: 480, y: 325 },
                { type: 'rope', x: 540, y: 200, length: 120, segments: 6, hanging: 'ball' },
                { type: 'seesaw', x: 650, y: 280, width: 180 },
                { type: 'pendulum', x: 700, y: 150, length: 100, radius: 20, initialVelocity: { x: 0, y: 0 } }
            ],
            availableObjects: [
                { type: 'platform', name: '📏 Platform', count: 3 },
                { type: 'ramp', name: '📐 Ramp', count: 2 },
                { type: 'box', name: '📦 Box', count: 1 }
            ],
            starThresholds: {
                threeStarTime: 25,
                threeStarMoves: 4,
                twoStarTime: 40,
                twoStarMoves: 7
            }
        };
    }
}

// Initialize game when page loads
let game;
window.addEventListener('DOMContentLoaded', () => {
    game = new PhysicsGame();
});
