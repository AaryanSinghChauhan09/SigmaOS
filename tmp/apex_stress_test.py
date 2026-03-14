
import sys
import os
import time
import json
import statistics
import threading

# Inject paths
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from sigma_core.kernel import SigmaKernel
from sigma_core.system.sigma_fs import SigmaFS, BlockHealth
from sigma_core.ui.fluid_design import FluidTheme, THEMES

class ApexPerformanceSuite:
    def __init__(self):
        self.results = {}
        self.kernel = None

    def run_all(self):
        print("🚀 STARTING SIGMA OS APEX PERFORMANCE SUITE...")
        self.test_kernel_boot()
        self.test_fs_performance()
        self.test_governor_vibe_latency()
        self.test_app_hydration()
        self.report()

    def test_kernel_boot(self):
        print("[1/4] Measuring Kernel Hydration Speed...")
        start = time.perf_counter()
        self.kernel = SigmaKernel()
        # Simulate hydration of core modules
        boot_time = time.perf_counter() - start
        self.results["boot_latency"] = f"{boot_time*1000:.2f}ms"
        print(f"  -> Boot Latency: {self.results['boot_latency']}")

    def test_fs_performance(self):
        print("[2/4] Benchmarking SigmaFS 3.0 (Quantum Sharding & Compression)...")
        fs = SigmaFS(self.kernel)
        
        # Test: Write 100 small "Quantum Shards"
        write_times = []
        for i in range(100):
            content = os.urandom(1024) # 1KB
            s = time.perf_counter()
            fs.create(f"/test/shard_{i}.bin", content)
            write_times.append(time.perf_counter() - s)
        
        avg_write = statistics.mean(write_times)
        self.results["fs_avg_write_1kb"] = f"{avg_write*1000:.4f}ms"
        
        # Test: Read 100 shards
        read_times = []
        for i in range(100):
            s = time.perf_counter()
            fs.read(f"/test/shard_{i}.bin")
            read_times.append(time.perf_counter() - s)
        
        avg_read = statistics.mean(read_times)
        self.results["fs_avg_read_1kb"] = f"{avg_read*1000:.4f}ms"
        
        # Test: Adaptive Compression
        large_content = b"A" * (1024 * 1024) # 1MB compressible
        s = time.perf_counter()
        res = fs.create("/test/large.bin", large_content)
        comp_time = time.perf_counter() - s
        self.results["fs_comp_ratio_1mb"] = res.get("comp", "N/A")
        self.results["fs_1mb_write_latency"] = f"{comp_time*1000:.2f}ms"
        
        print(f"  -> Avg Write (1KB): {self.results['fs_avg_write_1kb']}")
        print(f"  -> Avg Read (1KB): {self.results['fs_avg_read_1kb']}")
        print(f"  -> 1MB Write Latency: {self.results['fs_1mb_write_latency']} (Ratio: {res.get('comp')})")

    def test_governor_vibe_latency(self):
        print("[3/4] Testing Governor Chromatic Vibe Latency...")
        if not hasattr(self.kernel, "governor"):
            print("  -> Governor not loaded in manifest. Skipping.")
            return

        gov = self.kernel.governor
        latencies = []
        for vibe in ["APEX", "RESOURCE_SAVING", "STANDARD"]:
            s = time.perf_counter()
            gov.switch_vibe(vibe)
            latencies.append(time.perf_counter() - s)
        
        avg_lat = statistics.mean(latencies)
        self.results["vibe_switch_latency"] = f"{avg_lat*1000:.4f}ms"
        print(f"  -> Vibe Switch Latency: {self.results['vibe_switch_latency']}")

    def test_app_hydration(self):
        print("[4/4] Testing App Shard Import Stability...")
        apps_to_test = ["chess.py", "ncert_master_lab.py", "nexus_monitor.py", "sovereign_vision.py"]
        stable = 0
        for app_file in apps_to_test:
            try:
                mod_name = app_file.replace(".py", "")
                __import__(f"userland.apps.{mod_name}")
                stable += 1
            except Exception as e:
                print(f"  ! App Failed: {app_file} -> {e}")
        
        self.results["app_stability"] = f"{stable}/{len(apps_to_test)} Loaded"
        print(f"  -> Stability: {self.results['app_stability']}")

    def report(self):
        print("\n" + "="*50)
        print("🏆 APEX PERFORMANCE REPORT 🏆")
        print("="*50)
        print(json.dumps(self.results, indent=2))
        print("="*50)
        with open("apex_benchmark.json", "w") as f:
            json.dump(self.results, f)

if __name__ == "__main__":
    suite = ApexPerformanceSuite()
    suite.run_all()
