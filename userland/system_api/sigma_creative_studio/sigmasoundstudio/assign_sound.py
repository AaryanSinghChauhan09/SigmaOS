"""
Auto-split from userland\system_api\sigma_creative_studio.py — SigmaSoundStudio.assign_sound
"""



class SigmaSoundStudio:
    def assign_sound(self, event, sound_path):
        """Replaces any system sound event with a custom audio file."""
        if event in self.SYSTEM_SOUNDS:
            self.custom_sounds[event] = sound_path
            return f"SoundStudio: '{event}' now routes to '{sound_path}'."
        return f"SoundStudio Error: Event '{event}' not recognized."
