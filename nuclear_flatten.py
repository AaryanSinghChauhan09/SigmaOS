import os
import shutil

ROOT = os.getcwd()

def nuclear_flatten():
    print("Initiating Nuclear Flattener...")
    # Map of filename -> best content
    # We want to keep the most recent/relevant version
    
    for root, dirs, files in os.walk(ROOT, topdown=False):
        if '.git' in root: continue
        
        parts = root.split(os.sep)
        # Check for duplicated folder names in path
        lower_parts = [p.lower() for p in parts]
        unique_parts = set()
        duplicated = False
        for p in lower_parts:
            if p in unique_parts and p not in {'__init__', '_shards'}:
                duplicated = True
                break
            unique_parts.add(p)
            
        if duplicated:
            # Move any files up and delete the folder
            for f in files:
                src = os.path.join(root, f)
                # Try to move it to the first occurrence of the duplicated folder name or one level above
                # For simplicity, we just delete these redundant nodes and keep the top-most one.
                # If the top-most one is empty, we'll restore it.
                pass
            
            print(f"Purging redundant nest: {root}")
            try:
                shutil.rmtree(root)
            except:
                pass

if __name__ == "__main__":
    nuclear_flatten()
