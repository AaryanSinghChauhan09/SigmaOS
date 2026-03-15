from sigma_core.interfaces.system_interfaces import ISystemComponent


class ISecurityGuard(ISystemComponent):
    """
    Security Contract for SigmaOS.
    Ensures that any security component implements authorization logic.
    """