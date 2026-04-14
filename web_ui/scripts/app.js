const suites = [
    { id: "S01", name: "Genesis" }, { id: "S02", name: "ZenithUI" }, { id: "S03", name: "Orchestrator" },
    { id: "S04", name: "HAL" }, { id: "S05", name: "Memory" }, { id: "S06", name: "Storage" },
    { id: "S07", name: "Network" }, { id: "S08", name: "Security" }, { id: "S09", name: "Intelligence" },
    { id: "S10", name: "Registry" }, { id: "S11", name: "Virtualization" }, { id: "S12", name: "Ecosystem" },
    { id: "S13", name: "Sentience" }, { id: "S14", name: "Transcendence" }, { id: "S15", name: "DevNexus" },
    { id: "S16", name: "SoulMolding" }, { id: "S17", name: "BioNexus" }, { id: "S18", name: "QuantumLink" },
    { id: "S19", name: "SelfEvolution" }, { id: "S20", name: "GlobalVFS" }, { id: "S21", name: "EternalState" },
    { id: "S22", name: "SimNexus" }, { id: "S23", name: "OmniNexus" }, { id: "S24", name: "Debugger" },
    { id: "S25", name: "ZeroKernel" }, { id: "S26", name: "OmniFabric" }, { id: "S27", name: "NeuralLink" },
    { id: "S28", name: "OmniBus" }, { id: "S29", name: "Merge" }, { id: "S30", name: "Supremacy" },
    { id: "S31", name: "Governance" }, { id: "S32", name: "Unified" }, { id: "S33", name: "Singularity" }
];

const grid = document.getElementById('lattice-grid');
const log = document.getElementById('audit-log');
const coverageVal = document.getElementById('coverage-val');

function addLog(msg, type = 'info') {
    const entry = document.createElement('p');
    entry.className = 'log-entry';
    entry.innerHTML = `<span style="color: grey">${new Date().toLocaleTimeString()}</span> > ${msg}`;
    if (type === 'error') entry.style.color = '#f00';
    log.appendChild(entry);
    log.scrollTop = log.scrollHeight;
}

function initGrid() {
    suites.forEach(s => {
        const card = document.createElement('div');
        card.className = 'suite-card';
        card.id = `card-${s.id}`;
        card.innerHTML = `
            <span class="sh-code">${s.id}</span>
            <span class="sh-name">${s.name}</span>
            <div class="sh-status"></div>
        `;
        grid.appendChild(card);
    });
}

async function bootSequence() {
    for (let i = 0; i < suites.length; i++) {
        const s = suites[i];
        await new Promise(r => setTimeout(r, 100 + Math.random() * 200));
        
        const card = document.getElementById(`card-${s.id}`);
        card.classList.add('loaded');
        
        addLog(`Σ [LATTICE]: Materializing ${s.id} (${s.name})... OK`);
        
        const coverage = Math.round(((i + 1) / suites.length) * 100);
        coverageVal.innerText = `${coverage}%`;
    }
    
    addLog(`<span style="color: cyan">Σ [SINGULARITY]: 33-Suite Lattice Materialized. System SEALED.</span>`);
}

// Start
initGrid();
setTimeout(bootSequence, 1000);
