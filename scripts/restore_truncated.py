#!/usr/bin/env python3
"""
Batch restore SigmaOS source files from git history where conflict resolution
resulted in near-empty files (< 20 lines). 
For each such file, finds the most recent commit where the file was not empty
and restores from there.
"""
import subprocess
import os
import sys

def get_file_line_count(filepath):
    try:
        with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
            return len(f.readlines())
    except OSError:
        return 0

def get_git_log_for_file(filepath, repo_root):
    """Get git log for a specific file."""
    rel_path = os.path.relpath(filepath, repo_root)
    result = subprocess.run(
        ['git', 'log', '--oneline', '--follow', '-20', '--', rel_path],
        cwd=repo_root, capture_output=True, text=True
    )
    commits = []
    for line in result.stdout.strip().split('\n'):
        if line.strip():
            parts = line.split(' ', 1)
            if parts:
                commits.append(parts[0])
    return commits

def get_file_from_commit(commit, filepath, repo_root):
    """Get file content from a specific commit."""
    rel_path = os.path.relpath(filepath, repo_root)
    result = subprocess.run(
        ['git', 'show', f'{commit}:{rel_path}'],
        cwd=repo_root, capture_output=True, text=True
    )
    if result.returncode == 0:
        return result.stdout
    return None

def resolve_conflicts_from_content(content):
    """Resolve conflict markers by merging current + incoming."""
    lines = content.split('\n')
    result = []
    i = 0
    while i < len(lines):
        line = lines[i]
        if line.startswith('<<<<<<<'):
            i += 1
            while i < len(lines) and not lines[i].startswith('|||||||') and not lines[i].startswith('=======') and not lines[i].startswith('>>>>>>>'):
                result.append(lines[i])
                i += 1
        elif line.startswith('|||||||'):
            i += 1
            while i < len(lines) and not lines[i].startswith('=======') and not lines[i].startswith('>>>>>>>'):
                i += 1
        elif line.startswith('======='):
            i += 1
            while i < len(lines) and not lines[i].startswith('>>>>>>>'):
                result.append(lines[i])
                i += 1
        elif line.startswith('>>>>>>>'):
            i += 1
        else:
            result.append(line)
            i += 1
    return '\n'.join(result)

def find_good_commit_and_restore(filepath, repo_root, min_lines=20):
    """Find a good commit and restore if current file is too small."""
    current_lines = get_file_line_count(filepath)
    if current_lines >= min_lines:
        return False, current_lines, current_lines
    
    commits = get_git_log_for_file(filepath, repo_root)
    for commit in commits:
        content = get_file_from_commit(commit, filepath, repo_root)
        if content is None:
            continue
        
        # Try to resolve conflicts in the historical content
        if '|||||||' in content or '<<<<<<<' in content:
            resolved = resolve_conflicts_from_content(content)
            lines_count = len(resolved.strip().split('\n'))
            if lines_count >= min_lines:
                with open(filepath, 'w', encoding='utf-8') as f:
                    f.write(resolved)
                return True, current_lines, lines_count
        else:
            lines_count = len(content.strip().split('\n'))
            if lines_count >= min_lines:
                with open(filepath, 'w', encoding='utf-8') as f:
                    f.write(content)
                return True, current_lines, lines_count
    
    return False, current_lines, current_lines

def scan_and_restore(src_dir, repo_root, min_lines=20):
    """Scan all .rs files and restore those that are too small."""
    restored = []
    skipped = []
    failed = []
    
    skip_dirs = {'.git', 'target', 'node_modules'}
    
    for dirpath, dirnames, filenames in os.walk(src_dir):
        dirnames[:] = [d for d in dirnames if d not in skip_dirs]
        for filename in filenames:
            if not filename.endswith('.rs'):
                continue
            filepath = os.path.join(dirpath, filename)
            current_lines = get_file_line_count(filepath)
            
            if current_lines < min_lines:
                success, old_count, new_count = find_good_commit_and_restore(filepath, repo_root, min_lines)
                if success:
                    restored.append((filepath, old_count, new_count))
                    print(f"RESTORED: {os.path.relpath(filepath, repo_root)} ({old_count} -> {new_count} lines)")
                else:
                    failed.append((filepath, old_count))
                    print(f"FAILED: {os.path.relpath(filepath, repo_root)} (only {old_count} lines, couldn't find good version)")
    
    return restored, failed

if __name__ == '__main__':
    repo_root = '/home/aaryansinghchauhan/SigmaOS'
    src_dir = os.path.join(repo_root, 'src')
    print(f"Scanning {src_dir} for under-sized files...")
    restored, failed = scan_and_restore(src_dir, repo_root)
    print(f"\nDone: {len(restored)} files restored, {len(failed)} could not be restored")
    if failed:
        print("\nFailed files:")
        for f, lines in failed:
            print(f"  {f}: {lines} lines")
