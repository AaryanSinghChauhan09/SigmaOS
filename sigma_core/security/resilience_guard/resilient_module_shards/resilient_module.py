# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
from sigma_core.security.resilience_guard import resilient_module
import functools

@resilient_module
def resilient_module(fallback_func=None):
    """
    Decorator for SigmaOS modular functions to provide 
    automatic fault tolerance and telemetry reporting.
    """

    def decorator(func):

        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            try:
                return func(*args, **kwargs)
            except Exception as e:
                print(f'[RESI-GUARD] Error in {func.__name__}: {e}')
                if fallback_func:
                    return fallback_func(*args, **kwargs)
                return None
        return wrapper
    return decorator