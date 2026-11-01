"""SigmaOS-daemon-apt: APT package manager daemon with D-Bus interface.

Provides a privilege-separated D-Bus service for performing APT package
management operations (install, remove, upgrade, search) on behalf of
unprivileged callers, with PolicyKit-based authorisation.
"""

from __future__ import annotations

import enum
import logging
import os
import subprocess
import sys
from dataclasses import dataclass, field
from typing import Callable

# ---------------------------------------------------------------------------
# Logging setup
# ---------------------------------------------------------------------------

logging.basicConfig(
    level=logging.INFO,
    format="%(asctime)s [%(levelname)s] sigmaos-daemon-apt: %(message)s",
    stream=sys.stderr,
)
log = logging.getLogger(__name__)

# ---------------------------------------------------------------------------
# Transaction model
# ---------------------------------------------------------------------------

DBUS_SERVICE_NAME = "org.sigmaos.AptDaemon"
DBUS_OBJECT_PATH = "/org/sigmaos/AptDaemon"
APT_LOCK_FILE = "/var/lib/dpkg/lock-frontend"


class TransactionState(enum.Enum):
    """Lifecycle states for a package management transaction."""

    PENDING = "pending"
    RUNNING = "running"
    FINISHED = "finished"
    FAILED = "failed"
    CANCELLED = "cancelled"


@dataclass
class Transaction:
    """Represents a single APT operation request."""

    tid: str
    operation: str  # "install" | "remove" | "upgrade" | "search"
    packages: list[str]
    state: TransactionState = TransactionState.PENDING
    progress: int = 0  # 0-100
    error_message: str = ""
    output_lines: list[str] = field(default_factory=list)

    def as_dict(self) -> dict:
        """Return a serialisable representation of the transaction."""
        return {
            "tid": self.tid,
            "operation": self.operation,
            "packages": self.packages,
            "state": self.state.value,
            "progress": self.progress,
            "error_message": self.error_message,
        }


# ---------------------------------------------------------------------------
# PolicyKit authorisation stub
# ---------------------------------------------------------------------------


def check_authorization(action_id: str, caller_uid: int) -> bool:
    """Check whether *caller_uid* is authorised for *action_id*.

    In production this would call the PolicyKit D-Bus service. Here we
    grant root and members of the sudo group, and deny everyone else.
    """
    if caller_uid == 0:
        log.debug("Authorization granted to root for action %s", action_id)
        return True
    try:
        import grp  # pylint: disable=import-outside-toplevel
        sudo_gid = grp.getgrnam("sudo").gr_gid
        import pwd  # pylint: disable=import-outside-toplevel
        user_groups = os.getgrouplist(pwd.getpwuid(caller_uid).pw_name, sudo_gid)
        if sudo_gid in user_groups:
            log.debug(
                "Authorization granted to uid %d for action %s (sudo group)",
                caller_uid, action_id,
            )
            return True
    except (KeyError, AttributeError, OSError):
        pass
    log.warning(
        "Authorization denied for uid %d, action %s", caller_uid, action_id
    )
    return False


# ---------------------------------------------------------------------------
# APT operations
# ---------------------------------------------------------------------------


def _run_apt(
    args: list[str],
    progress_callback: Callable[[int], None] | None = None,
) -> tuple[bool, list[str]]:
    """Run an APT command and return (success, output_lines)."""
    cmd = ["apt-get", "-y", "--no-install-recommends"] + args
    log.info("Running: %s", " ".join(cmd))
    output: list[str] = []
    try:
        proc = subprocess.Popen(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
        )
        assert proc.stdout is not None
        for line in proc.stdout:
            stripped = line.rstrip()
            output.append(stripped)
            log.debug("apt> %s", stripped)
        proc.wait()
        return proc.returncode == 0, output
    except FileNotFoundError:
        msg = "apt-get not found; running in non-APT environment"
        log.warning(msg)
        return False, [msg]
    except OSError as exc:
        log.error("Failed to run apt-get: %s", exc)
        return False, [str(exc)]


def apt_install(packages: list[str]) -> tuple[bool, list[str]]:
    """Install the given packages via APT."""
    return _run_apt(["install"] + packages)


def apt_remove(packages: list[str]) -> tuple[bool, list[str]]:
    """Remove the given packages via APT."""
    return _run_apt(["remove"] + packages)


def apt_upgrade() -> tuple[bool, list[str]]:
    """Run a full system upgrade via APT."""
    ok1, out1 = _run_apt(["update"])
    ok2, out2 = _run_apt(["upgrade"])
    return ok1 and ok2, out1 + out2


def apt_search(query: str) -> list[str]:
    """Search the APT cache for packages matching *query*."""
    try:
        result = subprocess.run(
            ["apt-cache", "search", "--names-only", query],
            capture_output=True, text=True, check=False,
        )
        return result.stdout.strip().splitlines()
    except FileNotFoundError:
        return ["apt-cache not available in this environment"]


# ---------------------------------------------------------------------------
# Daemon / D-Bus stub
# ---------------------------------------------------------------------------


class AptDaemon:
    """Stub implementation of the SigmaOS APT daemon.

    In a production deployment this class would register as a D-Bus service
    on the system bus using the `dbus-python` or `dasbus` library.
    """

    def __init__(self) -> None:
        self._transactions: dict[str, Transaction] = {}
        self._next_tid = 1

    # -- D-Bus method stubs --------------------------------------------------

    def install_packages(
        self, packages: list[str], caller_uid: int = 0
    ) -> str:
        """Begin an install transaction; return the transaction ID."""
        if not check_authorization("org.sigmaos.apt.install", caller_uid):
            raise PermissionError("Not authorised to install packages")
        return self._submit(Transaction(
            tid=self._new_tid(),
            operation="install",
            packages=packages,
        ))

    def remove_packages(
        self, packages: list[str], caller_uid: int = 0
    ) -> str:
        """Begin a removal transaction; return the transaction ID."""
        if not check_authorization("org.sigmaos.apt.remove", caller_uid):
            raise PermissionError("Not authorised to remove packages")
        return self._submit(Transaction(
            tid=self._new_tid(),
            operation="remove",
            packages=packages,
        ))

    def upgrade_system(self, caller_uid: int = 0) -> str:
        """Begin a full system upgrade; return the transaction ID."""
        if not check_authorization("org.sigmaos.apt.upgrade", caller_uid):
            raise PermissionError("Not authorised to upgrade the system")
        return self._submit(Transaction(
            tid=self._new_tid(),
            operation="upgrade",
            packages=[],
        ))

    def search_packages(self, query: str) -> list[str]:
        """Search for packages matching *query* (no authorisation required)."""
        return apt_search(query)

    def get_transaction(self, tid: str) -> dict:
        """Return the state of a transaction as a dict."""
        txn = self._transactions.get(tid)
        if txn is None:
            raise KeyError(f"Unknown transaction ID: {tid}")
        return txn.as_dict()

    # -- Internal helpers ----------------------------------------------------

    def _new_tid(self) -> str:
        tid = f"txn-{self._next_tid:04d}"
        self._next_tid += 1
        return tid

    def _submit(self, txn: Transaction) -> str:
        """Register and execute a transaction synchronously."""
        self._transactions[txn.tid] = txn
        txn.state = TransactionState.RUNNING
        log.info("Starting transaction %s: %s %s", txn.tid, txn.operation, txn.packages)

        if txn.operation == "install":
            ok, out = apt_install(txn.packages)
        elif txn.operation == "remove":
            ok, out = apt_remove(txn.packages)
        elif txn.operation == "upgrade":
            ok, out = apt_upgrade()
        else:
            ok, out = False, [f"Unknown operation: {txn.operation}"]

        txn.output_lines = out
        txn.state = TransactionState.FINISHED if ok else TransactionState.FAILED
        txn.progress = 100
        if not ok:
            txn.error_message = out[-1] if out else "Unknown error"
            log.error("Transaction %s failed: %s", txn.tid, txn.error_message)
        else:
            log.info("Transaction %s completed successfully", txn.tid)
        return txn.tid


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------


def main(argv: list[str] | None = None) -> int:
    """Entry point for the SigmaOS APT daemon."""
    if argv is None:
        argv = sys.argv[1:]

    daemon = AptDaemon()

    if "--test-search" in argv:
        idx = argv.index("--test-search")
        query = argv[idx + 1] if idx + 1 < len(argv) else "python3"
        results = daemon.search_packages(query)
        for line in results[:20]:
            print(line)
        return 0

    if "--version" in argv:
        print("sigmaos-daemon-apt 1.0.0")
        return 0

    log.info("SigmaOS APT daemon starting on %s", DBUS_SERVICE_NAME)
    log.info(
        "D-Bus object path: %s (D-Bus runtime not available in stub mode)",
        DBUS_OBJECT_PATH,
    )
    log.info("Daemon ready. Waiting for D-Bus method calls…")

    # In a real deployment, enter the GLib/asyncio main loop here.
    # For now, emit a helpful message and exit cleanly.
    print(
        "sigmaos-daemon-apt: D-Bus main loop not started (stub mode).\n"
        "Pass --test-search <query> to exercise the search path."
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
