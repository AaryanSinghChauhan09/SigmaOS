from userland.system_api.theme_engine import SigmaThemeEngine
t = SigmaThemeEngine()
print(f"Has apply_aura: {hasattr(t, 'apply_aura')}")
print(f"Methods: {[m for m in dir(t) if not m.startswith('__')]}")
