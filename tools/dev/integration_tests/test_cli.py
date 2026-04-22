import subprocess
import pytest

def test_sigma_cli_help():
    result = subprocess.run(["python3", "sigma_cli.py", "--help"], capture_output=True, text=True)
    assert result.returncode == 0
    assert "SigmaOS Unified CLI" in result.stdout

def test_sigma_cli_telemetry():
    # Test telemetry command
    result = subprocess.run(["python3", "sigma_cli.py", "telemetry"], capture_output=True, text=True)
    assert result.returncode == 0
    # Assuming the local server is NOT running during CI, it should output an error or simulated text
    assert "telemetry" in result.stdout.lower() or "error" in result.stdout.lower()

def test_sigma_cli_automation():
    result = subprocess.run(["python3", "sigma_cli.py", "auto", "cache_flush"], capture_output=True, text=True)
    assert result.returncode == 0
    assert "Triggering automation: cache_flush" in result.stdout
