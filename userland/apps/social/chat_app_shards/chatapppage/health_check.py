from userland.system_api.gui_pkg.base_sovereign_page import BaseSovereignPage
from sigma_core.system_factory import get_factory

from ._base import ChatAppPage

class ChatAppPage:
    def health_check(self):
        return True