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
    keywords = ["todo", "unimplemented", "proposed", "not-yet-done", "missing", "gap", "future", "roadmap", "[ ]"]
    
    for f in all_md_files:
        if not is_focus_file(f):
            continue
            
        try:
            with open(os.path.join(repo_path, f), 'r', encoding='utf-8') as file:
                content = file.read()
                
            lines = content.split('\n')
            todos = []
            is_pure_docs = False
            
            if "contributing" in f.lower() or "guideline" in f.lower() or "policy" in f.lower():
                is_pure_docs = True
                
            for i, line in enumerate(lines):
                lower_line = line.lower()
                if any(kw in lower_line for kw in keywords) and len(line) < 150:
                    if line.strip().startswith('-') or line.strip().startswith('*') or line.strip().startswith('#') or "[ ]" in line:
                        todos.append(line.strip())
            
            results.append({
                "file": f,
                "todos": list(set(todos)),
                "is_pure_docs": is_pure_docs,
            })
        except Exception as e:
            pass
            
    return results

def check_src_for_todos(todos, repo_path):
    # extremely naive check
    # if we find the words in src file names
    # not very accurate but ok for summary
    return []

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
            partially_implemented.append((r['file'], r['todos'][:3]))
            
    with open("final_report.txt", "w") as out:
        out.write("### Fully Implemented (Ready to transfer to wiki)\n")
        for f in fully_implemented:
            out.write(f"- {f}\n")
            
        out.write("\n### Partially Implemented (With specific gaps)\n")
        for f, t in partially_implemented:
            out.write(f"- {f}\n")
            for todo in t:
                out.write(f"  * Gap/TODO: {todo}\n")
                
        out.write("\n### Pure Documentation (Guidelines, Contributing, etc.)\n")
        for f in pure_docs:
            out.write(f"- {f}\n")
