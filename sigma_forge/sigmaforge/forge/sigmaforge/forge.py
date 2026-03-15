# Generated method: SigmaForge.forge
import os
import sys
import argparse
from sigma_forge.forge_dispatcher import forge
from sigma_forge.list_templates import list_templates

class SigmaForge:
    def forge(self, template_type, name, output_dir='userland/apps'):
        return forge(template_type, name, output_dir)