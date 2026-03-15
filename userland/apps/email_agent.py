"""
email_agent.py — backward-compat shim.
Real implementation lives in email_agent/ package.
"""

from email_agent.EmailAgentPro import *  # noqa

__all__ = ['EmailAgentPro']
