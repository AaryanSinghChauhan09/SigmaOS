# Generated method: SigmaSyncSentinel._sentinel_loop
import os
import sys
import time
import threading
import subprocess
from typing import Dict, List, Any
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SigmaSyncSentinel:
    def _sentinel_loop(self):
        """Watches for changes and triggers sync scripts within 2s of a save."""
        while self._sync_active:
            time.sleep(2)
            try:
                changed = []
                for d in ['.', 'sigma_core', 'userland/system_api']:
                    dp = os.path.join(self._root, d)
                    if not os.path.exists(dp):
                        continue
                    for f in os.listdir(dp):
                        if f.endswith('.py') or f == 'sync.ps1':
                            fp = os.path.join(dp, f)
                            try:
                                mtime = os.path.getmtime(fp)
                                if fp not in self._file_hashes or self._file_hashes[fp] < mtime:
                                    self._file_hashes[fp] = mtime
                                    changed.append(f)
                            except OSError:
                                continue
                if changed:
                    if self._sync_lock.acquire(blocking=False):
                        try:
                            if self.kernel and hasattr(self.kernel, 'bus'):
                                self.kernel.bus.emit('kernel.automation', {'msg': f'Detected change in {changed}. Syncing...'})
                            if sys.platform == 'win32':
                                subprocess.Popen(['powershell.exe', '-ExecutionPolicy', 'Bypass', '-File', 'sync.ps1'], cwd=self._root, shell=True)
                            else:
                                subprocess.Popen(['bash', 'sync.sh'], cwd=self._root, shell=True)
                            if self.kernel and hasattr(self.kernel, 'ledger'):
                                self.kernel.ledger.commit('SYNC', 'GITHUB_AUTO_PUSH', {'files': changed})
                        finally:
                            threading.Timer(10, self._sync_lock.release).start()
                    else:
                        print('[SYNC-SENTINEL] Sync in progress. Skipping redundant trigger.')
            except Exception as e:
                print(f'[SYNC-SENTINEL] Error: {e}')