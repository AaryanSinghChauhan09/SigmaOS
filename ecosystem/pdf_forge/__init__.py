"""
pdf_forge.py — backward-compat shim.
Real implementation lives in pdf_forge/ package.
"""

from .pdf_forge.SigmaPDFForge import *  # noqa

__all__ = ['SigmaPDFForge']

"""Auto-generated package __init__.py"""
from .sigmapdfforge import *  # noqa: F401, F403
