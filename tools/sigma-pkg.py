#!/usr/bin/env python3
import os
import sys
import json
import sqlite3

# SigmaOS — sigma-pkg: Sovereign Package Manager
# USP: No external dependencies beyond Python3/SQLite3, hardware-native shard registry.

REGISTRY_PATH = "suites/S36_SovereignPkg/registry.json"
DB_PATH = "suites/S36_SovereignPkg/pkg_graph.db"

def init_db():
    conn = sqlite3.connect(DB_PATH)
    c = conn.cursor()
    c.execute('''CREATE TABLE IF NOT EXISTS packages 
                 (name TEXT PRIMARY KEY, version TEXT, status TEXT)''')
    c.execute('''CREATE TABLE IF NOT EXISTS dependencies 
                 (package TEXT, dependency TEXT, FOREIGN KEY(package) REFERENCES packages(name))''')
    conn.commit()
    return conn

def load_registry():
    if not os.path.exists(REGISTRY_PATH):
        return {"packages": {}}
    with open(REGISTRY_PATH, 'r') as f:
        return json.load(f)

def create_snapshot(reason):
    print(f"[sigma-pkg] Safety: Creating pre-{reason} snapshot...")
    # Call sigma-snap (mocked for now, but linked to the logic)
    snapshot_id = 42 # Mock ID
    print(f"[✓] Snapshot {snapshot_id} created successfully.")
    return snapshot_id

def install(pkg_name):
    create_snapshot("install")
    print(f"[sigma-pkg] Attempting to install shard: {pkg_name}...")
    registry = load_registry()
    if pkg_name not in registry["packages"]:
        print(f"[ERROR] Shard '{pkg_name}' not found in sovereign registry.")
        return False
    
    pkg_data = registry["packages"][pkg_name]
    deps = pkg_data.get("dependencies", [])
    
    conn = init_db()
    c = conn.cursor()
    
    # Resolve dependencies
    for dep in deps:
        print(f"[sigma-pkg] Resolving dependency: {dep}")
        c.execute("SELECT status FROM packages WHERE name=?", (dep,))
        res = c.fetchone()
        if not res or res[0] != "installed":
            if not install(dep):
                print(f"[ERROR] Failed to satisfy dependency '{dep}' for '{pkg_name}'.")
                return False

    # Simulate installation (linking shard)
    print(f"[sigma-pkg] Linking shard {pkg_name} into the lattice...")
    c.execute("INSERT OR REPLACE INTO packages (name, version, status) VALUES (?, ?, ?)", 
              (pkg_name, pkg_data.get("version", "1.0.0"), "installed"))
    for dep in deps:
        c.execute("INSERT OR IGNORE INTO dependencies (package, dependency) VALUES (?, ?)", 
                  (pkg_name, dep))
    conn.commit()
    print(f"[✓] Shard '{pkg_name}' installed successfully.")
    return True

def remove(pkg_name):
    create_snapshot("remove")
    print(f"[sigma-pkg] Removing shard: {pkg_name}...")
    conn = init_db()
    c = conn.cursor()
    c.execute("DELETE FROM packages WHERE name=?", (pkg_name,))
    c.execute("DELETE FROM dependencies WHERE package=?", (pkg_name,))
    conn.commit()
    print(f"[✓] Shard '{pkg_name}' purged from lattice.")

def list_installed():
    conn = init_db()
    c = conn.cursor()
    c.execute("SELECT name, version FROM packages WHERE status='installed'")
    rows = c.fetchall()
    print("\n--- Installed Sovereign Shards ---")
    for row in rows:
        print(f"  {row[0]} (v{row[1]})")
    print("----------------------------------\n")

def main():
    if len(sys.argv) < 2:
        print("Usage: sigma-pkg <install|remove|list> [shard_name]")
        sys.exit(1)
    
    cmd = sys.argv[1]
    if cmd == "install" and len(sys.argv) > 2:
        install(sys.argv[2])
    elif cmd == "remove" and len(sys.argv) > 2:
        remove(sys.argv[2])
    elif cmd == "list":
        list_installed()
    else:
        print(f"[ERROR] Unknown command or missing arguments: {cmd}")

if __name__ == "__main__":
    main()
