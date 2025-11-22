/**
 * Canvas Renderer for Game of Life
 * Handles smooth 60 FPS rendering with age-based coloring and zoom
 */

class GameRenderer {
    constructor(canvas, game) {
        this.canvas = canvas;
        this.ctx = canvas.getContext('2d');
        this.game = game;

        // Display options
        this.cellSize = 10;
        this.zoom = 1.0;
        this.showGrid = true;
        this.showAging = true;

        // Colors
        this.backgroundColor = '#0a0e27';
        this.gridColor = 'rgba(100, 150, 200, 0.1)';
        this.cellColor = '#00ff88';

        // Age-based color gradient (from young to old)
        this.ageColors = this.generateAgeGradient();

        // Pan offset for future pan/zoom features
        this.offsetX = 0;
        this.offsetY = 0;

        // Performance tracking
        this.lastFrameTime = 0;
        this.fps = 60;

        this.resizeCanvas();
        window.addEventListener('resize', () => this.resizeCanvas());
    }

    /**
     * Generate color gradient for cell aging
     */
    generateAgeGradient() {
        const colors = [];
        const maxAge = 50;

        for (let age = 0; age <= maxAge; age++) {
            const t = age / maxAge;

            // Color transition: cyan -> green -> yellow -> orange -> red -> purple
            let r, g, b;

            if (t < 0.2) {
                // Cyan to green
                const t2 = t / 0.2;
                r = Math.floor(0 * (1 - t2) + 0 * t2);
                g = Math.floor(255 * (1 - t2) + 255 * t2);
                b = Math.floor(255 * (1 - t2) + 100 * t2);
            } else if (t < 0.4) {
                // Green to yellow
                const t2 = (t - 0.2) / 0.2;
                r = Math.floor(0 * (1 - t2) + 255 * t2);
                g = Math.floor(255);
                b = Math.floor(100 * (1 - t2) + 0 * t2);
            } else if (t < 0.6) {
                // Yellow to orange
                const t2 = (t - 0.4) / 0.2;
                r = Math.floor(255);
                g = Math.floor(255 * (1 - t2) + 165 * t2);
                b = Math.floor(0);
            } else if (t < 0.8) {
                // Orange to red
                const t2 = (t - 0.6) / 0.2;
                r = Math.floor(255);
                g = Math.floor(165 * (1 - t2) + 50 * t2);
                b = Math.floor(0);
            } else {
                // Red to purple
                const t2 = (t - 0.8) / 0.2;
                r = Math.floor(255 * (1 - t2) + 200 * t2);
                g = Math.floor(50 * (1 - t2) + 0 * t2);
                b = Math.floor(0 * (1 - t2) + 255 * t2);
            }

            colors.push(`rgb(${r}, ${g}, ${b})`);
        }

        return colors;
    }

    /**
     * Resize canvas to fit container
     */
    resizeCanvas() {
        const container = this.canvas.parentElement;
        this.canvas.width = container.clientWidth;
        this.canvas.height = container.clientHeight;

        // Calculate cell size to fit grid
        const availableWidth = this.canvas.width - 40;
        const availableHeight = this.canvas.height - 40;

        const cellWidth = availableWidth / this.game.width;
        const cellHeight = availableHeight / this.game.height;

        this.cellSize = Math.floor(Math.min(cellWidth, cellHeight) * this.zoom);

        // Center the grid
        this.offsetX = (this.canvas.width - this.game.width * this.cellSize) / 2;
        this.offsetY = (this.canvas.height - this.game.height * this.cellSize) / 2;
    }

    /**
     * Set zoom level
     */
    setZoom(zoom) {
        this.zoom = Math.max(0.1, Math.min(5, zoom));
        this.resizeCanvas();
    }

    /**
     * Get cell coordinates from mouse position
     */
    getCellFromMouse(mouseX, mouseY) {
        const x = Math.floor((mouseX - this.offsetX) / this.cellSize);
        const y = Math.floor((mouseY - this.offsetY) / this.cellSize);

        if (x >= 0 && x < this.game.width && y >= 0 && y < this.game.height) {
            return { x, y };
        }

        return null;
    }

    /**
     * Render the game state
     */
    render() {
        const startTime = performance.now();

        // Clear canvas
        this.ctx.fillStyle = this.backgroundColor;
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

        // Draw grid lines
        if (this.showGrid) {
            this.drawGrid();
        }

        // Draw cells
        this.drawCells();

        // Calculate FPS
        const frameTime = performance.now() - startTime;
        this.lastFrameTime = frameTime;
        this.fps = Math.round(1000 / Math.max(frameTime, 1));
    }

    /**
     * Draw grid lines
     */
    drawGrid() {
        this.ctx.strokeStyle = this.gridColor;
        this.ctx.lineWidth = 1;

        // Vertical lines
        for (let x = 0; x <= this.game.width; x++) {
            const screenX = this.offsetX + x * this.cellSize;
            this.ctx.beginPath();
            this.ctx.moveTo(screenX, this.offsetY);
            this.ctx.lineTo(screenX, this.offsetY + this.game.height * this.cellSize);
            this.ctx.stroke();
        }

        // Horizontal lines
        for (let y = 0; y <= this.game.height; y++) {
            const screenY = this.offsetY + y * this.cellSize;
            this.ctx.beginPath();
            this.ctx.moveTo(this.offsetX, screenY);
            this.ctx.lineTo(this.offsetX + this.game.width * this.cellSize, screenY);
            this.ctx.stroke();
        }
    }

    /**
     * Draw all cells
     */
    drawCells() {
        for (let y = 0; y < this.game.height; y++) {
            for (let x = 0; x < this.game.width; x++) {
                if (this.game.grid[y][x]) {
                    this.drawCell(x, y);
                }
            }
        }
    }

    /**
     * Draw a single cell with age-based coloring
     */
    drawCell(x, y) {
        const screenX = this.offsetX + x * this.cellSize;
        const screenY = this.offsetY + y * this.cellSize;

        // Get cell age for coloring
        let color = this.cellColor;

        if (this.showAging) {
            const age = this.game.getCellAge(x, y);
            const colorIndex = Math.min(age - 1, this.ageColors.length - 1);

            if (colorIndex >= 0) {
                color = this.ageColors[colorIndex];
            }
        }

        // Draw cell with slight padding for visual appeal
        const padding = this.cellSize > 5 ? 1 : 0;

        this.ctx.fillStyle = color;
        this.ctx.fillRect(
            screenX + padding,
            screenY + padding,
            this.cellSize - padding * 2,
            this.cellSize - padding * 2
        );

        // Add glow effect for larger cells
        if (this.cellSize > 8) {
            this.ctx.shadowBlur = 5;
            this.ctx.shadowColor = color;
            this.ctx.fillRect(
                screenX + padding,
                screenY + padding,
                this.cellSize - padding * 2,
                this.cellSize - padding * 2
            );
            this.ctx.shadowBlur = 0;
        }
    }

    /**
     * Get current FPS
     */
    getFPS() {
        return this.fps;
    }
}

// Export for use in other modules
if (typeof module !== 'undefined' && module.exports) {
    module.exports = GameRenderer;
}
