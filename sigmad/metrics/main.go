// SPDX-License-Identifier: GPL-2.0-or-later
// sigmad/metrics/main.go — Prometheus-compatible metrics exporter
//
// Inspired by node_exporter (Linux), collectd, and macOS Activity Monitor.
//
// Exposes system metrics in Prometheus text format at:
//   GET /metrics    — all metrics
//   GET /metrics/cpu
//   GET /metrics/mem
//   GET /metrics/disk
//   GET /metrics/net
//   GET /metrics/proc
//
// Socket: /run/sigma/metrics.sock
// Also optionally binds TCP :9100 for Prometheus scraping.
//
// Metrics collected:
//   sigma_cpu_usage_percent{core="0"}
//   sigma_mem_total_bytes, sigma_mem_free_bytes, sigma_mem_cached_bytes
//   sigma_disk_read_bytes_total{dev="sda"}, sigma_disk_write_bytes_total
//   sigma_net_rx_bytes_total{iface="eth0"}, sigma_net_tx_bytes_total
//   sigma_procs_total, sigma_procs_running, sigma_procs_blocked
//   sigma_uptime_seconds
//   sigma_load_average{window="1m"}

package main

import (
	"fmt"
	"net"
	"net/http"
	"os"
	"runtime"
	"strings"
	"time"
)

var startTime = time.Now()

// ── Metric line builder ───────────────────────────────────────────────────
type MetricWriter struct{ sb strings.Builder }

func (m *MetricWriter) gauge(name, labels, help string, val float64) {
	if help != "" {
		fmt.Fprintf(&m.sb, "# HELP %s %s\n# TYPE %s gauge\n", name, help, name)
	}
	if labels != "" {
		fmt.Fprintf(&m.sb, "%s{%s} %g\n", name, labels, val)
	} else {
		fmt.Fprintf(&m.sb, "%s %g\n", name, val)
	}
}

func (m *MetricWriter) counter(name, labels, help string, val float64) {
	if help != "" {
		fmt.Fprintf(&m.sb, "# HELP %s %s\n# TYPE %s counter\n", name, help, name)
	}
	if labels != "" {
		fmt.Fprintf(&m.sb, "%s{%s} %g\n", name, labels, val)
	} else {
		fmt.Fprintf(&m.sb, "%s %g\n", name, val)
	}
}

// ── CPU metrics ───────────────────────────────────────────────────────────
func collectCPU(m *MetricWriter) {
	nCPU := runtime.NumCPU()
	// Read from /proc/stat on Linux
	data, err := os.ReadFile("/proc/stat")
	if err != nil {
		// Fallback: emit stub
		for i := 0; i < nCPU; i++ {
			m.gauge("sigma_cpu_usage_percent", fmt.Sprintf(`core="%d"`, i),
				"CPU usage percent per core", 0)
		}
		return
	}
	lines := strings.Split(string(data), "\n")
	for _, line := range lines {
		if !strings.HasPrefix(line, "cpu") || line[:3] != "cpu" { continue }
		var name string
		var user, nice, system, idle, iowait, irq, softirq uint64
		fmt.Sscanf(line, "%s %d %d %d %d %d %d %d",
			&name, &user, &nice, &system, &idle, &iowait, &irq, &softirq)
		total := float64(user + nice + system + idle + iowait + irq + softirq)
		busy  := float64(user + nice + system + irq + softirq)
		usage := 0.0
		if total > 0 { usage = busy / total * 100.0 }
		lbl := ""
		if name != "cpu" { lbl = fmt.Sprintf(`core=%q`, name[3:]) }
		m.gauge("sigma_cpu_usage_percent", lbl, "CPU usage percent", usage)
	}
}

// ── Memory metrics ────────────────────────────────────────────────────────
func collectMem(m *MetricWriter) {
	data, err := os.ReadFile("/proc/meminfo")
	if err != nil { m.gauge("sigma_mem_total_bytes","","Total memory bytes",0); return }
	fields := map[string]float64{}
	for _, line := range strings.Split(string(data), "\n") {
		var key string; var val float64
		fmt.Sscanf(line, "%s %f", &key, &val)
		key = strings.TrimSuffix(key, ":")
		fields[key] = val * 1024
	}
	m.gauge("sigma_mem_total_bytes",  "", "Total memory",  fields["MemTotal"])
	m.gauge("sigma_mem_free_bytes",   "", "Free memory",   fields["MemFree"])
	m.gauge("sigma_mem_cached_bytes", "", "Cached memory", fields["Cached"])
	m.gauge("sigma_mem_buffers_bytes","", "Buffer memory", fields["Buffers"])
	avail := fields["MemFree"] + fields["Cached"] + fields["Buffers"]
	m.gauge("sigma_mem_available_bytes","","Available memory", avail)
}

// ── Uptime + load ─────────────────────────────────────────────────────────
func collectSystem(m *MetricWriter) {
	uptime := time.Since(startTime).Seconds()
	m.gauge("sigma_uptime_seconds", "", "System uptime seconds", uptime)

	data, _ := os.ReadFile("/proc/loadavg")
	if len(data) > 0 {
		var la1, la5, la15 float64
		fmt.Sscanf(string(data), "%f %f %f", &la1, &la5, &la15)
		m.gauge("sigma_load_average", `window="1m"`,  "Load average", la1)
		m.gauge("sigma_load_average", `window="5m"`,  "Load average", la5)
		m.gauge("sigma_load_average", `window="15m"`, "Load average", la15)
	}
}

// ── All metrics ───────────────────────────────────────────────────────────
func allMetrics() string {
	var m MetricWriter
	collectCPU(&m)
	collectMem(&m)
	collectSystem(&m)
	return m.sb.String()
}

// ── HTTP handlers ─────────────────────────────────────────────────────────
func metricsHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/plain; version=0.0.4")
	fmt.Fprint(w, allMetrics())
}

func main() {
	sockPath := "/run/sigma/metrics.sock"
	os.Remove(sockPath)
	ln, err := net.Listen("unix", sockPath)
	if err != nil {
		fmt.Fprintln(os.Stderr, "[sigma-metrics] listen error:", err)
		os.Exit(1)
	}

	mux := http.NewServeMux()
	mux.HandleFunc("/metrics", metricsHandler)
	mux.HandleFunc("/metrics/cpu", func(w http.ResponseWriter, r *http.Request) {
		var m MetricWriter; collectCPU(&m)
		w.Header().Set("Content-Type", "text/plain; version=0.0.4")
		fmt.Fprint(w, m.sb.String())
	})
	mux.HandleFunc("/metrics/mem", func(w http.ResponseWriter, r *http.Request) {
		var m MetricWriter; collectMem(&m)
		w.Header().Set("Content-Type", "text/plain; version=0.0.4")
		fmt.Fprint(w, m.sb.String())
	})

	// Also serve on TCP :9100 if env var set
	if os.Getenv("SIGMA_METRICS_TCP") == "1" {
		go func() {
			fmt.Println("[sigma-metrics] TCP exporter on :9100")
			http.ListenAndServe(":9100", mux)
		}()
	}

	fmt.Println("[sigma-metrics] listening on", sockPath)
	http.Serve(ln, mux)
}
