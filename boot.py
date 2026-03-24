"""
Primary SigmaOS boot entrypoint.
"""

from __future__ import annotations

from launcher import main as launcher_main


if __name__ == "__main__":
    raise SystemExit(launcher_main())
