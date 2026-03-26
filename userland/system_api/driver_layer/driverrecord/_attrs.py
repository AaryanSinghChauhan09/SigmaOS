driver_id: str
name: str
cls: DriverClass
vendor: str
version: str
status: DriverStatus = DriverStatus.UNLOADED
is_signed: bool = True
is_sandboxed: bool = True
load_time_ms: float = 0.0
device_ids: list[str] = field(default_factory=list)
last_update: str = ''
