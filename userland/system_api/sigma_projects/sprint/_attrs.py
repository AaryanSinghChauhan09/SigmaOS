sprint_id: str
name: str
goal: str
start_date: float
end_date: float
status: str = 'ACTIVE'
tasks: list[str] = field(default_factory=list)
