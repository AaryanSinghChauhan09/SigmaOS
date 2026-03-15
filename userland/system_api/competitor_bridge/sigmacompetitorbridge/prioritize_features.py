"""
Auto-split from userland\system_api\competitor_bridge.py — SigmaCompetitorBridge.prioritize_features
"""



class SigmaCompetitorBridge:
    def prioritize_features(self, persona: str='default') -> dict:
        """AI-driven prioritization: selects the most relevant competitor USPs based on user persona."""
        all_feats = self.get_all_features()
        priorities = {}
        for platform, feats in all_feats.items():
            if persona == 'developer':
                keys = [k for k in feats if any((x in k for x in ['Package', 'Terminal', 'Kernel', 'Compatibility']))]
            elif persona == 'creative':
                keys = [k for k in feats if any((x in k for x in ['Retina', 'Compositor', 'Continuity', 'Multitasking']))]
            elif persona == 'hardened':
                keys = [k for k in feats if any((x in k for x in ['Privacy', 'Sandboxing', 'Verified', 'Permissions']))]
            else:
                keys = list(feats.keys())
            priorities[platform] = {k: feats[k] for k in keys[:3]}
        return priorities
