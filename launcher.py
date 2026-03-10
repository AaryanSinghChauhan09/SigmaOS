import sys
import os

# Add relevant directories to path
_ROOT = os.path.dirname(os.path.abspath(__file__))
if _ROOT not in sys.path:
    sys.path.insert(0, _ROOT)

try:
    from sigma_core.kernel import SigmaKernel
    from sigma_gui import launch_gui
    
    print(">>> HYDRATING SIGMAOS SOVEREIGN ECOSYSTEM <<<")
    print(">>> Target: UNIVERSAL DESKTOP APP MODE    <<<")
    
    # Initialize Kernel with auto-load enabled
    kernel = SigmaKernel(auto_load=True)
    
    # Launch GUI
    success = launch_gui(kernel)
    if not success:
        print("[CRITICAL] GUI hydration failed. Falling back to kernel shell.")

except ImportError as e:
    print(f"[FATAL] System file missing: {e}")
    print("Please ensure you are running from the SigmaOS root directory.")
    input("Press Enter to exit...")
except Exception as e:
    print(f"[ERROR] Unexpected crash: {e}")
    input("Press Enter to exit...")
