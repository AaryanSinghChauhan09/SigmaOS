"""
sigma_cli.py — backward-compat shim.
Real implementation lives in sigma_cli/ package.
"""

from .sigma_cli._ansi import *  # noqa
from .sigma_cli.ok import *  # noqa
from .sigma_cli.warn import *  # noqa
from .sigma_cli.err import *  # noqa
from .sigma_cli.info import *  # noqa
from .sigma_cli.hdr import *  # noqa
from .sigma_cli.cmd_boot import *  # noqa
from .sigma_cli.cmd_status import *  # noqa
from .sigma_cli.cmd_pdf import *  # noqa
from .sigma_cli.cmd_capture import *  # noqa
from .sigma_cli.cmd_module import *  # noqa
from .sigma_cli.cmd_security import *  # noqa
from .sigma_cli.cmd_browser import *  # noqa
from .sigma_cli.cmd_convert import *  # noqa
from .sigma_cli.cmd_boot_profile import *  # noqa
from .sigma_cli.cmd_events import *  # noqa
from .sigma_cli.cmd_perf import *  # noqa
from .sigma_cli.cmd_pkg import *  # noqa
from .sigma_cli.cmd_repair import *  # noqa
from .sigma_cli.cmd_automate import *  # noqa
from .sigma_cli.cmd_ai import *  # noqa
from .sigma_cli.cmd_customize import *  # noqa
from .sigma_cli.cmd_apex import *  # noqa
from .sigma_cli.cmd_crush import *  # noqa
from .sigma_cli.interactive_shell import *  # noqa
from .sigma_cli.build_parser import *  # noqa
from .sigma_cli.main import *  # noqa

__all__ = ['_ansi', 'ok', 'warn', 'err', 'info', 'hdr', 'cmd_boot', 'cmd_status', 'cmd_pdf', 'cmd_capture', 'cmd_module', 'cmd_security', 'cmd_browser', 'cmd_convert', 'cmd_boot_profile', 'cmd_events', 'cmd_perf', 'cmd_pkg', 'cmd_repair', 'cmd_automate', 'cmd_ai', 'cmd_customize', 'cmd_apex', 'cmd_crush', 'interactive_shell', 'build_parser', 'main']

"""Auto-generated package __init__.py"""
from ._ansi import *  # noqa: F401, F403
from .ok import *  # noqa: F401, F403
from .warn import *  # noqa: F401, F403
from .err import *  # noqa: F401, F403
from .info import *  # noqa: F401, F403
from .hdr import *  # noqa: F401, F403
from .cmd_boot import *  # noqa: F401, F403
from .cmd_status import *  # noqa: F401, F403
from .cmd_pdf import *  # noqa: F401, F403
from .cmd_capture import *  # noqa: F401, F403
from .cmd_module import *  # noqa: F401, F403
from .cmd_security import *  # noqa: F401, F403
from .cmd_browser import *  # noqa: F401, F403
from .cmd_convert import *  # noqa: F401, F403
from .cmd_boot_profile import *  # noqa: F401, F403
from .cmd_events import *  # noqa: F401, F403
from .cmd_perf import *  # noqa: F401, F403
from .cmd_pkg import *  # noqa: F401, F403
from .cmd_repair import *  # noqa: F401, F403
from .cmd_automate import *  # noqa: F401, F403
from .cmd_ai import *  # noqa: F401, F403
from .cmd_customize import *  # noqa: F401, F403
from .cmd_apex import *  # noqa: F401, F403
from .cmd_crush import *  # noqa: F401, F403
from .interactive_shell import *  # noqa: F401, F403
from .build_parser import *  # noqa: F401, F403
from .main import *  # noqa: F401, F403
