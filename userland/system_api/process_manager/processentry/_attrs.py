pid: str
name: str
qos: QoSClass
state: ProcessState = ProcessState.RUNNING
cpu_pct: float = 0.0
mem_mb: float = 0.0
nice: int = 0
cgroup: str = '/'
syscall_rate: int = 0
entropy: float = 0.0
burst_pred: float = 0.0
created_at: str = ''
tags: list[str] = field(default_factory=list)
