inode: str
path: str
is_dir: bool = False
size_bytes: int = 0
sha256: str = ''
encrypted: bool = False
compressed: bool = True
compression_ratio: float = 1.0
created_at: str = ''
modified_at: str = ''
uid: int = 1000
gid: int = 1000
mode: int = 420
attrs: dict = field(default_factory=dict)
snapshots: list[str] = field(default_factory=list)
