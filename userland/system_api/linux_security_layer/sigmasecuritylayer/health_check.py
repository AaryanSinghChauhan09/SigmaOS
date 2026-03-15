# Generated method: SigmaSecurityLayer.health_check
import time
import json
from pathlib import Path
from typing import Dict, List, Any

class SigmaSecurityLayer:
    def health_check(self) -> str:
        ufw_st = self.state['ufw']['status']
        sel_st = self.state['selinux']['mode']
        fb_st = self.state['fail2ban']['status']
        return f'OK — SecurityLayer: UFW={ufw_st.upper()} | SELinux={sel_st.upper()} | Fail2Ban={fb_st.upper()}'