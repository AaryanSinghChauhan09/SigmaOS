"""
boost_engine.py — backward-compat shim.
Real implementation lives in boost_engine/ package.
"""

from .boost_engine._native_cpu_usage import *  # noqa
from .boost_engine._native_set_high_priority import *  # noqa
from .boost_engine._native_trim_working_set import *  # noqa
from .boost_engine._flush_cache import *  # noqa
from .boost_engine._verify_integrity import *  # noqa
from .boost_engine._scrub_identity import *  # noqa
from .boost_engine._overclock_bus import *  # noqa
from .boost_engine._predictive_preheat import *  # noqa
from .boost_engine._agent_rebalance import *  # noqa
from .boost_engine.SigmaPerformanceBoost import *  # noqa

__all__ = ['_native_cpu_usage', '_native_set_high_priority', '_native_trim_working_set', '_flush_cache', '_verify_integrity', '_scrub_identity', '_overclock_bus', '_predictive_preheat', '_agent_rebalance', 'SigmaPerformanceBoost']

"""Auto-generated package __init__.py"""
from ._native_cpu_usage import *  # noqa: F401, F403
from ._native_set_high_priority import *  # noqa: F401, F403
from ._native_trim_working_set import *  # noqa: F401, F403
from ._flush_cache import *  # noqa: F401, F403
from ._verify_integrity import *  # noqa: F401, F403
from ._scrub_identity import *  # noqa: F401, F403
from ._overclock_bus import *  # noqa: F401, F403
from ._predictive_preheat import *  # noqa: F401, F403
from ._agent_rebalance import *  # noqa: F401, F403
from .sigmaperformanceboost import *  # noqa: F401, F403
