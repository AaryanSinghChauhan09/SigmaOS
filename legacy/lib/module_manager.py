import os
import json

class ModuleManager:
    def __init__(self, search_dirs=["modules", "suites"]):
        self.search_dirs = search_dirs
        self.modules = []

    def discover(self):
        self.modules = []
        for base_dir in self.search_dirs:
            if not os.path.exists(base_dir):
                continue
            for root, _, files in os.walk(base_dir):
                if "module.json" in files:
                    mod_path = os.path.join(root, "module.json")
                    with open(mod_path) as f:
                        meta = json.load(f)
                    meta["_dir"] = root
                    meta["_c_files"] = [os.path.join(root, f) for f in files if f.endswith(".c")]
                    meta["_s_files"] = [os.path.join(root, f) for f in files if f.endswith(".s") or f.endswith(".asm")]
                    self.modules.append(meta)
        return self.modules

    def get_ordered_modules(self, arch=None):
        name_map = {m["module"]: m for m in self.modules}
        visited, order = set(), []

        def visit(mod):
            if mod["module"] in visited:
                return
            visited.add(mod["module"])
            for dep in mod.get("dependencies", []):
                if dep in name_map:
                    visit(name_map[dep])
            
            # Filter by architecture if specified
            if arch and arch not in mod.get("arch", [arch]):
                return
            
            order.append(mod)

        for m in self.modules:
            visit(m)
        return order

    def get_graph(self):
        graph = {}
        for m in self.modules:
            graph[m["module"]] = m.get("dependencies", [])
        return graph
