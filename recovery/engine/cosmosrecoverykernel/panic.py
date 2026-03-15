# Generated method: CosmosRecoveryKernel.panic
import sys
from sovereign_lisp import SovereignLisp

class CosmosRecoveryKernel:
    def panic(self):
        print('[HALT] Recovery Environment corrupted. Manual re-flash required.')
        sys.exit(1)