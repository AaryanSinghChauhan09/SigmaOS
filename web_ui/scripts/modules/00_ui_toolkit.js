/**
 * SigmaOS: Sovereign UI Toolkit (SUI)
 * Inspired by SerenityOS GUI Toolkit.
 * USP: Modular, glassmorphic UI components with near-zero latency.
 */

const SovereignUI = {
    createComponent(type, props = {}, children = []) {
        const el = document.createElement(type);
        Object.entries(props).forEach(([key, value]) => {
            if (key.startsWith('on')) el.addEventListener(key.slice(2).toLowerCase(), value);
            else if (key === 'className') el.className = value;
            else el.setAttribute(key, value);
        });
        children.forEach(child => {
            if (typeof child === 'string') el.appendChild(document.createTextNode(child));
            else el.appendChild(child);
        });
        return el;
    },

    createWindow(title, content) {
        const win = this.createComponent('div', { className: 'sui-window mica-effect' }, [
            this.createComponent('div', { className: 'sui-window-titlebar' }, [
                this.createComponent('span', {}, [title]),
                this.createComponent('button', { className: 'sui-close-btn', onClick: () => win.remove() }, ['×'])
            ]),
            this.createComponent('div', { className: 'sui-window-content' }, [content])
        ]);
        document.body.appendChild(win);
        return win;
    }
};

if (typeof window !== 'undefined') {
    window.SovereignUI = SovereignUI;

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
}
