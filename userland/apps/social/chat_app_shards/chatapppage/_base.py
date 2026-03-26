from userland.system_api.gui_pkg.base_sovereign_page import BaseSovereignPage
from sigma_core.system_factory import get_factory


class ChatAppPage(BaseSovereignPage):
    __slots__ = ('_chat_engine',)
    '\n    Sovereign Chat Application Page.\n    Concrete implementation of BaseSovereignPage.\n    '