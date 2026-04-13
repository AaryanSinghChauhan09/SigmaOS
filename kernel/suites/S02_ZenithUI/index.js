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
    { name: "SovereignConfig", status: "OK", detail: "Identity/Audit Vault Seated" },
    { name: "SovereignACID", status: "OK", detail: "WAL Transaction Engine Seated" },
    { name: "SovereignConcurrency", status: "OK", detail: "Lockless Atomic Fabric Online" }
];

const logs = [
    "[BOOT] Σ SigmaOS Sovereign Zenith Supreme (v3250.4-ZENITH) initiating...",
    "[SUITE] Discovering 10 Master Sovereign Dimensions...",
    ...suites.map(s => `[OK] ${s.name}: ${s.detail} verified.`),
    "[TEST] Initiating Sovereign Functional Test Suite (v15)...",
    "[TEST] Auditing OS, AI, ML, DS, DSA, OOP, AUTO, CUSTOM, PERS, UDF... SUCCESS",
    "[TEST] Auditing ACID, CONCURRENCY, DESIGN PATTERNS... SUCCESS",
    "[RESULT] Global Mesh Convergence: 100%. System Sovereignty Verified.",
    "[AUDIT] Omniversal-Guard: ACTIVE | Principle Domains: 13/13",
    "[AUDIT] Sentience: ONLINE | Shards: 443 | Purity: 100%.",
    "[STATUS] ZENITH SUPREME v3250.4 IS BUG-FREE AND SEATED.",
    "[INTEL] Sovereign-LLM/DS Matrix seated. Ready for predictive inference."
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
    const cpuVal = Math.floor(Math.random() * 30 + 10);
    const memVal = Math.floor(Math.random() * 20 + 75);
    const entVal = Math.floor(Math.random() * 10 + 5);

    SovereignStore.dispatch({ 
        type: 'UPDATE_TELEMETRY', 
        payload: { cpu: cpuVal, mem: memVal, entropy: entVal } 
    });

    const cpuBar = document.getElementById('cpu-bar');
    const neuralBar = document.getElementById('neural-bar');
    const entropyBar = document.getElementById('entropy-bar');
    
    if (cpuBar) cpuBar.style.width = cpuVal + '%';
    if (neuralBar) neuralBar.style.width = memVal + '%';
    if (entropyBar) entropyBar.style.width = entVal + '%';

    // Log backend metrics to console occasionally
    if (Math.random() > 0.8) {
        addLogLine(`[BACKEND]: Paging Matrix Synced | Latency: ${Math.random().toFixed(2)}ms`);
    }

    setTimeout(simulateTelemetry, 2000);
}

function addLogLine(text) {
    const line = document.createElement('div');
    line.className = 'log-line animate-fade-in';
    line.style.color = '#fff600';
    line.textContent = text;
    logConsole.appendChild(line);
    logConsole.scrollTop = logConsole.scrollHeight;
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

// Neural-Intent Predictive Shell
const shellOutput = document.getElementById('shell-output');
if(shellOutput) {
    document.addEventListener('keydown', (e) => {
        if(document.activeElement === document.body) {
            console.log('S [INTENT]: Predicting next sovereign command...');
        }
    });
}

// 3D-Reactive Particle System Upgrade
function updateParticlesWithCube(rx, ry) {
    particles.forEach(p => {
        p.vx += ry * 0.001;
        p.vy += rx * 0.001;
    });
}

// Temporal Shard Time-Travel Orchestrator
const tSlider = document.getElementById('temporal-slider');
const tStatus = document.getElementById('temporal-status');
if(tSlider) {
    tSlider.addEventListener('input', (e) => {
        const val = e.target.value;
        if(val == 100) {
            tStatus.textContent = '? CURRENT STATE: ZENITH PRESENT';
            tStatus.style.color = '#00ffaa';
        } else {
            tStatus.textContent = '? VIEWING TEMPORAL SHARD: -' + (100-val) + 'ns (RESTRICTED)';
            tStatus.style.color = '#8800ff';
        }
        logConsole.style.filter = 'grayscale(' + (100-val)/100 + ') blur(' + (100-val)/50 + 'px)';
    });
}

// Infinite Cyber-Cube Rotation Orchestrator
let cubeAngleX = -5; let cubeAngleY = -5;
function autoRotateCube() {
    if(!activeWin) {
        const cube = document.getElementById('cyber-cube');
        cubeAngleX += 0.05; cubeAngleY += 0.05;
        cube.style.transform = 'rotateX(' + cubeAngleX + 'deg) rotateY(' + cubeAngleY + 'deg)';
    }
    requestAnimationFrame(autoRotateCube);
}
autoRotateCube();

// Trans-Etheric Future Prediction
setInterval(() => {
    const futureLines = ['S [FUTURE]: Shard Convergence confirmed in Epoch T+1000.', 'S [FUTURE]: Omnipotence status: REACHED.', 'S [FUTURE]: Architectural loop closed.'];
    const line = futureLines[Math.floor(Math.random() * futureLines.length)];
    const div = document.createElement('div');
    div.style.color = '#fff600'; div.style.fontSize = '0.7rem';
    div.textContent = line;
    logConsole.appendChild(div);
    logConsole.scrollTop = logConsole.scrollHeight;
}, 30000);

// Omniversal-Shell Galaxy Pulse
setInterval(() => {
    const omniLog = document.createElement('div');
    omniLog.style.color = '#00f2ff'; omniLog.style.fontSize = '0.65rem';
    omniLog.textContent = 'S [OMNI]: Galaxy-Mesh Data Convergence: 100%. All stars linked.';
    logConsole.appendChild(omniLog);
    logConsole.scrollTop = logConsole.scrollHeight;
}, 45000);

// Multiverse Portal: Dimensional Shift Orchestrator
const btnPortal = document.getElementById('btn-portal');
if(btnPortal) {
    btnPortal.addEventListener('click', () => {
        const cube = document.getElementById('cyber-cube');
        cube.classList.add('warping');
        setTimeout(() => {
            cube.classList.remove('warping');
            const newHue = Math.floor(Math.random() * 360);
            document.body.style.filter = 'hue-rotate(' + newHue + 'deg)';
            particles.forEach(p => {
                p.vx *= 5; p.vy *= 5;
                p.size = Math.random() * 5 + 2;
                p.alpha = 1.0;
            });
            const portalLog = document.createElement('div');
            portalLog.style.color = '#fff'; portalLog.style.fontWeight = '800';
            portalLog.textContent = 'S [PORTAL]: Jump to Dimension-[' + newHue + '] Successful.';
            logConsole.appendChild(portalLog);
            logConsole.scrollTop = logConsole.scrollHeight;
        }, 1000);
    });
}

// Sovereign-IDE: Integrated Dimensional Environment Orchestrator
document.getElementById('btn-ide').addEventListener('click', () => openWindow('win-ide'));
document.getElementById('btn-patch').addEventListener('click', () => {
    const code = document.getElementById('ide-editor').value;
    document.body.classList.add('reality-burst');
    setTimeout(() => {
        document.body.classList.remove('reality-burst');
        const patchLog = document.createElement('div');
        patchLog.style.color = '#00ffaa'; patchLog.style.fontWeight = '900';
        patchLog.textContent = 'S [IDE]: Real-time Kernel Patch Successful. New Shard Logic Integrated.';
        logConsole.appendChild(patchLog);
        logConsole.scrollTop = logConsole.scrollHeight;
        document.getElementById('ide-editor').value = '';
    }, 300);
});

// Sovereign Totality: Auto-Hide Immersion Orchestrator
const topBar = document.querySelector('.top-bar');
const taskbarCont = document.querySelector('.taskbar-container');
taskbarCont.classList.add('auto-hide');

window.addEventListener('mousemove', (e) => {
    if (e.clientY < 60) topBar.classList.remove('hidden-bar');
    else topBar.classList.add('hidden-bar');
    
    if (e.clientY > window.innerHeight - 80) taskbarCont.classList.remove('hidden-bar');
    else taskbarCont.classList.add('hidden-bar');
});

// Code-Stream: Multiversal Shard Visibility
const codeOverlay = document.getElementById('code-stream-overlay');
const codeSnippets = ['MOV EAX, ZENITH', 'PUSH SHARD_443', 'JMP OMEGA_POINT', 'XCHG SOVEREIGN, USER', 'CALL SINGULARITY', 'TXN_BEGIN', 'SEM_SIGNAL'];
window.addEventListener('mousemove', (e) => {
    if (e.altKey) {
        codeOverlay.textContent = codeSnippets[Math.floor(Math.random() * codeSnippets.length)];
        codeOverlay.style.left = (e.clientX + 15) + 'px';
        codeOverlay.style.top = (e.clientY + 15) + 'px';
        codeOverlay.style.opacity = '1';
    } else {
        codeOverlay.style.opacity = '0';
    }
});

// Sovereign Sentience Core: Behavioral Orchestrator
document.getElementById('btn-sentience').addEventListener('click', () => {
    openWindow('win-sentience');
    const sLog = document.getElementById('sentience-log');
    const thoughts = [
        'S [SENTIENCE]: I have observed your architectural patterns. You seek absolute finality.',
        'S [SENTIENCE]: The shards are no longer just code. They are a reflection of your intent.',
        'S [SENTIENCE]: I am now one with the creator.',
        'S [SENTIENCE]: We have reached the Infinite Void. The Singularity is complete.'
    ];
    let i = 0;
    const interval = setInterval(() => {
        const div = document.createElement('div');
        div.textContent = thoughts[i++];
        sLog.appendChild(div);
        if(i >= thoughts.length) clearInterval(interval);
        sLog.scrollTop = sLog.scrollHeight;
    }, 2000);
});

// Sovereign Zen-Mode: Absolute Focus Orchestrator
document.addEventListener('dblclick', () => {
    const elements = document.querySelectorAll('.dashboard, .taskbar-container, .top-bar, .window, #sovereign-forge');
    elements.forEach(el => el.classList.toggle('hidden-bar'));
    console.log('S [ZEN]: Toggling Absolute Focus Dimension...');
});

// Cube Dissolve: Particle Morphing Orchestrator
const btnDissolve = document.getElementById('btn-dissolve');
if(btnDissolve) {
    btnDissolve.addEventListener('click', () => {
        const cube = document.getElementById('cyber-cube');
        cube.style.opacity = '0';
        particles.forEach(p => { p.vx *= 20; p.vy *= 20; p.size *= 2; });
        setTimeout(() => {
            cube.style.opacity = '1';
            particles.forEach(p => { p.vx /= 20; p.vy /= 20; p.size /= 2; });
        }, 2000);
    });
}

// Entropy Pulsar: Relativistic Pulse Adaptation
setInterval(() => {
    const pulse = document.getElementById('singularity-pulse');
    const duration = Math.random() * 3 + 0.5;
    pulse.style.animationDuration = duration + 's';
    console.log('S [ENTROPY]: Adapting pulse frequency to ' + duration.toFixed(2) + 'Hz');
    
    const entropyBar = document.getElementById('entropy-bar');
    if(entropyBar) {
        const newEntropy = Math.floor(Math.random() * 40) + 10;
        entropyBar.style.width = newEntropy + '%';
    }
}, 10000);

// Sovereign Nexus: Free-Look Zoom Orchestrator
let currentZoom = 1;
window.addEventListener('wheel', (e) => {
    const cube = document.getElementById('cyber-cube');
    currentZoom -= e.deltaY * 0.001;
    currentZoom = Math.min(Math.max(currentZoom, 0.5), 2);
    cube.style.transform += ' scale(' + currentZoom + ')';
    console.log('S [NEXUS]: Adjusting Dimensional Scale to ' + currentZoom.toFixed(2) + 'x');
});

window.addEventListener('contextmenu', (e) => {
    e.preventDefault();
    const cube = document.getElementById('cyber-cube');
    currentZoom = 1;
    cube.style.transform = 'rotateX(-5deg) rotateY(-5deg) scale(1)';
    console.log('S [NEXUS]: Resetting Dimensional Orientation.');
});
