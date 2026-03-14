import tkinter as tk
from tkinter import ttk

class SigmaQuickSettings:
    """UI page that lets the user adjust core SigmaConfig flags.
    It reads/writes the global SigmaConfig instance (kernel.cfg).
    """
    def __init__(self, kernel, parent):
        self.kernel = kernel
        self.parent = parent
        self.frame = ttk.Frame(parent, padding=20)
        self._build_ui()

    def _build_ui(self):
        # Title
        ttk.Label(self.frame, text="Quick Settings", font=('Helvetica', 16, 'bold')).pack(pady=10)

        # Mode selector
        ttk.Label(self.frame, text="Performance Mode:").pack(anchor='w')
        self.mode_var = tk.StringVar(value=self.kernel.cfg.ACTIVE_MODE)
        mode_combo = ttk.Combobox(self.frame, textvariable=self.mode_var, state='readonly')
        mode_combo['values'] = list(self.kernel.cfg.MODES.keys())
        mode_combo.pack(fill='x', pady=5)
        mode_combo.bind('<<ComboboxSelected>>', self._apply_mode)

        # Developer mode toggle
        self.dev_var = tk.BooleanVar(value=self.kernel.cfg.DEVELOPER_MODE)
        dev_chk = ttk.Checkbutton(self.frame, text="Developer Mode (extra logs & debug console)", variable=self.dev_var, command=self._toggle_dev)
        dev_chk.pack(anchor='w', pady=5)

        # Shortcut editor (simple view)
        ttk.Label(self.frame, text="Global Shortcuts (editable JSON):").pack(anchor='w', pady=(15,5))
        self.shortcut_text = tk.Text(self.frame, height=8)
        self.shortcut_text.pack(fill='both', expand=True)
        self.shortcut_text.insert('1.0', self._shortcuts_as_json())
        ttk.Button(self.frame, text="Save Shortcuts", command=self._save_shortcuts).pack(pady=5)

    def _apply_mode(self, event=None):
        mode = self.mode_var.get()
        self.kernel.cfg.apply_mode(mode)
        # Refresh UI if needed
        print(f"[INFO] QuickSettings: Mode switched to {mode}")

    def _toggle_dev(self):
        self.kernel.cfg.DEVELOPER_MODE = self.dev_var.get()
        print(f"[INFO] QuickSettings: Developer mode set to {self.kernel.cfg.DEVELOPER_MODE}")

    def _shortcuts_as_json(self):
        import json
        return json.dumps(self.kernel.cfg.SHORTCUTS, indent=2)

    def _save_shortcuts(self):
        import json
        try:
            data = json.loads(self.shortcut_text.get('1.0', 'end'))
            self.kernel.cfg.SHORTCUTS = data
            self.kernel.cfg.save()
            print("[INFO] QuickSettings: Shortcuts saved to config.")
        except Exception as e:
            print(f"[ERROR] QuickSettings: Invalid JSON – {e}")

    def get_frame(self):
        return self.frame
