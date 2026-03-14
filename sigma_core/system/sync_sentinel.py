"""
SigmaOS Sovereign Sync Sentinel (v1.0 Apex)
============================================
USP: Real-Time Workspace Synchronization (Automated IDE-GitHub Sync).
Refactored from core kernel for better separation of concerns.
"""
import os
import sys
import time
import threading
import subprocess
from typing import Dict, List, Any
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SigmaSyncSentinel(SigmaModuleBase, ISigmaService):
    def __init__(self, kernel):
        super().__init__(kernel)
        self._sync_active = True
        self._sync_lock = threading.Lock()
        self._file_hashes: Dict[str, float] = {}
        self._root = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))

    def start_service(self) -> str:
        t = threading.Thread(target=self._sentinel_loop, daemon=True)
        t.start()
        return "Sync-Sentinel: Real-time Git-Sync ACTIVE."

    def stop_service(self):
        self._sync_active = False

    def _sentinel_loop(self):
        """Watches for changes and triggers sync scripts within 2s of a save."""
        while self._sync_active:
            time.sleep(2)
            try:
                changed = []
                # Watch critical directories
                for d in [".", "sigma_core", "userland/system_api"]:
                    dp = os.path.join(self._root, d)
                    if not os.path.exists(dp): continue
                    
                    for f in os.listdir(dp):
                        if f.endswith(".py") or f == "sync.ps1":
                            fp = os.path.join(dp, f)
                            try:
                                mtime = os.path.getmtime(fp)
                                if fp not in self._file_hashes or self._file_hashes[fp] < mtime:
                                    self._file_hashes[fp] = mtime
                                    changed.append(f)
                            except OSError: continue
                
                if changed:
                    if self._sync_lock.acquire(blocking=False):
                        try:
                            if self.kernel and hasattr(self.kernel, "bus"):
                                self.kernel.bus.emit("kernel.automation", {"msg": f"Detected change in {changed}. Syncing..."})
                            
                            # USP: Cross-Platform Sync Detection
                            if sys.platform == "win32":
                                subprocess.Popen(["powershell.exe", "-ExecutionPolicy", "Bypass", "-File", "sync.ps1"], 
                                                 cwd=self._root, shell=True)
                            else:
                                subprocess.Popen(["bash", "sync.sh"], cwd=self._root, shell=True)
                            
                            if self.kernel and hasattr(self.kernel, "ledger"):
                                self.kernel.ledger.commit("SYNC", "GITHUB_AUTO_PUSH", {"files": changed})
                        finally:
                            # Hold lock for 10s to prevent rapid-fire syncs
                            threading.Timer(10, self._sync_lock.release).start()
                    else:
                        print("[SYNC-SENTINEL] Sync in progress. Skipping redundant trigger.")
            except Exception as e:
                print(f"[SYNC-SENTINEL] Error: {e}")

    def health_check(self) -> str:
        return f"OK - Monitoring {len(self._file_hashes)} files"
