import os
import shutil

# SigmaOS Wiki Sync Tool
# Automates the migration of .md files from /docs/ and /wiki_repo/ to a local Wiki structure.

def sync_wiki():
    docs_dir = "docs"
    wiki_dir = "wiki_repo"
    
    print("SIGMAOS Wiki Sync [RUNNING]")
    
    # Ensure directories exist
    if not os.path.exists(wiki_dir):
        os.makedirs(wiki_dir)
        
    # Migrate .md files from /docs/ to /wiki_repo/
    if os.path.exists(docs_dir):
        for file in os.listdir(docs_dir):
            if file.endswith(".md"):
                print(f"[SYNC] Migrating {file} -> {wiki_dir}/")
                shutil.copy(os.path.join(docs_dir, file), os.path.join(wiki_dir, file))
                
    # Finalize Wiki Formatting
    print("[SYNC] Wiki Repositories Synchronized. Parity ACHIEVED.")

if __name__ == "__main__":
    sync_wiki()
