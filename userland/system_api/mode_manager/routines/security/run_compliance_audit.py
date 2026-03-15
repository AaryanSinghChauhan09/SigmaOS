# Generated file: run_compliance_audit


def run_compliance_audit(kernel=None, phase: str='') -> str:
    """Runs a full compliance audit via the compliance module."""
    if kernel and getattr(kernel, 'compliance', None):
        return str(kernel.compliance.run_full_compliance_audit())
    return 'Compliance Auditor offline.'