from userland.system_api.gui_pkg.base_sovereign_page import BaseSovereignPage
from sigma_core.system_factory import get_factory

from ._base import ChatAppPage

class ChatAppPage:
    def build_ui(self):
        print(f'[GUI-{self.name}] Building Sovereign Chat Interface...')
        self.add_element('HistoryViewport')
        self.add_element('MessageInput')
        self.add_element('SendButton')
        return 'UI_BUILD_SUCCESS'