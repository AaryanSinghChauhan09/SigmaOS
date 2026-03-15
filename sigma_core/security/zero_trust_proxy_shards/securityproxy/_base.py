from abc import ABC, abstractmethod
from ..system_factory import get_factory

class SecurityProxy:
    """
    Proxy Pattern: Wraps resource access with security validation.
    """