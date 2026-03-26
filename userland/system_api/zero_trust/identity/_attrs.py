identity_id: str
subject: str
kind: str
trust: TrustLevel
certificate: str
mfa_verified: bool = False
last_verified: str = ''
attributes: dict = field(default_factory=dict)
