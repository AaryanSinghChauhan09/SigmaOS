# Generated file: unseal_standard_vaults


def unseal_standard_vaults(kernel=None, phase: str='') -> str:
    """Restores standard vault access."""
    if kernel and hasattr(kernel, 'crypt_guard') and kernel.crypt_guard:
        return 'Standard vaults unsealed. Access restored to normal privilege level.'
    return 'Vaults unsealed (CryptGuard offline — fallback mode).'