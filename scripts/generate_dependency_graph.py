import os
import re

def generate_mermaid():
    print("[GRAPH]: Analyzing Sovereign Shard Dependencies...")
    
    # Root paths
    suites_dir = "kernel/suites"
    
    # Map of Suite -> Shards
    graph = {}
    
    suites = sorted([d for d in os.listdir(suites_dir) if os.path.isdir(os.path.join(suites_dir, d))])
    
    for suite in suites:
        graph[suite] = []
        suite_path = os.path.join(suites_dir, suite)
        for root, dirs, files in os.walk(suite_path):
            for file in files:
                if file.endswith(".c"):
                    graph[suite].append(file)

    # Output Mermaid
    mmd = ["graph TD"]
    
    # Core Genesis
    mmd.append("    S01_Genesis --> S02_ZenithUI")
    mmd.append("    S01_Genesis --> S04_HAL")
    mmd.append("    S01_Genesis --> S05_Memory")
    mmd.append("    S05_Memory --> S10_Orchestration")
    mmd.append("    S06_Storage --> S10_Orchestration")
    mmd.append("    S04_HAL --> S10_Orchestration")
    mmd.append("    S08_Security --> S10_Orchestration")
    mmd.append("    S10_Orchestration --> S09_Tooling")
    mmd.append("    S07_Network --> S03_Distros")
    
    # Build subdivisions
    for suite in suites:
        mmd.append(f"    subgraph {suite}")
        for file in graph[suite][:3]: # Limit to first 3 for readability
            mmd.append(f"        {file.replace('.c', '')}")
        mmd.append("    end")
    
    with open("SHARD_GRAPH.md", "w", encoding="utf-8") as f:
        f.write("# 📊 SigmaOS Shard Dependency Graph\n\n")
        f.write("```mermaid\n")
        f.write("\n".join(mmd))
        f.write("\n```\n")
    
    print("Σ [OK]: SHARD_GRAPH.md generated successfully.")

if __name__ == "__main__":
    try:
        generate_mermaid()
    except UnicodeEncodeError:
        # Fallback for terminals that don't support Sigma
        os.environ["PYTHONIOENCODING"] = "utf-8"
        generate_mermaid()
