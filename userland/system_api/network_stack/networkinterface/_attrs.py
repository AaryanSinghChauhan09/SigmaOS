iface_id: str
name: str
up: bool = True
ip4: str = ''
ip6: str = ''
mac: str = ''
speed_mbps: float = 1000.0
encryption: EncryptionMode = EncryptionMode.QUANTUM_TLS
mesh_capable: bool = False
rx_bytes: int = 0
tx_bytes: int = 0
