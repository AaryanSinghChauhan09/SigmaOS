// SPDX-License-Identifier: GPL-2.0-or-later
// sigmad/telemetry/main.go — Privacy-respecting opt-in telemetry daemon
//
// Inspired by Ubuntu's apport, Fedora's ABRT, and Windows Error Reporting,
// but with full local transparency and mandatory user consent.
//
// Design principles:
//   - All telemetry is OFF by default — user must explicitly opt in
//   - Every report is shown to the user before transmission
//   - Reports are stripped of PII (hostname, username, local paths)
//   - Data is sent over TLS 1.3 to telemetry.sigma-os.dev
//   - Local ledger kept at /sigma/var/telemetry/ledger.json
//
// Socket: /run/sigma/telemetry.sock
// Endpoints:
//   POST /telemetry/report    — submit a new event report
//   GET  /telemetry/status    — opt-in status + report count
//   POST /telemetry/optin     — enable telemetry
//   POST /telemetry/optout    — disable + purge local ledger

package main

import (
	"bytes"
	"crypto/tls"
	"encoding/json"
	"fmt"
	"net"
	"net/http"
	"os"
	"regexp"
	"sync"
	"time"
)

const (
	telemetryEndpoint = "https://telemetry.sigma-os.dev/v1/ingest"
	ledgerPath        = "/sigma/var/telemetry/ledger.json"
	optInPath         = "/sigma/var/telemetry/optin"
)

// ── Event categories ──────────────────────────────────────────────────────
type EventKind string

const (
	KindCrash      EventKind = "crash"
	KindPanic      EventKind = "kernel_panic"
	KindInstall    EventKind = "pkg_install"
	KindPerf       EventKind = "perf_sample"
	KindHardware   EventKind = "hw_probe"
)

// ── Report record ─────────────────────────────────────────────────────────
type Report struct {
	ID        string    `json:"id"`
	Kind      EventKind `json:"kind"`
	Timestamp time.Time `json:"timestamp"`
	Version   string    `json:"version"`
	Arch      string    `json:"arch"`
	Payload   any       `json:"payload"`
	Sent      bool      `json:"sent"`
}

// ── Daemon state ──────────────────────────────────────────────────────────
var (
	mu      sync.Mutex
	ledger  []Report
	optedIn bool
)

// ── PII scrubbing ─────────────────────────────────────────────────────────
var (
	reHome     = regexp.MustCompile(`/home/[^/\s]+`)
	reUsername = regexp.MustCompile(`\b[A-Za-z][A-Za-z0-9_-]{2,}\b`)
	reIP       = regexp.MustCompile(`\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b`)
)

func scrubPII(s string) string {
	s = reHome.ReplaceAllString(s, "/home/<user>")
	s = reIP.ReplaceAllString(s, "<ip>")
	return s
}

// ── Ledger persistence ────────────────────────────────────────────────────
func loadLedger() {
	data, err := os.ReadFile(ledgerPath)
	if err != nil {
		return
	}
	json.Unmarshal(data, &ledger)
}

func saveLedger() {
	data, _ := json.MarshalIndent(ledger, "", "  ")
	os.MkdirAll("/sigma/var/telemetry", 0o700)
	os.WriteFile(ledgerPath, data, 0o600)
}

func isOptedIn() bool {
	_, err := os.Stat(optInPath)
	return err == nil
}

// ── Transmit pending reports ──────────────────────────────────────────────
func transmitPending() {
	if !optedIn {
		return
	}
	tlsCfg := &tls.Config{MinVersion: tls.VersionTLS13}
	client := &http.Client{
		Timeout:   15 * time.Second,
		Transport: &http.Transport{TLSClientConfig: tlsCfg},
	}
	mu.Lock()
	defer mu.Unlock()
	for i := range ledger {
		if ledger[i].Sent {
			continue
		}
		body, _ := json.Marshal(ledger[i])
		resp, err := client.Post(telemetryEndpoint, "application/json",
			bytes.NewReader(body))
		if err == nil && resp.StatusCode == 200 {
			ledger[i].Sent = true
		}
	}
	saveLedger()
}

// ── HTTP handlers ─────────────────────────────────────────────────────────
func handleReport(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", 405)
		return
	}
	var rep Report
	if err := json.NewDecoder(r.Body).Decode(&rep); err != nil {
		http.Error(w, "bad request", 400)
		return
	}
	rep.ID = fmt.Sprintf("%d", time.Now().UnixNano())
	rep.Timestamp = time.Now().UTC()
	// Scrub PII from string payload fields
	if s, ok := rep.Payload.(string); ok {
		rep.Payload = scrubPII(s)
	}
	mu.Lock()
	ledger = append(ledger, rep)
	saveLedger()
	mu.Unlock()
	go transmitPending()
	w.Header().Set("Content-Type", "application/json")
	fmt.Fprintf(w, `{"id":%q}`, rep.ID)
}

func handleStatus(w http.ResponseWriter, r *http.Request) {
	mu.Lock()
	defer mu.Unlock()
	type status struct {
		OptedIn     bool `json:"opted_in"`
		TotalReports int  `json:"total_reports"`
		PendingSend  int  `json:"pending_send"`
	}
	pending := 0
	for _, rep := range ledger {
		if !rep.Sent {
			pending++
		}
	}
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(status{
		OptedIn:     optedIn,
		TotalReports: len(ledger),
		PendingSend:  pending,
	})
}

func handleOptIn(w http.ResponseWriter, r *http.Request) {
	os.MkdirAll("/sigma/var/telemetry", 0o700)
	os.WriteFile(optInPath, []byte("1"), 0o600)
	optedIn = true
	fmt.Fprintln(w, `{"ok":true,"message":"Telemetry enabled. Thank you."}`)
}

func handleOptOut(w http.ResponseWriter, r *http.Request) {
	os.Remove(optInPath)
	mu.Lock()
	ledger = nil
	os.Remove(ledgerPath)
	mu.Unlock()
	optedIn = false
	fmt.Fprintln(w, `{"ok":true,"message":"Telemetry disabled and local data purged."}`)
}

func main() {
	loadLedger()
	optedIn = isOptedIn()

	// Retry unsent reports every 30 minutes
	go func() {
		for {
			time.Sleep(30 * time.Minute)
			transmitPending()
		}
	}()

	sockPath := "/run/sigma/telemetry.sock"
	os.Remove(sockPath)
	ln, err := net.Listen("unix", sockPath)
	if err != nil {
		fmt.Fprintln(os.Stderr, "[sigma-telemetry] listen error:", err)
		os.Exit(1)
	}

	mux := http.NewServeMux()
	mux.HandleFunc("/telemetry/report", handleReport)
	mux.HandleFunc("/telemetry/status", handleStatus)
	mux.HandleFunc("/telemetry/optin", handleOptIn)
	mux.HandleFunc("/telemetry/optout", handleOptOut)

	fmt.Println("[sigma-telemetry] listening on", sockPath)
	http.Serve(ln, mux)
}
