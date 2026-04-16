import os
import json
import time
import psutil
from pathlib import Path

# Pillar 4: Bridge between underlying machine OS and the Web GUI

ROOT_DIR = Path(__file__).parent.parent
TELEMETRY_PATH = ROOT_DIR / 'kernel' / 'telemetry.json'

def gather_telemetry():
    cpu_usage = psutil.cpu_percent(interval=1)
    mem = psutil.virtual_memory()
    
    # Calculate synthetic "Coverage" and "IQ YIELD" based on system load
    coverage = 100 - int(cpu_usage)
    if coverage < 20:
        coverage = 20
        
    iq_yield = "TRANSCENDENT" if cpu_usage > 70 else "ABSOLUTE"
    mem_str = f"{mem.used / (1024**3):.1f}GB / {mem.total / (1024**3):.1f}GB"
    
    payload = {
        "coverage": f"{coverage}%",
        "iq_yield": iq_yield,
        "memory": mem_str,
        "cpu": f"{cpu_usage}%",
        "processes": len(psutil.pids())
    }
    
    with open(TELEMETRY_PATH, 'w') as f:
        json.dump(payload, f)
        
    print(f"[*] Telemetry injected to C-Kernel memory pipeline: {payload}")

if __name__ == "__main__":
    print("[SYSTEM] Sovereign Telemetry Daemon active. Polling real hardware metrics...")
    TELEMETRY_PATH.parent.mkdir(exist_ok=True)
    try:
        while True:
            gather_telemetry()
            time.sleep(2)
    except KeyboardInterrupt:
        print("\n[SYSTEM] Telemetry bridge dissolved.")
