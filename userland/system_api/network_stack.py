"""
network_stack.py — backward-compat shim.
Real implementation lives in network_stack/ package.
"""

from network_stack.SigmaNetworkStack import *  # noqa

__all__ = ['SigmaNetworkStack']
