"""
Sigma Automated Task Definitions
================================
Example automated tasks for 'Automation Friendly' SigmaOS.
These can be run headless or via the GUI Sidebar.
"""

from sigma_omni_api import SigmaOmniAPI

def run_security_scan():
    """Linux-style security sweep."""
    print("[*] Running Sovereign Security Audit...")
    # Add real audit logic here
    return "Audit Complete: 0 Vulnerabilities Found."

def run_data_backup():
    """Manual trigger for Sovereign Mesh Drive backup."""
    print("[*] Initiating Mesh P2P Sync/Backup...")
    # Add real sync logic here
    return "Backup Mesh Complete."

if __name__ == "__main__":
    # Integration with OmniAPI
    api = SigmaOmniAPI(None)
    api.register_task("SecurityAudit", run_security_scan)
    api.register_task("MeshBackup", run_data_backup)
    
    # Run based on command-line
    import sys
    if len(sys.argv) > 1:
        task = sys.argv[1]
        if task == "security": api.run_headless_task("SecurityAudit")
        elif task == "backup": api.run_headless_task("MeshBackup")
