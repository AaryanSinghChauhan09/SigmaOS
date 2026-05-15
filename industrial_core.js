/**
 * Σ SIGMAOS: INDUSTRIAL CORE EXTENSION
 * Advanced UI Telemetry & Lattice Visualization
 */

'use strict';

// Lattice Mesh Visualization
function initLatticeMesh() {
    const svg = document.getElementById('global-lattice-svg');
    if (!svg) return;
    
    const width = window.innerWidth;
    const height = window.innerHeight;
    svg.setAttribute('viewBox', `0 0 ${width} ${height}`);
    
    const nodes = [];
    const numNodes = 40;
    
    for (let i = 0; i < numNodes; i++) {
        nodes.push({
            x: Math.random() * width,
            y: Math.random() * height,
            vx: (Math.random() - 0.5) * 0.8,
            vy: (Math.random() - 0.5) * 0.8
        });
    }
    
    function draw() {
        svg.innerHTML = '';
        nodes.forEach(n => {
            n.x += n.vx;
            n.y += n.vy;
            
            if (n.x < 0 || n.x > width) n.vx *= -1;
            if (n.y < 0 || n.y > height) n.vy *= -1;
            
            const circle = document.createElementNS("http://www.w3.org/2000/svg", "circle");
            circle.setAttribute("cx", n.x);
            circle.setAttribute("cy", n.y);
            circle.setAttribute("r", "2");
            circle.setAttribute("fill", "var(--accent)");
            svg.appendChild(circle);
        });
        
        for (let i = 0; i < nodes.length; i++) {
            for (let j = i + 1; j < nodes.length; j++) {
                const dx = nodes[i].x - nodes[j].x;
                const dy = nodes[i].y - nodes[j].y;
                const dist = Math.sqrt(dx * dx + dy * dy);
                
                if (dist < 180) {
                    const line = document.createElementNS("http://www.w3.org/2000/svg", "line");
                    line.setAttribute("x1", nodes[i].x);
                    line.setAttribute("y1", nodes[i].y);
                    line.setAttribute("x2", nodes[j].x);
                    line.setAttribute("y2", nodes[j].y);
                    line.setAttribute("stroke", "var(--accent)");
                    line.setAttribute("stroke-width", "0.5");
                    line.setAttribute("opacity", (1 - dist / 180) * 0.3);
                    svg.appendChild(line);
                }
            }
        }
        requestAnimationFrame(draw);
    }
    draw();
}

// Industrial Toast Notifications
window.showToast = (message, type = 'info') => {
    const container = document.getElementById('toast-container') || createToastContainer();
    const toast = document.createElement('div');
    toast.className = `toast toast-${type}`;
    toast.innerHTML = `
        <div class="toast-content">
            <span class="toast-icon">${getToastIcon(type)}</span>
            <span class="toast-message">${message}</span>
        </div>
        <div class="toast-progress"></div>
    `;
    container.appendChild(toast);
    
    setTimeout(() => {
        toast.classList.add('toast-exit');
        setTimeout(() => toast.remove(), 400);
    }, 4000);
};

function createToastContainer() {
    const div = document.createElement('div');
    div.id = 'toast-container';
    div.className = 'toast-container';
    document.body.appendChild(div);
    return div;
}

function getToastIcon(type) {
    const icons = {
        success: '✓',
        error: '✕',
        warning: '⚠',
        info: 'ℹ'
    };
    return icons[type] || '•';
}

// Performance Optimization
window.addEventListener('load', () => {
    initLatticeMesh();
    window.showToast('Lattice Singularity Initialized', 'success');
});

// Resize handler
window.addEventListener('resize', () => {
    const svg = document.getElementById('global-lattice-svg');
    if (svg) {
        svg.setAttribute('viewBox', `0 0 ${window.innerWidth} ${window.innerHeight}`);
    }
});
