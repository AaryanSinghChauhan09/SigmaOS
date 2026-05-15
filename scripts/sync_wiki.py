import shutil
import os
import subprocess

root_dir = r'c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS'
wiki_dir = r'c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS\wiki_repo'

files_to_sync = [
    ('MANIFEST.md', 'Shard-Manifest.md'),
    ('RELEASES.md', 'Release-Manifest.md'),
    ('CHANGELOG.md', 'Changelog.md'),
    ('IDEAS_BACKLOG.md', 'Advanced-Future-Features.md'),
    ('docs/UNIFIED_TASK_MANIFEST.md', 'Unified-Task-Manifest.md'),
    ('docs/architecture/CORE_TOOLSET.md', 'Core-Toolset-Manifest.md'),
    ('GOVERNANCE.md', 'Governance-Manifesto.md'),
    ('HACKING.md', 'Hacking-Guide.md'),
    ('SUPPORT.md', 'Support-Nexus.md')
]

def run_git(cmd, cwd):
    subprocess.run(cmd, shell=True, cwd=cwd)

print("Syncing files to Wiki...")

for src_rel, dst_name in files_to_sync:
    src_path = os.path.join(root_dir, src_rel)
    dst_path = os.path.join(wiki_dir, dst_name)
    
    if os.path.exists(src_path):
        shutil.copy2(src_path, dst_path)
        print(f"Copied {src_rel} -> {dst_name}")
    else:
        print(f"Skipping {src_rel} (not found)")

print("Committing and pushing Wiki updates...")
run_git("git add .", wiki_dir)
run_git('git commit -m "Wiki v15.0.0: Unified Strategy and Shard Manifest synchronization."', wiki_dir)
run_git("git push origin main", wiki_dir)

print("Wiki synchronization COMPLETE.")
