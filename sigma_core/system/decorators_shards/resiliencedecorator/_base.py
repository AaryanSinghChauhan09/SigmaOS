from sigma_core.interfaces.base_sovereign import ISovereign
import time
import traceback
from ..sharddecorator._base import ShardDecorator

class ResilienceDecorator(ShardDecorator):
    """
    Fault Tolerance Shard Wrapper.
    """