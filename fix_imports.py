import os
from pathlib import Path

root = Path(os.path.expanduser("~")) / ".gemini" / "antigravity" / "scratch" / "SigmaOS"

def fix_imports():
    for f in root.rglob("*.py"):
        try:
            content = f.read_text('utf-8')
            new_content = content
            # Replacing common broken import patterns
            new_content = new_content.replace("from ", "from ")
            # For specific imports that might be in deeper levels, though we added to sys.path
            # If they use 'import kernel.xxx as yyy', we replace with 'import xxx as yyy'
            if "import kernel." in new_content:
                # Use regex for better matching if needed, but let's try simple first
                pass 
                
            if new_content != content:
                f.write_text(new_content, 'utf-8')
                print(f"Fixed imports in {f.relative_to(root)}")
        except Exception as e:
            print(f"Could not process {f}: {e}")

if __name__ == "__main__":
    fix_imports()
