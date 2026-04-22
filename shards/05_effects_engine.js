/**
 * SigmaOS Sovereign Effects Engine
 * Module 05: High-fidelity visual micro-interactions and silicate animations.
 */

const EffectsEngine = {
    init() {
        this.setupWindowInteractions();
    },

    setupWindowInteractions() {
        document.addEventListener('mouseover', (e) => {
            const win = e.target.closest('.sovereign-window');
            if (win) this.applyTilt(win, e);
        });

        document.addEventListener('mousemove', (e) => {
            const win = e.target.closest('.sovereign-window');
            if (win) this.applyTilt(win, e);
        });

        document.addEventListener('mouseout', (e) => {
            const win = e.target.closest('.sovereign-window');
            if (win) win.style.transform = 'perspective(1000px) rotateX(0deg) rotateY(0deg)';
        });
    },

    applyTilt(el, e) {
        const rect = el.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;
        
        const xc = rect.width / 2;
        const yc = rect.height / 2;
        
        const dx = (x - xc) / (rect.width / 2);
        const dy = (y - yc) / (rect.height / 2);
        
        el.style.transform = `perspective(1000px) rotateX(${-dy * 2}deg) rotateY(${dx * 2}deg)`;
    },

    glitchEffect(el) {
        el.classList.add('glitch-active');
        setTimeout(() => el.classList.remove('glitch-active'), 500);
    }
};

window.EffectsEngine = EffectsEngine;
