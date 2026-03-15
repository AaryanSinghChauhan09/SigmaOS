"""sigma_forge.forge_dispatcher — Routes forge commands to correct generators."""
from sigma_forge.forge_app import forge_app
from sigma_forge.forge_agent import forge_agent
from sigma_forge.forge_service import forge_service


def forge(template_type: str, name: str, output_dir: str = "userland/apps") -> str:
    """Dispatch to the appropriate forge generator based on template_type."""
    dispatch = {
        "app": forge_app,
        "agent": forge_agent,
        "service": forge_service,
    }
    fn = dispatch.get(template_type)
    if fn is None:
        return f"Error: Template type '{template_type}' unknown. Choose from: {list(dispatch.keys())}"
    return fn(name, output_dir)
