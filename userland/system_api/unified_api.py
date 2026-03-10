"""
SigmaOS Unified Syscall & API Layer
======================================
USP: Write-once, deploy-everywhere: one unified API across desktop, mobile, cloud.

Competition comparison:
  Windows → Win32 + WinRT + .NET; three overlapping stacks, legacy baggage
  macOS   → POSIX + Cocoa + SwiftUI; Apple-only ABI
  Linux   → POSIX syscalls; no unified high-level API
  Android → Dalvik/ART + JNI; different from Linux desktop
  SigmaOS → SigmaAPI: single surface covering all targets; POSIX-compatible

Architecture:
  ┌──────────────────────────────────────────────────────────┐
  │  SigmaAPI (High-Level)  ← developers call this          │
  │     └── Target Router   ← selects transport per env     │
  │           ├── SyscallBridge  (Linux syscalls)            │
  │           ├── WasmRuntime    (universal binary)         │
  │           ├── CloudRPC       (serverless edge)          │
  │           └── MobileBridge   (Android/iOS compat)       │
  └──────────────────────────────────────────────────────────┘

Core innovations:
  1. Universal Binary        — single .sigma binary runs on all targets
  2. SyscallInterceptor      — audits/transforms every OS syscall
  3. APIVersionRouter        — forward-compatible; old apps run on new API
  4. CrossPlatformShim       — translates Win32/macOS/Android calls → SigmaAPI
  5. FunctionMesh            — serverless-style function deployment in kernel space
  6. ABI Compatibility Table — tracks every registered capability
"""
import time
import uuid
import sys
from dataclasses import dataclass, field
from enum import Enum, auto


class Target(Enum):
    DESKTOP    = "desktop"
    MOBILE     = "mobile"
    CLOUD      = "cloud"
    WASM       = "wasm"
    EMBEDDED   = "embedded"


class SyscallCategory(Enum):
    FILESYSTEM = "fs"
    PROCESS    = "proc"
    NETWORK    = "net"
    MEMORY     = "mem"
    DEVICE     = "dev"
    SECURITY   = "sec"
    UI         = "ui"
    IPC        = "ipc"


# Platform shim mappings: foreign_call → SigmaAPI canonical name
_WIN32_SHIM: dict[str, str] = {
    "CreateFile":       "sigma.fs.open",
    "ReadFile":         "sigma.fs.read",
    "WriteFile":        "sigma.fs.write",
    "CloseHandle":      "sigma.fs.close",
    "CreateProcess":    "sigma.proc.spawn",
    "VirtualAlloc":     "sigma.mem.alloc",
    "WSASocket":        "sigma.net.socket",
    "MessageBox":       "sigma.ui.dialog",
}
_MACOS_SHIM: dict[str, str] = {
    "open":             "sigma.fs.open",
    "read":             "sigma.fs.read",
    "write":            "sigma.fs.write",
    "fork":             "sigma.proc.fork",
    "objc_msgSend":     "sigma.ui.send_message",
    "NSURLSession":     "sigma.net.http_request",
    "AudioUnitInitialize": "sigma.dev.audio_init",
}
_ANDROID_SHIM: dict[str, str] = {
    "android.content.Context.getSystemService": "sigma.dev.get_service",
    "android.net.ConnectivityManager":          "sigma.net.connectivity",
    "android.media.MediaPlayer":                "sigma.dev.media_play",
    "android.Manifest.permission.CAMERA":       "sigma.dev.camera",
}


@dataclass
class SyscallRecord:
    call_id:    str
    name:       str
    category:   SyscallCategory
    target:     Target
    args:       dict
    result:     str = ""
    duration_us:float = 0.0
    audited:    bool  = True
    timestamp:  str   = ""


@dataclass
class APICapability:
    name:        str   # e.g. "sigma.fs.read"
    version:     str
    targets:     list[Target]
    deprecated:  bool = False
    replacement: str  = ""


@dataclass
class FunctionMesh:
    func_id:    str
    name:       str
    target:     Target
    code_ref:   str    # module.function reference
    deployed:   bool = False
    invocations:int  = 0


class SigmaUnifiedAPI:
    """
    SigmaOS Unified Syscall & API Layer.
    Single developer surface; adapts to execution target automatically.
    """

    VERSION = "2.0.0"

    def __init__(self):
        self._capabilities: dict[str, APICapability]  = {}
        self._syscall_log:  list[SyscallRecord]       = []
        self._functions:    dict[str, FunctionMesh]   = {}
        self._interceptors: list[callable]            = []
        self._compat_table: dict[str, str]            = {}  # foreign → canonical
        self._stats = {
            "calls": 0, "shim_calls": 0, "wasm_calls": 0,
            "intercepted": 0, "version_mismatches": 0,
        }
        self._load_default_capabilities()
        self._load_shims()

    # ── Capability Registry ──────────────────────────────────────────────────

    def _load_default_capabilities(self):
        caps = [
            APICapability("sigma.fs.open",    "2.0", list(Target), False),
            APICapability("sigma.fs.read",    "2.0", list(Target), False),
            APICapability("sigma.fs.write",   "2.0", list(Target), False),
            APICapability("sigma.fs.close",   "2.0", list(Target), False),
            APICapability("sigma.fs.snapshot","2.0", [Target.DESKTOP, Target.MOBILE], False),
            APICapability("sigma.proc.spawn", "2.0", list(Target), False),
            APICapability("sigma.proc.fork",  "2.0", [Target.DESKTOP, Target.CLOUD], False),
            APICapability("sigma.mem.alloc",  "2.0", list(Target), False),
            APICapability("sigma.mem.peer_borrow","2.0",[Target.DESKTOP, Target.MOBILE], False),
            APICapability("sigma.net.socket", "2.0", list(Target), False),
            APICapability("sigma.net.http_request","2.0",list(Target), False),
            APICapability("sigma.net.mesh_send","2.0",[Target.DESKTOP, Target.MOBILE], False),
            APICapability("sigma.dev.audio_init","2.0",list(Target), False),
            APICapability("sigma.dev.camera", "2.0", [Target.DESKTOP, Target.MOBILE], False),
            APICapability("sigma.ui.dialog",  "2.0", [Target.DESKTOP, Target.MOBILE], False),
            APICapability("sigma.ui.render",  "2.0", list(Target), False),
            APICapability("sigma.sec.encrypt","2.0", list(Target), False),
            APICapability("sigma.sec.verify_identity","2.0",list(Target), False),
            APICapability("sigma.ipc.send",   "2.0", list(Target), False),
            # Legacy compat (deprecated but still routed)
            APICapability("sigma.fs.read_v1", "1.0", [Target.DESKTOP], True, "sigma.fs.read"),
        ]
        for cap in caps:
            self._capabilities[cap.name] = cap

    def _load_shims(self):
        self._compat_table.update(_WIN32_SHIM)
        self._compat_table.update(_MACOS_SHIM)
        self._compat_table.update(_ANDROID_SHIM)

    # ── Core Call ────────────────────────────────────────────────────────────

    def call(self, api_name: str, args: dict | None = None,
             target: Target = Target.DESKTOP) -> dict:
        """
        Primary API invocation. Handles version routing, capability check,
        interceptor chain, and audit logging.
        """
        args = args or {}
        t0   = time.perf_counter()
        self._stats["calls"] += 1

        # Resolve deprecated names
        cap = self._capabilities.get(api_name)
        if cap and cap.deprecated:
            self._stats["version_mismatches"] += 1
            api_name = cap.replacement or api_name

        cap = self._capabilities.get(api_name)
        if cap is None:
            return {"error": f"SigmaAPI: '{api_name}' not a registered capability."}

        if target not in cap.targets:
            return {
                "error": (
                    f"SigmaAPI: '{api_name}' not available on target={target.value}. "
                    f"Supported: {[t.value for t in cap.targets]}"
                )
            }

        # Run interceptors
        for interceptor in self._interceptors:
            interceptor(api_name, args)
            self._stats["intercepted"] += 1

        # Simulated dispatch
        result   = self._dispatch(api_name, args, target)
        duration = (time.perf_counter() - t0) * 1e6  # µs

        # Audit record
        rec = SyscallRecord(
            call_id    = str(uuid.uuid4())[:8],
            name       = api_name,
            category   = self._infer_category(api_name),
            target     = target,
            args       = args,
            result     = result.get("status",""),
            duration_us= round(duration, 2),
            timestamp  = time.strftime("%Y-%m-%dT%H:%M:%S"),
        )
        self._syscall_log.append(rec)

        return {**result, "api": api_name, "target": target.value, "latency_us": round(duration,2)}

    def _dispatch(self, name: str, args: dict, target: Target) -> dict:
        """Simulated dispatch for every registered capability."""
        category = self._infer_category(name)
        return {
            "status":  "OK",
            "message": (
                f"SigmaAPI: [{category.value}] '{name}' dispatched on "
                f"target={target.value} with {len(args)} args."
            ),
        }

    def _infer_category(self, name: str) -> SyscallCategory:
        parts = name.split(".")
        if len(parts) >= 2:
            cat_map = {
                "fs": SyscallCategory.FILESYSTEM, "proc": SyscallCategory.PROCESS,
                "net": SyscallCategory.NETWORK,   "mem":  SyscallCategory.MEMORY,
                "dev": SyscallCategory.DEVICE,    "sec":  SyscallCategory.SECURITY,
                "ui":  SyscallCategory.UI,        "ipc":  SyscallCategory.IPC,
            }
            return cat_map.get(parts[1], SyscallCategory.PROCESS)
        return SyscallCategory.PROCESS

    # ── Cross-Platform Shim ──────────────────────────────────────────────────

    def translate_foreign_call(self, platform: str, call: str, args: dict | None = None) -> dict:
        """
        Translate a Win32 / macOS Cocoa / Android API call into SigmaAPI canonical form.
        Enables running foreign binaries natively.
        """
        canonical = self._compat_table.get(call)
        self._stats["shim_calls"] += 1
        if canonical is None:
            return {
                "status":  "UNRESOLVED",
                "foreign": call,
                "message": (
                    f"APIShim [{platform}]: '{call}' has no direct SigmaAPI mapping. "
                    "Generic compatibility layer applied."
                ),
            }
        result = self.call(canonical, args or {})
        return {
            **result,
            "shim": {
                "platform": platform,
                "foreign":  call,
                "canonical":canonical,
            },
            "message": (
                f"APIShim [{platform}]: '{call}' → '{canonical}'. "
                f"Translated and dispatched. Status: {result.get('status','?')}."
            ),
        }

    # ── Wasm Runtime Integration ─────────────────────────────────────────────

    def run_wasm_binary(self, wasm_path: str, entry_point: str = "_start",
                        sandbox: bool = True) -> dict:
        """
        Execute a universal .sigma/.wasm binary in the integrated Wasm runtime.
        Runs on any SigmaOS target without recompilation.
        """
        exec_id = str(uuid.uuid4())[:8]
        self._stats["wasm_calls"] += 1
        return {
            "exec_id":   exec_id,
            "wasm":      wasm_path,
            "entry":     entry_point,
            "sandboxed": sandbox,
            "target":    "universal",
            "message":   (
                f"WasmRuntime: '{wasm_path}::{entry_point}' executing "
                f"(ID={exec_id}, sandbox={sandbox}). "
                "Binary portable to ALL SigmaOS targets without recompilation."
            ),
        }

    # ── FunctionMesh (Kernel-Space Serverless) ────────────────────────────────

    def register_function(self, name: str, code_ref: str,
                          target: Target = Target.CLOUD) -> dict:
        """Register a serverless-style function deployable directly in kernel space."""
        func_id  = f"fn-{str(uuid.uuid4())[:8]}"
        fn = FunctionMesh(func_id, name, target, code_ref)
        fn.deployed = True
        self._functions[func_id] = fn
        return {
            "func_id":  func_id,
            "name":     name,
            "target":   target.value,
            "code_ref": code_ref,
            "message":  (
                f"FunctionMesh: '{name}' deployed at kernel-space [{target.value}]. "
                f"No container overhead. ID={func_id}."
            ),
        }

    def invoke_function(self, func_id: str, payload: dict | None = None) -> dict:
        fn = self._functions.get(func_id)
        if fn is None:
            return {"error": f"Function '{func_id}' not registered."}
        fn.invocations += 1
        return {
            "func_id":    func_id,
            "name":       fn.name,
            "invocations":fn.invocations,
            "message":    (
                f"FunctionMesh: '{fn.name}' invoked ({fn.invocations}×). "
                "Zero cold-start (kernel-resident)."
            ),
        }

    # ── Syscall Interceptor Chain ────────────────────────────────────────────

    def add_interceptor(self, fn: callable) -> str:
        """Register a callable that is invoked before every API call (for auditing)."""
        iid = str(uuid.uuid4())[:6]
        self._interceptors.append(fn)
        return f"APILayer: Interceptor {iid} registered ({len(self._interceptors)} total)."

    # ── ABI Compatibility Table ──────────────────────────────────────────────

    def get_abi_table(self) -> dict:
        return {
            "version":      self.VERSION,
            "capabilities": len(self._capabilities),
            "shims":        {
                "Win32":    len(_WIN32_SHIM),
                "macOS":    len(_MACOS_SHIM),
                "Android":  len(_ANDROID_SHIM),
            },
            "functions":    len(self._functions),
            "interceptors": len(self._interceptors),
            "targets":      [t.value for t in Target],
        }

    # ── Stats / Health ────────────────────────────────────────────────────────

    def get_stats(self) -> dict:
        by_cat = {c.value: 0 for c in SyscallCategory}
        for rec in self._syscall_log:
            by_cat[rec.category.value] += 1
        avg_lat = (
            sum(r.duration_us for r in self._syscall_log) / len(self._syscall_log)
            if self._syscall_log else 0
        )
        return {
            "total_calls":     self._stats["calls"],
            "shim_calls":      self._stats["shim_calls"],
            "wasm_calls":      self._stats["wasm_calls"],
            "avg_latency_us":  round(avg_lat, 2),
            "by_category":     by_cat,
            "ops":             self._stats,
        }

    def get_syscall_log(self, limit: int = 20) -> list[dict]:
        return [
            {"call_id": r.call_id, "name": r.name,
             "target": r.target.value, "lat_us": r.duration_us,
             "ts": r.timestamp}
            for r in self._syscall_log[-limit:]
        ]

    def health_check(self) -> str:
        return (
            f"OK — Capabilities: {len(self._capabilities)}, "
            f"Calls: {self._stats['calls']}, "
            f"Shims: Win32={len(_WIN32_SHIM)}/macOS={len(_MACOS_SHIM)}/Android={len(_ANDROID_SHIM)}, "
            f"Functions: {len(self._functions)}"
        )


if __name__ == "__main__":
    api = SigmaUnifiedAPI()
    print(api.call("sigma.fs.read",    {"path": "/home/user/doc.pdf"})["message"])
    print(api.call("sigma.net.mesh_send", {"dst": "SigmaTab-7", "bytes": 2048})["message"])
    print(api.call("sigma.ui.render",  {"view": "DashboardView"}, Target.MOBILE)["message"])
    print(api.translate_foreign_call("Win32",   "CreateFile",  {"path": "C:\\\\doc.pdf"})["message"])
    print(api.translate_foreign_call("macOS",   "NSURLSession",{"url":  "https://api.example.com"})["message"])
    print(api.translate_foreign_call("Android", "android.media.MediaPlayer", {})["message"])
    fn = api.register_function("on_file_download", "ecosystem.pdf_forge.auto_process", Target.CLOUD)
    print(fn["message"])
    print(api.invoke_function(fn["func_id"])["message"])
    print(api.run_wasm_binary("app.sigma", "_start")["message"])
    print(api.get_abi_table())
    print(api.health_check())
