#!/usr/bin/env bash
# Σ SIGMAOS: UNIVERSAL DEPLOYMENT SCRIPT (v1.0 Pro)
# ===============================================
# This bash script automates the SigmaOS environment setup on POSIX systems.
# Usage: chmod +x setup.sh && ./setup.sh

echo "--- 🚀 SIGMAOS: DEPLOYING SOVEREIGN ENVIRONMENT ---"

# 1. Environment Verification
if ! command -v python3 &> /dev/null
then
    echo "[!] ERROR: Python 3 not found. Please install Python."
    exit 1
fi
echo "[+] Found Python: $(python3 --version)"

# 2. Dependency Injection
echo "[*] Zero-Dependency Mode: No 3rd Party Packages Required via Pip."

# 3. Running Sigma Setup Engine
echo "[*] Handing over to Sigma Setup Hub (Hydration Sequence)..."
python3 sigma_setup.py --portable || { echo "[!] Sigma Setup Engine failed. Check local file permissions."; exit 1; }

# 4. Identity Scrub (Sanity Check)
echo "[*] Performing final Forensic Scrub..."
python3 sigma_scrubber.py

# 5. Boot Confirmation
echo "--- [OK] DEPLOYMENT COMPLETE ---"
echo "You can now launch the OS using: ./SigmaOS.sh"
