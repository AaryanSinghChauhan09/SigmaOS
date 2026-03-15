# Generated file: activate_ghost_mask


def activate_ghost_mask(kernel=None, phase: str='') -> str:
    """Activates GhostChat anonymous mask."""
    if kernel and getattr(kernel, 'ghost_chat', None):
        return 'GhostChat mask active. Anonymous peer routing enabled.'
    return 'GhostChat offline.'