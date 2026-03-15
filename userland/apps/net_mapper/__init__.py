"""
net_mapper.py — backward-compat shim.
Real implementation lives in net_mapper/ package.
"""

from .net_mapper.AetherNetMapper import *  # noqa
from .net_mapper.sys_ip import *  # noqa

__all__ = ['AetherNetMapper', 'sys_ip']

"""Auto-generated package __init__.py"""
from .aethernetmapper import *  # noqa: F401, F403
from .sys_ip import *  # noqa: F401, F403
