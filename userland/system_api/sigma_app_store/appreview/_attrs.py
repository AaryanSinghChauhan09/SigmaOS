reviewer_id: str
rating: int
comment: str
timestamp: float = field(default_factory=time.time)
verified_purchase: bool = True
