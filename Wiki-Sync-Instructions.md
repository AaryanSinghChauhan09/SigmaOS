# GitHub Wiki Sync Instructions

## Issue
The automatic wiki sync has encountered technical issues with git processes on Windows (directory locking, file conflicts). This requires manual intervention to complete.

## Manual Sync Steps

### Option 1: Using GitHub Web Interface
1. Go to https://github.com/AaryanSinghChauhan09/SigmaOS/wiki
2. Click "Add Page" or edit existing pages
3. Copy the content from `wiki_repo/` directory files
4. Paste into the wiki editor
5. Save each page

### Option 2: Using Git Command Line (Manual)
```powershell
# Navigate to wiki repository
cd C:\Users\Aaryan\CascadeProjects\SigmaOS\wiki_repo

# Ensure clean state
git status
git pull origin main

# Add all files
git add .

# Commit changes
git commit -m "Update wiki with comprehensive documentation"

# Push to GitHub
git push origin main
```

### Option 3: Using GitHub CLI
```powershell
# Navigate to wiki repository
cd C:\Users\Aaryan\CascadeProjects\SigmaOS\wiki_repo

# Stage and commit
gh repo sync --source . --destination git@github.com:AaryanSinghChauhan09/SigmaOS.wiki.git
```

## Wiki Files to Sync
The following files in `wiki_repo/` need to be synchronized:

1. **Comprehensive-Gap-Analysis.md** - 7-category gap analysis vs Linux distros
2. **Comprehensive-Future-Development-Roadmap.md** - Detailed development roadmap
3. **Dependency-Reduction-Roadmap.md** - 4-phase dependency elimination plan
4. **Community-Governance-Model.md** - Governance structure and processes
5. **Bootloader.md** - Bootloader documentation
6. **Security-Privacy-Roadmap.md** - Security and privacy roadmap

## Troubleshooting
If you encounter git locking issues:
1. Close any IDE or editor that might have files open
2. Run: `git clean -fd` to remove untracked files
3. Run: `git reset --hard HEAD` to reset to last commit
4. Try the sync again

## Verification
After syncing, verify at: https://github.com/AaryanSinghChauhan09/SigmaOS/wiki
