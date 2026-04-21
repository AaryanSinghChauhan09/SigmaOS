/**
 * SigmaOS Zenith Canvas Animation Engine
 * Module 00: Ultra-high performance background rendering.
 */

const CanvasAnims = {
    canvas: null,
    ctx: null,
    orbs: [],
    
    init() {
        this.canvas = document.createElement('canvas');
        this.canvas.id = 'sentient-canvas';
        this.canvas.style.position = 'fixed';
        this.canvas.style.top = '0';
        this.canvas.style.left = '0';
        this.canvas.style.zIndex = '-1';
        this.canvas.style.width = '100vw';
        this.canvas.style.height = '100vh';
        document.body.prepend(this.canvas);
        
        this.ctx = this.canvas.getContext('2d');
        this.resize();
        window.addEventListener('resize', () => this.resize());
        
        this.createOrbs();
        this.animate();
        
        console.log("Σ Canvas Anims: Transitioned to high-performance Silicon rendering.");
    },

    resize() {
        this.canvas.width = window.innerWidth;
        this.canvas.height = window.innerHeight;
    },

    createOrbs() {
        const colors = ['rgba(0, 255, 234, 0.2)', 'rgba(112, 0, 255, 0.2)', 'rgba(255, 0, 255, 0.2)', 'rgba(255, 204, 0, 0.1)'];
        for (let i = 0; i < 4; i++) {
            this.orbs.push({
                x: Math.random() * this.canvas.width,
                y: Math.random() * this.canvas.height,
                r: Math.random() * 300 + 200,
                color: colors[i],
                vx: (Math.random() - 0.5) * 1.5,
                vy: (Math.random() - 0.5) * 1.5
            });
        }
    },

    animate() {
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        this.ctx.globalCompositeOperation = 'screen';
        
        this.orbs.forEach(orb => {
            orb.x += orb.vx;
            orb.y += orb.vy;
            
            if (orb.x < 0 || orb.x > this.canvas.width) orb.vx *= -1;
            if (orb.y < 0 || orb.y > this.canvas.height) orb.vy *= -1;
            
            const gradient = this.ctx.createRadialGradient(orb.x, orb.y, 0, orb.x, orb.y, orb.r);
            gradient.addColorStop(0, orb.color);
            gradient.addColorStop(1, 'rgba(0,0,0,0)');
            
            this.ctx.fillStyle = gradient;
            this.ctx.beginPath();
            this.ctx.arc(orb.x, orb.y, orb.r, 0, Math.PI * 2);
            this.ctx.fill();
        });
        
        requestAnimationFrame(() => this.animate());
    }
};

window.CanvasAnims = CanvasAnims;
