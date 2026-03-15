# Generated file: set_adaptive_theme


def set_adaptive_theme(state: dict, context: str) -> str:
    """Personalization: Autonomously shift UI based on environmental context."""
    from userland.system_api.fluid_ui.window_transparency import apply_window_transparency
    from userland.system_api.fluid_ui.configure_widgets import configure_widgets
    ctx = context.lower()
    if ctx == 'night':
        state['layout_mode'] = 'Abyssal_Dark'
        apply_window_transparency(state, alpha=0.95)
    elif ctx == 'gaming':
        state['layout_mode'] = 'Performance_Solid'
        apply_window_transparency(state, alpha=1.0)
    elif ctx == 'focus':
        state['layout_mode'] = 'Zen_Minimalist'
        apply_window_transparency(state, alpha=0.7)
        configure_widgets(state, remove=['social_feed', 'stocks'])
    else:
        state['layout_mode'] = 'Dynamic_Glass'
    return f"Adaptive Theme Engine: Metamorphosis to '{state['layout_mode']}' complete."