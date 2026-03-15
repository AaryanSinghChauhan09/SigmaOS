app_id: str
name: str
version: str
category: str
developer: str
description: str
size_mb: float
rating: float = 5.0
downloads: int = 0
verified: bool = True
sandbox_level: str = 'STRICT'
permissions: List[str] = field(default_factory=list)
installed: bool = False
install_path: Optional[str] = None
checksum: Optional[str] = None
