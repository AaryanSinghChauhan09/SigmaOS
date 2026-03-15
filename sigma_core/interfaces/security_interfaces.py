from .system_interfaces import ISystemComponent

class ISecurityGuard(ISystemComponent):
    """
    Security Contract for SigmaOS.
    Ensures that any security component implements authorization logic.
    """
    def authorize(self, actor: str, resource: str) -> bool:
        pass
