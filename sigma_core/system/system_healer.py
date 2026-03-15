"""
SigmaOS Modular Shim for system_healer.py
"""
from .system_healer.__os_trim_working_set_core import _os_trim_working_set # noqa
from .system_healer.__os_remove_stale_locks_core import _os_remove_stale_locks # noqa
from .system_healer.__os_native_set_high_priority_core import _os_native_set_high_priority # noqa
from .system_healer._SigmaSystemHealer_core import SigmaSystemHealer # noqa
