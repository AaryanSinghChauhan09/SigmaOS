// SPDX-License-Identifier: GPL-2.0-or-later
// sigmad/power/main.go — Power manager daemon
//
// Monitors battery, handles lid close/open, screen dim on idle,
// suspend/hibernate. Emits events on sigma.Power bus interface.
// Socket: /run/sigma/power.sock

package main

import (
	"encoding/json"
	"fmt"
	"net"
	"net/http"
	"os"
	"strconv"
	"strings"
	"time"
)

type PowerStatus struct {
	BatteryPercent int    `json:"battery_percent"` // -1 = no battery (AC only)
	ACPlugged      bool   `json:"ac_plugged"`
	LidOpen        bool   `json:"lid_open"`
	ScreenOn       bool   `json:"screen_on"`
	State          string `json:"state"` // "active" | "idle" | "suspended" | "hibernating"
	IdleSeconds    int    `json:"idle_seconds"`
}

var status = PowerStatus{BatteryPercent: -1, ACPlugged: true, LidOpen: true, ScreenOn: true, State: "active"}
var idleThresholdSec = 300  // dim screen after 5 min
var suspendThresholdSec = 600 // suspend after 10 min

// ── Battery reading (Linux /sys/class/power_supply/) ─────────────────────────
func readBattery() {
	batDir := "/sys/class/power_supply/BAT0"
	if _, err := os.Stat(batDir); os.IsNotExist(err) {
		status.BatteryPercent = -1 // no battery
		status.ACPlugged = true
		return
	}
	if b, err := os.ReadFile(batDir + "/capacity"); err == nil {
		status.BatteryPercent, _ = strconv.Atoi(strings.TrimSpace(string(b)))
	}
	if b, err := os.ReadFile("/sys/class/power_supply/AC/online"); err == nil {
		status.ACPlugged = strings.TrimSpace(string(b)) == "1"
	}
}

func readLid() {
	// /proc/acpi/button/lid/LID0/state
	if b, err := os.ReadFile("/proc/acpi/button/lid/LID0/state"); err == nil {
		status.LidOpen = strings.Contains(string(b), "open")
	}
}

// ── Suspend / hibernate ───────────────────────────────────────────────────────
func suspend() {
	status.State = "suspended"
	os.WriteFile("/sys/power/state", []byte("mem"), 0o200)
	status.State = "active" // resumed
	status.IdleSeconds = 0
}

func hibernate() {
	status.State = "hibernating"
	os.WriteFile("/sys/power/state", []byte("disk"), 0o200)
	status.State = "active"
	status.IdleSeconds = 0
}

// ── Monitor loop ──────────────────────────────────────────────────────────────
func monitorLoop() {
	ticker := time.NewTicker(10 * time.Second)
	for range ticker.C {
		readBattery()
		readLid()

		// Low battery warning
		if status.BatteryPercent > 0 && !status.ACPlugged {
			if status.BatteryPercent <= 5 {
				notifyBus("sigma.Notifications", "Notify",
					`{"title":"Critical Battery","body":"Shutting down in 30 seconds"}`)
				time.Sleep(30 * time.Second)
				suspend()
			} else if status.BatteryPercent <= 15 {
				notifyBus("sigma.Notifications", "Notify",
					`{"title":"Low Battery","body":"Connect power soon"}`)
			}
		}

		// Lid close → suspend
		if !status.LidOpen && status.State == "active" {
			suspend()
		}

		// Idle tracking
		status.IdleSeconds += 10
		if status.IdleSeconds >= suspendThresholdSec && !status.ACPlugged {
			suspend()
		}
	}
}

func notifyBus(iface, signal, body string) {
	// Send to sigma-busd — real impl uses sigma_bus_client
	fmt.Printf("[sigma-power] bus → %s::%s %s\n", iface, signal, body)
}

// ── HTTP API ──────────────────────────────────────────────────────────────────
func main() {
	go monitorLoop()

	sockPath := "/run/sigma/power.sock"
	os.Remove(sockPath)
	ln, _ := net.Listen("unix", sockPath)

	mux := http.NewServeMux()

	mux.HandleFunc("/power/status", func(w http.ResponseWriter, r *http.Request) {
		readBattery(); readLid()
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(status)
	})

	mux.HandleFunc("/power/suspend", func(w http.ResponseWriter, r *http.Request) {
		go suspend()
		fmt.Fprintln(w, `{"ok":true}`)
	})

	mux.HandleFunc("/power/hibernate", func(w http.ResponseWriter, r *http.Request) {
		go hibernate()
		fmt.Fprintln(w, `{"ok":true}`)
	})

	mux.HandleFunc("/power/activity", func(w http.ResponseWriter, r *http.Request) {
		status.IdleSeconds = 0 // user activity resets idle timer
		fmt.Fprintln(w, `{"ok":true}`)
	})

	fmt.Println("[sigma-power] listening on", sockPath)
	http.Serve(ln, mux)
}
