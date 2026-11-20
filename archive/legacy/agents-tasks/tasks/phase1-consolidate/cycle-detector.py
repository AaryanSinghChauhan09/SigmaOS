#!/usr/bin/env python3
"""
Dependency Cycle Detector for SigmaOS COMPONENTS.toml

Implements:
- Topological sort (Kahn's algorithm)
- Cycle detection via DFS
- Strongly connected component (SCC) identification
- Generates detailed cycle report

Usage:
    python3 cycle-detector.py [path/to/COMPONENTS.toml]

Exit Codes:
    0 - No cycles detected (DAG is valid)
    1 - Cycles detected (invalid DAG)
"""

import sys
import tomllib
from collections import defaultdict, deque
from pathlib import Path


class DependencyGraph:
    """Represents the component dependency graph."""

    def __init__(self):
        self.graph = defaultdict(list)  # adjacency list: comp -> [deps]
        self.all_components = set()
        self.cycles = []
        self.visited = set()
        self.rec_stack = set()

    def add_component(self, name):
        """Register a component in the graph."""
        self.all_components.add(name)
        if name not in self.graph:
            self.graph[name] = []

    def add_dependency(self, component, depends_on):
        """Add an edge: component depends on depends_on."""
        self.add_component(component)
        self.add_component(depends_on)
        self.graph[component].append(depends_on)

    def _dfs_cycle_detect(self, node, path):
        """
        DFS-based cycle detection.
        Records cycles found and returns True if a cycle is found.
        """
        self.visited.add(node)
        self.rec_stack.add(node)
        path.append(node)

        for neighbor in self.graph[node]:
            if neighbor not in self.visited:
                if self._dfs_cycle_detect(neighbor, path.copy()):
                    return True
            elif neighbor in self.rec_stack:
                # Cycle detected
                cycle_start_idx = path.index(neighbor) if neighbor in path else 0
                cycle = path[cycle_start_idx:] + [neighbor]
                self.cycles.append(cycle)
                return True

        self.rec_stack.remove(node)
        return False

    def detect_cycles(self):
        """
        Detect all cycles in the graph using DFS.
        Returns list of cycles found.
        """
        self.visited.clear()
        self.rec_stack.clear()
        self.cycles.clear()

        for component in self.all_components:
            if component not in self.visited:
                self._dfs_cycle_detect(component, [])

        return self.cycles

    def topological_sort(self):
        """
        Kahn's algorithm for topological sort.
        Returns sorted list if DAG, or empty list if cycles exist.
        """
        # Calculate in-degrees
        in_degree = defaultdict(int)
        for comp in self.all_components:
            in_degree[comp] = 0

        for comp in self.graph:
            for dep in self.graph[comp]:
                in_degree[dep] += 1

        # Queue all nodes with 0 in-degree
        queue = deque([comp for comp in self.all_components if in_degree[comp] == 0])
        sorted_list = []

        while queue:
            node = queue.popleft()
            sorted_list.append(node)

            # For each edge from this node
            for neighbor in self.graph[node]:
                in_degree[neighbor] -= 1
                if in_degree[neighbor] == 0:
                    queue.append(neighbor)

        # If not all components are in sorted_list, there's a cycle
        if len(sorted_list) != len(self.all_components):
            # Find which components are part of cycles
            remaining = [c for c in self.all_components if c not in sorted_list]
            return None, remaining

        return sorted_list, None

    def compute_scc(self):
        """
        Compute strongly connected components using Tarjan's algorithm.
        Returns list of SCCs (each SCC is a list of components).
        """
        index_counter = [0]
        stack = []
        lowlinks = {}
        index = {}
        on_stack = defaultdict(bool)
        sccs = []

        def strongconnect(node):
            index[node] = index_counter[0]
            lowlinks[node] = index_counter[0]
            index_counter[0] += 1
            stack.append(node)
            on_stack[node] = True

            for successor in self.graph[node]:
                if successor not in index:
                    strongconnect(successor)
                    lowlinks[node] = min(lowlinks[node], lowlinks[successor])
                elif on_stack[successor]:
                    lowlinks[node] = min(lowlinks[node], index[successor])

            if lowlinks[node] == index[node]:
                scc = []
                while True:
                    w = stack.pop()
                    on_stack[w] = False
                    scc.append(w)
                    if w == node:
                        break
                if len(scc) > 1:  # Only track non-trivial SCCs
                    sccs.append(scc)

        for node in self.all_components:
            if node not in index:
                strongconnect(node)

        return sccs

    def get_dependency_stats(self):
        """Compute basic dependency statistics."""
        total_deps = sum(len(deps) for deps in self.graph.values())
        leaf_nodes = [c for c in self.all_components if len(self.graph[c]) == 0]
        root_nodes = [
            c for c in self.all_components if all(c not in self.graph[other] for other in self.all_components)
        ]

        avg_deps = total_deps / max(len(self.all_components), 1)

        return {
            "total_components": len(self.all_components),
            "total_edges": total_deps,
            "avg_dependencies": avg_deps,
            "leaf_nodes": leaf_nodes,
            "root_nodes": root_nodes,
            "most_dependent": sorted([(c, len(self.graph[c])) for c in self.graph], key=lambda x: x[1], reverse=True)[:10],
        }


def parse_components_toml(toml_path):
    """Parse COMPONENTS.toml and build dependency graph."""
    graph = DependencyGraph()

    try:
        with open(toml_path, "rb") as f:
            content = tomllib.load(f)
    except FileNotFoundError:
        print(f"[ERROR] COMPONENTS.toml not found: {toml_path}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"[ERROR] Failed to parse COMPONENTS.toml: {e}", file=sys.stderr)
        sys.exit(1)

    if "component" not in content:
        print("[ERROR] No components found in COMPONENTS.toml", file=sys.stderr)
        sys.exit(1)

    for component_data in content["component"]:
        name = component_data.get("name")
        if not name:
            continue

        graph.add_component(name)

        # Add dependencies
        depends_on = component_data.get("depends_on", [])
        if isinstance(depends_on, str):
            depends_on = [depends_on]

        for dep in depends_on:
            if dep and dep != name:  # Avoid self-loops
                graph.add_dependency(name, dep)

    return graph


def generate_cycle_report(graph):
    """Generate a detailed cycle detection report."""
    report = []
    report.append("# Dependency Cycle Detection Report\n")
    report.append("=" * 80 + "\n\n")

    # Detection method
    report.append("## Detection Method\n")
    report.append("- **Algorithm:** DFS-based cycle detection + Tarjan's SCC algorithm\n")
    report.append("- **Validation:** Kahn's topological sort\n\n")

    # Summary
    stats = graph.get_dependency_stats()
    report.append("## Graph Statistics\n")
    report.append(f"- **Total Components:** {stats['total_components']}\n")
    report.append(f"- **Total Dependency Edges:** {stats['total_edges']}\n")
    report.append(f"- **Average Dependencies/Component:** {stats['avg_dependencies']:.2f}\n")
    report.append(f"- **Root Components (no dependencies):** {len(stats['root_nodes'])}\n")
    report.append(f"- **Leaf Components (no dependents):** {len(stats['leaf_nodes'])}\n\n")

    # Cycle detection results
    sccs = graph.compute_scc()
    cycles = graph.detect_cycles()

    report.append("## Cycle Detection Results\n")
    if not cycles and not sccs:
        report.append("✅ **NO CYCLES DETECTED** — Dependency graph is a valid DAG\n\n")
        is_valid = True
    else:
        report.append(f"❌ **CYCLES DETECTED** — {len(cycles)} cycle(s) found\n\n")
        is_valid = False

        if sccs:
            report.append("### Strongly Connected Components (SCCs)\n\n")
            for i, scc in enumerate(sccs, 1):
                report.append(f"**Cycle {i}:** {' → '.join(scc)}\n\n")

        if cycles:
            report.append("### Detected Cycles\n\n")
            for i, cycle in enumerate(cycles, 1):
                cycle_str = " → ".join(cycle)
                report.append(f"**Cycle {i}:** {cycle_str}\n\n")

    # Topological sort validation
    sorted_comps, remaining = graph.topological_sort()
    report.append("## Topological Sort Validation\n")
    if sorted_comps:
        report.append("✅ Valid topological ordering exists (first 20 in dependency order):\n\n")
        for comp in sorted_comps[:20]:
            report.append(f"  {comp}\n")
        if len(sorted_comps) > 20:
            report.append(f"  ... ({len(sorted_comps) - 20} more components)\n")
    else:
        report.append(f"❌ Topological sort failed — {len(remaining)} components in cycle:\n\n")
        for comp in remaining:
            report.append(f"  {comp}\n")
    report.append("\n")

    # Most dependent components
    report.append("## Most Connected Components (Top 10)\n\n")
    for comp, count in stats["most_dependent"]:
        report.append(f"- **{comp}**: {count} dependencies\n")
    report.append("\n")

    # Root nodes
    report.append("## Root Components (No Dependencies)\n\n")
    if stats["root_nodes"]:
        for comp in sorted(stats["root_nodes"])[:20]:
            report.append(f"- {comp}\n")
        if len(stats["root_nodes"]) > 20:
            report.append(f"- ... ({len(stats['root_nodes']) - 20} more)\n")
    else:
        report.append("(None — all components depend on at least one other)\n")
    report.append("\n")

    # Leaf nodes
    report.append("## Leaf Components (No Dependents)\n\n")
    if stats["leaf_nodes"]:
        count = min(20, len(stats["leaf_nodes"]))
        for comp in sorted(stats["leaf_nodes"])[:count]:
            report.append(f"- {comp}\n")
        if len(stats["leaf_nodes"]) > 20:
            report.append(f"- ... ({len(stats['leaf_nodes']) - 20} more)\n")
    else:
        report.append("(None — all components are depended on by at least one other)\n")
    report.append("\n")

    report.append("## Recommendation\n")
    if is_valid:
        report.append("✅ **APPROVED** — The dependency graph is acyclic and ready for multi-repo migration.\n")
    else:
        report.append("❌ **BLOCKED** — Resolve cycles before proceeding with migration.\n")
        report.append("   Break cycles by:\n")
        report.append("   1. Moving shared code to a common library\n")
        report.append("   2. Reorganizing components to eliminate mutual dependencies\n")
        report.append("   3. Using interfaces/abstractions to decouple components\n")

    return "".join(report), is_valid


def main():
    if len(sys.argv) > 1:
        toml_path = sys.argv[1]
    else:
        toml_path = "/home/aaryansinghchauhan/SigmaOS/COMPONENTS.toml"

    print(f"[*] Parsing {toml_path}...")
    graph = parse_components_toml(toml_path)
    print(f"[+] Loaded {len(graph.all_components)} components")
    print(f"[+] Loaded {sum(len(deps) for deps in graph.graph.values())} dependency edges")

    print("[*] Running cycle detection...")
    cycles = graph.detect_cycles()

    print("[*] Computing strongly connected components...")
    sccs = graph.compute_scc()

    print("[*] Running topological sort validation...")
    sorted_list, remaining = graph.topological_sort()

    print("[*] Generating report...")
    report, is_valid = generate_cycle_report(graph)

    # Write report
    report_path = "/home/aaryansinghchauhan/SigmaOS/.agents/tasks/phase1-consolidate/cycle-detection-report.md"
    with open(report_path, "w") as f:
        f.write(report)

    print(f"[+] Report written to {report_path}")
    print("\n" + "=" * 80)
    if is_valid:
        print("✅ RESULT: NO CYCLES DETECTED (DAG valid)")
        print("=" * 80)
        exit_code = 0
    else:
        print("❌ RESULT: CYCLES DETECTED")
        print("=" * 80)
        print(f"Found {len(cycles)} cycle(s) and {len(sccs)} strongly connected component(s)")
        exit_code = 1

    print(f"Total components: {len(graph.all_components)}")
    print(f"Total dependency edges: {sum(len(deps) for deps in graph.graph.values())}")
    print(f"Report: {report_path}")

    return exit_code


if __name__ == "__main__":
    sys.exit(main())
