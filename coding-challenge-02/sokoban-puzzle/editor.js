/**
 * SOKOBAN LEVEL EDITOR
 * Create and edit custom Sokoban levels
 */

class LevelEditor {
    constructor() {
        this.canvas = null;
        this.ctx = null;
        this.grid = [];
        this.width = 10;
        this.height = 10;
        this.selectedTool = 'wall';
        this.isDrawing = false;
        this.customLevels = [];

        this.loadCustomLevels();
    }

    init() {
        this.canvas = document.getElementById('editorCanvas');
        this.ctx = this.canvas.getContext('2d');

        this.setupEventListeners();
        this.createGrid();
        this.render();
    }

    setupEventListeners() {
        // Tool selection
        document.querySelectorAll('.tool-btn[data-tool]').forEach(btn => {
            btn.addEventListener('click', (e) => {
                document.querySelectorAll('.tool-btn[data-tool]').forEach(b => b.classList.remove('active'));
                e.target.classList.add('active');
                this.selectedTool = e.target.dataset.tool;
            });
        });

        // Canvas drawing
        this.canvas.addEventListener('mousedown', (e) => {
            this.isDrawing = true;
            this.handleDraw(e);
        });

        this.canvas.addEventListener('mousemove', (e) => {
            if (this.isDrawing) {
                this.handleDraw(e);
            }
        });

        this.canvas.addEventListener('mouseup', () => {
            this.isDrawing = false;
        });

        this.canvas.addEventListener('mouseleave', () => {
            this.isDrawing = false;
        });

        // Right click to erase
        this.canvas.addEventListener('contextmenu', (e) => {
            e.preventDefault();
            const rect = this.canvas.getBoundingClientRect();
            const x = Math.floor((e.clientX - rect.left) / TILE_SIZE);
            const y = Math.floor((e.clientY - rect.top) / TILE_SIZE);

            if (x >= 0 && x < this.width && y >= 0 && y < this.height) {
                this.grid[y][x] = 'floor';
                this.render();
            }
        });

        // Touch support
        this.canvas.addEventListener('touchstart', (e) => {
            e.preventDefault();
            this.isDrawing = true;
            this.handleTouch(e);
        });

        this.canvas.addEventListener('touchmove', (e) => {
            e.preventDefault();
            if (this.isDrawing) {
                this.handleTouch(e);
            }
        });

        this.canvas.addEventListener('touchend', () => {
            this.isDrawing = false;
        });
    }

    handleDraw(e) {
        const rect = this.canvas.getBoundingClientRect();
        const x = Math.floor((e.clientX - rect.left) / TILE_SIZE);
        const y = Math.floor((e.clientY - rect.top) / TILE_SIZE);

        if (x >= 0 && x < this.width && y >= 0 && y < this.height) {
            this.placeTile(x, y);
        }
    }

    handleTouch(e) {
        const rect = this.canvas.getBoundingClientRect();
        const touch = e.touches[0];
        const x = Math.floor((touch.clientX - rect.left) / TILE_SIZE);
        const y = Math.floor((touch.clientY - rect.top) / TILE_SIZE);

        if (x >= 0 && x < this.width && y >= 0 && y < this.height) {
            this.placeTile(x, y);
        }
    }

    placeTile(x, y) {
        // Special handling for player - only one allowed
        if (this.selectedTool === 'player') {
            // Remove existing player
            for (let row = 0; row < this.height; row++) {
                for (let col = 0; col < this.width; col++) {
                    if (this.grid[row][col] === 'player') {
                        this.grid[row][col] = 'floor';
                    }
                }
            }
        }

        this.grid[y][x] = this.selectedTool;
        this.render();
    }

    createGrid() {
        this.grid = [];
        for (let y = 0; y < this.height; y++) {
            const row = [];
            for (let x = 0; x < this.width; x++) {
                row.push('floor');
            }
            this.grid.push(row);
        }

        // Add border walls
        for (let x = 0; x < this.width; x++) {
            this.grid[0][x] = 'wall';
            this.grid[this.height - 1][x] = 'wall';
        }
        for (let y = 0; y < this.height; y++) {
            this.grid[y][0] = 'wall';
            this.grid[y][this.width - 1] = 'wall';
        }

        this.canvas.width = this.width * TILE_SIZE;
        this.canvas.height = this.height * TILE_SIZE;
    }

    clearGrid() {
        if (confirm('Clear the entire grid?')) {
            this.createGrid();
            this.render();
        }
    }

    resizeGrid() {
        const newWidth = parseInt(document.getElementById('gridWidth').value);
        const newHeight = parseInt(document.getElementById('gridHeight').value);

        if (newWidth < 5 || newWidth > 20 || newHeight < 5 || newHeight > 20) {
            alert('Grid size must be between 5 and 20.');
            return;
        }

        this.width = newWidth;
        this.height = newHeight;
        this.createGrid();
        this.render();
    }

    render() {
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

        for (let y = 0; y < this.height; y++) {
            for (let x = 0; x < this.width; x++) {
                this.drawTile(x, y, this.grid[y][x]);
            }
        }

        // Draw grid lines
        this.ctx.strokeStyle = '#475569';
        this.ctx.lineWidth = 1;
        for (let x = 0; x <= this.width; x++) {
            this.ctx.beginPath();
            this.ctx.moveTo(x * TILE_SIZE, 0);
            this.ctx.lineTo(x * TILE_SIZE, this.height * TILE_SIZE);
            this.ctx.stroke();
        }
        for (let y = 0; y <= this.height; y++) {
            this.ctx.beginPath();
            this.ctx.moveTo(0, y * TILE_SIZE);
            this.ctx.lineTo(this.width * TILE_SIZE, y * TILE_SIZE);
            this.ctx.stroke();
        }
    }

    drawTile(x, y, type) {
        const px = x * TILE_SIZE;
        const py = y * TILE_SIZE;

        this.ctx.save();

        switch (type) {
            case 'wall':
                this.ctx.fillStyle = '#64748b';
                this.ctx.fillRect(px, py, TILE_SIZE, TILE_SIZE);
                this.ctx.strokeStyle = '#475569';
                this.ctx.lineWidth = 2;
                this.ctx.strokeRect(px, py, TILE_SIZE, TILE_SIZE);
                break;

            case 'floor':
                this.ctx.fillStyle = '#cbd5e1';
                this.ctx.fillRect(px, py, TILE_SIZE, TILE_SIZE);
                break;

            case 'box':
                this.ctx.fillStyle = '#cbd5e1';
                this.ctx.fillRect(px, py, TILE_SIZE, TILE_SIZE);
                // Draw box
                const boxSize = TILE_SIZE * 0.7;
                const boxOffset = (TILE_SIZE - boxSize) / 2;
                this.ctx.fillStyle = '#d97706';
                this.ctx.fillRect(px + boxOffset, py + boxOffset, boxSize, boxSize);
                this.ctx.strokeStyle = '#b45309';
                this.ctx.lineWidth = 2;
                this.ctx.strokeRect(px + boxOffset, py + boxOffset, boxSize, boxSize);
                break;

            case 'target':
                this.ctx.fillStyle = '#cbd5e1';
                this.ctx.fillRect(px, py, TILE_SIZE, TILE_SIZE);
                // Draw target
                this.ctx.strokeStyle = '#10b981';
                this.ctx.lineWidth = 3;
                this.ctx.beginPath();
                this.ctx.arc(px + TILE_SIZE / 2, py + TILE_SIZE / 2, TILE_SIZE / 3, 0, Math.PI * 2);
                this.ctx.stroke();
                this.ctx.beginPath();
                this.ctx.arc(px + TILE_SIZE / 2, py + TILE_SIZE / 2, TILE_SIZE / 6, 0, Math.PI * 2);
                this.ctx.stroke();
                break;

            case 'player':
                this.ctx.fillStyle = '#cbd5e1';
                this.ctx.fillRect(px, py, TILE_SIZE, TILE_SIZE);
                // Draw player
                this.ctx.fillStyle = '#3b82f6';
                this.ctx.beginPath();
                this.ctx.arc(px + TILE_SIZE / 2, py + TILE_SIZE / 2, TILE_SIZE / 3, 0, Math.PI * 2);
                this.ctx.fill();
                this.ctx.strokeStyle = '#1e40af';
                this.ctx.lineWidth = 3;
                this.ctx.stroke();
                break;

            case 'box_on_target':
                this.ctx.fillStyle = '#cbd5e1';
                this.ctx.fillRect(px, py, TILE_SIZE, TILE_SIZE);
                // Draw target first
                this.ctx.strokeStyle = '#10b981';
                this.ctx.lineWidth = 3;
                this.ctx.beginPath();
                this.ctx.arc(px + TILE_SIZE / 2, py + TILE_SIZE / 2, TILE_SIZE / 3, 0, Math.PI * 2);
                this.ctx.stroke();
                // Draw box on top
                const botBoxSize = TILE_SIZE * 0.6;
                const botBoxOffset = (TILE_SIZE - botBoxSize) / 2;
                this.ctx.fillStyle = '#059669';
                this.ctx.fillRect(px + botBoxOffset, py + botBoxOffset, botBoxSize, botBoxSize);
                this.ctx.strokeStyle = '#047857';
                this.ctx.lineWidth = 2;
                this.ctx.strokeRect(px + botBoxOffset, py + botBoxOffset, botBoxSize, botBoxSize);
                break;
        }

        this.ctx.restore();
    }

    validateLevel() {
        let playerCount = 0;
        let boxCount = 0;
        let targetCount = 0;

        for (let y = 0; y < this.height; y++) {
            for (let x = 0; x < this.width; x++) {
                const tile = this.grid[y][x];
                if (tile === 'player') playerCount++;
                if (tile === 'box') boxCount++;
                if (tile === 'target') targetCount++;
                if (tile === 'box_on_target') {
                    boxCount++;
                    targetCount++;
                }
            }
        }

        const errors = [];

        if (playerCount === 0) {
            errors.push('Missing player position');
        } else if (playerCount > 1) {
            errors.push('Multiple players found (only one allowed)');
        }

        if (boxCount === 0) {
            errors.push('No boxes found (at least one required)');
        }

        if (targetCount === 0) {
            errors.push('No targets found (at least one required)');
        }

        if (boxCount !== targetCount) {
            errors.push(`Box count (${boxCount}) must equal target count (${targetCount})`);
        }

        return {
            valid: errors.length === 0,
            errors: errors
        };
    }

    exportToString() {
        const lines = [];

        for (let y = 0; y < this.height; y++) {
            let line = '';
            for (let x = 0; x < this.width; x++) {
                const tile = this.grid[y][x];
                switch (tile) {
                    case 'wall':
                        line += '#';
                        break;
                    case 'floor':
                        line += ' ';
                        break;
                    case 'box':
                        line += '$';
                        break;
                    case 'target':
                        line += '.';
                        break;
                    case 'player':
                        line += '@';
                        break;
                    case 'box_on_target':
                        line += '*';
                        break;
                }
            }
            lines.push(line);
        }

        return lines;
    }

    testLevel() {
        const validation = this.validateLevel();

        if (!validation.valid) {
            alert('Level validation failed:\n\n' + validation.errors.join('\n'));
            return;
        }

        // Convert to game format and test
        const levelData = {
            name: 'Custom Level',
            difficulty: 'custom',
            optimal: 0,
            grid: this.exportToString()
        };

        // Create temporary level pack
        if (!LEVEL_PACKS.custom) {
            LEVEL_PACKS.custom = {
                name: 'Custom Levels',
                icon: '✨',
                levels: []
            };
        }

        LEVEL_PACKS.custom.levels = [levelData];
        game.state.currentPack = 'custom';
        game.state.currentLevel = 0;
        game.loadLevel(levelData);
        game.showScreen('gameScreen');
        game.startTimer();

        document.getElementById('levelTitle').textContent = 'Custom Level (Test)';
        document.getElementById('levelDifficulty').textContent = 'CUSTOM';
        document.getElementById('optimalMoves').textContent = '-';

        game.updateGameUI();
    }

    saveLevel() {
        const validation = this.validateLevel();

        if (!validation.valid) {
            alert('Cannot save invalid level:\n\n' + validation.errors.join('\n'));
            return;
        }

        const name = prompt('Enter level name:');
        if (!name) return;

        const levelData = {
            name: name,
            difficulty: 'custom',
            optimal: 0,
            grid: this.exportToString()
        };

        this.customLevels.push(levelData);
        this.saveCustomLevels();

        alert('Level saved successfully!');
    }

    loadLevel() {
        if (this.customLevels.length === 0) {
            alert('No saved levels found.');
            return;
        }

        const levelNames = this.customLevels.map((l, i) => `${i + 1}. ${l.name}`).join('\n');
        const index = prompt('Select level to load:\n\n' + levelNames + '\n\nEnter number:');

        if (!index) return;

        const levelIndex = parseInt(index) - 1;
        if (levelIndex < 0 || levelIndex >= this.customLevels.length) {
            alert('Invalid level number.');
            return;
        }

        const level = this.customLevels[levelIndex];
        this.importFromString(level.grid);
        this.render();
    }

    importFromString(gridLines) {
        this.height = gridLines.length;
        this.width = Math.max(...gridLines.map(line => line.length));

        document.getElementById('gridWidth').value = this.width;
        document.getElementById('gridHeight').value = this.height;

        this.grid = [];

        for (let y = 0; y < this.height; y++) {
            const row = [];
            const line = gridLines[y] || '';

            for (let x = 0; x < this.width; x++) {
                const char = line[x] || ' ';

                switch (char) {
                    case '#':
                        row.push('wall');
                        break;
                    case '$':
                        row.push('box');
                        break;
                    case '.':
                        row.push('target');
                        break;
                    case '@':
                        row.push('player');
                        break;
                    case '*':
                        // Box on target - need to create both box and target
                        // Store as special tile type to preserve this information
                        row.push('box_on_target');
                        break;
                    default:
                        row.push('floor');
                }
            }

            this.grid.push(row);
        }

        this.canvas.width = this.width * TILE_SIZE;
        this.canvas.height = this.height * TILE_SIZE;
    }

    exportLevel() {
        const validation = this.validateLevel();

        if (!validation.valid) {
            alert('Cannot export invalid level:\n\n' + validation.errors.join('\n'));
            return;
        }

        const levelData = {
            name: prompt('Enter level name:') || 'Custom Level',
            difficulty: 'custom',
            optimal: 0,
            grid: this.exportToString()
        };

        const dataStr = JSON.stringify(levelData, null, 2);
        const blob = new Blob([dataStr], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `${levelData.name.replace(/[^a-z0-9]/gi, '_')}.json`;
        a.click();
        URL.revokeObjectURL(url);
    }

    saveCustomLevels() {
        try {
            localStorage.setItem('sokoban_custom_levels', JSON.stringify(this.customLevels));
        } catch (e) {
            console.error('Failed to save custom levels:', e);
        }
    }

    loadCustomLevels() {
        try {
            const saved = localStorage.getItem('sokoban_custom_levels');
            if (saved) {
                this.customLevels = JSON.parse(saved);

                // Add custom pack to game if we have levels
                if (this.customLevels.length > 0) {
                    LEVEL_PACKS.custom = {
                        name: 'Custom Levels',
                        icon: '✨',
                        levels: this.customLevels
                    };
                }
            }
        } catch (e) {
            console.error('Failed to load custom levels:', e);
        }
    }
}

// Initialize editor
let editor;

window.addEventListener('DOMContentLoaded', () => {
    editor = new LevelEditor();
});
