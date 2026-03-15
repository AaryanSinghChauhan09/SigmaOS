# Generated class core: UserSession
import time
import uuid
import threading
from dataclasses import dataclass, field

@dataclass
class UserSession:
    session_id: str
    user_id: str
    username: str
    login_time: float
    expiry: float
    is_root: bool = False
    vibe: str = 'Enterprise'
    workspace_id: str = field(default_factory=lambda: str(uuid.uuid4()).split('-')[0])