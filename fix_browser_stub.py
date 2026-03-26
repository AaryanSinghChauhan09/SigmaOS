"""Remove bad stubs and re-inject clean versions."""
import re

with open("sigma_gui.py", "r", encoding="utf-8") as f:
    src = f.read()

# Remove any previously injected bad stubs
bad_stub_pattern = r"\n    def _build_browser_page\(self\):[^\n]*\n(?:.*\n)*?(?=\ndef launch_gui)"
src = re.sub(bad_stub_pattern, "\n", src)

# Also clean via simple string approach - find bad block
marker_start = "\n    def _build_browser_page(self):\n        p = tk.Frame(self._content"
marker_end   = "\n\ndef launch_gui("
if marker_start in src:
    start_idx = src.index(marker_start)
    end_idx   = src.index(marker_end)
    src = src[:start_idx] + src[end_idx:]
    print("Removed bad browser stub.")

# Inject a clean stub just before launch_gui
clean_stub = '''
    def _build_browser_page(self):
        p = tk.Frame(self._content, bg=PAL["bg"])
        self._pages["browser_main"] = p
        tk.Label(p, text="\U0001f310 Sovereign Browser",
                 font=FONT_LOGO, fg=PAL["cyan"], bg=PAL["bg"]).pack(anchor="w", pady=(0, 8))
        tk.Label(p,
                 text="Privacy Shield ON  \u2022  Anti-Tracking  \u2022  Incognito  \u2022  Zero Cookies",
                 font=FONT_MED, fg=PAL["teal"], bg=PAL["bg"]).pack(anchor="w")
        tip = tk.Frame(p, bg=PAL["bg"])
        tip.pack(fill="both", expand=True)
        tk.Label(tip, text="Use the Sovereign Browser via the taskbar or\nopen 'Speed & Shield' from the sidebar.",
                 font=FONT_BOLD, fg=PAL["dim"], bg=PAL["bg"]).pack(expand=True)

'''

inject_marker = "\ndef launch_gui("
if inject_marker in src:
    src = src.replace(inject_marker, clean_stub + inject_marker)
    print("Injected clean browser stub.")
else:
    print("ERROR: Could not find injection point.")

with open("sigma_gui.py", "w", encoding="utf-8") as f:
    f.write(src)

print("Done.")
