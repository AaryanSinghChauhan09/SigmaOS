"""
omni_tweak_daemon.py — backward-compat shim.
Real implementation lives in omni_tweak_daemon/ package.
"""

from .omni_tweak_daemon.OmniTweakDaemon import *  # noqa

__all__ = ['OmniTweakDaemon']

"""Auto-generated package __init__.py"""
from .omnitweakdaemon import *  # noqa: F401, F403
