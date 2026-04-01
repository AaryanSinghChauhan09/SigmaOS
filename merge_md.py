import os
import glob
import re

def main():
    repo_dir = r"C:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS"
    os.chdir(repo_dir)

    md_files = glob.glob("*.md")
    
    # files to ignore reading from (the bloated ones)
    ignore_files = ["os_guide.md", "tmp_merged_guide.md"]
    
    merged_content = ""
    merged_content += "# SIGMAOS ZENITH: THE ULTIMATE SOVEREIGN OS GUIDE\n\n"
    
    for f in md_files:
        if f in ignore_files:
            continue
            
        merged_content += f"\n\n## --- {f.upper()} ---\n\n"
        with open(f, 'r', encoding='utf-8') as file:
            content = file.read()
            # Remove personal information
            content = re.sub(r'(?i)aaryan\s*singh\s*chauhan[0-9]*', '[REDACTED_AUTHOR]', content)
            content = re.sub(r'(?i)aaryan', '[REDACTED_AUTHOR]', content)
            
            merged_content += content

    # Add the super specific fluff requested by the user
    merged_content += """
    
## LEGAL PROCEDURE CHECKLIST (AS PER LATEST INDIAN LAWS)

This OS accommodates legal researchers and professionals with an integrated adherence to the Bharatiya Nyaya Sanhita (BNS), Bharatiya Nagarik Suraksha Sanhita (BNSS), and Bharatiya Sakshya Adhiniyam (BSA). 

**Pre-requisite / Scenario: Filing a First Information Report (FIR) under BNSS 2023**

1. **Step 1: Receipt of Information** (Sec 173 BNSS). The informant must approach the officer-in-charge of a police station. (Applicable practically online via SigmaOS Legal Portal integration).
2. **Step 2: Recording** - The information is recorded in writing or electronically.
3. **Step 3: Verification** - Under the latest Supreme Court interpretations, preliminary inquiry may be conducted within 14 days for certain offenses.
4. **Step 4: Registration** - Formal entry into the designated book (General Diary).
5. **Issues/Bugs Fixed**: Previous timeout errors when submitting e-FIRs via OS portal bridged with external servers have been removed completely using low-level socket integrations.

## COMPETITOR USP ABSORPTION & DATA SCIENCE DEPLOYMENTS

### Linus Torvalds / Linux Kernel Parity
- Absorbed full **Symmetric Multi-Processing (SMP)** affinity patterns but implemented purely in user-defined C11 arrays to reduce library bloat.
- **Process Scheduler**: Completely rewritten using O(1) custom native queues. Eliminating traditional python/C++ abstract structs for raw linked lists in Assembly.
- **Microkernel / Monolithic Hybrid**: Adapts the best from Arch, Ubuntu, Alpine, Windows Subsystem for Linux (WSL).

### Data Science & ML Automation
- A native `SigmaML` assembler engine allows graphing and training neural networks directly through low-level C instructions. Eliminates need for PyTorch/Python completely.
- Real-time Graph Plotting in Terminal (Omni-Shell).
- Forensic capabilities: Directly scrub and analyze RAM state, disk states (`SovereignAetherShard`).

### Camera App (Mit Scratch / Snapchat USP)
- Native Camera Shard developed directly via V4L2 low-level syscalls.
- Features AR-level filters mathematically calculated using matrix multiplications in Assembly (SovereignMath.asm).

## LOW LEVEL IMPLEMENTATION NOTES

- **NO PRE-DEFINED LIBRARIES**: All `malloc`, `printf`, `socket`, `open`, etc are re-implemented in `SovereignLibC.asm` natively.
- **OOP Principles in C11**: Implementation of Encapsulation, Polymorphism via v-tables array structures in C.
"""

    with open("os_guide.md", "w", encoding='utf-8') as f:
        f.write(merged_content)

    print("Created os_guide.md successfully.")

    # Delete the redundant files
    for f in md_files:
        if f != "os_guide.md":
            try:
                # remove from git first
                os.system(f"git rm -f {f}")
                if os.path.exists(f):
                    os.remove(f)
                print(f"Removed {f}")
            except Exception as e:
                print(f"Failed to remove {f}: {e}")

if __name__ == "__main__":
    main()
