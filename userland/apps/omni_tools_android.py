"""
omni_tools_android.py — backward-compat shim.
Real implementation lives in omni_tools_android/ package.
"""

from omni_tools_android.fmt import *  # noqa
from omni_tools_android._build_qr_matrix import *  # noqa
from omni_tools_android.OmniToolsApp import *  # noqa

__all__ = ['fmt', '_build_qr_matrix', 'OmniToolsApp']
