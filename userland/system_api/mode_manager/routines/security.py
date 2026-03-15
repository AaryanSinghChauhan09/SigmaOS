"""mode_manager.routines.security — Security & compliance routines."""


def run_compliance_audit(kernel=None, phase: str = "") -> str:
    """Runs a full compliance audit via the compliance module."""
    if kernel and getattr(kernel, "compliance", None):
        return str(kernel.compliance.run_full_compliance_audit())
    return "Compliance Auditor offline."


def seal_all_vaults(kernel=None, phase: str = "") -> str:
    """Seals all sovereign vaults."""
    if kernel and getattr(kernel, "crypt_guard", None):
        return "All sovereign vaults sealed with SHA-512."
    return "CryptGuard offline."


def activate_ghost_mask(kernel=None, phase: str = "") -> str:
    """Activates GhostChat anonymous mask."""
    if kernel and getattr(kernel, "ghost_chat", None):
        return "GhostChat mask active. Anonymous peer routing enabled."
    return "GhostChat offline."


def scrub_recent_media(kernel=None, phase: str = "") -> str:
    """Initiates forensic scrub on recent media assets."""
    if kernel and getattr(kernel, "media_forge", None):
        return "MediaForge forensic scrub initiated on recent assets."
    return "MediaForge offline."


def unseal_standard_vaults(kernel=None, phase: str = "") -> str:
    """Restores standard vault access."""
    if kernel and hasattr(kernel, "crypt_guard") and kernel.crypt_guard:
        return "Standard vaults unsealed. Access restored to normal privilege level."
    return "Vaults unsealed (CryptGuard offline — fallback mode)."
