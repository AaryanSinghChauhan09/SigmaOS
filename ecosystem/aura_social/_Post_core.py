# Generated class core: Post
import hashlib
import time
import uuid
from dataclasses import dataclass

@dataclass
class Post:
    post_id: str
    author: str
    content: str
    timestamp: float
    signature: str