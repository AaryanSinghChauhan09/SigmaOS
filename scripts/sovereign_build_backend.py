"""
Sovereign Python PEP 517 Zero-Dependency Build Backend for SigmaOS.
Provides independence from setuptools, flit, or external build frameworks.
"""

import os
import tarfile
import zipfile
import glob

def get_requires_for_build_wheel(config_settings=None):
    return []

def get_requires_for_build_sdist(config_settings=None):
    return []

def build_wheel(wheel_directory, config_settings=None, metadata_directory=None):
    """Builds a zero-dependency .whl package for SigmaOS."""
    wheel_name = "sigmaos_core-4.2.0-py3-none-any.whl"
    wheel_path = os.path.join(wheel_directory, wheel_name)

    with zipfile.ZipFile(wheel_path, "w", zipfile.ZIP_DEFLATED) as zip_file:
        # Add metadata files
        dist_info = "sigmaos_core-4.2.0.dist-info"
        zip_file.writestr(
            f"{dist_info}/METADATA",
            "Metadata-Version: 2.1\nName: sigmaos-core\nVersion: 4.2.0\nSummary: SigmaOS Sovereign Ecosystem\n"
        )
        zip_file.writestr(f"{dist_info}/WHEEL", "Wheel-Version: 1.0\nGenerator: sovereign_build_backend\nRoot-Is-Purelib: true\nTag: py3-none-any\n")

        # Include python scripts in the repository
        for py_file in glob.glob("scripts/*.py"):
            zip_file.write(py_file, os.path.relpath(py_file))

    return wheel_name

def build_sdist(sdist_directory, config_settings=None):
    """Builds a zero-dependency sdist tarball for SigmaOS."""
    sdist_name = "sigmaos_core-4.2.0.tar.gz"
    sdist_path = os.path.join(sdist_directory, sdist_name)

    with tarfile.open(sdist_path, "w:gz") as tar:
        tar.add("pyproject.toml", arcname="sigmaos_core-4.2.0/pyproject.toml")
        if os.path.exists("scripts"):
            tar.add("scripts", arcname="sigmaos_core-4.2.0/scripts")

    return sdist_name
