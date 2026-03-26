from sigma_core.interfaces.base_sovereign import ISovereign
import time
import traceback
from ..sharddecorator._base import ShardDecorator

class MetricsDecorator(ShardDecorator):
    __slots__ = ('_exec_count', '_total_latency')
    '\n    Analytics & Performance Collector.\n    Tracks execution counts and latencies.\n    '