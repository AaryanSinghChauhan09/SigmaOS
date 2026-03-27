#!/bin/bash
# Σ SIGMA OS: SOVEREIGN DEPLOYMENT & INSTALLATION ENGINE (v5.0)
# ==========================================================
# Mission: Convert the SigmaOS development state into a production-ready
#         Live-Boot ISO, Docker Container, and Portable Application.
# ==========================================================

set -e

echo "========================================================="
echo "   SIGMA OS SOVEREIGN DEPLOYMENT ENGINE (v5.0)           "
echo "========================================================="

# 1. Independent Installation (Bare-Metal)
echo "[1/4] Preparing Bare-Metal Installation Blueprint..."
# We use the existing UEFI bootloader and kernel binaries.
make all
mv build/sigma_kernel.exe ./sigma_kernel_production

# 2. Virtualization & Containerization
echo "[2/4] Building Docker Sovereign Shard..."
docker build -t sigmaos:zenith .

# 3. Live Boot (ISO Generation)
echo "[3/4] Generating Bootable SigmaOS.iso..."
gcc sigma_iso_builder.c -o sigma_iso_builder
./sigma_iso_builder

# 4. Cloud Hosting & Portability
echo "[4/4] Preparing Sigma Cloud Registry (GitHub Push)..."
git add .
git commit -m "SigmaOS v5.0 Production Ready - Sovereign Sovereignty Achieved"
git push

echo "========================================================="
echo "   DEPLOIMENT COMPLETE: SigmaOS ISO & Container Ready    "
echo "========================================================="
