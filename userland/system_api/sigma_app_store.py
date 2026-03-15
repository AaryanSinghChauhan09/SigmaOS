"""
sigma_app_store.py — backward-compat shim.
Real implementation lives in sigma_app_store/ package.
"""

from sigma_app_store.SigmaApp import *  # noqa
from sigma_app_store.SigmaAppStore import *  # noqa

__all__ = ['SigmaApp', 'SigmaAppStore']
