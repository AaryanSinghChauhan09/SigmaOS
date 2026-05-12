const fs = require('fs');
const path = require('path');

const professions = [
    { 
        name: 'accountant', 
        tools: 'Ledger system, tax compliance modules, balance sheet generator, audit tools',
        rules: 'Ind-AS, GST, Income Tax Act, Companies Act',
        calculators: 'GST, TDS, depreciation, advance tax',
        algorithms: 'Automated ledger balancing, audit trail generation, tax filing optimization',
        self_healing: 'Continuous audit log integrity check, transaction consistency validation',
        rollback: 'Snapshot-per-transaction, daily fiscal state preservation'
    },
    { 
        name: 'doctor', 
        tools: 'Patient records, prescription management, lab results integration, telemedicine tools',
        rules: 'Medical Council of India guidelines, Ayushman Bharat protocols, HIPAA',
        calculators: 'Dosage, BMI, medical billing',
        algorithms: 'Patient record indexing, diagnostic prediction, lab result interpretation',
        self_healing: 'Medical data integrity scan, prescription conflict detection',
        rollback: 'Patient record versioning, emergency state restoration'
    },
    { 
        name: 'lawyer', 
        tools: 'Case indexing, legal references, compliance document management, secure communication',
        rules: 'Bharatiya Nyaya Sanhita (BNS) 2023, Bharatiya Nagarik Suraksha Sanhita (BNSS) 2023, Bharatiya Sakshya Adhiniyam (BSA) 2023, Constitution of India',
        calculators: 'Court fee, stamp duty, Limitation Period Calc, Alimony Estimator',
        algorithms: 'Case law search, compliance checker, automated legal drafting, BNS-IPC cross-mapper',
        self_healing: 'Legal reference sync, document authenticity verification',
        rollback: 'Case history snapshots, immutable filing records'
    },
    { 
        name: 'engineer', 
        tools: 'Compilers, CAD tools, simulation engines, project management dashboards',
        rules: 'Bureau of Indian Standards (BIS), IS codes',
        calculators: 'Structural load, electrical circuit, fluid mechanics',
        algorithms: 'CAD simulation, stress analysis, project scheduling',
        self_healing: 'Simulation state verification, build-cache integrity check',
        rollback: 'Project-wide git-lattice rollback, CAD design versioning'
    },
    {
        name: 'architect',
        tools: 'BIM software, 3D modeling, site analysis tools, rendering engines',
        rules: 'Building bye-laws, National Building Code (NBC)',
        calculators: 'FSI/FAR, structural stability, lighting/ventilation',
        algorithms: 'Generative design, urban density simulation',
        self_healing: 'BIM model consistency check, asset link validation',
        rollback: 'Iterative design snapshots, client approval state rollback'
    },
    {
        name: 'data_scientist',
        tools: 'Jupyter notebooks, ML libraries, visualization dashboards, data lake access',
        rules: 'GDPR, Data Protection Act (DPDP)',
        calculators: 'Statistical confidence, model accuracy, feature importance',
        algorithms: 'Neural architecture search, automated feature engineering',
        self_healing: 'Data drift detection, model weight integrity check',
        rollback: 'Training checkpoint restoration, dataset versioning'
    },
    {
        name: 'cybersecurity_analyst',
        tools: 'Threat detection, penetration testing, compliance dashboards, IDS/IPS',
        rules: 'ISO 27001, NIST framework, CERT-In guidelines',
        calculators: 'Risk score, CVSS, entropy check',
        algorithms: 'Anomaly detection, heuristic threat hunting',
        self_healing: 'Automated firewall rules, sandboxed malware containment',
        rollback: 'Immutable audit logs, system-wide security baseline restoration'
    },
    {
        name: 'forensic_scientist',
        tools: 'Evidence indexing, chain of custody manager, DNA analysis, bit-stream imaging',
        rules: 'Evidence Act, BNSS forensic protocols',
        calculators: 'Probability of match, decomposition rate',
        algorithms: 'Pattern recognition, digital fingerprinting',
        self_healing: 'Hash-based integrity check for evidence files',
        rollback: 'Chain-of-custody immutable state restoration'
    },
    {
        name: 'finance_analyst',
        tools: 'Market dashboards, risk analysis tools, AI forecasting, bloomberg-lattice integration',
        rules: 'SEBI guidelines, Basel III',
        calculators: 'VaR, Sharpe ratio, Monte Carlo sims',
        algorithms: 'Algorithmic trading models, sentiment analysis',
        self_healing: 'Portfolio exposure verification, feed latency correction',
        rollback: 'Pre-trade state restoration, historical data reconciliation'
    },
    {
        name: 'teacher',
        tools: 'Lesson planning, grading system, student records, virtual classroom tools',
        rules: 'CBSE/NCERT curriculum, NEP 2020 guidelines',
        calculators: 'Grade average, attendance tracker',
        algorithms: 'Adaptive learning paths, exam paper generator',
        self_healing: 'Student data sync, curriculum update validation',
        rollback: 'Gradebook versioning, lesson plan history'
    }
    // ... adding more professions dynamically below
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
    'Quantum Computing Researcher', 'IoT Specialist', 'AR/VR Developer', 'Drone Operator', 'Ethical Hacker',
    'Sustainability Consultant', 'Climate Change Analyst', 'Smart City Planner', 'Digital Twin Engineer',
    'Customer Support Specialist', 'Sales Executive', 'Event Manager', 'Travel Agent', 'Logistics Coordinator',
    'Real Estate Analyst', 'Insurance Underwriter', 'Retail Manager', 'Hospitality Manager', 'NGO Worker'
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
