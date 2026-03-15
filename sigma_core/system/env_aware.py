import os
import platform

def detect_environment():
    """
    Detects the current hardware and software environment to adjust OS behavior.
    """
    env = {
        "os": platform.system(),
        "arch": platform.machine(),
        "is_low_power": os.environ.get("SIGMA_POWERSAVE", "0") == "1",
        "is_stealth": os.environ.get("SIGMA_STEALTH", "0") == "1",
        "is_compliance_mode": os.environ.get("SIGMA_COMPLIANCE", "1") == "1"
    }
    return env

def apply_environmental_policies():
    env = detect_environment()
    if env["is_stealth"]:
        # Disable logging to disk, use volatile memory only
        pass
    if env["is_compliance_mode"]:
        # Enforce child-safe and professional content across all modules
        pass
    return env
