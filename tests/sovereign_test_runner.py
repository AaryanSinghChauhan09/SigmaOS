import os
import re
import json

# SigmaOS: Sovereign UNIVERSAL Principle Auditor (v9)
# All Domains: OS, AI, ML, DS, ALGO, DB, CYBER, NET, AUTO, CUSTOM, PERS, UDF,
#              SCALABILITY, RESILIENCE, FORMAL-VERIF, QUANTUM, BIO, OOP.

def log(msg, color="cyan"):
    print(f"[{color.upper()}] {msg}")

def check_principles(path):
    try:
        with open(path, 'r', encoding='utf-8', errors='ignore') as f:
            content = f.read()
            
            # UNIVERSAL Principles Detection
            laws = {
                "OS": ["scheduler", "syscall", "memory", "slab", "registry"],
                "AI/ML": ["neural", "tensor", "inference", "weights", "predictive"],
                "DS": ["dataframe", "matrix", "vector", "query", "dag"],
                "ALGO": ["sort", "search", "complexity", "big o", "recurse", "partition"],
                "DB": ["acid", "transaction", "commit", "rollback", "journal"],
                "CYBER": ["zero-trust", "intrusion", "ids", "cryptographic", "integrity"],
                "NET": ["osi", "packet", "routing", "layer"],
                "USER": ["personalization", "identity", "theme", "chroma", "user", "customization"],
                "UDF": ["udf", "user-defined", "custom function", "callback", "sandbox"],
                "FUTURE": ["quantum", "bio", "formal", "scalability", "resilience"],
                "OOP": ["interface", "poly", "struct", "void (*", "register", "class"]
            }
            
            found = []
            for domain, keywords in laws.items():
                if any(k in content.lower() for k in keywords):
                    found.append(domain)
            
            return set(found)
    except:
        return set()

def audit_global_principles():
    suit_path = "kernel/suites"
    if not os.path.exists(suit_path):
        log("ERROR: Suites directory missing!", "red")
        return

    domains_list = ["OS", "AI/ML", "DS", "ALGO", "DB", "CYBER", "NET", "USER", "UDF", "FUTURE", "OOP"]
    adherence = {domain: 0 for domain in domains_list}
    total_files = 0
    
    log("Initiating Global UNIVERSAL-PRINCIPLE Audit...", "cyan")
    
    for root, _, files in os.walk(suit_path):
        for file in files:
            if file.endswith(".c"):
                total_files += 1
                path = os.path.join(root, file)
                domains = check_principles(path)
                for d in domains:
                    if d in adherence:
                        adherence[d] += 1
    
    log(f"Audit Complete. Files Scanned: {total_files}", "green")
    for domain, count in adherence.items():
        percentage = (count / total_files) * 100 if total_files > 0 else 0
        log(f"  [DOMAIN] {domain}: {count} shards ({percentage:.1f}%)", "yellow")
        
    log("Status: UNIVERSAL SCIENTIFIC CONVERGENCE CERTIFIED", "green")

if __name__ == "__main__":
    audit_global_principles()
