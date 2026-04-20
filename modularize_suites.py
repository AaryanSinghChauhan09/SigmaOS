import os
import shutil

# Target 33-suite Lattice
TARGET_SUITES = {
    "S01": "S01_Genesis",
    "S02": "S02_ZenithUI",
    "S03": "S03_Orchestrator",
    "S04": "S04_HAL",
    "S05": "S05_Memory",
    "S06": "S06_Storage",
    "S07": "S07_Network",
    "S08": "S08_Security",
    "S09": "S09_Intelligence",
    "S10": "S10_Registry",
    "S11": "S11_Virtualization",
    "S12": "S12_Ecosystem",
    "S13": "S13_Sentience",
    "S14": "S14_Transcendence",
    "S15": "S15_DevNexus",
    "S16": "S16_SoulMolding",
    "S17": "S17_BioNexus",
    "S18": "S18_QuantumLink",
    "S19": "S19_SelfEvolution",
    "S20": "S20_Interconnect",
    "S21": "S21_EternalState",
    "S22": "S22_SimulationNexus",
    "S23": "S23_OmniNexus",
    "S24": "S24_GlobalDebugger",
    "S25": "S25_ZeroKernel",
    "S26": "S26_OmniFabric",
    "S27": "S27_NeuralLink",
    "S28": "S28_OmniBus",
    "S29": "S29_LatticeMerge",
    "S30": "S30_Supremacy",
    "S31": "S31_GlobalGovernance",
    "S32": "S32_UnifiedSovereignty",
    "S33": "S33_TerminalFulfillment"
}

def consolidate_suites(root_path):
    print(f"Consolidating suites in {root_path}...")
    if not os.path.exists(root_path):
        return

    # 1. Identify all folders
    all_folders = [f for f in os.listdir(root_path) if os.path.isdir(os.path.join(root_path, f)) and f.startswith('S')]
    
    # 2. Map and Move
    for folder in all_folders:
        prefix = folder[:3] # e.g. 'S01'
        if prefix in TARGET_SUITES:
            target_name = TARGET_SUITES[prefix]
            if folder != target_name:
                src = os.path.join(root_path, folder)
                dst = os.path.join(root_path, target_name)
                
                print(f"Merging {folder} -> {target_name}")
                if not os.path.exists(dst):
                    os.makedirs(dst)
                
                # Move contents
                for item in os.listdir(src):
                    s = os.path.join(src, item)
                    d = os.path.join(dst, item)
                    if os.path.isdir(s):
                        if os.path.exists(d):
                            # Merge subdirectory
                            for subitem in os.listdir(s):
                                shutil.move(os.path.join(s, subitem), os.path.join(d, subitem))
                            os.rmdir(s)
                        else:
                            shutil.move(s, d)
                    else:
                        shutil.move(s, d)
                
                # Try to remove empty source
                try:
                    os.rmdir(src)
                except OSError:
                    print(f"Warning: Could not remove {src}")

def update_includes():
    print("Updating includes to match new lattice...")
    # Map from old prefix to new full name
    # Actually, we just need to replace "suites/SXX_..." with "suites/TARGET_NAME/..."
    # But since many files might already use the target name, we should be careful.
    
    # Let's build a map of old folder names to new folder names
    # This is tricky because we don't know all old names.
    # But we can look for any "suites/SXX_..." and replace the "SXX_..." part.
    
    for root, dirs, files in os.walk('.'):
        for file in files:
            if file.endswith(('.h', '.c')):
                path = os.path.join(root, file)
                try:
                    with open(path, 'r', encoding='utf-8', errors='replace') as f:
                        content = f.read()
                    
                    new_content = content
                    for prefix, target in TARGET_SUITES.items():
                        # Look for include patterns like "suites/SXX_"
                        # We should replace anything that starts with SXX_ and isn't the target
                        # But simpler: replace all variants of SXX_ with the target
                        # Actually, better to use regex to find "suites/SXX_[^/"]+"
                        import re
                        pattern = r'suites/' + prefix + r'_[^/"]+'
                        new_content = re.sub(pattern, 'suites/' + target, new_content)
                    
                    if new_content != content:
                        print(f"Updated includes in {path}")
                        with open(path, 'w', encoding='utf-8', errors='replace') as f:
                            f.write(new_content)
                except Exception as e:
                    print(f"Error processing {path}: {e}")

if __name__ == "__main__":
    consolidate_suites('kernel/suites')
    consolidate_suites('include/suites')
    update_includes()
    print("Modularization complete.")
