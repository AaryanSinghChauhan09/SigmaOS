from sigma_core.interfaces.system_interfaces import ISystemComponent
from abc import abstractmethod


class IDeviceDriver(ISystemComponent):
    """
    Interface for all Low-Level Device Drivers.
    Abstraction over hardware.
    """