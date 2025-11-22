/**
 * Main Application
 * Handles initialization, game loop, and user interactions
 */

class PhysicsSandbox {
    constructor() {
        // Canvas setup
        this.canvas = document.getElementById('canvas');
        this.resizeCanvas();

        // Initialize systems
        this.physicsEngine = new PhysicsEngine(this.canvas.width, this.canvas.height);
        this.renderer = new Renderer(this.canvas);

        // Interaction state
        this.mouse = {
            x: 0,
            y: 0,
            prevX: 0,
            prevY: 0,
            down: false,
            dragging: false,
            draggedObject: null
        };

        // UI state
        this.selectedTool = 'circle'; // 'circle' or 'box'
        this.spawnSize = 30;
        this.isPaused = false;

        // FPS tracking
        this.fps = 60;
        this.frameCount = 0;
        this.lastFpsUpdate = Date.now();

        // Time tracking
        this.lastTime = performance.now();
        this.targetFPS = 60;
        this.fixedDeltaTime = 1 / this.targetFPS;

        // Setup event listeners
        this.setupEventListeners();

        // Add some initial objects for demo
        this.initializeDemo();

        // Start game loop
        this.gameLoop();
    }

    resizeCanvas() {
        this.canvas.width = window.innerWidth;
        this.canvas.height = window.innerHeight;

        if (this.physicsEngine) {
            this.physicsEngine.width = this.canvas.width;
            this.physicsEngine.height = this.canvas.height;
        }

        if (this.renderer) {
            this.renderer.resize(this.canvas.width, this.canvas.height);
        }
    }

    setupEventListeners() {
        // Mouse events
        this.canvas.addEventListener('mousedown', (e) => this.handleMouseDown(e));
        this.canvas.addEventListener('mousemove', (e) => this.handleMouseMove(e));
        this.canvas.addEventListener('mouseup', (e) => this.handleMouseUp(e));
        this.canvas.addEventListener('mouseleave', (e) => this.handleMouseUp(e));

        // Touch events for mobile
        this.canvas.addEventListener('touchstart', (e) => this.handleTouchStart(e));
        this.canvas.addEventListener('touchmove', (e) => this.handleTouchMove(e));
        this.canvas.addEventListener('touchend', (e) => this.handleTouchEnd(e));

        // Window resize
        window.addEventListener('resize', () => this.resizeCanvas());

        // Keyboard shortcuts
        window.addEventListener('keydown', (e) => this.handleKeyDown(e));

        // UI Controls
        this.setupUIControls();
    }

    setupUIControls() {
        // Tool selection
        document.getElementById('tool-circle').addEventListener('click', () => {
            this.selectedTool = 'circle';
            this.updateToolButtons();
        });

        document.getElementById('tool-box').addEventListener('click', () => {
            this.selectedTool = 'box';
            this.updateToolButtons();
        });

        // Size slider
        document.getElementById('size-slider').addEventListener('input', (e) => {
            this.spawnSize = parseInt(e.target.value);
            document.getElementById('size-value').textContent = this.spawnSize;
        });

        // Control buttons
        document.getElementById('btn-gravity').addEventListener('click', () => {
            this.physicsEngine.toggleGravity();
            this.updateGravityButton();
        });

        document.getElementById('btn-clear').addEventListener('click', () => {
            this.physicsEngine.clear();
        });

        document.getElementById('btn-pause').addEventListener('click', () => {
            this.isPaused = !this.isPaused;
            this.updatePauseButton();
        });

        document.getElementById('btn-trails').addEventListener('click', () => {
            this.renderer.toggleTrails();
            this.updateTrailsButton();
        });

        document.getElementById('btn-spawn-rain').addEventListener('click', () => {
            this.spawnRain();
        });
    }

    updateToolButtons() {
        document.getElementById('tool-circle').classList.toggle('active', this.selectedTool === 'circle');
        document.getElementById('tool-box').classList.toggle('active', this.selectedTool === 'box');
    }

    updateGravityButton() {
        const btn = document.getElementById('btn-gravity');
        btn.textContent = this.physicsEngine.enableGravity ? 'Gravity: ON' : 'Gravity: OFF';
        btn.classList.toggle('active', this.physicsEngine.enableGravity);
    }

    updatePauseButton() {
        const btn = document.getElementById('btn-pause');
        btn.textContent = this.isPaused ? 'Resume' : 'Pause';
    }

    updateTrailsButton() {
        const btn = document.getElementById('btn-trails');
        btn.textContent = this.renderer.enableTrails ? 'Trails: ON' : 'Trails: OFF';
        btn.classList.toggle('active', this.renderer.enableTrails);
    }

    initializeDemo() {
        // Spawn a few demo objects
        const centerX = this.canvas.width / 2;
        const centerY = this.canvas.height / 3;

        // Create a pyramid of circles
        for (let row = 0; row < 4; row++) {
            for (let col = 0; col <= row; col++) {
                const x = centerX + (col - row / 2) * 60;
                const y = centerY + row * 60;
                const circle = new Circle(x, y, 25, 1);
                this.physicsEngine.addObject(circle);
            }
        }

        // Add a few boxes
        for (let i = 0; i < 3; i++) {
            const x = Math.random() * this.canvas.width;
            const y = 100;
            const box = new Box(x, y, 40, 40, 1.5);
            this.physicsEngine.addObject(box);
        }
    }

    handleMouseDown(e) {
        const rect = this.canvas.getBoundingClientRect();
        this.mouse.prevX = this.mouse.x;
        this.mouse.prevY = this.mouse.y;
        this.mouse.x = e.clientX - rect.left;
        this.mouse.y = e.clientY - rect.top;
        this.mouse.down = true;

        // Check if clicking on an existing object
        const clickedObject = this.physicsEngine.getObjectAtPoint(this.mouse.x, this.mouse.y);

        if (clickedObject) {
            // Start dragging
            this.mouse.dragging = true;
            this.mouse.draggedObject = clickedObject;
            clickedObject.isGrabbed = true;
            this.mouse.dragOffsetX = this.mouse.x - clickedObject.position.x;
            this.mouse.dragOffsetY = this.mouse.y - clickedObject.position.y;
        } else {
            // Spawn new object
            this.spawnObject(this.mouse.x, this.mouse.y);
        }
    }

    handleMouseMove(e) {
        const rect = this.canvas.getBoundingClientRect();
        this.mouse.prevX = this.mouse.x;
        this.mouse.prevY = this.mouse.y;
        this.mouse.x = e.clientX - rect.left;
        this.mouse.y = e.clientY - rect.top;

        if (this.mouse.dragging && this.mouse.draggedObject) {
            // Update dragged object position
            this.mouse.draggedObject.position.x = this.mouse.x - this.mouse.dragOffsetX;
            this.mouse.draggedObject.position.y = this.mouse.y - this.mouse.dragOffsetY;
        }
    }

    handleMouseUp(e) {
        if (this.mouse.draggedObject) {
            this.mouse.draggedObject.isGrabbed = false;

            // Apply velocity based on mouse movement (throw effect)
            if (this.mouse.dragging && e.clientX !== undefined) {
                const rect = this.canvas.getBoundingClientRect();
                const currentX = e.clientX - rect.left;
                const currentY = e.clientY - rect.top;

                // Use the velocity between previous and current mouse position
                const throwVelocityX = (currentX - this.mouse.prevX) * 20;
                const throwVelocityY = (currentY - this.mouse.prevY) * 20;

                this.mouse.draggedObject.velocity = new Vector2(throwVelocityX, throwVelocityY);
            }
        }

        this.mouse.down = false;
        this.mouse.dragging = false;
        this.mouse.draggedObject = null;
    }

    handleTouchStart(e) {
        e.preventDefault();
        const touch = e.touches[0];
        const mouseEvent = new MouseEvent('mousedown', {
            clientX: touch.clientX,
            clientY: touch.clientY
        });
        this.handleMouseDown(mouseEvent);
    }

    handleTouchMove(e) {
        e.preventDefault();
        const touch = e.touches[0];
        const mouseEvent = new MouseEvent('mousemove', {
            clientX: touch.clientX,
            clientY: touch.clientY
        });
        this.handleMouseMove(mouseEvent);
    }

    handleTouchEnd(e) {
        e.preventDefault();
        // Use the last known touch position for throw velocity
        const lastTouch = e.changedTouches[0] || { clientX: this.mouse.x, clientY: this.mouse.y };
        this.handleMouseUp(new MouseEvent('mouseup', {
            clientX: lastTouch.clientX,
            clientY: lastTouch.clientY
        }));
    }

    handleKeyDown(e) {
        switch(e.key) {
            case '1':
                this.selectedTool = 'circle';
                this.updateToolButtons();
                break;
            case '2':
                this.selectedTool = 'box';
                this.updateToolButtons();
                break;
            case 'g':
            case 'G':
                this.physicsEngine.toggleGravity();
                this.updateGravityButton();
                break;
            case 'c':
            case 'C':
                this.physicsEngine.clear();
                break;
            case ' ':
                e.preventDefault();
                this.isPaused = !this.isPaused;
                this.updatePauseButton();
                break;
            case 't':
            case 'T':
                this.renderer.toggleTrails();
                this.updateTrailsButton();
                break;
            case 'r':
            case 'R':
                this.spawnRain();
                break;
        }
    }

    spawnObject(x, y) {
        let obj;

        if (this.selectedTool === 'circle') {
            const mass = (this.spawnSize / 30) * (this.spawnSize / 30);
            obj = new Circle(x, y, this.spawnSize, mass);
        } else if (this.selectedTool === 'box') {
            const mass = (this.spawnSize / 30) * (this.spawnSize / 30) * 1.5;
            obj = new Box(x, y, this.spawnSize * 2, this.spawnSize * 2, mass);
        }

        if (obj) {
            this.physicsEngine.addObject(obj);
        }
    }

    spawnRain() {
        const count = 50;
        for (let i = 0; i < count; i++) {
            setTimeout(() => {
                const x = Math.random() * this.canvas.width;
                const y = -50;
                const radius = 10 + Math.random() * 15;
                const mass = (radius / 30) * (radius / 30);
                const circle = new Circle(x, y, radius, mass);
                circle.velocity = new Vector2(0, Math.random() * 200 + 100);
                this.physicsEngine.addObject(circle);
            }, i * 50);
        }
    }

    update(deltaTime) {
        if (!this.isPaused) {
            this.physicsEngine.update(deltaTime);
        }
    }

    render() {
        this.renderer.render(this.physicsEngine);
        this.renderUI();
    }

    renderUI() {
        const ctx = this.renderer.ctx;

        // Draw FPS counter
        ctx.fillStyle = 'rgba(255, 255, 255, 0.9)';
        ctx.font = 'bold 18px monospace';
        ctx.fillText(`FPS: ${this.fps}`, 10, 30);

        // Draw object count
        ctx.fillText(`Objects: ${this.physicsEngine.objects.length}`, 10, 55);

        // Draw instructions
        ctx.font = '14px monospace';
        ctx.fillStyle = 'rgba(255, 255, 255, 0.7)';
        ctx.fillText('Click: Spawn | Drag: Move | Space: Pause', 10, this.canvas.height - 20);
    }

    updateFPS() {
        this.frameCount++;
        const now = Date.now();

        if (now - this.lastFpsUpdate >= 1000) {
            this.fps = this.frameCount;
            this.frameCount = 0;
            this.lastFpsUpdate = now;
        }
    }

    gameLoop() {
        const currentTime = performance.now();
        const deltaTime = Math.min((currentTime - this.lastTime) / 1000, 0.1); // Cap at 100ms
        this.lastTime = currentTime;

        // Update physics
        this.update(this.fixedDeltaTime);

        // Render
        this.render();

        // Update FPS
        this.updateFPS();

        // Continue loop
        requestAnimationFrame(() => this.gameLoop());
    }
}

// Initialize the application when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    new PhysicsSandbox();
});
