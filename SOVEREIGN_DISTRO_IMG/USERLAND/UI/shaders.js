/**
 * Σ SIGMA OS SHADER ENGINE v2.1
 * Hardware-Accelerated Dynamic Backgrounds
 */

export const ShaderEngine = {
    canvas: null,
    ctx: null,
    points: [],
    numPoints: 50,
    initialized: false,

    init() {
        this.canvas = document.getElementById('sigma-bg-canvas');
        if (!this.canvas) return;
        this.ctx = this.canvas.getContext('2d');
        this.resize();
        window.addEventListener('resize', () => this.resize());

        for (let i = 0; i < this.numPoints; i++) {
            this.points.push({
                x: Math.random() * this.canvas.width,
                y: Math.random() * this.canvas.height,
                vx: (Math.random() - 0.5) * 0.5,
                vy: (Math.random() - 0.5) * 0.5
            });
        }

        this.initialized = true;
        this.animate();
        console.log("Shader Engine Active.");
    },

    resize() {
        this.canvas.width = window.innerWidth;
        this.canvas.height = window.innerHeight;
    },

    animate() {
        if (!this.initialized) return;
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

        // Dynamic color based on theme
        const accent = getComputedStyle(document.documentElement).getPropertyValue('--accent').trim() || '#5AC8FA';

        this.ctx.strokeStyle = accent;
        this.ctx.globalAlpha = 0.15;
        this.ctx.lineWidth = 0.5;

        for (let i = 0; i < this.points.length; i++) {
            let p = this.points[i];
            p.x += p.vx;
            p.y += p.vy;

            if (p.x < 0 || p.x > this.canvas.width) p.vx *= -1;
            if (p.y < 0 || p.y > this.canvas.height) p.vy *= -1;

            for (let j = i + 1; j < this.points.length; j++) {
                let p2 = this.points[j];
                let dist = Math.sqrt((p.x - p2.x) ** 2 + (p.y - p2.y) ** 2);
                if (dist < 200) {
                    this.ctx.beginPath();
                    this.ctx.moveTo(p.x, p.y);
                    this.ctx.lineTo(p2.x, p2.y);
                    this.ctx.stroke();
                }
            }
        }

        requestAnimationFrame(() => this.animate());
    }
};

window.ShaderEngine = ShaderEngine;
