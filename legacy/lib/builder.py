import os
import hashlib
import json
import subprocess
import sys

class SigmaBuilder:
    TOOLCHAINS = {
        "x86_64":  {"cc": "gcc",                 "ld": "ld",                 "objcopy": "objcopy"},
        "aarch64": {"cc": "aarch64-linux-gnu-gcc", "ld": "aarch64-linux-gnu-ld", "objcopy": "aarch64-linux-gnu-objcopy"},
        "riscv64": {"cc": "riscv64-linux-gnu-gcc", "ld": "riscv64-linux-gnu-ld", "objcopy": "riscv64-linux-gnu-objcopy"},
    }

    def __init__(self, arch, cflags, build_dir="build"):
        self.arch = arch
        self.cflags = cflags
        self.build_dir = build_dir
        self.hash_cache_path = os.path.join(build_dir, ".build_cache.json")
        self.cache = self._load_cache()
        self.toolchain = self.TOOLCHAINS.get(arch, self.TOOLCHAINS["x86_64"])

    def _load_cache(self):
        if os.path.exists(self.hash_cache_path):
            with open(self.hash_cache_path) as f:
                return json.load(f)
        return {}

    def _save_cache(self):
        os.makedirs(self.build_dir, exist_ok=True)
        with open(self.hash_cache_path, "w") as f:
            json.dump(self.cache, f, indent=2)

    def _file_hash(self, path):
        h = hashlib.sha256()
        with open(path, "rb") as f:
            h.update(f.read())
        return h.hexdigest()

    def build_module(self, mod):
        cc = self.toolchain["cc"]
        obj_files = []

        for src in mod["_c_files"]:
            src_hash = self._file_hash(src)
            cache_key = f"{self.arch}::{src}"

            obj = src.replace(".c", ".o")
            if self.cache.get(cache_key) == src_hash and os.path.exists(obj):
                print(f"    [SKIP] {os.path.basename(src)}")
                obj_files.append(obj)
                continue

            cmd = f"{cc} {self.cflags} -c {src} -o {obj}"
            print(f"    [CC]   {os.path.basename(src)}")
            try:
                subprocess.run(cmd.split(), check=True)
                self.cache[cache_key] = src_hash
                obj_files.append(obj)
            except Exception as e:
                print(f"    [ERR]  Failed to compile {src}: {e}")
                sys.exit(1)

        return obj_files

    def link_image(self, all_objects, output_name="sigmaos"):
        ld = self.toolchain["ld"]
        out_bin = os.path.join(self.build_dir, f"{output_name}_{self.arch}.bin")
        print(f"\n[*] Linking -> {out_bin}")
        
        linker_script = "linker.ld"
        if not os.path.exists(linker_script):
            with open(linker_script, "w") as f:
                f.write("ENTRY(_start) SECTIONS { . = 0x100000; .text : { *(.text) } }\n")

        try:
            subprocess.run([ld, "-T", linker_script, "-o", out_bin] + all_objects, check=True)
            print(f"[+] Kernel image ready: {out_bin}")
            self._save_cache()
            return out_bin
        except Exception as e:
            print(f"    [ERR] Linking failed: {e}")
            sys.exit(1)

    def package_iso(self, kernel_bin):
        iso_path = kernel_bin.replace(".bin", ".iso")
        print(f"[*] Packaging ISO -> {iso_path}")
        # Placeholder for actual ISO creation logic
        # subprocess.run(["grub-mkrescue", "-o", iso_path, "isodir"], check=False)
        print(f"[+] ISO ready: {iso_path}")
        return iso_path

    def clean(self):
        print(f"[*] Cleaning {self.build_dir}...")
        if os.path.exists(self.build_dir):
            import shutil
            shutil.rmtree(self.build_dir)
        print("[+] Clean complete.")
