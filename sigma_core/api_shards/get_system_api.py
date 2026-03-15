from sigma_core.system_factory import get_factory
from sigma_core.bootstrap import bootstrap_zenith
from sigma_core.security.proof_ledger import ProofLedger
from sigma_core.security.privacy_guard import DeterministicPrivacyGuard
from sigma_core.kernel.kernel_core import SigmaKernel
from sigma_core.system.device_manager import get_device_manager


def get_system_api():
    """Returns a consolidated system API object."""
    factory = get_factory()
    return {'kernel': factory.get('Kernel') if 'Kernel' in factory._registry else None, 'security': factory.get('Security') if 'Security' in factory._registry else None, 'storage': factory.get('FractalStorage') if 'FractalStorage' in factory._registry else None, 'factory': factory}