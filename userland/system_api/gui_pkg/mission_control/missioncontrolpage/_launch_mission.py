# Generated method: MissionControlPage._launch_mission
import tkinter as tk
from tkinter import scrolledtext
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_BOLD, FONT_TITLE

class MissionControlPage:
    def _launch_mission(self, mission_id: str, name: str):
        """Hand-off to OmniAutomator for mission execution."""
        self.mc_log.insert('end', f'\n[LAUNCH] Starting Mission: {name}...\n', 'launch')
        self.mc_log.see('end')
        if self.controller.kernel and hasattr(self.controller.kernel, 'automator') and self.controller.kernel.automator:
            self.controller.kernel.bus.emit('claw.mission.launch', {'id': mission_id, 'mission': name})
            self.controller._update_morphic_status('MISSION', f'Launching {name}', PAL['cyan'])
        else:
            self.mc_log.insert('end', '[ERROR] Agentic Subsystems Offline.\n', 'err')