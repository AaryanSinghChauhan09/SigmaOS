"""
sigma_projects.py — backward-compat shim.
Real implementation lives in sigma_projects/ package.
"""

from .sigma_projects.SigmaProjects import *  # noqa

__all__ = ['SigmaProjects']

"""Auto-generated package __init__.py"""
from .sigmaprojects import *  # noqa: F401, F403
