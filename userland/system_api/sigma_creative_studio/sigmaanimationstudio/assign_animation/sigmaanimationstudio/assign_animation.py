# Generated method: SigmaAnimationStudio.assign_animation


class SigmaAnimationStudio:
    def assign_animation(self, element_id, animation='spring_bounce', duration_ms=300):
        """Attaches a named animation preset to any UI element ID."""
        self.active_animations[element_id] = {'animation': animation, 'duration': duration_ms}
        return f"AnimStudio: Element '{element_id}' => '{animation}' ({duration_ms}ms). Applied to compositor."