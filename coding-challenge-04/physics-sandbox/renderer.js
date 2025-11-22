/**
 * Canvas Renderer with Visual Effects
 * Handles all rendering including trails, shadows, and particle effects
 */

class Renderer {
    constructor(canvas) {
        this.canvas = canvas;
        this.ctx = canvas.getContext('2d');
        this.width = canvas.width;
        this.height = canvas.height;

        // Visual settings
        this.enableTrails = true;
        this.enableShadows = true;
        this.enableGlow = true;
        this.backgroundParticles = [];
        this.initBackgroundParticles();
    }

    initBackgroundParticles() {
        for (let i = 0; i < 50; i++) {
            this.backgroundParticles.push({
                x: Math.random() * this.width,
                y: Math.random() * this.height,
                radius: Math.random() * 2 + 1,
                alpha: Math.random() * 0.3,
                speedX: (Math.random() - 0.5) * 0.5,
                speedY: (Math.random() - 0.5) * 0.5
            });
        }
    }

    resize(width, height) {
        this.width = width;
        this.height = height;
        // Canvas is already resized in app.js, no need to do it again
        // Reinitialize background particles for new dimensions
        this.backgroundParticles = [];
        this.initBackgroundParticles();
    }

    clear() {
        // Create gradient background
        const gradient = this.ctx.createLinearGradient(0, 0, 0, this.height);
        gradient.addColorStop(0, '#0a0e27');
        gradient.addColorStop(1, '#1a1e37');
        this.ctx.fillStyle = gradient;
        this.ctx.fillRect(0, 0, this.width, this.height);

        // Draw animated background particles
        this.drawBackgroundParticles();

        // Draw grid
        this.drawGrid();
    }

    drawBackgroundParticles() {
        this.backgroundParticles.forEach(particle => {
            particle.x += particle.speedX;
            particle.y += particle.speedY;

            // Wrap around screen
            if (particle.x < 0) particle.x = this.width;
            if (particle.x > this.width) particle.x = 0;
            if (particle.y < 0) particle.y = this.height;
            if (particle.y > this.height) particle.y = 0;

            this.ctx.beginPath();
            this.ctx.arc(particle.x, particle.y, particle.radius, 0, Math.PI * 2);
            this.ctx.fillStyle = `rgba(100, 150, 255, ${particle.alpha})`;
            this.ctx.fill();
        });
    }

    drawGrid() {
        const gridSize = 50;
        this.ctx.strokeStyle = 'rgba(100, 150, 255, 0.1)';
        this.ctx.lineWidth = 1;

        // Vertical lines
        for (let x = 0; x <= this.width; x += gridSize) {
            this.ctx.beginPath();
            this.ctx.moveTo(x, 0);
            this.ctx.lineTo(x, this.height);
            this.ctx.stroke();
        }

        // Horizontal lines
        for (let y = 0; y <= this.height; y += gridSize) {
            this.ctx.beginPath();
            this.ctx.moveTo(0, y);
            this.ctx.lineTo(this.width, y);
            this.ctx.stroke();
        }
    }

    render(physicsEngine) {
        this.clear();

        // Draw all objects
        physicsEngine.objects.forEach(obj => {
            // Draw trail first (behind object)
            if (this.enableTrails) {
                this.drawTrail(obj);
            }

            // Draw object
            if (obj.type === 'circle') {
                this.drawCircle(obj);
            } else if (obj.type === 'box') {
                this.drawBox(obj);
            }

            // Draw selection indicator if grabbed
            if (obj.isGrabbed) {
                this.drawSelectionIndicator(obj);
            }
        });
    }

    drawTrail(obj) {
        if (obj.trail.length < 2) return;

        this.ctx.strokeStyle = obj.color;
        this.ctx.lineWidth = 2;
        this.ctx.lineCap = 'round';
        this.ctx.lineJoin = 'round';

        for (let i = 0; i < obj.trail.length - 1; i++) {
            const point = obj.trail[i];
            const nextPoint = obj.trail[i + 1];

            this.ctx.globalAlpha = point.alpha * 0.3;
            this.ctx.beginPath();
            this.ctx.moveTo(point.x, point.y);
            this.ctx.lineTo(nextPoint.x, nextPoint.y);
            this.ctx.stroke();
        }

        this.ctx.globalAlpha = 1;
    }

    drawCircle(circle) {
        const x = circle.position.x;
        const y = circle.position.y;
        const radius = circle.radius;

        // Draw shadow
        if (this.enableShadows) {
            this.ctx.shadowColor = 'rgba(0, 0, 0, 0.5)';
            this.ctx.shadowBlur = 20;
            this.ctx.shadowOffsetX = 5;
            this.ctx.shadowOffsetY = 5;
        }

        // Draw glow effect
        if (this.enableGlow) {
            const glowGradient = this.ctx.createRadialGradient(x, y, radius * 0.5, x, y, radius * 1.5);
            glowGradient.addColorStop(0, circle.color);
            glowGradient.addColorStop(1, 'rgba(0, 0, 0, 0)');

            this.ctx.fillStyle = glowGradient;
            this.ctx.beginPath();
            this.ctx.arc(x, y, radius * 1.5, 0, Math.PI * 2);
            this.ctx.fill();
        }

        // Draw main circle
        const gradient = this.ctx.createRadialGradient(
            x - radius * 0.3, y - radius * 0.3, radius * 0.1,
            x, y, radius
        );
        gradient.addColorStop(0, this.lightenColor(circle.color, 40));
        gradient.addColorStop(1, circle.color);

        this.ctx.fillStyle = gradient;
        this.ctx.beginPath();
        this.ctx.arc(x, y, radius, 0, Math.PI * 2);
        this.ctx.fill();

        // Draw outline
        this.ctx.strokeStyle = this.darkenColor(circle.color, 20);
        this.ctx.lineWidth = 2;
        this.ctx.stroke();

        // Reset shadow
        this.ctx.shadowColor = 'transparent';
        this.ctx.shadowBlur = 0;
        this.ctx.shadowOffsetX = 0;
        this.ctx.shadowOffsetY = 0;

        // Draw velocity indicator
        if (circle.velocity.magnitude() > 50) {
            this.drawVelocityArrow(circle);
        }
    }

    drawBox(box) {
        const x = box.position.x;
        const y = box.position.y;
        const width = box.width;
        const height = box.height;

        this.ctx.save();
        this.ctx.translate(x, y);
        this.ctx.rotate(box.angle);

        // Draw shadow
        if (this.enableShadows) {
            this.ctx.shadowColor = 'rgba(0, 0, 0, 0.5)';
            this.ctx.shadowBlur = 20;
            this.ctx.shadowOffsetX = 5;
            this.ctx.shadowOffsetY = 5;
        }

        // Draw glow effect
        if (this.enableGlow) {
            this.ctx.fillStyle = box.color.replace(')', ', 0.3)').replace('hsl', 'hsla');
            this.ctx.fillRect(-width / 2 - 10, -height / 2 - 10, width + 20, height + 20);
        }

        // Draw main box with gradient
        const gradient = this.ctx.createLinearGradient(-width / 2, -height / 2, width / 2, height / 2);
        gradient.addColorStop(0, this.lightenColor(box.color, 30));
        gradient.addColorStop(1, box.color);

        this.ctx.fillStyle = gradient;
        this.ctx.fillRect(-width / 2, -height / 2, width, height);

        // Draw outline
        this.ctx.strokeStyle = this.darkenColor(box.color, 20);
        this.ctx.lineWidth = 2;
        this.ctx.strokeRect(-width / 2, -height / 2, width, height);

        // Draw diagonal lines for texture
        this.ctx.strokeStyle = this.lightenColor(box.color, 20);
        this.ctx.lineWidth = 1;
        this.ctx.beginPath();
        this.ctx.moveTo(-width / 2, -height / 2);
        this.ctx.lineTo(width / 2, height / 2);
        this.ctx.moveTo(width / 2, -height / 2);
        this.ctx.lineTo(-width / 2, height / 2);
        this.ctx.stroke();

        this.ctx.restore();

        // Reset shadow
        this.ctx.shadowColor = 'transparent';
        this.ctx.shadowBlur = 0;
        this.ctx.shadowOffsetX = 0;
        this.ctx.shadowOffsetY = 0;

        // Draw velocity indicator
        if (box.velocity.magnitude() > 50) {
            this.drawVelocityArrow(box);
        }
    }

    drawVelocityArrow(obj) {
        const vel = obj.velocity.multiply(0.1); // Scale down for visualization
        const startX = obj.position.x;
        const startY = obj.position.y;
        const endX = startX + vel.x;
        const endY = startY + vel.y;

        this.ctx.strokeStyle = 'rgba(255, 255, 0, 0.7)';
        this.ctx.lineWidth = 3;
        this.ctx.beginPath();
        this.ctx.moveTo(startX, startY);
        this.ctx.lineTo(endX, endY);
        this.ctx.stroke();

        // Arrow head
        const angle = Math.atan2(vel.y, vel.x);
        const headLength = 10;
        this.ctx.beginPath();
        this.ctx.moveTo(endX, endY);
        this.ctx.lineTo(
            endX - headLength * Math.cos(angle - Math.PI / 6),
            endY - headLength * Math.sin(angle - Math.PI / 6)
        );
        this.ctx.moveTo(endX, endY);
        this.ctx.lineTo(
            endX - headLength * Math.cos(angle + Math.PI / 6),
            endY - headLength * Math.sin(angle + Math.PI / 6)
        );
        this.ctx.stroke();
    }

    drawSelectionIndicator(obj) {
        this.ctx.strokeStyle = 'rgba(255, 255, 255, 0.8)';
        this.ctx.lineWidth = 3;
        this.ctx.setLineDash([5, 5]);

        if (obj.type === 'circle') {
            this.ctx.beginPath();
            this.ctx.arc(obj.position.x, obj.position.y, obj.radius + 5, 0, Math.PI * 2);
            this.ctx.stroke();
        } else if (obj.type === 'box') {
            this.ctx.strokeRect(
                obj.position.x - obj.width / 2 - 5,
                obj.position.y - obj.height / 2 - 5,
                obj.width + 10,
                obj.height + 10
            );
        }

        this.ctx.setLineDash([]);
    }

    drawSpawnPreview(x, y, type, size) {
        this.ctx.strokeStyle = 'rgba(255, 255, 255, 0.5)';
        this.ctx.lineWidth = 2;
        this.ctx.setLineDash([5, 5]);

        if (type === 'circle') {
            this.ctx.beginPath();
            this.ctx.arc(x, y, size, 0, Math.PI * 2);
            this.ctx.stroke();
        } else if (type === 'box') {
            this.ctx.strokeRect(x - size, y - size, size * 2, size * 2);
        }

        this.ctx.setLineDash([]);
    }

    lightenColor(color, percent) {
        // Parse HSL color
        const match = color.match(/hsl\((\d+),\s*(\d+)%,\s*(\d+)%\)/);
        if (!match) return color;

        const h = parseInt(match[1]);
        const s = parseInt(match[2]);
        const l = Math.min(100, parseInt(match[3]) + percent);

        return `hsl(${h}, ${s}%, ${l}%)`;
    }

    darkenColor(color, percent) {
        // Parse HSL color
        const match = color.match(/hsl\((\d+),\s*(\d+)%,\s*(\d+)%\)/);
        if (!match) return color;

        const h = parseInt(match[1]);
        const s = parseInt(match[2]);
        const l = Math.max(0, parseInt(match[3]) - percent);

        return `hsl(${h}, ${s}%, ${l}%)`;
    }

    toggleTrails() {
        this.enableTrails = !this.enableTrails;
    }

    toggleShadows() {
        this.enableShadows = !this.enableShadows;
    }

    toggleGlow() {
        this.enableGlow = !this.enableGlow;
    }
}

// Export for use in other files
if (typeof module !== 'undefined' && module.exports) {
    module.exports = { Renderer };
}
