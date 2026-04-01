#!/bin/bash
# Σ SIGMAOS: SOVEREIGN WIKI SYNC (v160.0)
# Syncs industrial (/WIKI) shards to the GitHub Wiki repository.

WIKI_URL="https://github.com/AaryanSinghChauhan09/SigmaOS.wiki.git"

echo "Σ SIGMAOS: Initiating Wiki Shard Sync..."

# Check if wiki repo exists locally, or clone
if [ ! -d "sigma_wiki_temp" ]; then
    echo "[GIT] Cloning Sovereign Wiki Shard: $WIKI_URL"
    git clone $WIKI_URL sigma_wiki_temp
fi

if [ ! -d "sigma_wiki_temp" ]; then
    echo "ERROR: Wiki repo NOT FOUND. Please initialize the first page on GitHub manually."
    exit 1
fi

# Copy all WIKI content to the wiki repo
echo "[SYNC] Moving /WIKI/ shards to wiki repo..."
cp -rv WIKI/* sigma_wiki_temp/

# Push changes
cd sigma_wiki_temp
git add .
git commit -m "Σ SigmaOS Zenith: Syncing Sovereign Manifesto & Industrial Shards"
git push origin master

echo "Σ SIGMAOS: Wiki Shards SYNCHRONIZED with GitHub Mastery."
