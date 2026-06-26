// SPDX-License-Identifier: GPL-2.0-or-later
// sigmad/healthd/main.go — structured node health daemon (CoreOS + Flatpak-inspired)
//
// Exposes /run/sigma/healthd.sock with a JSON health endpoint per subsystem.
// sigmactl health reads this socket and prints:
//   ✓ zero-trust   ok      audit ring: 0 violations in 24h
//   ✗ cryptfs      FAILED  derive_key() is a stub — filesystem unencrypted
//
// This surfaces BR2_BROKEN stubs at RUNTIME, not just at build time.

package main

import (
	"encoding/json"
	"fmt"
	"net"
	"net/http"
	"os"
	"os/exec"
	"strings"
	"time"
)

// ── Types ────────────────────────────────────────────────────────────────────

type SubsystemHealth struct {
	Name    string `json:"name"`
	Status  string `json:"status"`  // "ok" | "degraded" | "failed" | "stub"
	Message string `json:"message,omitempty"`
	Since   string `json:"since"`
}

type NodeHealth struct {
	Node       string            `json:"node"`
	Timestamp  string            `json:"timestamp"`
	Overall    string            `json:"overall"` // "ok" | "degraded" | "failed"
	UptimeSecs float64           `json:"uptime_seconds"`
	Subsystems []SubsystemHealth `json:"subsystems"`
}

// ── Node identity ────────────────────────────────────────────────────────────

func getSigmaNodeID() string {
	hostname, err := os.Hostname()
	if err != nil {
		return "sigma-unknown"
	}
	return "sigma-" + hostname
}

func getUptime() float64 {
	data, err := os.ReadFile("/proc/uptime")
	if err != nil {
		return 0
	}
	var up float64
	fmt.Sscanf(string(data), "%f", &up)
	return up
}

// ── Individual subsystem checks ──────────────────────────────────────────────

func checkZeroTrust() SubsystemHealth {
	// Check if the audit ring socket is responding
	conn, err := net.DialTimeout("unix", "/run/sigma/zerotrust.sock", 500*time.Millisecond)
	if err != nil {
		return SubsystemHealth{
			Name:    "zero-trust",
			Status:  "degraded",
			Message: "audit socket unreachable — zero-trust daemon not running",
			Since:   time.Now().UTC().Format(time.RFC3339),
		}
	}
	conn.Close()
	return SubsystemHealth{Name: "zero-trust", Status: "ok",
		Message: "audit ring active", Since: time.Now().UTC().Format(time.RFC3339)}
}

func checkCryptFS() SubsystemHealth {
	// Check if derive_key stub is still present (BR2_BROKEN surface at runtime)
	_, err := os.Stat("/dev/mapper/sigma-data")
	if err != nil {
		return SubsystemHealth{
			Name:   "cryptfs",
			Status: "failed",
			Message: "derive_key() is a stub — /dev/mapper/sigma-data not present. " +
				"Filesystem is NOT encrypted. See github.com/AaryanSinghChauhan09/SigmaOS/issues/44",
			Since: time.Now().UTC().Format(time.RFC3339),
		}
	}
	return SubsystemHealth{Name: "cryptfs", Status: "ok",
		Message: "encrypted volume mounted", Since: time.Now().UTC().Format(time.RFC3339)}
}

func checkHypervisor() SubsystemHealth {
	conn, err := net.DialTimeout("unix", "/run/sigma/hypervisor.sock", 500*time.Millisecond)
	if err != nil {
		return SubsystemHealth{
			Name:    "hypervisor",
			Status:  "degraded",
			Message: "hypervisor socket unreachable",
			Since:   time.Now().UTC().Format(time.RFC3339),
		}
	}
	conn.Close()
	return SubsystemHealth{Name: "hypervisor", Status: "ok",
		Since: time.Now().UTC().Format(time.RFC3339)}
}

func checkPackageDB() SubsystemHealth {
	_, err := os.Stat("/sigma/pkg/db/packages.db")
	if err != nil {
		return SubsystemHealth{
			Name:    "sigma-pkg",
			Status:  "degraded",
			Message: "package database not initialised — run: sigma-pkg init",
			Since:   time.Now().UTC().Format(time.RFC3339),
		}
	}
	return SubsystemHealth{Name: "sigma-pkg", Status: "ok",
		Since: time.Now().UTC().Format(time.RFC3339)}
}

func checkNetFirewall() SubsystemHealth {
	// Read conntrack entry count from sysctl
	out, err := exec.Command("sigma-sysctl", "net.firewall.conntrack_max").Output()
	if err != nil {
		return SubsystemHealth{
			Name: "net-firewall", Status: "degraded",
			Message: "sigma-sysctl not responding",
			Since:   time.Now().UTC().Format(time.RFC3339),
		}
	}
	return SubsystemHealth{
		Name: "net-firewall", Status: "ok",
		Message: strings.TrimSpace(string(out)) + " max flows configured",
		Since:   time.Now().UTC().Format(time.RFC3339),
	}
}

func checkInitWatchdog() SubsystemHealth {
	// PID 1 is always present; check its cmdline
	data, err := os.ReadFile("/proc/1/comm")
	if err != nil {
		return SubsystemHealth{Name: "init-watchdog", Status: "failed",
			Message: "cannot read PID 1 comm", Since: time.Now().UTC().Format(time.RFC3339)}
	}
	comm := strings.TrimSpace(string(data))
	return SubsystemHealth{
		Name:    "init-watchdog",
		Status:  "ok",
		Message: fmt.Sprintf("PID 1 (%s) alive", comm),
		Since:   time.Now().UTC().Format(time.RFC3339),
	}
}

// ── Overall health computation ────────────────────────────────────────────────

func computeOverall(subs []SubsystemHealth) string {
	for _, s := range subs {
		if s.Status == "failed" {
			return "failed"
		}
	}
	for _, s := range subs {
		if s.Status == "degraded" || s.Status == "stub" {
			return "degraded"
		}
	}
	return "ok"
}

// ── Collect all health checks ─────────────────────────────────────────────────

func collectHealth() NodeHealth {
	subs := []SubsystemHealth{
		checkZeroTrust(),
		checkCryptFS(),
		checkHypervisor(),
		checkPackageDB(),
		checkNetFirewall(),
		checkInitWatchdog(),
	}
	return NodeHealth{
		Node:       getSigmaNodeID(),
		Timestamp:  time.Now().UTC().Format(time.RFC3339),
		Overall:    computeOverall(subs),
		UptimeSecs: getUptime(),
		Subsystems: subs,
	}
}

// ── HTTP handler ──────────────────────────────────────────────────────────────

func healthHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	health := collectHealth()
	if health.Overall != "ok" {
		w.WriteHeader(http.StatusServiceUnavailable) // 503 if degraded/failed
	}
	json.NewEncoder(w).Encode(health)
}

// ── Main ──────────────────────────────────────────────────────────────────────

func main() {
	sockPath := "/run/sigma/healthd.sock"
	os.Remove(sockPath)

	ln, err := net.Listen("unix", sockPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "[sigma-healthd] listen failed: %v\n", err)
		os.Exit(1)
	}

	mux := http.NewServeMux()
	mux.HandleFunc("/health", healthHandler)
	mux.HandleFunc("/health/ready", func(w http.ResponseWriter, r *http.Request) {
		h := collectHealth()
		if h.Overall == "failed" {
			w.WriteHeader(http.StatusServiceUnavailable)
			return
		}
		w.WriteHeader(http.StatusOK)
	})

	fmt.Println("[sigma-healthd] listening on", sockPath)
	http.Serve(ln, mux)
}
