import os

def find_duplicates(root_dir):
    file_map = {}
    for root, dirs, files in os.walk(root_dir):
        if '.git' in dirs: dirs.remove('.git')
        if 'node_modules' in dirs: dirs.remove('node_modules')
        
        for file in files:
            file_map.setdefault(file, []).append(os.path.join(root, file))
            
    for file, paths in file_map.items():
        if len(paths) > 1:
            print(f"{file}:")
            for p in paths:
                print(f"  {p}")

if __name__ == "__main__":
    find_duplicates(".")
