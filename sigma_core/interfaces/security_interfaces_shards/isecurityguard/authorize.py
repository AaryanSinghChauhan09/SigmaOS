from sigma_core.interfaces.system_interfaces import ISystemComponent

from ._base import ISecurityGuard

class ISecurityGuard:
    def authorize(self, actor: str, resource: str) -> bool:
        pass