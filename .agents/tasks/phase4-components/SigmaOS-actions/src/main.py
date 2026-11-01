"""
SigmaOS Actions – File Manager Context Menu Extension
=====================================================

This module implements the context menu action framework for the SigmaOS file
manager. Actions are discovered from descriptor files (``*.action``) located in
``~/.local/share/sigmaos/actions/`` and the system directory
``/usr/share/sigmaos/actions/``.

Each action descriptor specifies:
- The display name and icon shown in the context menu.
- One or more mime-type or file-extension patterns that determine when the action
  is visible.
- A command or Python callable to execute when the user triggers the action.

Architecture
------------
- ActionRegistry: scans descriptor directories and maintains an ordered list of
  available actions.
- ActionFilter: given a list of selected file paths, returns the subset of
  registered actions whose selection criteria match.
- ActionRunner: executes the command or callable associated with a triggered
  action, substituting ``%F`` / ``%f`` placeholders with the selected paths.

Usage
-----
    python3 src/main.py --list
    python3 src/main.py --run <action-id> /path/to/file1 /path/to/file2
"""

import os
import sys
import logging
import subprocess
from typing import List

logging.basicConfig(level=logging.INFO, format="%(levelname)s %(name)s: %(message)s")
logger = logging.getLogger("sigmaos.actions")


class ActionDescriptor:
    """Represents a single context menu action loaded from a descriptor file."""

    def __init__(
        self,
        action_id: str,
        name: str,
        command: str,
        mimetypes: List[str] = None,
        extensions: List[str] = None,
    ) -> None:
        self.action_id = action_id
        self.name = name
        self.command = command
        self.mimetypes = mimetypes or []
        self.extensions = extensions or []

    def matches(self, file_path: str) -> bool:
        """Return True if this action applies to the given file path."""
        _, ext = os.path.splitext(file_path)
        if self.extensions and ext.lower() in self.extensions:
            return True
        # When no filters are defined the action is universal.
        return not self.mimetypes and not self.extensions

    def __repr__(self) -> str:
        return f"ActionDescriptor(id={self.action_id!r}, name={self.name!r})"


class ActionRegistry:
    """Discovers and stores all available context menu actions."""

    SYSTEM_DIR = "/usr/share/sigmaos/actions"
    USER_DIR = os.path.expanduser("~/.local/share/sigmaos/actions")

    def __init__(self) -> None:
        self._actions: List[ActionDescriptor] = []

    def load(self) -> None:
        """Load action descriptors from system and user directories."""
        for directory in (self.SYSTEM_DIR, self.USER_DIR):
            if not os.path.isdir(directory):
                logger.debug("Action directory not found, skipping: %s", directory)
                continue
            for filename in sorted(os.listdir(directory)):
                if filename.endswith(".action"):
                    self._load_file(os.path.join(directory, filename))
        logger.info("Loaded %d action(s).", len(self._actions))

    def _load_file(self, path: str) -> None:
        """Parse a single .action descriptor file (INI-style, simplified)."""
        # Real implementation uses configparser; placeholder adds a stub action.
        action_id = os.path.splitext(os.path.basename(path))[0]
        logger.debug("Loading action descriptor: %s", path)
        descriptor = ActionDescriptor(
            action_id=action_id,
            name=action_id.replace("-", " ").title(),
            command=f"echo 'Running {action_id} on %F'",
        )
        self._actions.append(descriptor)

    @property
    def actions(self) -> List[ActionDescriptor]:
        return list(self._actions)


class ActionFilter:
    """Filters actions based on the currently selected files."""

    def __init__(self, registry: ActionRegistry) -> None:
        self._registry = registry

    def applicable(self, selected_paths: List[str]) -> List[ActionDescriptor]:
        """Return actions that match all files in the selection."""
        return [
            action
            for action in self._registry.actions
            if all(action.matches(p) for p in selected_paths)
        ]


class ActionRunner:
    """Executes a context menu action against a list of selected file paths."""

    def run(self, action: ActionDescriptor, selected_paths: List[str]) -> int:
        """Run the action command, replacing %F with the quoted path list."""
        paths_str = " ".join(f'"{p}"' for p in selected_paths)
        command = action.command.replace("%F", paths_str).replace(
            "%f", f'"{selected_paths[0]}"' if selected_paths else ""
        )
        logger.info("Executing action '%s': %s", action.name, command)
        result = subprocess.run(command, shell=True, check=False)
        return result.returncode


def main(argv=None) -> int:
    """Entry point for the SigmaOS actions CLI helper."""
    argv = argv or sys.argv[1:]

    registry = ActionRegistry()
    registry.load()

    if "--list" in argv:
        for action in registry.actions:
            print(f"  {action.action_id:30s} {action.name}")
        return 0

    if "--run" in argv:
        idx = argv.index("--run")
        if idx + 1 >= len(argv):
            logger.error("--run requires an action-id argument.")
            return 1
        action_id = argv[idx + 1]
        selected_paths = argv[idx + 2:]
        match = next(
            (a for a in registry.actions if a.action_id == action_id), None
        )
        if match is None:
            logger.error("Action '%s' not found.", action_id)
            return 1
        runner = ActionRunner()
        return runner.run(match, selected_paths)

    logger.info("Use --list to show available actions or --run <id> <files...> to execute one.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
