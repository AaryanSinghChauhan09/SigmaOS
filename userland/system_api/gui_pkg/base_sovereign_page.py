from sigma_core.interfaces.base_sovereign import SovereignModule
from abc import abstractmethod

class BaseSovereignPage(SovereignModule):
    """
    Base OOP class for all Userland Pages.
    Inherits from the core SovereignModule.
    """
    def __init__(self, page_name):
        super().__init__(f"GUI_PAGE_{page_name}")
        self._elements = []
        self._theme = "SovereignDark"

    @abstractmethod
    def build_ui(self):
        """Abstraction: UI construction must be handled by subclasses."""
        pass

    def add_element(self, element):
        """Encapsulation: Indirect access to page elements."""
        self._elements.append(element)

    def execute(self, action, *args, **kwargs):
        if action == "RENDER":
            return self.build_ui()
        return super().execute(action, *args, **kwargs)

    def initialize(self):
        print(f"[GUI] Page '{self.name}' Initializing...")
        self.build_ui()

    def shutdown(self):
        self._elements.clear()
        print(f"[GUI] Page '{self.name}' Shutdown.")

    def health_check(self):
        return self._status == "READY"
