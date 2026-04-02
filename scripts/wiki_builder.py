import os
import glob
from pathlib import Path

def main():
    repo_dir = r"C:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS"
    wiki_dir = os.path.join(repo_dir, "WIKI")
    os.makedirs(wiki_dir, exist_ok=True)
    out_file = os.path.join(wiki_dir, "SigmaOS_Comprehensive_Wiki.md")
    
    merged_content = "# SigmaOS: Comprehensive Wiki Overview\n\n"
    merged_content += "This document contains an aggregated overview of all markdown documentation across the repository, serving as the ultimate reference for the OS architecture, roadmaps, and components.\n\n"
    
    md_files = []
    for root, dirs, files in os.walk(repo_dir):
        # Avoid double-importing anything already currently in WIKI
        if 'WIKI' in root or '.git' in root or '.github' in root:
            continue
        for file in files:
            if file.endswith('.md'):
                md_files.append(os.path.join(root, file))
    
    for fpath in md_files:
        rel_path = os.path.relpath(fpath, repo_dir)
        merged_content += f"\n\n## --- Documentation: {rel_path} ---\n\n"
        with open(fpath, 'r', encoding='utf-8', errors='ignore') as file:
            content = file.read()
            merged_content += content
    
    with open(out_file, 'w', encoding='utf-8') as f:
        f.write(merged_content)
        
    print(f"Successfully generated {out_file} with {len(md_files)} files.")

if __name__ == "__main__":
    main()
