#!/bin/bash
# SigmaOS Universal Launcher (POSIX)
# Detects environment and launches the kernel.

echo "--- SIGMAOS SOVEREIGN LAUNCHER ---"
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$DIR"

# Ensure hydration
if [ ! -f "ecosystem/registry.json" ]; then
    echo "[!] Environment not hydrated. Running Setup..."
    python3 sigma_setup.py
fi

echo "[*] Initializing Kernel..."
python3 boot.py
