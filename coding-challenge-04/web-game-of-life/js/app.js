/**
 * Main Application Controller
 * Connects UI, game logic, and renderer
 */

class GameOfLifeApp {
    constructor() {
        // Initialize game and renderer
        this.game = new GameOfLife(50, 50);
        this.canvas = document.getElementById('gameCanvas');
        this.renderer = new GameRenderer(this.canvas, this.game);

        // Simulation state
        this.isRunning = false;
        this.simulationSpeed = 10; // generations per second
        this.lastStepTime = 0;

        // Drawing state
        this.isDrawing = false;
        this.drawMode = 'draw'; // 'draw' or 'erase'

        // Animation frame
        this.animationId = null;

        this.initializeUI();
        this.setupEventListeners();
        this.render();
    }

    /**
     * Initialize UI elements
     */
    initializeUI() {
        this.updateStats();
    }

    /**
     * Setup all event listeners
     */
    setupEventListeners() {
        // Playback controls
        document.getElementById('playPauseBtn').addEventListener('click', () => this.togglePlayPause());
        document.getElementById('stepBtn').addEventListener('click', () => this.step());
        document.getElementById('clearBtn').addEventListener('click', () => this.clear());
        document.getElementById('randomBtn').addEventListener('click', () => this.randomize());

        // Speed control
        const speedSlider = document.getElementById('speedSlider');
        speedSlider.addEventListener('input', (e) => {
            this.simulationSpeed = parseInt(e.target.value);
            document.getElementById('speedValue').textContent = this.simulationSpeed;
        });

        // Grid size buttons
        document.querySelectorAll('[data-size]').forEach(btn => {
            btn.addEventListener('click', (e) => {
                const size = parseInt(e.target.dataset.size);
                this.resizeGrid(size, size);

                // Update active button
                document.querySelectorAll('[data-size]').forEach(b => b.classList.remove('active'));
                e.target.classList.add('active');
            });
        });

        // Zoom controls
        document.getElementById('zoomInBtn').addEventListener('click', () => this.zoomIn());
        document.getElementById('zoomOutBtn').addEventListener('click', () => this.zoomOut());
        document.getElementById('zoomResetBtn').addEventListener('click', () => this.zoomReset());

        // Pattern buttons
        document.querySelectorAll('.pattern-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                const patternName = e.target.dataset.pattern;
                this.loadPattern(patternName);
            });
        });

        // Drawing mode
        document.getElementById('drawBtn').addEventListener('click', () => this.setDrawMode('draw'));
        document.getElementById('eraseBtn').addEventListener('click', () => this.setDrawMode('erase'));

        // Options checkboxes
        document.getElementById('agingCheckbox').addEventListener('change', (e) => {
            this.renderer.showAging = e.target.checked;
            this.render();
        });

        document.getElementById('gridLinesCheckbox').addEventListener('change', (e) => {
            this.renderer.showGrid = e.target.checked;
            this.render();
        });

        document.getElementById('wrapCheckbox').addEventListener('change', (e) => {
            this.game.wrapAround = e.target.checked;
        });

        // Canvas mouse events
        this.canvas.addEventListener('mousedown', (e) => this.handleMouseDown(e));
        this.canvas.addEventListener('mousemove', (e) => this.handleMouseMove(e));
        this.canvas.addEventListener('mouseup', () => this.handleMouseUp());
        this.canvas.addEventListener('mouseleave', () => this.handleMouseUp());

        // Keyboard shortcuts
        document.addEventListener('keydown', (e) => this.handleKeyPress(e));
    }

    /**
     * Handle mouse down on canvas
     */
    handleMouseDown(e) {
        this.isDrawing = true;
        this.handleCellClick(e);
    }

    /**
     * Handle mouse move on canvas
     */
    handleMouseMove(e) {
        if (this.isDrawing) {
            this.handleCellClick(e);
        }
    }

    /**
     * Handle mouse up
     */
    handleMouseUp() {
        this.isDrawing = false;
    }

    /**
     * Handle cell click/drag
     */
    handleCellClick(e) {
        const rect = this.canvas.getBoundingClientRect();
        const mouseX = e.clientX - rect.left;
        const mouseY = e.clientY - rect.top;

        const cell = this.renderer.getCellFromMouse(mouseX, mouseY);

        if (cell) {
            if (this.drawMode === 'draw') {
                this.game.setCell(cell.x, cell.y, true);
            } else {
                this.game.setCell(cell.x, cell.y, false);
            }
            this.updateStats();
            this.render();
        }
    }

    /**
     * Handle keyboard shortcuts
     */
    handleKeyPress(e) {
        switch(e.key) {
            case ' ':
                e.preventDefault();
                this.togglePlayPause();
                break;
            case 'Enter':
                e.preventDefault();
                this.step();
                break;
            case 'c':
                if (e.ctrlKey || e.metaKey) {
                    return; // Let browser handle copy
                }
                this.clear();
                break;
            case 'r':
                this.randomize();
                break;
        }
    }

    /**
     * Set drawing mode
     */
    setDrawMode(mode) {
        this.drawMode = mode;

        document.getElementById('drawBtn').classList.toggle('active', mode === 'draw');
        document.getElementById('eraseBtn').classList.toggle('active', mode === 'erase');
    }

    /**
     * Toggle play/pause
     */
    togglePlayPause() {
        this.isRunning = !this.isRunning;

        const btn = document.getElementById('playPauseBtn');
        const icon = document.getElementById('playPauseIcon');

        if (this.isRunning) {
            btn.innerHTML = '<span id="playPauseIcon">⏸</span> Pause';
            this.lastStepTime = performance.now();
            this.animate();
        } else {
            btn.innerHTML = '<span id="playPauseIcon">▶</span> Play';
            if (this.animationId) {
                cancelAnimationFrame(this.animationId);
                this.animationId = null;
            }
        }
    }

    /**
     * Execute one generation step
     */
    step() {
        this.game.step();
        this.updateStats();
        this.render();
    }

    /**
     * Clear all cells
     */
    clear() {
        this.game.clear();
        this.updateStats();
        this.render();
    }

    /**
     * Randomize grid
     */
    randomize() {
        this.game.randomize(0.3);
        this.updateStats();
        this.render();
    }

    /**
     * Resize grid
     */
    resizeGrid(width, height) {
        const wasRunning = this.isRunning;
        if (wasRunning) {
            this.togglePlayPause();
        }

        this.game.resize(width, height);
        this.renderer.resizeCanvas();
        this.updateStats();
        this.render();
    }

    /**
     * Zoom in
     */
    zoomIn() {
        this.renderer.setZoom(this.renderer.zoom * 1.2);
        this.updateZoomDisplay();
        this.render();
    }

    /**
     * Zoom out
     */
    zoomOut() {
        this.renderer.setZoom(this.renderer.zoom / 1.2);
        this.updateZoomDisplay();
        this.render();
    }

    /**
     * Reset zoom
     */
    zoomReset() {
        this.renderer.setZoom(1.0);
        this.updateZoomDisplay();
        this.render();
    }

    /**
     * Update zoom display
     */
    updateZoomDisplay() {
        document.getElementById('zoomLevel').textContent =
            Math.round(this.renderer.zoom * 100) + '%';
    }

    /**
     * Load pattern from library
     */
    loadPattern(patternName) {
        if (PATTERNS[patternName]) {
            this.game.loadPattern(PATTERNS[patternName]);
            this.updateStats();
            this.render();
        }
    }

    /**
     * Animation loop
     */
    animate() {
        if (!this.isRunning) return;

        const currentTime = performance.now();
        const elapsed = currentTime - this.lastStepTime;

        // Step at desired speed (generations per second)
        const stepInterval = 1000 / this.simulationSpeed;

        if (elapsed >= stepInterval) {
            this.game.step();
            this.updateStats();
            this.lastStepTime = currentTime;
        }

        this.render();
        this.animationId = requestAnimationFrame(() => this.animate());
    }

    /**
     * Render current state
     */
    render() {
        this.renderer.render();
        document.getElementById('fps').textContent = this.renderer.getFPS();
    }

    /**
     * Update statistics display
     */
    updateStats() {
        document.getElementById('generation').textContent = this.game.generation;
        document.getElementById('population').textContent = this.game.getPopulation();
    }
}

// Initialize app when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    window.app = new GameOfLifeApp();
});
