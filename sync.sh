#!/bin/bash
# SigmaOS Workspace Sync (Linux/macOS Protocol)
# Automatically commits and force-pushes changes to the master branch.

echo "--- SIGMAOS MESH SYNC INITIATED (POSIX) ---"

# Step 1: Check for git
if ! command -v git &> /dev/null
then
    echo "[ERROR] Git not found. Please install git for your distribution."
    exit 1
fi

# Step 2: Identity (Local override for consistency)
git config user.email "sovereign@users.noreply.github.com"
git config user.name "Sovereign-User"

# Step 3: Atomic Commit
TIMESTAMP=$(date +"%Y-%m-%d %H:%M:%S")
MSG="Apex Sync (Linux): $TIMESTAMP [Ledger Signed]"

git add .
git commit -m "$MSG" -a --allow-empty

# Step 4: Force Push
echo "[*] Pushing to master branch..."
git push origin master --force

if [ $? -eq 0 ]; then
    echo "[OK] Workspace Synced via Linux/macOS Shell."
else
    echo "[!] Push failed. Verify SSH/PAT credentials."
fi
