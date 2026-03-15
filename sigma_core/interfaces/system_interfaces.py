from abc import ABC, abstractmethod

class ISystemComponent(ABC):
    @abstractmethod
    def initialize(self): pass
    @abstractmethod
    def shutdown(self): pass
    @abstractmethod
    def health_check(self) -> bool: pass

class IScheduler(ISystemComponent):
    @abstractmethod
    def schedule_task(self, task_id, priority): pass

class IMemoryManager(ISystemComponent):
    @abstractmethod
    def allocate(self, size): pass

class IFileSystem(ISystemComponent):
    @abstractmethod
    def mount(self, path): pass
