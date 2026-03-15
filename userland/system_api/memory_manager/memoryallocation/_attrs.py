alloc_id: str
process: str
size_mb: float
tier: MemoryTier
state: PageState
compressed: bool = False
compression_ratio: float = 1.0
pinned: bool = False
peer_node: str | None = None
created_at: str = ''
