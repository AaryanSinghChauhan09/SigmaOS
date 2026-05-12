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
        name: 'terraforming_engineer',
        tools: 'Atmospheric modeler, lithosphere stabilizer, bio-sphere injector, planetary thermal monitor',
        rules: 'Planetary protection protocols, ethical terraforming guidelines',
        calculators: 'Atmospheric pressure delta, solar radiation shielding, oxygen-cycle rate',
        algorithms: 'Climate engineering simulation, ecosystem stability prediction',
        self_healing: 'Atmospheric probe calibration, bio-dome integrity monitoring',
        rollback: 'Planetary state snapshots, bio-injection phase rollback'
    },
    {
        name: 'fusion_reactor_technician',
        tools: 'Tokamak-monitor, magnetic-containment-nexus, plasma-density-Lattice, tritium-breeder-auditor',
        rules: 'IAEA fusion safety standards, high-energy-physics protocols',
        calculators: 'Plasma-beta-ratio, Lawson-criterion-delta, neutron-flux-calc',
        algorithms: 'Automated plasma stability control, ELM-suppression orchestration',
        self_healing: 'Magnetic-field-calibration sync, containment-integrity verify',
        rollback: 'Reactor-state snapshots, baseline-plasma-parameters'
    },
    {
        name: 'quantum_battery_chemist',
        tools: 'Ion-transport-simulator, anode-cathode-Lattice, charge-density-mapper, SEI-layer-auditor',
        rules: 'Material safety data, quantum-chemistry standards',
        calculators: 'Energy-density-delta, cycle-life-predictor, thermal-runaway-risk',
        algorithms: 'Generative electrolyte design, molecular-structure optimization',
        self_healing: 'Charge-balance verification, thermal-sensor-sync',
        rollback: 'Chemical-state snapshots, battery-configuration history'
    },
    {
        name: 'geo_engineering_analyst',
        tools: 'Albedo-monitor, stratospheric-aerosol-Lattice, carbon-cycle-modeler',
        rules: 'UN Environmental protocols, SRM-governance guidelines',
        calculators: 'Radiative-forcing-delta, aerosol-depth-ratio, precipitation-impact',
        algorithms: 'Climate-impact simulation, unintended-consequence modeling',
        self_healing: 'Aerosol-delivery-sync, atmospheric-probe verification',
        rollback: 'Geo-state snapshots, atmospheric-baseline restoration'
    },
    {
        name: 'carbon_sequestration_engineer',
        tools: 'CCS-nexus, geological-storage-Lattice, plume-migration-monitor',
        rules: 'EPA CCS regulations, carbon-capture standards',
        calculators: 'Injection-pressure-delta, storage-capacity-ratio, leakage-probability',
        algorithms: 'Plume-migration prediction, sequestration-efficiency optimization',
        self_healing: 'Well-integrity verification, plume-sensor-sync',
        rollback: 'Injection-state snapshots, geological-baseline restoration'
    },
    {
        name: 'erp_consultant',
        tools: 'Odoo-Lattice, SAP-Sovereign-Bridge, business process modeler, ERP-migration-nexus, module-optimizer',
        rules: 'Business logic standards, ERP-best-practices, compliance-frameworks',
        calculators: 'TCO (Total Cost of Ownership), ROI-estimator, process-latency-calc',
        algorithms: 'Automated workflow mapping, database schema normalization, inventory-prediction',
        self_healing: 'ERP-database consistency check, module-dependency validation',
        rollback: 'ERP-state snapshots, transactional-history restoration'
    },
    {
        name: 'chartered_accountant',
        tools: 'Tally-Lattice, GST-Nexus, Income Tax Auditor, balance-sheet-lattice, audit-trail-engine',
        rules: 'ICAI standards, Income Tax Act, GST laws, Companies Act',
        calculators: 'Tax-liability, TDS-estimator, capital-gains-calc, depreciation-lattice',
        algorithms: 'Anomaly detection in ledgers, tax-optimization modeling, automated-audit-sampling',
        self_healing: 'Tally-data-integrity scan, GST-reconciliation verification',
        rollback: 'Fiscal-year snapshots, historical ledger restoration'
    },
    {
        name: 'crm_administrator',
        tools: 'Salesforce-Lattice, HubSpot-Bridge, lead-scoring-engine, customer-journey-mapper',
        rules: 'GDPR, CCPA, CRM data-privacy standards',
        calculators: 'CLV (Customer Lifetime Value), churn-rate, conversion-delta',
        algorithms: 'Lead prioritization modeling, automated-marketing-pathing',
        self_healing: 'Lead-data consistency check, automation-workflow validation',
        rollback: 'Customer-record snapshots, marketing-campaign rollback'
    },
    {
        name: 'ecommerce_developer',
        tools: 'Shopify-Lattice, Magento-Sovereign-Core, inventory-sync-nexus, payment-gateway-bridge',
        rules: 'PCI-DSS, e-commerce consumer laws',
        calculators: 'AOV (Average Order Value), cart-abandonment rate, shipping-cost-delta',
        algorithms: 'Inventory-replenishment prediction, personalized-recommendation engine',
        self_healing: 'Inventory-sync verification, payment-link validation',
        rollback: 'Store-state snapshots, order-history restoration'
    },
    {
        name: 'digital_marketer',
        tools: 'SERP-Lattice, SEM-Analyzer, crawl-budget-optimizer, backlink-auditor, conversion-pixel-nexus',
        rules: 'Search engine guidelines, advertising ethics',
        calculators: 'CPC/CPM, ROAS, keyword-density, domain-authority-score',
        algorithms: 'Keyword-ranking prediction, automated-bidding optimization',
        self_healing: 'Pixel-tracking verification, backlink-integrity scan',
        rollback: 'SEO-state snapshots, campaign-history restoration'
    },
    {
        name: 'defi_architect',
        tools: 'Liquidity-Lattice, AMM-simulator, yield-curve-nexus, flash-loan-auditor',
        rules: 'Smart-contract security standards, DeFi-governance protocols',
        calculators: 'Impermanent loss, slippage-delta, TVL-ratio, gas-optimization',
        algorithms: 'Automated market making, risk-tranche simulation',
        self_healing: 'Contract-state verification, liquidity-mesh sync',
        rollback: 'Vault-state snapshots, protocol-baseline restoration'
    },
    {
        name: 'digital_evidence_analyst',
        tools: 'Chain-of-custody-Lattice, hash-nexus, cold-storage-bridge, metadata-carver',
        rules: 'BSA 2023 evidence standards, digital-forensics protocols',
        calculators: 'Entropy-density, file-integrity-score, timeline-delta',
        algorithms: 'Automated artifact correlation, encrypted-volume detection',
        self_healing: 'Evidence-seal verification, hash-lattice integrity check',
        rollback: 'Analysis-state snapshots, evidence-baseline restoration'
    },
    {
        name: 'generative_art_curator',
        tools: 'Latent-space-navigator, GAN-monitor, NFT-lattice-bridge, style-transfer-engine',
        rules: 'AI-ethics in art, digital-provenance standards',
        calculators: 'Visual-entropy, stylistic-divergence, aesthetic-score',
        algorithms: 'Prompt-optimization, latent-walk orchestration',
        self_healing: 'Art-lattice integrity check, provenance-seal verification',
        rollback: 'Curation-state snapshots, aesthetic-baseline restoration'
    },
    {
        name: 'virtual_world_historian',
        tools: 'Metaverse-archiver, avatar-ancestry-Lattice, world-state-tracer',
        rules: 'Digital-preservation ethics, virtual-heritage standards',
        calculators: 'World-age, social-graph-density, event-epoch-delta',
        algorithms: 'Cross-world event correlation, virtual-evolution mapping',
        self_healing: 'Archival-sync verification, world-state integrity check',
        rollback: 'Historical-state snapshots, timeline-baseline restoration'
    },
    {
        name: 'particle_physicist',
        tools: 'CERN-Lattice-link, collider-event analyzer, Higgs-boson-search-nexus, muon-chamber monitor',
        rules: 'Open-science protocols, high-energy safety standards',
        calculators: 'Energy-to-mass delta, collision-luminosity, decay-width',
        algorithms: 'Automated track reconstruction, dark-matter candidate search',
        self_healing: 'Sensor-lattice calibration, data-stream consistency check',
        rollback: 'Experiment-state snapshots, baseline-simulation restoration'
    },
    {
        name: 'submarine_navigator',
        tools: 'SONAR-Lattice, pressure-hull monitor, bathymetric-map-engine, silent-running-nexus',
        rules: 'Maritime law, deep-sea exploration protocols',
        calculators: 'Buoyancy-delta, acoustic-range, pressure-limit-calc',
        algorithms: 'Sub-surface obstacle avoidance, passive sonar triangulation',
        self_healing: 'Acoustic-link verification, hull-sensor-integrity scan',
        rollback: 'Dive-log snapshots, ballast-configuration restoration'
    },
    {
        name: 'smart_materials_engineer',
        tools: 'Nano-lattice modeler, shape-memory-alloy simulator, self-healing-polymer monitor',
        rules: 'Material safety standards, nano-tech regulations',
        calculators: 'Young\'s modulus delta, phase-transition temp, molecular-strain',
        algorithms: 'Generative material design, micro-structure simulation',
        self_healing: 'Material-property-sync, molecular-model validation',
        rollback: 'Material-iteration snapshots, chemical-baseline restoration'
    },
    {
        name: 'hyperloop_operator',
        tools: 'Vacuum-tube monitor, maglev-alignment-nexus, pod-telemetry-mesh, capsule-life-support',
        rules: 'High-speed transit safety protocols',
        calculators: 'G-force delta, braking-distance, pod-spacing-minima',
        algorithms: 'Pod-dispatch optimization, vacuum-pressure regulation',
        self_healing: 'Track-alignment verification, pod-telemetry-sync',
        rollback: 'Transit-state snapshots, pod-dispatch-baseline restoration'
    },
    {
        name: 'foley_artist',
        tools: 'Sound-lattice, spectral-layering-engine, Foley-prop-index, real-time-resynthesizer',
        rules: 'Audio post-production standards',
        calculators: 'Sample-rate delta, reverb-tail-calc, sync-offset',
        algorithms: 'Automated footstep matching, textural sound synthesis',
        self_healing: 'Audio-buffer consistency, Prop-Lattice sync',
        rollback: 'Mix-state snapshots, sound-design-history restoration'
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
    },
    {
        name: 'grid_resilience_engineer',
        tools: 'Smart-grid monitor, load balancer-Lattice, phase-angle sync, SCADA-shield',
        rules: 'IEEE standards, FERC regulations, NERC CIP compliance',
        calculators: 'Frequency stability, reactive power, peak-load predictor',
        algorithms: 'Dynamic line rating, islanding detection, black-start optimization',
        self_healing: 'Automatic breaker re-closing, load-shedding lattice priority',
        rollback: 'Pre-fault grid state restoration, historical load profiles'
    },
    {
        name: 'viticulturist',
        tools: 'Soil-moisture mesh, brix-analyzer, phenolic-maturity monitor, weather-Lattice',
        rules: 'Appellation d\'origine contrôlée (AOC) standards, organic certification rules',
        calculators: 'Pruning density, irrigation delta, fermentation heat-rate',
        algorithms: 'Harvest-window prediction, yield-estimation simulation',
        self_healing: 'Micro-climate sensor sync, pest-pressure verification',
        rollback: 'Seasonal growth logs, irrigation state history'
    },
    {
        name: 'actuary',
        tools: 'Risk-modeling lattice, mortality-table engine, solvency-check nexus, monte-carlo-sim',
        rules: 'IFRS 17, Solvency II, actuarial standards',
        calculators: 'Net present value (NPV), liability-duration, loss-ratio',
        algorithms: 'Stochastic modeling, predictive claim analytics',
        self_healing: 'Data consistency verification, model-drift detection',
        rollback: 'Policy-state snapshots, historical loss development'
    },
    {
        name: 'crisis_negotiator',
        tools: 'Voice-stress analyzer, semantic sentiment engine, hostage-link bridge, secure-comms',
        rules: 'Negotiation protocols, de-escalation guidelines',
        calculators: 'Time-elapsed risk, psychological profile score',
        algorithms: 'Behavioral change staircase model, automated sentiment tracking',
        self_healing: 'Communication-link stability check, secure-line verification',
        rollback: 'Negotiation log history, previous incident case-restoration'
    },
    {
        name: 'luthier',
        tools: 'Acoustic-frequency analyzer, wood-density scanner, micro-planer, resonance-Lattice',
        rules: 'CITES wood-usage regs, traditional craft standards',
        calculators: 'String-tension, bridge-pressure, vibrational-mode delta',
        algorithms: 'Resonance-box simulation, structural-vibration analysis',
        self_healing: 'Workshop-climate sync, resonance-profile verification',
        rollback: 'Instrument-design snapshots, vibrational-history restoration'
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
    'Computational Linguist', 'Cultural Heritage Guardian', 'Odoo Developer', 'Tally Operator', 'SAP Consultant',
    'Salesforce Developer', 'Shopify Expert', 'E-commerce Manager',
    'Seismologist', 'Tunneling Expert', 'Urban Agriculture Designer', 'Myrologist', 'Astrobiologist',
    'Colorist', 'Calligrapher', 'Signal Jammer', 'EOD Technician', 'Hyperloop Operator',
    'Maglev Engineer', 'Vexillologist', 'Numismatist', 'Particle Physicist',
    'DeFi Architect', 'Digital Evidence Analyst', 'Generative Art Curator', 'Virtual World Historian',
    'Predictive Policing Strategist', 'CBDC Developer', 'Key Custodian', 'Digital Legacy Manager'
];

additionalProfessions.forEach(name => {
    const slug = name.toLowerCase().replace(/[\s\/\-\.]/g, '_').replace(/_+/g, '_').replace(/^_|_$/g, '');
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
    toolsMd += `## 🛡 Self-Healing Strategy\n- ${prof_healing = prof.self_healing}\n\n`;
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
