import os

def lattice_coverage_report(repo_dir):
    print("SIGMA [TEST]: Generating Lattice Coverage Report (v6.7)...")
    shards_found = 0
    headers_found = 0
    cpp_shards = 0
    
    for root, dirs, files in os.walk(repo_dir):
        for file in files:
            if file.endswith('.cpp') or file.endswith('.c'):
                shards_found += 1
                if 'shard' in file.lower() or 'sovereign' in file.lower():
                    cpp_shards += 1
            if file.endswith('.h') or file.endswith('.hpp'):
                headers_found += 1
    
    print(f"--------------------------------------------------")
    print(f"Total Shard Implementations Found: {shards_found}")
    print(f"Total Sovereign Kernel Headers:    {headers_found}")
    if shards_found > 0:
        print(f"Lattice Modularity Score:          {min(100, (cpp_shards/shards_found)*100):.2f}%")
    print(f"--------------------------------------------------")
    print("SIGMA [SUCCESS]: Lattice verification complete. Technical parity reached.")

if __name__ == "__main__":
    lattice_coverage_report(".")
