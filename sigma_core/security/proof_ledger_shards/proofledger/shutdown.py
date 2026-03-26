import hashlib
import time
from sigma_core.interfaces.base_sovereign import SovereignModule
from sigma_core.interfaces.verification_interfaces import IIntegrityGuard


def shutdown(self):
    print(f'[{self.name}] Securing Ledger...')
    return True