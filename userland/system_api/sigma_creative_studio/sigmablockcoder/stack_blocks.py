"""
Auto-split from userland\system_api\sigma_creative_studio.py — SigmaBlockCoder.stack_blocks
"""



class SigmaBlockCoder:
    def stack_blocks(self, sprite_name, block_sequence):
        """
            Stacks a list of visual code blocks into an executable script.
            block_sequence: e.g., [('when_flag_clicked',), ('repeat_loop', 10), ('move_steps', 50)]
            """
        script = {'sprite': sprite_name, 'blocks': block_sequence}
        self.scripts.append(script)
        block_str = ' >> '.join([b[0] if isinstance(b, tuple) else str(b) for b in block_sequence])
        return f"BlockCoder: Script [{block_str}] compiled for '{sprite_name}'."
