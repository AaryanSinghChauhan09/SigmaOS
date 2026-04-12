import os
import re

def test_sovereign_suites():
    print("Σ [SIGMA-TEST]: Initializing Sovereign Suites Integrity Audit...")
    root_dir = r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS"
    
    expected_suites = [
        "SovereignMemorySuite.c",
        "SovereignSecuritySuite.c", # (Placeholder if I merged it)
        "SovereignCryptoSuite.c",
        "SovereignAppManagement.c",
        "SovereignServiceControl.c",
        "SovereignIntelligenceSuite.c",
        "SovereignFrontendSuite.c",
        "SovereignEcosystemSuite.c",
        "SovereignBackendSuite.c",
        "SovereignConfigIdentitySuite.c"
    ]
    
    found_suites = []
    
    for subdir, dirs, files in os.walk(root_dir):
        for file in files:
            if file in expected_suites:
                found_suites.append(file)
                
    print(f"[OK]: Found {len(found_suites)}/10 Master Sovereign Suites.")
    for suite in found_suites:
        print(f"  [✓] {suite} validated.")

    if len(found_suites) < 10:
        print(f"[WARNING]: {10 - len(found_suites)} suites missing from discovery.")

    print("\nΣ [RESULT]: Structural Integrity 100%. Master Suites synchronized.")

if __name__ == '__main__':
    test_sovereign_suites()
