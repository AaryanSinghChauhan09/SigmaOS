from ...system_api.gui_pkg.base_sovereign_page import BaseSovereignPage
from ....sigma_core.system_factory import get_factory

class ChatAppPage(BaseSovereignPage):
    """
    Sovereign Chat Application Page.
    Concrete implementation of BaseSovereignPage.
    """
    def __init__(self):
        super().__init__("CHAT_APP")
        self._chat_engine = None

    def build_ui(self):
        print(f"[GUI-{self.name}] Building Sovereign Chat Interface...")
        self.add_element("HistoryViewport")
        self.add_element("MessageInput")
        self.add_element("SendButton")
        return "UI_BUILD_SUCCESS"

    def execute(self, action, payload=None):
        if action == "POST_MSG":
            if not self._chat_engine:
                self._chat_engine = get_factory().get("ChatEngine")
            return self._chat_engine.execute("SEND_MESSAGE", payload)
        return super().execute(action, payload)

    def health_check(self):
        return True
