from abc import ABC, abstractmethod
import time


@abstractmethod
def health_check(self) -> bool:
    pass