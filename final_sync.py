import os
import subprocess

WORKSPACE_DIR = r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS"
WIKI_DIR = os.path.join(WORKSPACE_DIR, "wiki_repo")

def run_git(args, cwd=WORKSPACE_DIR):
    subprocess.run(["git"] + args, cwd=cwd, check=False)

# Sync Wiki
run_git(["add", "."], cwd=WIKI_DIR)
run_git(["commit", "-m", "Enforce strict zero-dependency documentation"], cwd=WIKI_DIR)
run_git(["push", "origin", "main"], cwd=WIKI_DIR)

# Sync Main Repo
run_git(["add", "."])
run_git(["commit", "-m", "Enforce absolute zero-dependency on high-level languages & pre-defined functions"])
run_git(["push", "origin", "--all"])

print("Final Zero-Dependency Synchronization Complete!")
