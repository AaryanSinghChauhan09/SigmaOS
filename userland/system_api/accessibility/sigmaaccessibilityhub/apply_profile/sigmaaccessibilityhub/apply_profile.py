# Generated method: SigmaAccessibilityHub.apply_profile
from dataclasses import dataclass
from enum import Enum, auto
import threading

class SigmaAccessibilityHub:
    def apply_profile(self, profile_key: str) -> dict[str, str]:
        """USP: Personalized Accessibility Profiles. Loads an entire user setup instantly."""
        prof: dict = self.ACCESSIBILITY_PROFILES.get(profile_key, {})
        if not prof:
            return {'error': 'Profile not found.'}
        messages = []
        raw_feat = prof.get('features', {})
        if not isinstance(raw_feat, dict):
            raw_feat = {}
        for feat in list(raw_feat.keys()):
            state = raw_feat[feat]
            res = self.toggle_feature(str(feat), bool(state))
            if 'message' in res:
                messages.append(res['message'])
        i_mode_val = prof.get('input_mode', InputMode.STANDARD)
        i_mode = i_mode_val if isinstance(i_mode_val, InputMode) else InputMode.STANDARD
        self._current_input_mode = i_mode
        messages.append(f'Input Mode forced to: {i_mode.name}')
        self._stats['sessions_assisted'] += 5
        return {'status': 'PROFILE APPLIED', 'summary': ' | '.join(messages)}