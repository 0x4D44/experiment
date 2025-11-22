/**
 * Confetti Animation
 * Creates celebration confetti effect when player wins
 */

class ConfettiEffect {
    constructor(canvasId) {
        this.canvas = document.getElementById(canvasId);
        if (!this.canvas) return;

        this.ctx = this.canvas.getContext('2d');
        this.particles = [];
        this.animationId = null;
        this.isActive = false;

        this.resize();
        window.addEventListener('resize', () => this.resize());
    }

    /**
     * Resize canvas to window size
     */
    resize() {
        if (!this.canvas) return;
        this.canvas.width = window.innerWidth;
        this.canvas.height = window.innerHeight;
    }

    /**
     * Create confetti particle
     */
    createParticle() {
        const colors = [
            '#667eea', '#764ba2', '#f093fb', '#10b981',
            '#fbbf24', '#f59e0b', '#ef4444', '#8b5cf6',
            '#ec4899', '#06b6d4'
        ];

        return {
            x: Math.random() * this.canvas.width,
            y: Math.random() * this.canvas.height - this.canvas.height,
            rotation: Math.random() * 360,
            rotationSpeed: Math.random() * 10 - 5,
            size: Math.random() * 8 + 5,
            velocityX: Math.random() * 4 - 2,
            velocityY: Math.random() * 3 + 2,
            color: colors[Math.floor(Math.random() * colors.length)],
            opacity: 1,
            shape: Math.random() > 0.5 ? 'circle' : 'square'
        };
    }

    /**
     * Update particle position
     */
    updateParticle(particle) {
        particle.x += particle.velocityX;
        particle.y += particle.velocityY;
        particle.rotation += particle.rotationSpeed;
        particle.velocityY += 0.1; // Gravity
        particle.opacity -= 0.005;

        return particle.y < this.canvas.height && particle.opacity > 0;
    }

    /**
     * Draw particle
     */
    drawParticle(particle) {
        this.ctx.save();
        this.ctx.translate(particle.x, particle.y);
        this.ctx.rotate((particle.rotation * Math.PI) / 180);
        this.ctx.globalAlpha = particle.opacity;
        this.ctx.fillStyle = particle.color;

        if (particle.shape === 'circle') {
            this.ctx.beginPath();
            this.ctx.arc(0, 0, particle.size / 2, 0, Math.PI * 2);
            this.ctx.fill();
        } else {
            this.ctx.fillRect(
                -particle.size / 2,
                -particle.size / 2,
                particle.size,
                particle.size
            );
        }

        this.ctx.restore();
    }

    /**
     * Animation loop
     */
    animate() {
        if (!this.isActive) return;

        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

        // Update and draw particles
        this.particles = this.particles.filter(particle => {
            const isAlive = this.updateParticle(particle);
            if (isAlive) {
                this.drawParticle(particle);
            }
            return isAlive;
        });

        // Add new particles
        if (this.particles.length < 150) {
            for (let i = 0; i < 5; i++) {
                this.particles.push(this.createParticle());
            }
        }

        this.animationId = requestAnimationFrame(() => this.animate());
    }

    /**
     * Start confetti effect
     */
    start(duration = 5000) {
        if (!this.canvas) return;

        this.isActive = true;
        this.particles = [];

        // Initial burst
        for (let i = 0; i < 100; i++) {
            this.particles.push(this.createParticle());
        }

        this.animate();

        // Stop after duration
        setTimeout(() => this.stop(), duration);
    }

    /**
     * Stop confetti effect
     */
    stop() {
        this.isActive = false;
        if (this.animationId) {
            cancelAnimationFrame(this.animationId);
            this.animationId = null;
        }

        // Fade out remaining particles
        const fadeOut = () => {
            this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

            this.particles = this.particles.filter(particle => {
                particle.opacity -= 0.02;
                if (particle.opacity > 0) {
                    this.drawParticle(particle);
                    return true;
                }
                return false;
            });

            if (this.particles.length > 0) {
                requestAnimationFrame(fadeOut);
            }
        };

        fadeOut();
    }

    /**
     * Clear canvas
     */
    clear() {
        if (!this.canvas) return;
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        this.particles = [];
    }
}

// Create global confetti instance
const confetti = new ConfettiEffect('confetti');
