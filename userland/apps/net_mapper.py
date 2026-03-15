"""
net_mapper.py — backward-compat shim.
Real implementation lives in net_mapper/ package.
"""

from net_mapper.AetherNetMapper import *  # noqa
from net_mapper.sys_ip import *  # noqa

__all__ = ['AetherNetMapper', 'sys_ip']
