# Generated method: SigmaScrubber.run
import os
import re
from pathlib import Path

class SigmaScrubber:
    def run(self):
        print('--- SIGMA IDENTITY SCRUB BATTLE-READY ---')
        scrubbed_count = 0
        for root, dirs, files in os.walk(self.root):
            if '.git' in root or '__pycache__' in root or 'evidence_vault' in root:
                continue
            for f in files:
                if f.endswith(('.py', '.json', '.sh', '.ps1', '.txt', '.md')):
                    if self.scrub_file(os.path.join(root, f)):
                        scrubbed_count += 1
        from userland.system_api.sigma_std import SigmaCrypto
        for tmp_f in ['_errors.txt', '_errors2.txt', 'system_audit.sigma.tmp']:
            if os.path.exists(tmp_f):
                SigmaCrypto.secure_shred(tmp_f)
                print(f'[SHREDDED] Trace file: {tmp_f}')
        print(f'--- SCRUB COMPLETE. {scrubbed_count} files sanitized. ---')