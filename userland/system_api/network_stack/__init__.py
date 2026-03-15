"""
network_stack.py — backward-compat shim.
Real implementation lives in network_stack/ package.
"""

from .network_stack.SigmaNetworkStack import *  # noqa

__all__ = ['SigmaNetworkStack']

"""Auto-generated package __init__.py"""
from .sigmanetworkstack import *  # noqa: F401, F403
