import os
import re

focus_files = [
    "DEVELOPMENT_PLAN.md",
    "FUTURE-DEVELOPMENT-ROADMAP.md",
    "ImprovementPlan.md",
    "NEXT_STEPS_GUIDELINES.md",
    "COMPLETION_STATUS.md",
    "WHAT_IS_WORKING_AND_NOT_WORKING.md",
    "docs/ARCHITECTURE_DECISIONS.md",
    "docs/PROJECT_STATUS.md"
]

def is_focus_file(path):
    if any(path.endswith(f) for f in focus_files):
        return True
    if "docs/" in path or "Agents/" in path or "wiki/" in path or "WIKI/" in path:
        return True
    return False

def analyze_md_files(repo_path):
    all_md_files = []
    for root, dirs, files in os.walk(repo_path):
        for f in files:
            if f.endswith('.md'):
                all_md_files.append(os.path.relpath(os.path.join(root, f), repo_path))

    results = []
    
    keywords = ["todo", "unimplemented", "proposed", "not-yet-done", "not yet done", "missing", "gap", "future", "roadmap"]
    
    for f in all_md_files:
        if not is_focus_file(f):
            continue
            
        try:
            with open(os.path.join(repo_path, f), 'r', encoding='utf-8') as file:
                content = file.read()
                
            lines = content.split('\n')
            todos = []
            is_pure_docs = False
            
            # Simple heuristic for pure docs: guidelines, contributing, etc.
            if "contributing" in f.lower() or "guideline" in f.lower() or "policy" in f.lower():
                is_pure_docs = True
                
            for i, line in enumerate(lines):
                lower_line = line.lower()
                if any(kw in lower_line for kw in keywords) and len(line) < 100:
                    if line.strip().startswith('-') or line.strip().startswith('*') or line.strip().startswith('#'):
                        todos.append(line.strip())
            
            results.append({
                "file": f,
                "todos": todos,
                "is_pure_docs": is_pure_docs,
                "content_len": len(content)
            })
        except Exception as e:
            pass
            
    return results

if __name__ == "__main__":
    res = analyze_md_files("/home/aaryansinghchauhan/SigmaOS-work")
    fully_implemented = []
    partially_implemented = []
    pure_docs = []
    
    for r in res:
        if r['is_pure_docs']:
            pure_docs.append(r['file'])
        elif len(r['todos']) == 0:
            fully_implemented.append(r['file'])
        else:
            partially_implemented.append((r['file'], r['todos'][:5]))
            
    print("FULLY IMPLEMENTED (Ready for wiki):")
    for f in fully_implemented:
        print(f)
        
    print("\nPARTIALLY IMPLEMENTED (Has gaps/TODOs):")
    for f, t in partially_implemented:
        print(f"{f} - Sample TODOs: {len(t)}")
        
    print("\nPURE DOCUMENTATION:")
    for f in pure_docs:
        print(f)
