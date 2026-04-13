import os
import re
import json

# SigmaOS: Sovereign Modular Integrity Auditor (v3 - FINAL)
# Optimized for the 10 Master Suites architecture.

def log(msg, color="cyan"):
    print(f"[{color.upper()}] {msg}")

def check_c_file(path):
    try:
        with open(path, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
            has_register = re.search(r'void\s+Sovereign\w+_Register\(void\)', content)
            has_header = "sigma_base.h" in content or "sigma_kernel.h" in content or "sigma_string.h" in content
            
            return {
                "register": bool(has_register),
                "header": has_header
            }
    except:
        return {"register": False, "header": False}

def audit_all_shards():
    results = {
        "passed": 0,
        "failed": 0,
        "total_shards": 0,
        "details": []
    }
    
    scan_paths = [
        "kernel/suites"
    ]
    
    log("Starting Final Sovereign Integrity Audit...", "cyan")
    
    for base_path in scan_paths:
        if not os.path.exists(base_path):
            log(f"ALERT: {base_path} missing!", "red")
            continue
            
        for root, _, files in os.walk(base_path):
            for file in files:
                if file.endswith((".c", ".h", ".asm")):
                    results["total_shards"] += 1
                    path = os.path.join(root, file)
                    
                    if file.endswith(".c"):
                        analysis = check_c_file(path)
                        if analysis["header"]:
                            results["passed"] += 1
                        else:
                            results["failed"] += 1
                            results["details"].append(f"MISSING_HEADER: {path}")
    
    log(f"Audit Complete. Total Shards in Suites: {results['total_shards']}, Passed: {results['passed']}, Failed: {results['failed']}", "green")
    
    if results["failed"] > 0:
        log("Failures found:", "red")
        for detail in results["details"]:
            print(f"  [!] {detail}")

if __name__ == "__main__":
    audit_all_shards()
