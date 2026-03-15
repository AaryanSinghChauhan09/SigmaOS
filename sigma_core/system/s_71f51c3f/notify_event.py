# SigmaOS Shard: notify_event
from abc import ABC, abstractmethod

def notify_event(event_type, data):
    _global_bus.notify(event_type, data)