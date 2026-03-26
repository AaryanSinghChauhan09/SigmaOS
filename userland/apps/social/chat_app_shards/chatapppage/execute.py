from userland.system_api.gui_pkg.base_sovereign_page import BaseSovereignPage
from sigma_core.system_factory import get_factory

from ._base import ChatAppPage

class ChatAppPage:
    def execute(self, action, payload=None):
        if action == 'POST_MSG':
            if not self._chat_engine:
                self._chat_engine = get_factory().get('ChatEngine')
            return self._chat_engine.execute('SEND_MESSAGE', payload)
        return super().execute(action, payload)