from abc import ABC, abstractmethod
from ..system_factory import get_factory

class ZeroTrustPolicy(IAccessControl):
    """
    Standard Zero-Trust: Never Trust, Always Verify.
    """