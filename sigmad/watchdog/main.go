// SPDX-License-Identifier: GPL-2.0-or-later
// sigmad/watchdog/main.go — Hardware + software watchdog daemon
//
// Inspired by Linux watchdog(8), systemd-watchdog, and OpenBSD rcctl.
//
// Responsibilities:
//   1. Pet /dev/watchdog (hardware WDT) every WDT_INTERVAL seconds
//   2. Monitor a list of critical daemons (sigma-healthd, sigma-busd, etc.)
//   3. If a daemon misses N heartbeats → restart it via sigma-rs
//   4. If petting fails N times → log + trigger controlled reboot
//   5. Emit metrics on /run/sigma/watchdog.sock
//
// Socket: /run/sigma/watchdog.sock
// Endpoints:
//   GET  /watchdog/status   — watched processes + WDT pet countdown
//   POST /watchdog/register — register a daemon for monitoring
//   POST /watchdog/heartbeat — daemon sends its own heartbeat
//   POST /watchdog/unregister

package main

import (
	"encoding/json"
	"fmt"
	"net"
	"net/http"
	"os"
	"os/exec"
	"sync"
	"time"
)

const (
	wdtDevice       = "/dev/watchdog"
	wdtInterval     = 15 * time.Second  // pet every 15s
	wdtMaxMiss      = 3                 // missed pets before reboot
	daemonMissLimit = 3                 // missed heartbeats before restart
)

// ── Watched daemon entry ──────────────────────────────────────────────────
type WatchedDaemon struct {
	Name         string    `json:"name"`
	RestartCmd   string    `json:"restart_cmd"`
	LastBeat     time.Time `json:"last_heartbeat"`
	MaxInterval  int       `json:"max_interval_sec"` // expected heartbeat period
	MissedBeats  int       `json:"missed_beats"`
	Restarts     int       `json:"total_restarts"`
}

// ── State ─────────────────────────────────────────────────────────────────
var (
	mu        sync.Mutex
	daemons   = map[string]*WatchedDaemon{}
	wdtMiss   = 0
	wdtFd     *os.File
	lastPet   time.Time
)

// ── Hardware watchdog petting ──────────────────────────────────────────────
func petWatchdog() {
	mu.Lock()
	defer mu.Unlock()
	if wdtFd != nil {
		_, err := wdtFd.Write([]byte("1"))
		if err != nil {
			wdtMiss++
			fmt.Fprintf(os.Stderr, "[sigma-watchdog] WDT pet failed (%d/%d): %v\n",
				wdtMiss, wdtMaxMiss, err)
			if wdtMiss >= wdtMaxMiss {
				fmt.Fprintln(os.Stderr, "[sigma-watchdog] WDT pet failed too many times — triggering reboot")
				exec.Command("reboot", "-f").Run()
			}
		} else {
			wdtMiss = 0
			lastPet = time.Now()
		}
	} else {
		// Simulated pet when no hardware WDT
		lastPet = time.Now()
	}
}

// ── Daemon health check loop ───────────────────────────────────────────────
func checkDaemons() {
	mu.Lock()
	defer mu.Unlock()
	now := time.Now()
	for name, d := range daemons {
		if d.LastBeat.IsZero() {
			continue
		}
		elapsed := now.Sub(d.LastBeat)
		if elapsed > time.Duration(d.MaxInterval)*time.Second {
			d.MissedBeats++
			fmt.Printf("[sigma-watchdog] %s missed heartbeat (%d/%d)\n",
				name, d.MissedBeats, daemonMissLimit)
			if d.MissedBeats >= daemonMissLimit && d.RestartCmd != "" {
				fmt.Printf("[sigma-watchdog] restarting %s\n", name)
				go exec.Command("sh", "-c", d.RestartCmd).Run()
				d.Restarts++
				d.MissedBeats = 0
				d.LastBeat = now
			}
		} else {
			d.MissedBeats = 0
		}
	}
}

// ── Monitor loop ──────────────────────────────────────────────────────────
func monitorLoop() {
	ticker := time.NewTicker(wdtInterval)
	for range ticker.C {
		petWatchdog()
		checkDaemons()
	}
}

// ── HTTP handlers ─────────────────────────────────────────────────────────
type statusResp struct {
	LastPet   time.Time                 `json:"last_pet"`
	WdtMiss   int                       `json:"wdt_miss_count"`
	Daemons   map[string]*WatchedDaemon `json:"daemons"`
}

func handleStatus(w http.ResponseWriter, r *http.Request) {
	mu.Lock()
	defer mu.Unlock()
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(statusResp{
		LastPet: lastPet, WdtMiss: wdtMiss, Daemons: daemons,
	})
}

func handleRegister(w http.ResponseWriter, r *http.Request) {
	var d WatchedDaemon
	if err := json.NewDecoder(r.Body).Decode(&d); err != nil {
		http.Error(w, "bad request", 400); return
	}
	if d.MaxInterval == 0 { d.MaxInterval = 30 }
	d.LastBeat = time.Now()
	mu.Lock()
	daemons[d.Name] = &d
	mu.Unlock()
	fmt.Fprintf(w, `{"ok":true,"registered":%q}`, d.Name)
}

func handleHeartbeat(w http.ResponseWriter, r *http.Request) {
	var req struct{ Name string `json:"name"` }
	json.NewDecoder(r.Body).Decode(&req)
	mu.Lock()
	if d, ok := daemons[req.Name]; ok {
		d.LastBeat = time.Now()
		d.MissedBeats = 0
	}
	mu.Unlock()
	fmt.Fprintln(w, `{"ok":true}`)
}

func handleUnregister(w http.ResponseWriter, r *http.Request) {
	var req struct{ Name string `json:"name"` }
	json.NewDecoder(r.Body).Decode(&req)
	mu.Lock()
	delete(daemons, req.Name)
	mu.Unlock()
	fmt.Fprintln(w, `{"ok":true}`)
}

func main() {
	// Open hardware watchdog if available
	if f, err := os.OpenFile(wdtDevice, os.O_WRONLY, 0); err == nil {
		wdtFd = f
		fmt.Println("[sigma-watchdog] hardware WDT opened:", wdtDevice)
	} else {
		fmt.Println("[sigma-watchdog] no hardware WDT, software-only mode")
	}

	// Pre-register critical system daemons
	mu.Lock()
	for _, name := range []string{"sigma-healthd", "sigma-busd", "sigma-trustd", "sigma-netd"} {
		daemons[name] = &WatchedDaemon{
			Name: name, MaxInterval: 60,
			RestartCmd: fmt.Sprintf("sigmactl restart %s", name),
			LastBeat:   time.Now(),
		}
	}
	mu.Unlock()

	go monitorLoop()

	sockPath := "/run/sigma/watchdog.sock"
	os.Remove(sockPath)
	ln, err := net.Listen("unix", sockPath)
	if err != nil {
		fmt.Fprintln(os.Stderr, "[sigma-watchdog] listen error:", err)
		os.Exit(1)
	}

	mux := http.NewServeMux()
	mux.HandleFunc("/watchdog/status",      handleStatus)
	mux.HandleFunc("/watchdog/register",    handleRegister)
	mux.HandleFunc("/watchdog/heartbeat",   handleHeartbeat)
	mux.HandleFunc("/watchdog/unregister",  handleUnregister)

	fmt.Println("[sigma-watchdog] listening on", sockPath)
	http.Serve(ln, mux)
}
