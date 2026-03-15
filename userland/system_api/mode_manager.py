"""
mode_manager.py — SigmaOS Mode Manager (v5.0 Apex)
====================================================
Backward-compat shim.  Real implementation lives in mode_manager/ package.
  - Core class: userland/system_api/mode_manager/mode_manager_core.py
  - Routines:   userland/system_api/mode_manager/routines/<category>.py
"""

from userland.system_api.mode_manager.mode_manager_core import SigmaModeManager  # noqa: F401

__all__ = ["SigmaModeManager"]
