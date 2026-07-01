import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Paths
const REGISTRY_PATH = path.join(__dirname, '..', 'suites', 'S36_SovereignPkg', 'registry.json');
const DB_PATH = path.join(__dirname, '..', 'suites', 'S36_SovereignPkg', 'pkg_graph.json');

// Helper to ensure target directories exist
function ensureDirs() {
    const registryDir = path.dirname(REGISTRY_PATH);
    if (!fs.existsSync(registryDir)) {
        fs.mkdirSync(registryDir, { recursive: true });
    }
}

// Load Registry
function loadRegistry() {
    ensureDirs();
    if (!fs.existsSync(REGISTRY_PATH)) {
        // Seed default registry if not present
        const defaultReg = {
            packages: {
                "sentinel": { version: "1.0.0", dependencies: ["sovereign-net"] },
                "sovereign-net": { version: "1.2.0", dependencies: [] },
                "zenith-ui": { version: "2.1.0", dependencies: ["sentinel"] }
            }
        };
        fs.writeFileSync(REGISTRY_PATH, JSON.stringify(defaultReg, null, 2));
        return defaultReg;
    }
    return JSON.parse(fs.readFileSync(REGISTRY_PATH, 'utf8'));
}

// Load JSON Database
function loadDB() {
    ensureDirs();
    if (!fs.existsSync(DB_PATH)) {
        const defaultDB = { packages: {}, dependencies: {} };
        fs.writeFileSync(DB_PATH, JSON.stringify(defaultDB, null, 2));
        return defaultDB;
    }
    return JSON.parse(fs.readFileSync(DB_PATH, 'utf8'));
}

// Save JSON Database
function saveDB(db) {
    fs.writeFileSync(DB_PATH, JSON.stringify(db, null, 2));
}

// Create snapshot mock
function createSnapshot(reason) {
    console.log(`[sigma-pkg] Safety: Creating pre-${reason} snapshot...`);
    const snapshotId = Math.floor(Math.random() * 1000) + 1;
    console.log(`[✓] Snapshot ${snapshotId} created successfully.`);
    return snapshotId;
}

// Install package
function install(pkgName) {
    createSnapshot("install");
    console.log(`[sigma-pkg] Attempting to install shard: ${pkgName}...`);
    
    const registry = loadRegistry();
    if (!registry.packages[pkgName]) {
        console.error(`[ERROR] Shard '${pkgName}' not found in sovereign registry.`);
        return false;
    }

    const pkgData = registry.packages[pkgName];
    const deps = pkgData.dependencies || [];

    const db = loadDB();

    // Resolve dependencies
    for (const dep of deps) {
        console.log(`[sigma-pkg] Resolving dependency: ${dep}`);
        if (!db.packages[dep] || db.packages[dep].status !== "installed") {
            if (!install(dep)) {
                console.error(`[ERROR] Failed to satisfy dependency '${dep}' for '${pkgName}'.`);
                return false;
            }
        }
    }

    // Update database (reload since recursive calls might have updated it)
    const activeDB = loadDB();
    activeDB.packages[pkgName] = {
        version: pkgData.version || "1.0.0",
        status: "installed"
    };
    activeDB.dependencies[pkgName] = deps;
    saveDB(activeDB);

    console.log(`[sigma-pkg] Linking shard ${pkgName} into the lattice...`);
    console.log(`[✓] Shard '${pkgName}' installed successfully.`);
    return true;
}

// Remove package
function remove(pkgName) {
    createSnapshot("remove");
    console.log(`[sigma-pkg] Removing shard: ${pkgName}...`);
    
    const db = loadDB();
    if (db.packages[pkgName]) {
        delete db.packages[pkgName];
    }
    if (db.dependencies[pkgName]) {
        delete db.dependencies[pkgName];
    }
    saveDB(db);
    
    console.log(`[✓] Shard '${pkgName}' purged from lattice.`);
}

// List installed packages
function listInstalled() {
    const db = loadDB();
    console.log("\n--- Installed Sovereign Shards ---");
    const pkgs = Object.keys(db.packages);
    if (pkgs.length === 0) {
        console.log("  No shards installed.");
    } else {
        for (const pkg of pkgs) {
            if (db.packages[pkg].status === "installed") {
                console.log(`  ${pkg} (v${db.packages[pkg].version})`);
            }
        }
    }
    console.log("----------------------------------\n");
}

function main() {
    const args = process.argv.slice(2);
    if (args.length < 1) {
        console.log("Usage: node tools/sigma-pkg.js <install|remove|list> [shard_name]");
        process.exit(1);
    }

    const cmd = args[0];
    if (cmd === "install" && args.length > 1) {
        install(args[1]);
    } else if (cmd === "remove" && args.length > 1) {
        remove(args[1]);
    } else if (cmd === "list") {
        listInstalled();
    } else {
        console.error(`[ERROR] Unknown command or missing arguments: ${cmd}`);
    }
}

main();
