from abc import ABC, abstractmethod

class SigmaSovereignBase(ABC):
    """
    The Abstract Base Class for all SigmaOS modules.
    Ensures that every module implements core OS protocols.
    """
    
    @abstractmethod
    def boot(self):
        """Initializes the module."""
        pass

    @abstractmethod
    def health_check(self) -> bool:
        """Returns the integrity status of the module."""
        pass

    @property
    @abstractmethod
    def signature(self):
        """The cryptographic signature of this shard."""
        pass

class SigmaModule(SigmaSovereignBase):
    """
    Standard implementation with default behaviors.
    """
    def __init__(self, name):
        self._name = name # Encapsulation: Protected attribute

    def boot(self, mode=None):
        # Method Overloading (Simulated via default args)
        if mode == "FAST":
            print(f"Sigma Module [{self._name}] is ascending at lightspeed...")
        else:
            print(f"Sigma Module [{self._name}] is ascending...")

    def health_check(self):
        return True

    @property
    def signature(self):
        return "SIGMA_PRIME_0x00"

class HardwareAbstractionLayer(ABC):
    @abstractmethod
    def read_interrupt(self):
         pass

class SovereignHardwareModule(SigmaModule, HardwareAbstractionLayer):
    """
    Example of Multiple Inheritance: 
    A module that is both a standard Sigma Module and a HAL.
    """
    def read_interrupt(self):
        return "IRQ_0"
