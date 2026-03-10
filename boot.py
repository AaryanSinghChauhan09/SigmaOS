import sys
import os
from sigma_core.kernel import SigmaKernel
from sigma_gui import SigmaGUI

def main():
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
