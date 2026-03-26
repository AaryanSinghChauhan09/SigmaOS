from sigma_core.interfaces.base_sovereign import ISovereign
import time
import traceback

from ._base import ShardDecorator

class ShardDecorator:
    def __init__(self, component: ISovereign):
        self._component = component