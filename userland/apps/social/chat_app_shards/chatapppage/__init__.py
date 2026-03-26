from userland.system_api.gui_pkg.base_sovereign_page import BaseSovereignPage
from sigma_core.system_factory import get_factory

from ._base import ChatAppPage

class ChatAppPage:
    def __init__(self):
        super().__init__('CHAT_APP')
        self._chat_engine = None