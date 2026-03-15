"""
email_agent.py — backward-compat shim.
Real implementation lives in email_agent/ package.
"""

from .email_agent.EmailAgentPro import *  # noqa

__all__ = ['EmailAgentPro']

"""Auto-generated package __init__.py"""
from .emailagentpro import *  # noqa: F401, F403
