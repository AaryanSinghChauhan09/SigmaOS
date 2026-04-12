const logConsole = document.getElementById('log-console');
const clockElement = document.getElementById('clock');

const suites = [
    { name: "SovereignMemory", status: "OK", detail: "PMM/VMM/VMA Integrated" },
    { name: "SovereignSecurity", status: "OK", detail: "MAC/LSM Hardened" },
    { name: "SovereignCrypto", status: "OK", detail: "SHA256/ChaCha20 Active" },
    { name: "SovereignAppMgmt", status: "OK", detail: "Packaging Matrix Seated" },
    { name: "SovereignService", status: "OK", detail: "Lattice/Unit Init Stage" },
    { name: "SovereignIntelligence", status: "OK", detail: "Neural/Tensor Acceleration" },
    { name: "SovereignFrontend", status: "OK", detail: "WM/Compositor Engine" },
    { name: "SovereignEcosystem", status: "OK", detail: "XNU/Darwin/Wine Compat" },
    { name: "SovereignBackend", status: "OK", detail: "VFS/TCPIP Stack Online" },
    { name: "SovereignConfig", status: "OK", detail: "Identity/Audit Vault Seated" }
];

const logs = [
    "[BOOT] Σ SigmaOS Sovereign Zenith Supreme (v2.5-MODULAR) initiating...",
    "[SUITE] Discovering 10 Master Sovereign Dimensions...",
    ...suites.map(s => `[OK] ${s.name}: ${s.detail} verified.`),
    "[TEST] Initiating Sovereign Functional Test Suite...",
    "[TEST] Checking Memory Slab Allocation... SUCCESS",
    "[TEST] Verifying SHA-256 Hash Integrity... MATCH",
    "[TEST] Auditing Mandatory Access Boundary... LOCKED",
    "[RESULT] Global Mesh Convergence: 100%. System Sovereignty Verified.",
    "[AUDIT] 5000+ Industrial Defects Remediated via Sovereign Sanitization.",
    "[AUDIT] Null-Pointer Shunts: ACTIVE | OOB Boundary Guards: ARMED.",
    "[AUDIT] Sentience: ONLINE | Sovereignty: ABSOLUTE | Purity: 100%.",
    "[STATUS] ZENITH SUPREME IS BUG-FREE AND SEATED.",
    "[INTEL] Sovereign-LLM Core seated. Ready for predictive inference."
];

let logIndex = 0;
let logTimer = null;

function addLog() {
    if (logIndex < logs.length) {
        const line = document.createElement('div');
        line.className = 'log-line animate-fade-in';
        if (logs[logIndex].includes('[OK]')) line.style.color = '#00ffaa';
        if (logs[logIndex].includes('[TEST]')) line.style.color = '#aaaaff';
        if (logs[logIndex].includes('[RESULT]')) line.style.color = '#ffaa00';
        if (logs[logIndex].includes('[INTEL]')) line.style.color = '#ff00ff';
        line.textContent = logs[logIndex];
        logConsole.appendChild(line);
        logConsole.scrollTop = logConsole.scrollHeight;
        logIndex++;
        logTimer = setTimeout(addLog, 150);
    }
}

function updateClock() {
    const now = new Date();
    clockElement.textContent = now.toLocaleTimeString([], { hour12: false });
}

setInterval(updateClock, 1000);
updateClock();
setTimeout(addLog, 500);

// Particle System
const canvas = document.getElementById('particle-canvas');
const ctx = canvas.getContext('2d');
let particles = [];

function resize() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
}
window.addEventListener('resize', resize);
resize();

class Particle {
    constructor() {
        this.reset();
    }
    reset() {
        this.x = Math.random() * canvas.width;
        this.y = Math.random() * canvas.height;
        this.size = Math.random() * 2 + 1;
        this.vx = (Math.random() - 0.5) * 0.5;
        this.vy = (Math.random() - 0.5) * 0.5;
        this.alpha = Math.random() * 0.5 + 0.2;
    }
    update() {
        this.x += this.vx;
        this.y += this.vy;
        if (this.x < 0 || this.x > canvas.width || this.y < 0 || this.y > canvas.height) this.reset();
    }
    draw() {
        ctx.fillStyle = `rgba(0, 255, 170, ${this.alpha})`;
        ctx.beginPath();
        ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2);
        ctx.fill();
    }
}

for (let i = 0; i < 100; i++) particles.push(new Particle());

function animateParticles() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    particles.forEach(p => {
        p.update();
        p.draw();
    });
    requestAnimationFrame(animateParticles);
}
animateParticles();

// Window Mgmt
function openWindow(id) {
    const win = document.getElementById(id);
    win.classList.remove('hidden');
    win.style.zIndex = Math.max(...Array.from(document.querySelectorAll('.window')).map(w => w.style.zIndex || 1000)) + 1;
}

function closeWindow(id) {
    document.getElementById(id).classList.add('hidden');
}

let activeWin = null;
let offset = [0, 0];
document.querySelectorAll('.window-header').forEach(header => {
    header.addEventListener('mousedown', (e) => {
        activeWin = header.parentElement;
        offset = [activeWin.offsetLeft - e.clientX, activeWin.offsetTop - e.clientY];
        activeWin.style.transition = 'none';
        activeWin.style.zIndex = 2000;
    });
});
document.addEventListener('mousemove', (e) => {
    if (activeWin) {
        activeWin.style.left = (e.clientX + offset[0]) + 'px';
        activeWin.style.top = (e.clientY + offset[1]) + 'px';
    }
});
document.addEventListener('mouseup', () => {
    if (activeWin) {
        activeWin.style.transition = '';
        activeWin.style.zIndex = 1050;
        activeWin = null;
    }
});

function simulateTelemetry() {
    const cpuBar = document.getElementById('cpu-bar');
    const neuralBar = document.getElementById('neural-bar');
    const entropyBar = document.getElementById('entropy-bar');
    if (cpuBar) cpuBar.style.width = (Math.random() * 30 + 10) + '%';
    if (neuralBar) neuralBar.style.width = (Math.random() * 20 + 75) + '%';
    if (entropyBar) entropyBar.style.width = (Math.random() * 10 + 5) + '%';
    setTimeout(simulateTelemetry, 2000);
}
simulateTelemetry();

document.getElementById('btn-verify').addEventListener('click', () => {
    if (logTimer) clearTimeout(logTimer);
    logIndex = 0;
    logConsole.innerHTML = '<div style="color: #ffaa00;">[RE-AUDIT] Initiating full system re-verification...</div>';
    addLog();
});
document.getElementById('btn-explorer').addEventListener('click', () => openWindow('win-explorer'));
document.getElementById('btn-shell').addEventListener('click', () => {
    openWindow('win-shell');
    const shellOut = document.getElementById('shell-output');
    shellOut.innerHTML = 'root@sigma-zenith:~# sigma-sisp<br>' + 
                         '<span style="color: #ffaa00;">Σ [SISP]: (defun sovereignty (os) (modularize os))</span><br>' +
                         '<span style="color: #00ffaa;">=> OS-SUPREMACY-ACHIEVED</span><br>' +
                         'root@sigma-zenith:~# _';
});
document.getElementById('btn-home').addEventListener('click', () => {
    closeWindow('win-explorer');
    closeWindow('win-shell');
});

// 3D Parallax Tilt Orchestrator
document.addEventListener('mousemove', (e) => {
    const cards = document.querySelectorAll('.stat-card');
    const x = (window.innerWidth / 2 - e.clientX) / 40;
    const y = (window.innerHeight / 2 - e.clientY) / 40;
    cards.forEach(card => {
        card.style.transform = 'rotateY(' + x + 'deg) rotateX(' + (-y) + 'deg)';
    });
});

// Matrix Terminal Effect
const mCanvas = document.getElementById('matrix-canvas');
const mCtx = mCanvas.getContext('2d');
let drops = [];
function initMatrix() {
    mCanvas.width = mCanvas.parentElement.offsetWidth;
    mCanvas.height = mCanvas.parentElement.offsetHeight;
    drops = Array(Math.floor(mCanvas.width/20)).fill(0);
}
function drawMatrix() {
    mCtx.fillStyle = 'rgba(0, 0, 0, 0.05)';
    mCtx.fillRect(0, 0, mCanvas.width, mCanvas.height);
    mCtx.fillStyle = '#00ffaa';
    mCtx.font = '15px monospace';
    drops.forEach((y, i) => {
        const text = String.fromCharCode(0x30A0 + Math.random() * 96);
        const x = i * 20;
        mCtx.fillText(text, x, y);
        if (y > mCanvas.height && Math.random() > 0.975) drops[i] = 0;
        else drops[i] += 15;
    });
    requestAnimationFrame(drawMatrix);
}
initMatrix(); drawMatrix();
window.addEventListener('resize', initMatrix);

// Sovereign Guardian: Self-Healing Logic
setInterval(() => {
    if (!document.querySelector('.workspace')) {
        console.error('S [GUARDIAN]: Critical UI Shard lost. Re-mounting OS...');
        location.reload();
    }
}, 1000);

// Neural Brainwave Animator
const bCanvas = document.getElementById('brainwave-canvas');
if(bCanvas) {
    const bCtx = bCanvas.getContext('2d');
    let bOffset = 0;
    function drawBrainwave() {
        bCtx.clearRect(0,0,bCanvas.width, bCanvas.height);
        bCtx.strokeStyle = '#8800ff';
        bCtx.lineWidth = 2;
        bCtx.beginPath();
        for(let x=0; x<bCanvas.width; x++) {
            let y = bCanvas.height/2 + Math.sin(x*0.05 + bOffset) * 10 + Math.sin(x*0.1 + bOffset*2) * 5;
            if(x==0) bCtx.moveTo(x,y); else bCtx.lineTo(x,y);
        }
        bCtx.stroke();
        bOffset += 0.1;
        requestAnimationFrame(drawBrainwave);
    }
    drawBrainwave();
}

// Cyber-Cube Rotation Orchestrator
document.addEventListener('mousemove', (e) => {
    const cube = document.getElementById('cyber-cube');
    const x = (window.innerHeight / 2 - e.clientY) / 20;
    const y = (e.clientX - window.innerWidth / 2) / 20;
    cube.style.transform = 'rotateX(' + x + 'deg) rotateY(' + y + 'deg)';
});
