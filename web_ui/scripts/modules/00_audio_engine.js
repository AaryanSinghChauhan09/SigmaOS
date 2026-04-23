/**
 * SigmaOS Sovereign Audio Engine
 * Module 00: High-fidelity sound orchestration and aesthetic auditory feedback.
 */

const AudioEngine = {
    audioContext: null,
    
    init() {
        this.audioContext = new (window.AudioContext || window.webkitAudioContext)();
        console.log("Σ Audio Engine: Auditory Lattice Synchronized.");
    },

    playSystemSound(freq = 440, type = 'sine', duration = 0.1) {
        if (!this.audioContext) return;
        
        const osc = this.audioContext.createOscillator();
        const gain = this.audioContext.createGain();
        
        osc.type = type;
        osc.frequency.setValueAtTime(freq, this.audioContext.currentTime);
        
        gain.gain.setValueAtTime(0.05, this.audioContext.currentTime);
        gain.gain.exponentialRampToValueAtTime(0.0001, this.audioContext.currentTime + duration);
        
        osc.connect(gain);
        gain.connect(this.audioContext.destination);
        
        osc.start();
        osc.stop(this.audioContext.currentTime + duration);
    },

    playSuccess() {
        this.playSystemSound(880, 'triangle', 0.2);
    },

    playNotify() {
        this.playSystemSound(660, 'sine', 0.15);
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

window.AudioEngine = AudioEngine;
