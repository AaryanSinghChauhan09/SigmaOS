from sigma_core.interfaces.base_sovereign import ISovereign
import time
import traceback


class ShardDecorator(ISovereign):
    __slots__ = ('_component',)
    '\n    Base Decorator / Proxy for Sovereign Components.\n    Proxies all attributes to the underlying component.\n    '