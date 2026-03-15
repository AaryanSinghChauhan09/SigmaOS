# Generated method: CosmosRecoveryKernel.boot
import sys
from sovereign_lisp import SovereignLisp

class CosmosRecoveryKernel:
    def boot(self, script_path='recovery/rescue.lisp'):
        print(f'[RECOVERY] Loading {script_path} into RAM...')
        try:
            with open(script_path, 'r') as f:
                core_logic = f.read()
            self.lisp.eval(core_logic)
        except Exception as e:
            print(f'[CRITICAL] Recovery Script Failed: {e}')
            self.panic()