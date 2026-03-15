"""
privacy_shield.py — backward-compat shim.
Real implementation lives in privacy_shield/ package.
"""

from .privacy_shield.SigmaPrivacyShield import *  # noqa

__all__ = ['SigmaPrivacyShield']

"""Auto-generated package __init__.py"""
from .sigmaprivacyshield import *  # noqa: F401, F403
