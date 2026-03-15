# Generated method: SigmaAnimationStudio.create_custom_keyframes


class SigmaAnimationStudio:
    def create_custom_keyframes(self, name, keyframes):
        """
                CSS-style @keyframes builder: Define from/to states for any UI property.
                keyframes: dict e.g. {'0%': {'opacity': 0}, '100%': {'opacity': 1}}
                """
        return f"AnimStudio (Custom): Keyframe animation '{name}' compiled with {len(keyframes)} states."