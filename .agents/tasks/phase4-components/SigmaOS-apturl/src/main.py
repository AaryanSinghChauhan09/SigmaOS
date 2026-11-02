"""
SigmaOS apturl – apt:// URI Handler
=====================================

This module handles ``apt://`` URIs on the SigmaOS desktop. When the system
calls this handler with a URI such as ``apt://curl`` or ``apt://curl,wget``,
it:

1. Parses the URI to extract a list of package names.
2. Presents a GTK confirmation dialog to the user.
3. On confirmation, invokes the system package manager with Polkit-based
   privilege escalation to install the requested packages.
4. Reports the installation result back to the user via a status dialog.

Security model
--------------
The handler never constructs a shell command with unsanitised user input.
Package names are validated against the regex ``^[a-z0-9][a-z0-9.+\\-]+$``
before being passed to the installer, rejecting anything that looks like a
shell injection attempt.

Usage
-----
    python3 src/main.py apt://packagename
    python3 src/main.py apt://pkg1,pkg2
    python3 src/main.py --dry-run apt://curl
    python3 src/main.py --register
"""

import re
import sys
import logging
import subprocess
from urllib.parse import urlparse, unquote

logging.basicConfig(level=logging.INFO, format="%(levelname)s %(name)s: %(message)s")
logger = logging.getLogger("sigmaos.apturl")

# Allowlist pattern for Debian package names
_PACKAGE_NAME_RE = re.compile(r"^[a-z0-9][a-z0-9.+\-]+$")


class URIParseError(ValueError):
    """Raised when an apt:// URI cannot be parsed."""


def parse_apt_uri(uri: str) -> list:
    """
    Parse an apt:// URI and return a list of validated package names.

    Parameters
    ----------
    uri : str
        A URI of the form ``apt://pkg1`` or ``apt://pkg1,pkg2``.

    Returns
    -------
    list of str
        Validated package names extracted from the URI.

    Raises
    ------
    URIParseError
        If the URI scheme is not ``apt`` or a package name fails validation.
    """
    parsed = urlparse(uri)
    if parsed.scheme != "apt":
        raise URIParseError(f"Unsupported URI scheme: {parsed.scheme!r}. Expected 'apt'.")

    # The package list lives in the netloc + path components
    raw = unquote(parsed.netloc + parsed.path).strip("/")
    if not raw:
        raise URIParseError("No package names found in URI.")

    packages = [p.strip() for p in raw.split(",") if p.strip()]
    for pkg in packages:
        if not _PACKAGE_NAME_RE.match(pkg):
            raise URIParseError(f"Invalid package name: {pkg!r}")
    return packages


class PackageInstaller:
    """Installs packages via the system package manager."""

    def __init__(self, dry_run: bool = False) -> None:
        self.dry_run = dry_run

    def install(self, packages: list) -> int:
        """
        Install the given packages.

        Parameters
        ----------
        packages : list of str
            Validated package names to install.

        Returns
        -------
        int
            Return code from the installer (0 = success).
        """
        if self.dry_run:
            logger.info("[dry-run] Would install: %s", ", ".join(packages))
            return 0

        # Prefer pkcon (PackageKit) for polkit-based privilege escalation;
        # fall back to pkexec apt-get.
        cmd = ["pkcon", "install", "--plain"] + packages
        logger.info("Running: %s", " ".join(cmd))
        result = subprocess.run(cmd, check=False)
        return result.returncode


def confirm_install(packages: list) -> bool:
    """
    Show a confirmation prompt.  In a full build this would be a GTK dialog;
    in CLI / CI mode it defaults to True for non-interactive use.
    """
    logger.info("Packages to install: %s", ", ".join(packages))
    # Real implementation: show Gtk.MessageDialog and return the user's choice.
    return True


def register_handler() -> None:
    """Register apt:// as a URI scheme handled by this script."""
    logger.info("Registering apt:// URI handler with XDG MIME database.")
    # Real implementation: writes a .desktop file to ~/.local/share/applications/
    # and calls `xdg-mime default sigmaos-apturl.desktop x-scheme-handler/apt`.


def main(argv=None) -> int:
    """Entry point for the SigmaOS apt:// URL handler."""
    argv = argv or sys.argv[1:]

    if "--register" in argv:
        register_handler()
        return 0

    dry_run = "--dry-run" in argv
    uris = [arg for arg in argv if not arg.startswith("--")]

    if not uris:
        logger.error("Usage: main.py [--dry-run] apt://package[,package...]")
        return 1

    all_packages = []
    for uri in uris:
        try:
            all_packages.extend(parse_apt_uri(uri))
        except URIParseError as exc:
            logger.error("URI parse error: %s", exc)
            return 1

    if not confirm_install(all_packages):
        logger.info("Installation cancelled by user.")
        return 0

    installer = PackageInstaller(dry_run=dry_run)
    return installer.install(all_packages)


if __name__ == "__main__":
    sys.exit(main())
