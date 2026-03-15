from abc import ABC, abstractmethod

class SystemBus:
    def __init__(self):
        self._observers = []