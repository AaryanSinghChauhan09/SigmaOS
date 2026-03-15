from abc import ABC, abstractmethod

from ._base import IChaosResilience

class IChaosResilience:
    @abstractmethod
    def perturb_system(self):
        raise NotImplementedError