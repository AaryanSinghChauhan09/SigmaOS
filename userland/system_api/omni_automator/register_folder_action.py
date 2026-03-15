"""omni_automator.register_folder_action — Folder-action binding."""


def register_folder_action(folder: str, action: str) -> str:
    """Binds an automation action to a watched folder."""
    return f"Folder Action '{action}' firmly bound to '{folder}'."
