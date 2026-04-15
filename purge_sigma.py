import os

def purge_sigma():
    target_dirs = ['.']
    for root_dir in target_dirs:
        for root, dirs, files in os.walk(root_dir):
            for file in files:
                if file.endswith(('.h', '.c', '.txt', '.md')):
                    path = os.path.join(root, file)
                    try:
                        with open(path, 'r', encoding='utf-8') as f:
                            content = f.read()
                        if 'Σ' in content:
                            print(f"Purging Σ from {path}")
                            new_content = content.replace('Σ', 'S')
                            with open(path, 'w', encoding='utf-8') as f:
                                f.write(new_content)
                    except Exception as e:
                        pass

if __name__ == "__main__":
    purge_sigma()
