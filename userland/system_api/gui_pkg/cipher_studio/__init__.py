"""
cipher_studio.py — backward-compat shim.
Real implementation lives in cipher_studio/ package.
"""

from .cipher_studio.CipherStudioPage import *  # noqa

__all__ = ['CipherStudioPage']

"""Auto-generated package __init__.py"""
from .cipherstudiopage import *  # noqa: F401, F403
