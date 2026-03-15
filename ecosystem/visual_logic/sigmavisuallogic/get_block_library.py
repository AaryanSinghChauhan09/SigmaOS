# Generated method: SigmaVisualLogic.get_block_library
from dataclasses import dataclass, field
from typing import List, Dict, Any, Union
import json

class SigmaVisualLogic:
    def get_block_library(self):
        """Returns the 'Scratch Pallete' available for the user."""
        return {'Triggers': ['On_Timer', 'On_Location', 'On_Mesh_Event', 'On_Battery'], 'Actions': ['Set_Theme', 'Lock_Vault', 'Broadcast_Mesh', 'Start_Mission', 'Apply_Routine'], 'Control': ['If/Else', 'Wait', 'Repeat', 'Parallel_Wait']}