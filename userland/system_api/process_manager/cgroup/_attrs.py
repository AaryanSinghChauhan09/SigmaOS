name: str
cpu_quota: float
mem_limit_mb: float
io_weight: int
children: list[str] = field(default_factory=list)
