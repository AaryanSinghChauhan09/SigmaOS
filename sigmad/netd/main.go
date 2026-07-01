// SPDX-License-Identifier: GPL-2.0-or-later
// sigmad/netd/main.go — Network manager daemon
//
// Manages WiFi connections, DHCP leases, and DNS caching.
// Exposes sigma.Network interface on sigma-bus.
// Without this, the OS boots with no IP address even if WiFi driver loads.

package main

import (
	"encoding/json"
	"fmt"
	"net"
	"net/http"
	"os"
	"os/exec"
	"time"
)

type NetworkStatus struct {
	Connected  bool   `json:"connected"`
	SSID       string `json:"ssid,omitempty"`
	Interface  string `json:"interface"`
	IP         string `json:"ip,omitempty"`
	Gateway    string `json:"gateway,omitempty"`
	DNS        string `json:"dns,omitempty"`
	Signal     int    `json:"signal_dbm,omitempty"`
	LinkSpeed  int    `json:"link_speed_mbps,omitempty"`
}

type WiFiNetwork struct {
	SSID     string `json:"ssid"`
	Signal   int    `json:"signal_dbm"`
	Security string `json:"security"` // "WPA3", "WPA2", "Open"
	Known    bool   `json:"known"`
}

var currentStatus = NetworkStatus{Interface: "wlan0"}

// ── DHCP client ───────────────────────────────────────────────────────────────
func runDHCP(iface string) error {
	// In production: implement RFC 2131 DISCOVER→OFFER→REQUEST→ACK
	// For now: delegate to busybox udhcpc if available
	cmd := exec.Command("udhcpc", "-i", iface, "-n", "-q")
	if err := cmd.Run(); err != nil {
		// Fall back: try ip link set + dhclient
		exec.Command("ip", "link", "set", iface, "up").Run()
		return exec.Command("dhclient", "-1", iface).Run()
	}
	return nil
}

func getIPAddress(iface string) string {
	ifaces, _ := net.Interfaces()
	for _, i := range ifaces {
		if i.Name != iface { continue }
		addrs, _ := i.Addrs()
		for _, a := range addrs {
			if ipnet, ok := a.(*net.IPNet); ok && !ipnet.IP.IsLoopback() {
				if ipnet.IP.To4() != nil { return ipnet.IP.String() }
			}
		}
	}
	return ""
}

// ── WiFi management ───────────────────────────────────────────────────────────
func scanWiFi() []WiFiNetwork {
	// In production: use nl80211 netlink socket for scan
	// Stub: return empty list for now
	return []WiFiNetwork{}
}

func connectWiFi(ssid, passphrase string) error {
	// In production: write wpa_supplicant config + associate
	// For now: use wpa_supplicant binary if available
	conf := fmt.Sprintf(`network={
    ssid="%s"
    psk="%s"
    key_mgmt=SAE WPA-PSK
}`, ssid, passphrase)
	f, err := os.CreateTemp("", "sigma-wpa-*.conf")
	if err != nil { return err }
	defer os.Remove(f.Name())
	f.WriteString(conf); f.Close()

	return exec.Command("wpa_supplicant", "-B",
		"-i", currentStatus.Interface,
		"-c", f.Name()).Run()
}

// ── HTTP API ──────────────────────────────────────────────────────────────────
func main() {
	// Bring up primary interface
	go func() {
		time.Sleep(2 * time.Second) // wait for driver to settle
		if err := runDHCP(currentStatus.Interface); err != nil {
			fmt.Fprintf(os.Stderr, "[sigma-netd] DHCP failed on %s: %v\n",
				currentStatus.Interface, err)
		} else {
			currentStatus.IP = getIPAddress(currentStatus.Interface)
			currentStatus.Connected = (currentStatus.IP != "")
			fmt.Printf("[sigma-netd] IP: %s\n", currentStatus.IP)
		}
	}()

	sockPath := "/run/sigma/netd.sock"
	os.Remove(sockPath)
	ln, _ := net.Listen("unix", sockPath)

	mux := http.NewServeMux()

	mux.HandleFunc("/net/status", func(w http.ResponseWriter, r *http.Request) {
		currentStatus.IP = getIPAddress(currentStatus.Interface)
		currentStatus.Connected = currentStatus.IP != ""
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(currentStatus)
	})

	mux.HandleFunc("/net/scan", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(scanWiFi())
	})

	mux.HandleFunc("/net/connect", func(w http.ResponseWriter, r *http.Request) {
		var req struct{ SSID, Passphrase string }
		json.NewDecoder(r.Body).Decode(&req)
		err := connectWiFi(req.SSID, req.Passphrase)
		if err != nil {
			w.WriteHeader(500)
			fmt.Fprintf(w, `{"error":"%v"}`, err)
			return
		}
		time.Sleep(3 * time.Second)
		if err := runDHCP(currentStatus.Interface); err == nil {
			currentStatus.SSID = req.SSID
			currentStatus.IP   = getIPAddress(currentStatus.Interface)
		}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(currentStatus)
	})

	fmt.Println("[sigma-netd] listening on", sockPath)
	http.Serve(ln, mux)
}
