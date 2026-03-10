import sys
import os
from sigma_core.kernel import SigmaKernel
from sigma_gui import SigmaGUI

def main():
    # --- Environment Hydration Check ---
    if not os.path.exists("ecosystem/registry.json"):
        print("[!] Environment not hydrated. Running Sovereign Setup...")
        import subprocess
        subprocess.run([sys.executable, "sigma_setup.py"])
    
    print("Initializing SigmaOS Kernel...")
    kernel = SigmaKernel()
    
    # Initialize Core Modules
    if not kernel.pulse.heartbeat():
        print("PULSE FAILURE: Kernel startup aborted.")
        sys.exit(1)
        
    print("Starting Sovereign GUI Layer...")
    app = SigmaGUI(kernel)
    app.mainloop()

if __name__ == "__main__":
    main()
