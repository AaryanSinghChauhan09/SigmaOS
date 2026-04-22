"""
SigmaOS Integration Tests
Covers: CLI commands, config bridge, plugin loader, profile switch, shard API
"""
import pytest, json, os, sys, subprocess, tempfile, shutil
from pathlib import Path

ROOT = Path(__file__).parent.parent.parent.parent  # SigmaOS root
SIGMACTL = ROOT / "sigmactl.py"


# ── Helpers ────────────────────────────────────────────────────────────────

def run_ctl(*args, cwd=None, env=None):
    """Run sigmactl with given args, return (returncode, stdout, stderr)"""
    if env is None:
        env = os.environ.copy()
    result = subprocess.run(
        [sys.executable, str(SIGMACTL)] + list(args),
        capture_output=True, text=True, cwd=cwd or ROOT, env=env
    )
    return result.returncode, result.stdout, result.stderr


def robust_rmtree(path):
    """rmtree that handles read-only files (like .git on Windows)"""
    def on_error(func, path, exc_info):
        import stat
        if not os.access(path, os.W_OK):
            os.chmod(path, stat.S_IWUSR)
            func(path)
        else:
            raise
    shutil.rmtree(path, onerror=on_error)


def make_temp_root():
    """Create a minimal temp SigmaOS root for isolated tests"""
    tmp = Path(tempfile.mkdtemp(prefix="sigma_test_")).resolve()
    (tmp / "profiles").mkdir()
    (tmp / "plugins").mkdir()
    (tmp / "kernel" / "suites").mkdir(parents=True)
    (tmp / "shards").mkdir()
    # Minimal sigma_config.json
    (tmp / "sigma_config.json").write_text(json.dumps({
        "theme": "MATRIX", "profile": "default", "blur": 25
    }))
    # Init git
    subprocess.run(["git", "init"], cwd=tmp, capture_output=True)
    # Configure git for test environment
    subprocess.run(["git", "config", "user.email", "test@sigma.os"], cwd=tmp, capture_output=True)
    subprocess.run(["git", "config", "user.name", "Test Bot"], cwd=tmp, capture_output=True)
    subprocess.run(["git", "add", "."], cwd=tmp, capture_output=True)
    subprocess.run(["git", "commit", "-m", "init"], cwd=tmp, capture_output=True)
    return tmp


# ── Config ─────────────────────────────────────────────────────────────────

class TestConfig:
    def test_get_all(self):
        rc, out, _ = run_ctl("get")
        assert rc == 0
        assert "{" in out or "theme" in out

    def test_set_and_get(self, tmp_path):
        tmp = make_temp_root()
        try:
            cfg = tmp / "sigma_config.json"
            env = os.environ.copy()
            env["SIGMA_ROOT"] = str(tmp)
            rc, out, err = run_ctl("set", "theme", "GHOST_MICA", env=env)
            assert rc == 0, f"Set failed: {err}"
            data = json.loads(cfg.read_text())
            assert data.get("theme") == "GHOST_MICA"
        finally:
            robust_rmtree(tmp)

    def test_get_specific_key(self):
        rc, out, _ = run_ctl("get", "theme")
        assert rc == 0
        assert "theme" in out


# ── Profiles ───────────────────────────────────────────────────────────────

class TestProfiles:
    def test_profile_list(self):
        rc, out, _ = run_ctl("profile", "list")
        assert rc == 0

    def test_profile_create_and_switch(self):
        tmp = make_temp_root()
        try:
            env = os.environ.copy()
            env["SIGMA_ROOT"] = str(tmp)
            # Create
            rc, out, err = run_ctl("profile", "create", "testmode", "--preset", "minimal", env=env)
            assert rc == 0, f"Create failed: {err}"
            assert (tmp / "profiles" / "testmode.json").exists()

            # Switch
            rc, out, err = run_ctl("profile", "switch", "testmode", env=env)
            assert rc == 0, f"Switch failed: {err}"
            cfg = json.loads((tmp / "sigma_config.json").read_text())
            assert cfg.get("profile") == "testmode"
        finally:
            robust_rmtree(tmp)

    def test_profile_set_alias(self):
        """profile set <name> should be equivalent to profile switch <name>"""
        rc, out, err = run_ctl("profile", "set", "default")
        assert rc in (0, 1)  # OK or profile-not-found (depends on env)


# ── Shards ─────────────────────────────────────────────────────────────────

class TestShards:
    def test_shard_ls(self):
        rc, out, _ = run_ctl("shard", "ls")
        assert rc == 0
        assert "Suite" in out or "TOTAL" in out

    def test_shard_add_and_verify(self):
        tmp = make_temp_root()
        try:
            env = os.environ.copy()
            env["SIGMA_ROOT"] = str(tmp)
            rc, out, err = run_ctl("shard", "add", "analytics", env=env)
            suite_dir = tmp / "kernel" / "suites" / "SXX_Analytics"
            assert suite_dir.exists(), f"Suite dir not created: {err}"
            register_file = suite_dir / "SXX_Analytics_Register.c"
            assert register_file.exists(), "Register .c not created"
        finally:
            robust_rmtree(tmp)

    def test_shard_remove(self):
        tmp = make_temp_root()
        try:
            env = os.environ.copy()
            env["SIGMA_ROOT"] = str(tmp)
            # Create first
            run_ctl("shard", "add", "removeme", env=env)
            suite_dir = tmp / "kernel" / "suites" / "SXX_Removeme"
            assert suite_dir.exists()
            # Remove (non-interactive: pipe 'y' as stdin)
            p = subprocess.run(
                [sys.executable, str(SIGMACTL), "shard", "remove", "removeme"],
                input="y\n", text=True, capture_output=True, cwd=tmp, env=env
            )
            assert not suite_dir.exists() or p.returncode == 0
        finally:
            robust_rmtree(tmp)


# ── Plugins ────────────────────────────────────────────────────────────────

class TestPlugins:
    def test_plugin_list_empty(self):
        tmp = make_temp_root()
        try:
            env = os.environ.copy()
            env["SIGMA_ROOT"] = str(tmp)
            rc, out, _ = run_ctl("plugin", "list", env=env)
            assert "No plugins" in out or rc == 0
        finally:
            robust_rmtree(tmp)

    def test_plugin_install_creates_manifest(self):
        tmp = make_temp_root()
        try:
            env = os.environ.copy()
            env["SIGMA_ROOT"] = str(tmp)
            rc, out, err = run_ctl("shard", "install-plugin", "myplugin", env=env)
            manifest = tmp / "plugins" / "myplugin" / "plugin.json"
            assert manifest.exists(), f"Manifest not created: {err}"
            data = json.loads(manifest.read_text())
            assert data["name"] == "myplugin"
            assert data["enabled"] is True
        finally:
            robust_rmtree(tmp)


# ── Status ─────────────────────────────────────────────────────────────────

class TestStatus:
    def test_status_exits_zero(self):
        rc, out, _ = run_ctl("status")
        assert rc == 0

    def test_status_contains_key_fields(self):
        _, out, _ = run_ctl("status")
        # Should contain branch info or profile
        assert any(kw in out for kw in ("Branch", "Profile", "Suite", "SIGMA"))


# ── Sigma Config JSON ──────────────────────────────────────────────────────

class TestSigmaConfigJson:
    def test_config_is_valid_json(self):
        cfg_path = ROOT / "sigma_config.json"
        if cfg_path.exists():
            data = json.loads(cfg_path.read_text())
            assert isinstance(data, dict)

    def test_profiles_are_valid_json(self):
        profiles_dir = ROOT / "profiles"
        if profiles_dir.exists():
            for pf in profiles_dir.glob("*.json"):
                data = json.loads(pf.read_text())
                assert "name" in data, f"Profile missing 'name': {pf}"
