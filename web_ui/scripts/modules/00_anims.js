/**
 * SigmaOS Zenith Animation Engine
 * Module 00: Optimized background effects and orbital synchronization.
 */

const ZenithAnims = {
    init() {
        this.setupSentientBackground();
    },

    setupSentientBackground() {
        const orbs = document.querySelectorAll('.orb');
        if (!orbs.length) return;

        console.log("Σ Zenith Anims: Activating Sentient Background...");

        // Optimized mouse tracking using requestAnimationFrame
        let mouseX = 0, mouseY = 0;
        let targetX = 0, targetY = 0;

        document.addEventListener('mousemove', (e) => {
            targetX = (e.clientX / window.innerWidth - 0.5) * 60;
            targetY = (e.clientY / window.innerHeight - 0.5) * 60;
        });

        const animate = () => {
            // Smooth lerp for liquid-like motion
            mouseX += (targetX - mouseX) * 0.05;
            mouseY += (targetY - mouseY) * 0.05;

            orbs.forEach((orb, i) => {
                const speed = 1 + (i * 0.4);
                const rotation = (Date.now() / 5000) * (i + 1);
                orb.style.transform = `translate(${mouseX * speed}px, ${mouseY * speed}px) rotate(${rotation}deg)`;
            });

            requestAnimationFrame(animate);
        };

        animate();
    }

    selfEvolve() {
        const mutations = [
            "Optimizing lattice resonance...",
            "Expanding semantic context...",
            "Hardening silicon primitives...",
            "Refining cross-kernel synthesis..."
        ];
        const mutation = mutations[Math.floor(Math.random() * mutations.length)];
        console.log(`Σ://EVOLUTION [${this.shardId}]> ${mutation}`);
        this.lastMutation = mutation;
    }
};

window.ZenithAnims = ZenithAnims;
