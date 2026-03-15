# Generated class core: BridgeConfig
from enum import Enum
from dataclasses import dataclass
import uuid

@dataclass
class BridgeConfig:
    app_id: str
    os_type: OSStack
    vfs_root: str
    ui_engine: str = 'Sigma_Shader_Canvas'
    input_shim: InputMode = InputMode.MOUSE