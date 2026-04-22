const fs = require('fs');
const path = require('path');

const modules = [
    { name: '131_data_warehouse_engine.js', title: 'Data Warehouse Engine', desc: 'Snowflake inspired decoupled compute/storage for tab data.', cli: 'snow-query' },
    { name: '132_unified_analytics_workspace.js', title: 'Unified Analytics Workspace', desc: 'Databricks inspired notebook-based unified analytics.', cli: 'dbx-notebook' },
    { name: '133_distributed_data_processing.js', title: 'Distributed Data Processing', desc: 'Apache Spark inspired RDD processing for huge DOM states.', cli: 'spark-submit' },
    { name: '134_business_intelligence_dashboard.js', title: 'Business Intelligence Dashboard', desc: 'Tableau/Power BI inspired interactive visual analytics.', cli: 'bi-render' },
    { name: '135_data_transformation_pipeline.js', title: 'Data Transformation Pipeline', desc: 'dbt inspired data build tool for transforming OS data.', cli: 'dbt-run' },
    { name: '136_workflow_orchestrator.js', title: 'Workflow Orchestrator', desc: 'Apache Airflow inspired directed acyclic graph task scheduling.', cli: 'airflow-dag' },
    { name: '137_automated_data_sync.js', title: 'Automated Data Sync', desc: 'Fivetran inspired automated ELT pipelines from external APIs.', cli: 'fivetran-sync' },
    { name: '138_log_aggregation_splunk.js', title: 'Log Aggregation & Analysis', desc: 'Splunk inspired searching and monitoring machine-generated data.', cli: 'splunk-search' },
    { name: '139_graph_database_engine.js', title: 'Graph Database Engine', desc: 'Neo4j inspired graph relationships between tabs, tasks, and notes.', cli: 'cypher-query' },
    { name: '140_document_store_engine.js', title: 'Document Store Engine', desc: 'MongoDB inspired flexible JSON-like document storage.', cli: 'mongo-find' },
    { name: '141_time_series_database.js', title: 'Time Series Database', desc: 'InfluxDB inspired high write load metrics tracking.', cli: 'influx-query' },
    { name: '142_open_table_format.js', title: 'Open Table Format', desc: 'Apache Iceberg inspired huge analytic tables management.', cli: 'iceberg-table' },
    { name: '143_data_catalog_governance.js', title: 'Data Catalog & Governance', desc: 'Collibra inspired metadata management and data governance.', cli: 'data-catalog' },
    { name: '144_stream_processing_engine.js', title: 'Stream Processing Engine', desc: 'Apache Flink inspired stateful computations over data streams.', cli: 'flink-stream' },
    { name: '145_interactive_data_science.js', title: 'Interactive Data Science', desc: 'Jupyter/Pandas inspired interactive dataframe manipulation.', cli: 'jupyter-cell' }
];

const dir = 'web_ui/scripts/modules';

modules.forEach(m => {
    const className = m.title.replace(/[^a-zA-Z0-9]/g, '');
    const content = `/**
 * SigmaOS ${m.title} Shard
 * USP/Logic: ${m.desc}
 */

class ${className} {
    constructor() {
        this.shardId = "S" + "${m.name}".split('_')[0] + "_${className}";
        this.active = false;
        
        console.log(\`Σ://INIT> \${this.shardId} Initializing: ${m.title}...\`);
        this.init();
    }

    init() {
        window.addEventListener('sigma.core.boot', () => {
            this.active = true;
            console.log(\`Σ://DATA_OS> \${this.shardId} Online. ${m.desc}\`);
            this.registerCLI();
        });
    }

    registerCLI() {
        if(!window.SigmaCLI) window.SigmaCLI = {};
        window.SigmaCLI['${m.cli}'] = (args) => {
            return \`[${m.title}] Executing \${args.join(' ')}...\`;
        };
    }
}

window.Sigma${className} = new ${className}();
`;
    fs.writeFileSync(path.join(dir, m.name), content);
});

// Update kernel_loader.js
const kernelPath = 'web_ui/scripts/kernel_loader.js';
let kernelContent = fs.readFileSync(kernelPath, 'utf8');

const files = fs.readdirSync(dir).filter(f => f.endsWith('.js'));
const modulePaths = files.map(f => '    "scripts/modules/' + f + '"').join(',\\n');
const replacement = 'const SYSTEM_MODULES = [\\n' + modulePaths + ',\\n    "scripts/audit.js"\\n];';

kernelContent = kernelContent.replace(/const SYSTEM_MODULES = \[[\s\S]*?\];/, replacement.replace(/\\n/g, '\n'));
fs.writeFileSync(kernelPath, kernelContent);

console.log('Created Data Market tools modules (131-145) and updated kernel_loader.js');
