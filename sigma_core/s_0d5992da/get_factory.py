# SigmaOS Shard: get_factory
from .interfaces.system_interfaces import ISystemComponent
import threading

def get_factory():
    return SystemFactory()