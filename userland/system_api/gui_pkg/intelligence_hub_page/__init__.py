"""
intelligence_hub_page.py — backward-compat shim.
Real implementation lives in intelligence_hub_page/ package.
"""

from .intelligence_hub_page.IntelligenceHubPage import *  # noqa

__all__ = ['IntelligenceHubPage']

"""Auto-generated package __init__.py"""
from .intelligencehubpage import *  # noqa: F401, F403
