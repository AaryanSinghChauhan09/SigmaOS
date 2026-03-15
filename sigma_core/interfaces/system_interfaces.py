from abc import ABC, abstractmethod

class ISystemComponent(ABC):
    """
    Base contract for all high-level system components.
    """
    @abstractmethod
    def initialize(self):
        raise NotImplementedError

    @abstractmethod
    def shutdown(self):
        raise NotImplementedError

    @abstractmethod
    def health_check(self) -> bool:
        raise NotImplementedError

class ISchedulingStrategy(ABC):
    """
    Strategy Pattern for task execution.
    Demonstrates Polymorphism.
    """
    @abstractmethod
    def select_next(self, queue: list) -> str:
        raise NotImplementedError

class IScheduler(ISystemComponent):
    @abstractmethod
    def set_strategy(self, strategy: ISchedulingStrategy):
        raise NotImplementedError

    @abstractmethod
    def schedule_task(self, task_id, priority):
        raise NotImplementedError

class IMemoryManager(ISystemComponent):
    @abstractmethod
    def allocate(self, size):
        raise NotImplementedError

class IFileSystem(ISystemComponent):
    @abstractmethod
    def mount(self, path):
        raise NotImplementedError
