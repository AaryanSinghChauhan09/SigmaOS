"""
pdf_forge.py — backward-compat shim.
Real implementation lives in pdf_forge/ package.
"""

from .pdf_forge.SovereignPDFEditor import *  # noqa

__all__ = ['SovereignPDFEditor']

"""Auto-generated package __init__.py"""
from .sovereignpdfeditor import *  # noqa: F401, F403
