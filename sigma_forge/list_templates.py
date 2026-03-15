"""sigma_forge.list_templates — Lists available forge templates."""


def list_templates() -> list:
    """Returns all available scaffold template types."""
    return ["app", "agent", "service"]
