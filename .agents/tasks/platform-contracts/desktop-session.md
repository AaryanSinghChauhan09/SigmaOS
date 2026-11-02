# Desktop Session Contract

> **Shared SigmaOS platform contract.** Define the desktop session lifecycle: startup, lock, unlock, logout, suspend, resume, save, and restore — shared across Linux and BSD.

---

## Interface

SessionManager { start(user) -> Session; lock() -> (); unlock() -> (); suspend() -> (); resume() -> (); save() -> SessionState; restore(state) -> Session; end() -> (); }

## Invariants

- Session save/restore: on logout or suspend, session state (open windows, workspace layout, running apps) is saved; on next login, restored.
- Lock: screen locks after idle timeout or explicit lock; requires auth to unlock.
- Suspend: all services notified (pre-suspend hook); on resume, post-resume hook runs.
- Desktop/service separation: the session manager manages lifecycle; the desktop shell manages UI.

## Lifecycle

Login -> Session.start -> Desktop.start -> [Lock -> Unlock]* -> Logout/Suspend/Shutdown -> Session.save -> Session.end

## Linux Adapter

systemd-logind (seat/session management); loginctl lock/unlock/suspend; D-Bus session bus.

## BSD Adapter

acpid/apmd (suspend); X display manager for login; rc.d for services.

## Failure Modes and Recovery

Desktop shell crash (fallback to TTY or minimal shell), auth failure on unlock (retry with throttling), restore failure (start fresh session + log).

## Open Questions

Multi-seat support — current scope: single-seat only; multi-seat planned post-stable.
