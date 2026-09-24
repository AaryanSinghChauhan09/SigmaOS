#!/bin/bash
set -e

echo "Ensuring Wiki clone is healthy..."
cd /home/aaryansinghchauhan/SigmaOS.wiki
git restore --source=HEAD :/ || true
git pull origin master || true

echo "Copying documentation to Wiki..."
cd /home/aaryansinghchauhan/SigmaOS
# Copy all markdown files except README.md and CONTRIBUTING.md
find . -name "*.md" -not -name "README.md" -not -name "CONTRIBUTING.md" -not -path "*/.git/*" -not -path "*/.agents/*" -not -path "*/target/*" -exec cp {} ../SigmaOS.wiki/ \;

echo "Committing to Wiki..."
cd /home/aaryansinghchauhan/SigmaOS.wiki
git add .
git commit -m "docs: Migrate fully implemented markdown files from main repository" || true
git push origin master || true

echo "Removing markdown files from main repository..."
cd /home/aaryansinghchauhan/SigmaOS
find . -name "*.md" -not -name "README.md" -not -name "CONTRIBUTING.md" -not -path "*/.git/*" -not -path "*/.agents/*" -not -path "*/target/*" -exec git rm -f {} \; || true
git commit -m "docs: Remove fully implemented documentation transferred to Wiki" || true
git push origin main || true

echo "Wiki Migration Complete."
