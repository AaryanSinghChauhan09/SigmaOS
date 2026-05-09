const fs = require('fs');
const path = require('path');

const professions = [
    { 
        name: 'accountant', 
        tools: 'Ledger system, tax compliance modules, balance sheet generator, audit tools',
        rules: 'Ind-AS, GST, Income Tax Act, Companies Act',
        calculators: 'GST, TDS, depreciation, advance tax',
        algorithms: 'Automated ledger balancing, audit trail generation, tax filing optimization'
    },
    { 
        name: 'doctor', 
        tools: 'Patient records, prescription management, lab results integration, telemedicine tools',
        rules: 'Medical Council of India guidelines, Ayushman Bharat protocols',
        calculators: 'Dosage, BMI, medical billing',
        algorithms: 'Patient record indexing, diagnostic prediction, lab result interpretation'
    },
    { 
        name: 'lawyer', 
        tools: 'Case indexing, legal references, compliance document management, secure communication',
        rules: 'IPC, CrPC, Contract Act, Evidence Act',
        calculators: 'Court fee, stamp duty',
        algorithms: 'Case law search, compliance checker, automated legal drafting'
    },
    { 
        name: 'engineer', 
        tools: 'Compilers, CAD tools, simulation engines, project management dashboards',
        rules: 'Bureau of Indian Standards (BIS), IS codes',
        calculators: 'Structural load, electrical circuit, fluid mechanics',
        algorithms: 'CAD simulation, stress analysis, project scheduling'
    },
    { 
        name: 'farmer', 
        tools: 'Crop management, weather forecasting, supply chain integration, IoT sensor sync',
        rules: 'PM-Kisan, crop insurance, subsidy schemes',
        calculators: 'Fertilizer, irrigation water, yield cost',
        algorithms: 'Yield prediction, weather forecasting, supply chain optimization'
    },
    { 
        name: 'teacher', 
        tools: 'Lesson planning, grading system, student records, virtual classroom tools',
        rules: 'CBSE/NCERT curriculum, UGC guidelines',
        calculators: 'Grade average, attendance tracker',
        algorithms: 'Adaptive learning paths, exam paper generator'
    },
    { 
        name: 'banker', 
        tools: 'Loan management, compliance, transaction dashboards',
        rules: 'RBI guidelines, Basel norms',
        calculators: 'EMI, loan interest, NPA provisioning',
        algorithms: 'Risk scoring, fraud detection, portfolio optimization'
    },
    { 
        name: 'civil_servant', 
        tools: 'Policy tools, compliance, citizen records',
        rules: 'Service rules, policy drafting guidelines',
        calculators: 'Budget allocation, pension, pay scale',
        algorithms: 'Policy impact simulation, citizen data indexing'
    },
    { 
        name: 'entrepreneur', 
        tools: 'Business planning, financial dashboards, CRM tools',
        rules: 'Startup India compliance, MCA rules',
        calculators: 'Break-even, ROI, funding valuation',
        algorithms: 'Business plan optimizer, CRM automation'
    },
    { 
        name: 'scientist', 
        tools: 'Data analysis suite, lab notebooks, simulation modules, visualization tools',
        rules: 'Research ethics, lab safety protocols',
        calculators: 'Statistical significance, molarity, physics constants',
        algorithms: 'Simulation engines, data visualization, AI model training'
    },
    // Standard professions
    { name: 'cashier', tools: 'POS interface, barcode scanner integration, receipt printing, inventory sync, daily reconciliation' },
    { name: 'student', tools: 'Study planner, flashcards, assignment tracker, research tools' },
    { name: 'artist', tools: 'Creative suite (drawing, design, video editing), portfolio manager, media export tools' },
    { name: 'journalist', tools: 'Research tools, publishing suite, compliance checks, transcription tools' },
    { name: 'pilot', tools: 'Flight logs, navigation systems, simulation modules, weather sync' },
    { name: 'software_dev', tools: 'IDE, version control (Git), CI/CD pipelines, debugging tools' },
    { name: 'researcher', tools: 'Data mining, citation management, analytics dashboards, visualization' },
    { name: 'retail_manager', tools: 'Inventory management, staff scheduling, sales dashboards, supply chain tools' },
    { name: 'security_officer', tools: 'Surveillance integration, compliance monitoring, audit logs, incident response tools' },
    { name: 'government_official', tools: 'Policy drafting, compliance modules, secure communication, citizen data management' },
    { name: 'finance_trader', tools: 'Market dashboards, risk analysis tools, compliance modules, AI forecasting' },
    { name: 'architect', tools: 'CAD tools, 3D modeling, project management, simulation' },
    { name: 'chef', tools: 'Recipe management, supply chain integration, scheduling, nutritional analysis' },
    { name: 'nurse', tools: 'Patient monitoring, medication schedules, compliance with medical protocols' },
    { name: 'construction_worker', tools: 'Project scheduling, safety compliance, equipment tracking' },
    { name: 'musician', tools: 'Digital audio workstation, sheet music manager, recording tools' },
    { name: 'writer', tools: 'Manuscript editor, publishing tools, citation/reference manager' },
    { name: 'designer', tools: 'Wireframing tools, prototyping, accessibility testing' },
    { name: 'data_analyst', tools: 'Data visualization, statistical analysis, machine learning integration' },
    { name: 'hr_manager', tools: 'Employee records, payroll, compliance, recruitment tools' },
    { name: 'healthcare_admin', tools: 'Hospital management dashboards, compliance, scheduling' },
    { name: 'ai_ml_scientist', tools: 'Model training, dataset management, GPU orchestration' },
    { name: 'photographer', tools: 'Photo editing suite, portfolio manager, export tools' },
    { name: 'videographer', tools: 'Video editing suite, rendering tools, streaming integration' },
    { name: 'athlete_coach', tools: 'Training planner, performance analytics, health monitoring' },
    { name: 'event_planner', tools: 'Scheduling, resource allocation, ticketing, communication tools' },
    { name: 'biotech_scientist', tools: 'Lab management, DNA analysis, compliance' },
    { name: 'physicist', tools: 'Simulation engines, data visualization, research notebooks' },
    { name: 'chemist', tools: 'Molecular modeling, lab notebooks, compliance' },
    { name: 'astronomer', tools: 'Telescope data integration, simulation, visualization' },
    { name: 'geologist', tools: 'Mapping tools, seismic data analysis, visualization' },
    { name: 'environmental_scientist', tools: 'Climate modeling, sustainability dashboards, compliance' },
    { name: 'social_scientist', tools: 'Survey tools, statistical analysis, visualization' },
    { name: 'economist', tools: 'Market modeling, forecasting, compliance' },
    { name: 'psychologist', tools: 'Experiment management, patient records, compliance' },
    { name: 'robotics_engineer', tools: 'Simulation, hardware integration, AI orchestration' },
    { name: 'nanotech_scientist', tools: 'Simulation, visualization, compliance' },
    { name: 'energy_scientist', tools: 'Grid modeling, sustainability dashboards, compliance' },
    { name: 'marine_biologist', tools: 'Ocean data integration, visualization, compliance' },
    { name: 'agricultural_scientist', tools: 'Crop modeling, IoT integration, compliance' },
    { name: 'logistics_manager', tools: 'Fleet tracking, supply chain dashboards, warehouse management' },
    { name: 'hospitality_worker', tools: 'Booking systems, customer service dashboards, scheduling' },
    { name: 'retail_worker', tools: 'POS, stock management, customer service tools' },
    { name: 'cybersecurity_analyst', tools: 'Threat detection, penetration testing, compliance dashboards' },
    { name: 'game_developer', tools: 'Game engines, asset pipelines, GPU orchestration' },
    { name: 'animator', tools: 'Animation suite, rendering tools, storyboard manager' },
    { name: 'marketing_pro', tools: 'Campaign dashboards, analytics, CRM integration' },
    { name: 'sales_exec', tools: 'CRM tools, lead tracking, reporting dashboards' },
    { name: 'neuroscientist', tools: 'Brain imaging integration, simulation, compliance' },
    { name: 'genomics_scientist', tools: 'DNA sequencing tools, visualization, compliance' },
    { name: 'materials_scientist', tools: 'Simulation, modeling, compliance' },
    { name: 'meteorologist', tools: 'Weather modeling, forecasting dashboards' },
    { name: 'oceanographer', tools: 'Ocean data visualization, simulation' },
    { name: 'archaeologist', tools: 'Artifact cataloging, GIS mapping, compliance' },
    { name: 'anthropologist', tools: 'Survey tools, cultural data visualization' },
    { name: 'linguist', tools: 'Corpus analysis, visualization, compliance' },
    { name: 'dentist', tools: 'Dental charting, X-ray viewer, appointment scheduler, medical compliance' },
    { name: 'pharmacist', tools: 'Prescription management, drug interaction checker, inventory control, compliance' },
    { name: 'veterinarian', tools: 'Animal patient records, lab results, scheduling, veterinary standards' },
    { name: 'civil_engineer', tools: 'CAD tools, structural simulation, project management, safety compliance' },
    { name: 'mechanical_engineer', tools: 'Simulation engines, CAD, manufacturing workflow integration' },
    { name: 'electrical_engineer', tools: 'Circuit design tools, simulation, compliance testing' },
    { name: 'data_scientist', tools: 'Jupyter notebooks, ML libraries, visualization dashboards' },
    { name: 'statistician', tools: 'Statistical analysis suite, survey tools, visualization' },
    { name: 'supply_chain_manager', tools: 'Logistics dashboards, fleet tracking, warehouse management' },
    { name: 'hospitality_manager', tools: 'Booking systems, staff scheduling, customer service dashboards' },
    { name: 'ai_robotics_scientist', tools: 'Hardware integration, simulation, AI orchestration' }
];

const profilesDir = path.join(__dirname, 'profiles');

professions.forEach(prof => {
    const profDir = path.join(profilesDir, prof.name);
    if (!fs.existsSync(profDir)) {
        fs.mkdirSync(profDir, { recursive: true });
    }

    // tools.md
    let toolsMd = `# 🛠 ${prof.name.toUpperCase()} Tools\n\nRequired Tools & Functions:\n- ${prof.tools.split(', ').join('\n- ')}`;
    if (prof.rules) toolsMd += `\n\n## 📜 Compliance Rules\n- ${prof.rules.split(', ').join('\n- ')}`;
    if (prof.calculators) toolsMd += `\n\n## 🧮 Professional Calculators\n- ${prof.calculators.split(', ').join('\n- ')}`;
    if (prof.algorithms) toolsMd += `\n\n## 🧬 Core Algorithms\n- ${prof.algorithms.split(', ').join('\n- ')}`;
    
    fs.writeFileSync(path.join(profDir, 'tools.md'), toolsMd);

    // config.json
    const config = {
        profession: prof.name,
        compliance_level: 'sovereign-high',
        resource_priority: 'adaptive',
        modules: prof.tools.split(', ').map(t => t.toLowerCase().replace(/ /g, '_')),
        rules: prof.rules ? prof.rules.split(', ') : [],
        calculators: prof.calculators ? prof.calculators.split(', ') : [],
        algorithms: prof.algorithms ? prof.algorithms.split(', ') : []
    };
    fs.writeFileSync(path.join(profDir, 'config.json'), JSON.stringify(config, null, 4));
});

console.log('Profession profiles updated successfully.');
