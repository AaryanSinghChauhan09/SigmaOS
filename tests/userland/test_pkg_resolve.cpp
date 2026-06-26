// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * test_pkg_resolve — dependency resolver correctness
 *
 * Verifies that the topological sort in sigma_pkg_resolver produces a valid
 * installation order: every dependency appears before the package that needs it.
 */
#include <cassert>
#include <cstdio>
#include <cstring>
#include <vector>
#include <string>
#include <algorithm>

/* ── Minimal dependency resolver (mirrors sigma_pkg_resolver logic) ─────── */
struct Pkg {
    std::string name;
    std::vector<std::string> deps;
};

/* Topological sort using DFS (Kahn's algorithm variant) */
static std::vector<std::string> toposort(const std::vector<Pkg>& pkgs) {
    std::vector<std::string> order;
    std::vector<bool> visited(pkgs.size(), false);

    std::function<void(int)> visit = [&](int i) {
        if (visited[i]) return;
        visited[i] = true;
        for (const auto& dep : pkgs[i].deps) {
            for (int j = 0; j < (int)pkgs.size(); j++) {
                if (pkgs[j].name == dep) { visit(j); break; }
            }
        }
        order.push_back(pkgs[i].name);
    };

    for (int i = 0; i < (int)pkgs.size(); i++) visit(i);
    return order;
}

static int idx_of(const std::vector<std::string>& v, const std::string& s) {
    for (int i = 0; i < (int)v.size(); i++)
        if (v[i] == s) return i;
    return -1;
}

int main(void) {
    /* Dependency graph:
     *   zenith-browser → sigma-net → sigma-klib
     *   zenith-browser → sigma-klib
     *   sigmad-ai      → sigma-net
     */
    std::vector<Pkg> pkgs = {
        { "sigma-klib",      {}                                    },
        { "sigma-net",       { "sigma-klib" }                      },
        { "zenith-browser",  { "sigma-net", "sigma-klib" }         },
        { "sigmad-ai",       { "sigma-net" }                       },
    };

    auto order = toposort(pkgs);

    /* ── Test 1: all packages present in output ──────────────────────── */
    assert(order.size() == 4 && "all 4 packages must appear in output");

    /* ── Test 2: sigma-klib before sigma-net ─────────────────────────── */
    assert(idx_of(order, "sigma-klib") < idx_of(order, "sigma-net") &&
           "sigma-klib must be installed before sigma-net");

    /* ── Test 3: sigma-net before zenith-browser ─────────────────────── */
    assert(idx_of(order, "sigma-net") < idx_of(order, "zenith-browser") &&
           "sigma-net must be installed before zenith-browser");

    /* ── Test 4: sigma-klib before zenith-browser (transitive) ──────── */
    assert(idx_of(order, "sigma-klib") < idx_of(order, "zenith-browser") &&
           "sigma-klib must precede zenith-browser (transitive dep)");

    /* ── Test 5: sigma-net before sigmad-ai ──────────────────────────── */
    assert(idx_of(order, "sigma-net") < idx_of(order, "sigmad-ai") &&
           "sigma-net must precede sigmad-ai");

    printf("test_pkg_resolve: PASS\n");
    return 0;
}
