import os

def test_sovereign_suites():
    print("[SIGMA-TEST]: Initializing Sovereign Suites Integrity Audit...")
    
    # 1. Check for 10 Master Suites
    suites = [
        "SovereignMemorySuite.c", "SovereignSecuritySuite.c", "SovereignAppManagement.c",
        "SovereignServiceControl.c", "SovereignIntelligenceSuite.c", "SovereignFrontendSuite.c",
        "SovereignEcosystemSuite.c", "SovereignBackendSuite.c", "SovereignConfigIdentitySuite.c",
        "SovereignPrincipleSuite.c"
    ]
    
    missing = []
    for s in suites:
        if not os.path.exists(f"kernel/modules/core/{s}") and not os.path.exists(f"kernel/modules/security/{s}"):
            missing.append(s)
            
    if missing:
        print(f"[FAIL]: Missing Suites: {missing}")
    else:
        print("[PASS]: All 10 Master Suites are seated.")

    # 2. Check for Master Aggregator Init
    with open("include/sigma_kernel.h", "r") as f:
        content = f.read()
        if "SovereignMaster_InitAll" in content and "SovereignPrinciple_Register" in content:
            print("[PASS]: Master Aggregator is correctly unified.")
        else:
            print("[FAIL]: Master Aggregator is missing principles or init sequence.")

    # 3. Check for Linux-Grade CLI
    if os.path.exists("kernel/modules/core/cli/SovereignCLI_DistroSuite.c"):
        print("[PASS]: Linux-Grade Distro tools detected.")
    else:
        print("[FAIL]: Distro suite missing.")

    print("[RESULT]: Sovereign Global Audit COMPLETE.")

if __name__ == "__main__":
    test_sovereign_suites()
