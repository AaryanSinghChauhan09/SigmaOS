1


SigmaOS replaces traditional background daemons and systemd services with **Autonomous Agents**. These agents operate with self-healing capabilities, orchestrating system quotas, enforcing governance policies, and dynamically adjusting the system based on the Context Manager.


1


The agent hierarchy is modularised to prevent tight coupling:


1



1



1



1



1



1



1


Developers can add new agents by subclassing `AgentBase`. All agents must communicate exclusively via the `SovereignEventBus` or through the `ContextManager` to avoid hardcoded dependency coupling.

