const fs = require('fs');
const path = require('path');

const professions = [
    { 
        name: 'accountant', 
        tools: 'Ledger system, tax compliance modules, balance sheet generator, audit tools, Tally-lattice integration',
        rules: 'Ind-AS, GST, Income Tax Act, Companies Act',
        calculators: 'GST, TDS, depreciation, advance tax, annuity',
        algorithms: 'Automated ledger balancing, audit trail generation, tax filing optimization',
        self_healing: 'Continuous audit log integrity check, transaction consistency validation',
        rollback: 'Snapshot-per-transaction, daily fiscal state preservation'
    },
    { 
        name: 'doctor', 
        tools: 'Patient records, prescription management, lab results integration, telemedicine tools, DICOM viewer',
        rules: 'Medical Council of India guidelines, Ayushman Bharat protocols, HIPAA, GDPR',
        calculators: 'Dosage, BMI, medical billing, glomerular filtration rate',
        algorithms: 'Patient record indexing, diagnostic prediction, lab result interpretation',
        self_healing: 'Medical data integrity scan, prescription conflict detection',
        rollback: 'Patient record versioning, emergency state restoration'
    },
    { 
        name: 'lawyer', 
        tools: 'Case indexing, legal references, compliance document management, secure communication, BNS-IPC cross-mapper',
        rules: 'Bharatiya Nyaya Sanhita (BNS) 2023, Bharatiya Nagarik Suraksha Sanhita (BNSS) 2023, Bharatiya Sakshya Adhiniyam (BSA) 2023, Constitution of India',
        calculators: 'Court fee, stamp duty, Limitation Period Calc, Alimony Estimator',
        algorithms: 'Case law search, compliance checker, automated legal drafting',
        self_healing: 'Legal reference sync, document authenticity verification',
        rollback: 'Case history snapshots, immutable filing records'
    },
    { 
        name: 'engineer', 
        tools: 'Compilers, CAD tools, simulation engines, project management dashboards, Finite Element Analysis (FEA)',
        rules: 'Bureau of Indian Standards (BIS), IS codes, Eurocodes',
        calculators: 'Structural load, electrical circuit, fluid mechanics, thermal stress',
        algorithms: 'CAD simulation, stress analysis, project scheduling',
        self_healing: 'Simulation state verification, build-cache integrity check',
        rollback: 'Project-wide git-lattice rollback, CAD design versioning'
    },
    {
        name: 'architect',
        tools: 'BIM software, 3D modeling, site analysis tools, rendering engines, GIS mapping',
        rules: 'Building bye-laws, National Building Code (NBC), sustainability standards',
        calculators: 'FSI/FAR, structural stability, lighting/ventilation, carbon footprint',
        algorithms: 'Generative design, urban density simulation',
        self_healing: 'BIM model consistency check, asset link validation',
        rollback: 'Iterative design snapshots, client approval state rollback'
    },
    {
        name: 'data_scientist',
        tools: 'Jupyter notebooks, ML libraries, visualization dashboards, data lake access, GPU-Lattice',
        rules: 'GDPR, Data Protection Act (DPDP), AI Ethics guidelines',
        calculators: 'Statistical confidence, model accuracy, feature importance, TPU allocation',
        algorithms: 'Neural architecture search, automated feature engineering, hyperparameter tuning',
        self_healing: 'Data drift detection, model weight integrity check',
        rollback: 'Training checkpoint restoration, dataset versioning'
    },
    {
        name: 'quantum_algorithm_researcher',
        tools: 'Qiskit-Lattice, QuIP interpreter, cryogenic state monitor, entanglement analyzer',
        rules: 'Quantum supremacy protocols, error correction standards',
        calculators: 'Qubit fidelity, decoherence rate, gate-error probability',
        algorithms: 'Shor\'s optimization, Grover\'s search, Quantum Fourier Transform',
        self_healing: 'Continuous decoherence correction, qubit state validation',
        rollback: 'Quantum state checkpointing, entanglement-graph restoration'
    },
    {
        name: 'nuclear_physicist',
        tools: 'Reactor core monitor, neutron flux analyzer, isotope tracking, SCADA-Lattice',
        rules: 'IAEA safety standards, NRC protocols',
        calculators: 'Half-life, criticality index, radiation dose',
        algorithms: 'Monte Carlo N-Particle (MCNP), fission rate simulation',
        self_healing: 'Fail-safe core monitoring, containment integrity scan',
        rollback: 'Reactor state snapshot, safety protocol baseline restoration'
    },
    {
        name: 'ship_captain',
        tools: 'ECDIS navigation, AIS-Lattice, weather routing, ballast manager',
        rules: 'SOLAS, MARPOL, STCW',
        calculators: 'Fuel consumption, eta, cargo stability (GM)',
        algorithms: 'Route optimization, collision avoidance, tidal prediction',
        self_healing: 'Navigational data sync, engine diagnostic monitoring',
        rollback: 'Voyage log snapshots, autopilot calibration restoration'
    },
    {
        name: 'disaster_recovery_coordinator',
        tools: 'Real-time hazard mapping, resource allocation mesh, secure satellite comms',
        rules: 'NDMA guidelines, FEMA protocols',
        calculators: 'Casualty estimation, resource burn rate, logistics pathing',
        algorithms: 'Multi-agent evacuation routing, supply chain optimization',
        self_healing: 'Communication mesh integrity, sensor network validation',
        rollback: 'Incident command state restoration, resource map versioning'
    },
    {
        name: 'intelligence_officer',
        tools: 'Signals intelligence (SIGINT) analyzer, OSINT-Lattice, secure satellite uplink, steganography detector',
        rules: 'Classified protocols, zero-trust communication',
        calculators: 'Encryption strength, signal-to-noise ratio, geolocation triangulator',
        algorithms: 'Heuristic pattern matching, semantic link analysis',
        self_healing: 'Secure channel re-establishment, payload sanitization',
        rollback: 'Redaction-safe state restoration, operational security (OPSEC) baseline'
    },
    {
        name: 'detective',
        tools: 'Evidence timeline manager, witness statement index, facial recognition-Lattice, case-link visualizer',
        rules: 'Miranda rights protocols, evidence chain-of-custody',
        calculators: 'Probability of occurrence, forensic time-delta',
        algorithms: 'Behavioral pattern analysis, link-graph traversal',
        self_healing: 'Evidence database sync, case file integrity check',
        rollback: 'Investigation state snapshots, immutable evidence log restoration'
    },
    {
        name: 'air_traffic_controller',
        tools: 'Radar-Lattice, flight path predictor, collision alert nexus, comm-link bridge',
        rules: 'ICAO standards, FAA protocols',
        calculators: 'Separation minima, fuel-to-weight ratio, descent rate',
        algorithms: 'Conflict detection and resolution (CD&R), 4D trajectory prediction',
        self_healing: 'Redundant radar link verification, comms-failover automation',
        rollback: 'Flight state historical restoration, sequence logs'
    },
    {
        name: 'perfumer',
        tools: 'Scent-molecule database, GC-MS analyzer link, formula optimizer, olfactory sim',
        rules: 'IFRA standards, allergen compliance',
        calculators: 'Concentration ratio, evaporation rate, sillage estimator',
        algorithms: 'Scent-chord generator, molecular stability prediction',
        self_healing: 'Formula consistency check, component availability sync',
        rollback: 'Iterative scent-state restoration, allergen safety baseline'
    },
    {
        name: 'terraforming_engineer',
        tools: 'Atmospheric modeler, lithosphere stabilizer, bio-sphere injector, planetary thermal monitor',
        rules: 'Planetary protection protocols, ethical terraforming guidelines',
        calculators: 'Atmospheric pressure delta, solar radiation shielding, oxygen-cycle rate',
        algorithms: 'Climate engineering simulation, ecosystem stability prediction',
        self_healing: 'Atmospheric probe calibration, bio-dome integrity monitoring',
        rollback: 'Planetary state snapshots, bio-injection phase rollback'
    },
    {
        name: 'asteroid_miner',
        tools: 'Spectroscopic analyzer, drill-bit thermal monitor, low-gravity navigation, ore-density mapper',
        rules: 'Outer Space Treaty compliance, resource extraction protocols',
        calculators: 'Delta-V, ore-yield probability, trajectory-delta',
        algorithms: 'Autonomous excavation pathing, structural stability prediction in vacuum',
        self_healing: 'Thruster-link verification, drill-head cooling sync',
        rollback: 'Excavation state snapshots, orbital-return baseline restoration'
    },
    {
        name: 'bio_ethicist',
        tools: 'Genetic-modification auditor, bio-risk modeler, ethical-compliance dashboard, CRISPR-log-Lattice',
        rules: 'Declaration of Helsinki, Belmont Report, bio-ethics guidelines',
        calculators: 'Risk-benefit ratio, genetic-diversity index',
        algorithms: 'Ethical-dilemma simulation, long-term impact analysis',
        self_healing: 'Compliance-log integrity check, ethical-seal verification',
        rollback: 'Decision-state snapshots, protocol baseline restoration'
    },
    {
        name: 'digital_archaeologist',
        tools: 'Data-carving suite, legacy-system emulator, bit-rot detector, archival-lattice',
        rules: 'Digital preservation standards, copyright-fair-use protocols',
        calculators: 'Data-integrity-hash, bit-error rate, compression-ratio',
        algorithms: 'Fragmented data reconstruction, legacy-format detection',
        self_healing: 'Bit-rot correction, archival-sync verification',
        rollback: 'Data-state snapshots, archival-integrity restoration'
    },
    {
        name: 'orbital_debris_manager',
        tools: 'Debris-tracking radar, laser-ablation monitor, collision-avoidance nexus, TLE-Lattice',
        rules: 'IADC space debris mitigation guidelines',
        calculators: 'Probability of collision (Pc), orbital-decay rate, delta-V for de-orbit',
        algorithms: 'Cascading collision prediction, debris-cloud propagation',
        self_healing: 'Radar-link consistency, laser-aiming-calibration verification',
        rollback: 'Orbital-map state snapshots, de-orbit sequence logs'
    },
    {
        name: 'pqc_auditor',
        tools: 'Lattice-based-crypto analyzer, quantum-threat-modeler, PQC-seal-validator, entropy-mesh',
        rules: 'NIST PQC standards, FIPS 140-3',
        calculators: 'Quantum-resistance-score, entropy-density, key-strength delta',
        algorithms: 'Post-quantum signature verification, cryptographic-mesh auditing',
        self_healing: 'Key-lattice integrity check, PQC-seal validation',
        rollback: 'Security-state snapshots, PQC-baseline restoration'
    }
];

// Extend professions with the requested list
const additionalProfessions = [
    'Software Developer', 'Data Engineer', 'Machine Learning Developer', 'AI Trainer', 'Cloud Architect',
    'DevOps Engineer', 'Embedded Systems Engineer', 'Game Developer', 'Web Developer', 'Mobile App Developer',
    'Project Manager', 'Business Analyst', 'Product Manager', 'Financial Analyst', 'HR Manager',
    'Operations Manager', 'Supply Chain Analyst', 'Compliance Officer', 'Medical Researcher', 'Biotechnologist',
    'Geneticist', 'Radiologist', 'Public Health Official', 'Clinical Trial Manager', 'Healthcare Administrator',
    'Graphic Designer', 'Video Editor', 'Animator', 'Photographer', 'Music Producer', 'Writer', 'Journalist',
    'Content Creator', 'UX/UI Designer', 'Marketing Specialist', 'Physicist', 'Chemist', 'Mathematician',
    'Statistician', 'Astronomer', 'Environmental Scientist', 'Geologist', 'Oceanographer', 'Social Scientist',
    'Economist', 'Policymaker', 'Civil Servant', 'Diplomat', 'Military Officer', 'Intelligence Analyst',
    'Forensic Expert', 'Tax Official', 'Urban Planner', 'Professor', 'Curriculum Designer',
    'Educational Researcher', 'Instructioner Designer', 'E-Learning Specialist', 'School Administrator',
    'Librarian', 'Language Instructor', 'Vocational Trainer', 'Mechanical Engineer', 'Electrical Engineer',
    'Civil Engineer', 'Chemical Engineer', 'Industrial Designer', 'Quality Assurance Specialist',
    'Robotics Engineer', 'Automotive Engineer', 'Aerospace Engineer', 'Energy Analyst', 'Blockchain Developer',
    'IoT Specialist', 'AR/VR Developer', 'Drone Operator', 'Ethical Hacker',
    'Sustainability Consultant', 'Climate Change Analyst', 'Smart City Planner', 'Digital Twin Engineer',
    'Customer Support Specialist', 'Sales Executive', 'Event Manager', 'Travel Agent', 'Logistics Coordinator',
    'Real Estate Analyst', 'Insurance Underwriter', 'Retail Manager', 'Hospitality Manager', 'NGO Worker',
    'Gemologist', 'Horologist', 'Carpenter', 'Fire Chief', 'Emergency Dispatcher', 'VR Director', 'Podcaster',
    'Synthetic Biologist', 'Philatelist', 'Genealogist', 'Astronaut', 'Mission Controller',
    'Cryptographer', 'Ballistics Expert', 'Special Ops Coordinator', 'Cybercrime Investigator', 'Detective',
    'CSI', 'Railway Signaling Engineer', 'Autonomous Fleet Manager', 'Audio Engineer', 'Cinematographer',
    'Restoration Artist', 'Sociologist', 'Political Campaign Manager', 'Archivist', 'Botanist', 'Zoologist',
    'Soil Scientist', 'Transhumanist Researcher', 'Behavioral Economist', 'Crypto-Asset Auditor',
    'Sports Psychologist', 'Physiotherapist', 'Set Designer', 'Pyrotechnician', 'Ethnomusicologist',
    'Philologist', 'Blacksmith', 'Glassblower', 'Bodyguard', 'Cyber-Warfare Tactician', 'Water Treatment Manager',
    'Xeno-Biologist', 'Prosthetics Engineer', 'Cyber-Insurance Underwriter', 'OSINT Investigator',
    'Underwater Architect', 'Master Sommelier', 'Yacht Designer', 'Private Island Manager',
    'Computational Linguist', 'Cultural Heritage Guardian'
];

additionalProfessions.forEach(name => {
    const slug = name.toLowerCase().replace(/ /g, '_').replace(/-/g, '_');
    if (!professions.find(p => p.name === slug)) {
        professions.push({
            name: slug,
            tools: `${name} workspace, specialized analytics, compliance manager`,
            rules: 'Industry standards, professional ethics',
            calculators: 'Efficiency metrics, cost analysis',
            algorithms: 'Workflow optimization, adaptive assistance',
            self_healing: `Continuous health check for ${name} tools`,
            rollback: 'Automatic state snapshots'
        });
    }
});

const profilesDir = path.join(__dirname, 'profiles');

professions.forEach(prof => {
    const profDir = path.join(profilesDir, prof.name);
    if (!fs.existsSync(profDir)) {
        fs.mkdirSync(profDir, { recursive: true });
    }

    // tools.md
    let toolsMd = `# 🛠 ${prof.name.toUpperCase()} Profile\n\n`;
    toolsMd += `## Required Tools & Functions\n- ${prof.tools.split(', ').join('\n- ')}\n\n`;
    if (prof.rules) toolsMd += `## 📜 Compliance Rules\n- ${prof.rules.split(', ').join('\n- ')}\n\n`;
    if (prof.calculators) toolsMd += `## 🧮 Professional Calculators\n- ${prof.calculators.split(', ').join('\n- ')}\n\n`;
    if (prof.algorithms) toolsMd += `## 🧬 Core Algorithms\n- ${prof.algorithms.split(', ').join('\n- ')}\n\n`;
    toolsMd += `## 🛡 Self-Healing Strategy\n- ${prof.self_healing}\n\n`;
    toolsMd += `## 🔄 Rollback Strategy\n- ${prof.rollback}\n`;
    
    fs.writeFileSync(path.join(profDir, 'tools.md'), toolsMd);

    // config.json
    const config = {
        profession: prof.name,
        compliance_level: 'sovereign-high',
        resource_priority: 'adaptive',
        modules: prof.tools.split(', ').map(t => t.toLowerCase().replace(/ /g, '_')),
        rules: prof.rules ? prof.rules.split(', ') : [],
        calculators: prof.calculators ? prof.calculators.split(', ') : [],
        algorithms: prof.algorithms ? prof.algorithms.split(', ') : [],
        automation: {
            self_healing: prof.self_healing,
            rollback: prof.rollback
        }
    };
    fs.writeFileSync(path.join(profDir, 'config.json'), JSON.stringify(config, null, 4));
});

console.log(`Profession profiles updated successfully (${professions.length} profiles).`);
