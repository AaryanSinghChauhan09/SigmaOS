flow_id: str
src: str
dst: str
protocol: Protocol
priority: FlowPriority
encrypted: bool = True
bytes_sent: int = 0
latency_ms: float = 0.0
