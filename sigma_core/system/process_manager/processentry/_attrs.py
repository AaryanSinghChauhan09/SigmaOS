pid: str
name: str
qos: QoSClass
cgroup: str
cpu_pct: float = 0.0
mem_mb: float = 0.0
nice: int = 0
state: ProcessState = ProcessState.RUNNING
created_at: str = ''
burst_pred: float = 0.0
entropy: float = 0.0
syscall_rate: int = 0
