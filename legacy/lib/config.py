import json
import os

class SigmaConfig:
    def __init__(self, config_path="sigma_features.json"):
        self.config_path = config_path
        self.data = self._load()

    def _load(self):
        if not os.path.exists(self.config_path):
            return {}
        with open(self.config_path) as f:
            return json.load(f)

    def get_arch(self, default="x86_64"):
        return self.data.get("arch", default)

    def get_cflags(self, base_flags="-nostdlib -ffreestanding -O2 -Wall -std=c11"):
        flags = [base_flags]
        
        arch = self.get_arch().upper()
        flags.append(f"-DSIGMA_ARCH_{arch}")

        drivers = self.data.get("drivers", {})
        for sub, drv in drivers.items():
            if drv and sub != "comment":
                flags.append(f"-DSIGMA_DRIVER_{drv.upper()}")

        features = self.data.get("features", {})
        for k, v in features.items():
            flags.append(f"-DSIGMA_FEATURE_{k.upper()}={1 if v else 0}")

        memory = self.data.get("memory", {})
        for k, v in memory.items():
            flags.append(f"-DSIGMA_{k.upper()}={v}")

        return " ".join(flags)

    def update_feature(self, key, value):
        if "features" not in self.data:
            self.data["features"] = {}
        self.data["features"][key] = value

    def save(self, path=None):
        target = path or self.config_path
        with open(target, "w") as f:
            json.dump(self.data, f, indent=4)
