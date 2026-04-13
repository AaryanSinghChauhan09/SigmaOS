import os
import subprocess
import time

def run_step(name, cmd):
    print(f"[PIPELINE]: Step -> {name}")
    try:
        # Using 'py' to ensure compatibility
        process = subprocess.run(cmd, shell=True, capture_output=True, text=True)
        if process.returncode == 0:
            print(f"  [OK] {name} successful.")
            return True
        else:
            print(f"  [ERROR] {name} failed.")
            print(process.stdout)
            print(process.stderr)
            return False
    except Exception as e:
        print(f"  [CRITICAL] Error executing {name}: {e}")
        return False

def main():
    print("=========================================")
    print(" SIGMAOS SOVEREIGN ADVANCEMENT PIPELINE ")
    print("=========================================")
    
    steps = [
        ("Logical Validation", "py tests/sovereign_logic_tester.py"),
        ("Structural Audit", "py tests/sovereign_test_runner.py"),
        ("Dependency Graph Update", "py scripts/generate_dependency_graph.py"),
        ("Shard Manifest Check", "py -c \"import json; f=open('SOVEREIGN_MANIFEST.json'); d=json.load(f); print(f'Shards: {d[\\\"shards\\\"]}'); f.close()\"")
    ]
    
    success = True
    for name, cmd in steps:
        if not run_step(name, cmd):
            success = False
            break
            
    if success:
        print("\n[PIPELINE]: ADVANCEMENT VERIFIED. System is ready for the next Epoch.")
    else:
        print("\n[PIPELINE]: ADVANCEMENT HALTED. System requires manual patching.")

if __name__ == "__main__":
    main()
