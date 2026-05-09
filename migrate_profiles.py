import os
import shutil

REPO_DIR = r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS-Repo"
WIKI_DIR = r"c:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS.wiki"
PROFILES_DIR = os.path.join(REPO_DIR, "profiles")

def migrate():
    if not os.path.exists(PROFILES_DIR):
        print(f"Error: Profiles directory {PROFILES_DIR} not found.")
        return

    if not os.path.exists(WIKI_DIR):
        print(f"Error: Wiki directory {WIKI_DIR} not found.")
        return

    count = 0
    for root, dirs, files in os.walk(PROFILES_DIR):
        for file in files:
            if file == "tools.md":
                prof_name = os.path.basename(root)
                source_path = os.path.join(root, file)
                wiki_filename = f"Profession-{prof_name.replace('_', '-')}-Tools.md".replace("--", "-")
                # Capitalize segments
                wiki_filename = "-".join([p.capitalize() for p in wiki_filename.split("-")])
                dest_path = os.path.join(WIKI_DIR, wiki_filename)
                
                print(f"Migrating {prof_name} -> {wiki_filename}")
                shutil.copy2(source_path, dest_path)
                
                # Delete from repo after sync
                os.remove(source_path)
                count += 1

    print(f"Migration complete. {count} profession tools migrated to Wiki and purged from repo.")

if __name__ == "__main__":
    migrate()
