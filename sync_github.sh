#!/bin/bash
# =========================================================================
# SigmaOS GitHub Sync Script
# =========================================================================
# This script commits all recent changes to the main repository and 
# pushes the docs/wiki folder to the GitHub Wiki repository.
#
# Prerequisite: You must be authenticated to GitHub via SSH or HTTPS.
# =========================================================================

set -e

MAIN_REPO_URL="https://github.com/AaryanSinghChauhan09/SigmaOS.git"
WIKI_REPO_URL="https://github.com/AaryanSinghChauhan09/SigmaOS.wiki.git"
COMMIT_MSG="SigmaOS: Locked down Orchestrator (Phase 3) and initiated Zenith Desktop (Phase 4)"

echo "🚀 Starting GitHub Sync for SigmaOS..."

# 1. Sync the Main Repository
echo "[1/3] Syncing Main Repository..."
git add .
# Prevent failing if there are no new changes
git commit -m "$COMMIT_MSG" || echo "No changes to commit in main repo."
git push origin main || echo "Failed to push main repo. Are you authenticated?"

# 2. Sync the Wiki Repository
echo "[2/3] Syncing Wiki Repository..."
# Check if the wiki repo is cloned in a temporary directory
if [ ! -d "/tmp/SigmaOS.wiki" ]; then
    echo "Cloning Wiki repository..."
    git clone $WIKI_REPO_URL /tmp/SigmaOS.wiki
fi

echo "Copying wiki files..."
# Copy the markdown files from our docs/wiki to the actual wiki repo
cp -r docs/wiki/* /tmp/SigmaOS.wiki/

cd /tmp/SigmaOS.wiki
git add .
git commit -m "Docs: Update Wiki for Phase 3 and Orchestrator" || echo "No wiki changes."
git push origin master || echo "Failed to push wiki. (Note: Wikis usually use 'master' branch)"

echo "[3/3] ✅ GitHub Sync Complete!"
echo "Main Repo: $MAIN_REPO_URL"
echo "Wiki Repo: $WIKI_REPO_URL"
