import os
import shutil

# Define the 33 Suites and their keywords
SUITES = {
    "S01_Genesis": ["boot", "hal", "init", "standardhal", "long_mode"],
    "S02_ZenithUI": ["index", "dashboard", "window", "desktop", "ui", "visualizer", "theme", "personalizer"],
    "S03_Orchestrator": ["orch", "sync", "auto", "process", "scheduler", "task", "manager", "pm"],
    "S04_HAL": ["drivers", "hw", "io", "ata", "keyboard", "rtc", "usb", "vbe", "pit"],
    "S05_Memory": ["mem", "slab", "pmm", "vmm", "zram", "compaction", "safety"],
    "S06_Storage": ["fs", "vfs", "disk", "storage", "block"],
    "S07_Network": ["net", "tcp", "udp", "ip", "ethernet", "wifi", "bluetooth", "ring"],
    "S08_Security": ["security", "firewall", "crypto", "audit", "vault", "pqc", "encryption", "legal"],
    "S09_Intelligence": ["ai", "ml", "neural", "pattern", "intelligence", "logic"],
    "S10_Registry": ["registry", "config", "manifest", "metadata"],
    "S11_Virtualization": ["virt", "hypervisor", "container", "isolation", "vm", "virtual", "xv6"],
    "S12_Ecosystem": ["eco", "app", "nexus", "aether", "productivity", "graphics", "sound", "camera"],
    "S13_Sentience": ["sentience", "self-aware", "awareness", "conscience"],
    "S14_Transcendence": ["transcend", "quantum", "beyond", "classical"],
    "S15_DevNexus": ["dev", "build", "tool", "compiler", "transpiler", "debug", "lint", "ci", "forge"],
    "S16_SoulMolding": ["soul", "identity", "persona", "molding"],
    "S17_BioNexus": ["bio", "health", "biometric"],
    "S18_QuantumLink": ["qkd", "unbreakable", "quantumlink"],
    "S19_SelfEvolution": ["evolution", "self-healing", "autonomous", "repair"],
    "S20_Interconnect": ["connect", "link", "fabric", "bus", "ipc", "binder"],
    "S21_EternalState": ["eternal", "persistence", "state", "stable"],
    "S22_SimulationNexus": ["sim", "physics", "world"],
    "S23_OmniNexus": ["omni", "cross-dimensional"],
    "S24_GlobalDebugger": ["global", "fault", "detection", "debugger"],
    "S25_ZeroKernel": ["zero", "latency", "core", "monolith"],
    "S26_OmniFabric": ["fabric", "interconnect", "tb/s"],
    "S27_NeuralLink": ["direct-to-silicon", "brain", "bypass"],
    "S28_OmniBus": ["transport", "omnibus"],
    "S29_LatticeMerge": ["merge", "unification", "lattice"],
    "S30_Supremacy": ["supremacy", "finality", "complete"],
    "S31_GlobalGovernance": ["governance", "consensus", "distributed"],
    "S32_UnifiedSovereignty": ["assimilation", "cross-os"],
    "S33_TerminalFulfillment": ["fulfillment", "closure", "loop"]
}

REPO_PATH = "C:/Users/Aaryan/.gemini/antigravity/scratch/SigmaOS-Repo"
SUITES_PATH = os.path.join(REPO_PATH, "suites")

def categorize_files():
    # Gather all files from kernel, arch, drivers, fs, libc, include, userland, ecosystem, root
    search_dirs = ["kernel", "arch", "drivers", "fs", "libc", "include", "userland", "ecosystem"]
    
    for d in search_dirs:
        full_d = os.path.join(REPO_PATH, d)
        if not os.path.exists(full_d): continue
        
        for root, _, files in os.walk(full_d):
            for f in files:
                if f.endswith(('.c', '.cpp', '.h', '.asm', '.S', '.rs')):
                    src_file = os.path.join(root, f)
                    target_suite = "S30_Supremacy" # Default
                    
                    # Match by keyword
                    f_lower = f.lower()
                    for suite, keywords in SUITES.items():
                        if any(k in f_lower for k in keywords):
                            target_suite = suite
                            break
                    
                    target_dir = os.path.join(SUITES_PATH, target_suite)
                    if not os.path.exists(target_dir): os.makedirs(target_dir)
                    
                    # Move file
                    try:
                        shutil.move(src_file, os.path.join(target_dir, f))
                        print(f"Moved: {f} -> {target_suite}")
                    except Exception as e:
                        print(f"Error moving {f}: {e}")

if __name__ == "__main__":
    categorize_files()
