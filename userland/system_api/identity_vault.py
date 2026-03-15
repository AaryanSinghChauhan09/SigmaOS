"""
identity_vault.py — backward-compat shim.
Real implementation lives in identity_vault/ package.
"""

from identity_vault.SigmaIdentityVault import *  # noqa

__all__ = ['SigmaIdentityVault']
