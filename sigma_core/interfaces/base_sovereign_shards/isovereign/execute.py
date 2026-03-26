from abc import ABC, abstractmethod
import time


@abstractmethod
def execute(self, action, *args, **kwargs):
    pass