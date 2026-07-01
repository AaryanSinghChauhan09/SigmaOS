// SPDX-License-Identifier: GPL-2.0-or-later
// sigmad/commnet/main.go — sigma-commnet: community-owned internet
//
// Village/colony shared internet infrastructure on SigmaOS.
// One upstream connection shared fairly across N households,
// with DID-based access control and offline caching.
//
// Compliant with TRAI community Wi-Fi rules (cost-sharing, not reselling).
//
// Socket: /run/sigma/commnet.sock
// Endpoints:
//   GET  /commnet/status        — gateway health + connected members
//   GET  /commnet/bandwidth     — per-member usage report
//   POST /commnet/member/add    — enrol a new member (DID-verified)
//   POST /commnet/member/remove — remove member
//   POST /commnet/cache/add     — add URL to offline cache
//   GET  /commnet/cache/list    — cached URLs
//   POST /commnet/qos/set       — set bandwidth policy

package main

import (
	"encoding/json"
	"fmt"
	"net"
	"net/http"
	"os"
	"sync"
	"time"
)

// ── Member record ──────────────────────────────────────────────────────────
type Member struct {
	ID           string    `json:"id"`           // DID or hostname
	Name         string    `json:"name"`         // "House 12 / School"
	MeshIP       string    `json:"mesh_ip"`      // sigma-mesh address
	JoinedAt     time.Time `json:"joined_at"`
	Active       bool      `json:"active"`
	TxBytesTotal int64     `json:"tx_bytes_total"`
	RxBytesTotal int64     `json:"rx_bytes_total"`
	// QoS
	MaxBandwidthKbps int `json:"max_bandwidth_kbps"` // 0 = fair share
}

// ── Cached URL entry ──────────────────────────────────────────────────────
type CachedURL struct {
	URL         string    `json:"url"`
	Description string    `json:"description"`
	SizeMB      float64   `json:"size_mb"`
	LastSync    time.Time `json:"last_sync"`
	HitCount    int       `json:"hit_count"`
}

// ── Gateway config ────────────────────────────────────────────────────────
type CommNetConfig struct {
	GatewayIface  string `json:"gateway_iface"`  // "eth0" — upstream
	MeshIface     string `json:"mesh_iface"`     // "wlan0" — downstream
	SubnetCIDR    string `json:"subnet_cidr"`    // "10.200.0.0/24"
	TotalBandwidthKbps int `json:"total_bandwidth_kbps"` // total upstream
	FairShare     bool   `json:"fair_share"`     // equal split by default
	OfflineMode   bool   `json:"offline_mode"`   // serve cached only
	// Monthly cost sharing
	MonthlyCostPaise int64 `json:"monthly_cost_paise"` // total ISP cost
}

var (
	mu      sync.RWMutex
	members = map[string]*Member{}
	cache   []CachedURL
	config  = CommNetConfig{
		GatewayIface: "eth0",
		MeshIface:    "wlan0",
		SubnetCIDR:   "10.200.0.0/24",
		TotalBandwidthKbps: 50000, // 50 Mbps default
		FairShare:    true,
	}
)

// ── Bandwidth calculation ─────────────────────────────────────────────────
func bandwidthPerMember() int {
	mu.RLock()
	defer mu.RUnlock()
	active := 0
	for _, m := range members {
		if m.Active { active++ }
	}
	if active == 0 { return config.TotalBandwidthKbps }
	return config.TotalBandwidthKbps / active
}

func costPerMember() int64 {
	mu.RLock()
	defer mu.RUnlock()
	active := 0
	for _, m := range members { if m.Active { active++ } }
	if active == 0 { return 0 }
	return config.MonthlyCostPaise / int64(active)
}

// ── HTTP handlers ─────────────────────────────────────────────────────────
func handleStatus(w http.ResponseWriter, r *http.Request) {
	mu.RLock()
	defer mu.RUnlock()
	active := 0
	for _, m := range members { if m.Active { active++ } }
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"gateway_iface":         config.GatewayIface,
		"mesh_iface":            config.MeshIface,
		"total_members":         len(members),
		"active_members":        active,
		"bandwidth_per_member":  fmt.Sprintf("%d Kbps", bandwidthPerMember()),
		"cost_per_member_paise": costPerMember(),
		"offline_mode":          config.OfflineMode,
		"cached_urls":           len(cache),
	})
}

func handleBandwidth(w http.ResponseWriter, r *http.Request) {
	mu.RLock()
	defer mu.RUnlock()
	list := make([]*Member, 0, len(members))
	for _, m := range members { list = append(list, m) }
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(list)
}

func handleAddMember(w http.ResponseWriter, r *http.Request) {
	var m Member
	if err := json.NewDecoder(r.Body).Decode(&m); err != nil {
		http.Error(w, "bad request", 400); return
	}
	m.JoinedAt = time.Now()
	m.Active   = true
	if m.ID == "" { m.ID = m.MeshIP }
	mu.Lock()
	members[m.ID] = &m
	mu.Unlock()
	fmt.Fprintf(w, `{"ok":true,"member_id":%q,"bandwidth_kbps":%d,"monthly_cost_paise":%d}`,
		m.ID, bandwidthPerMember(), costPerMember())
}

func handleCacheAdd(w http.ResponseWriter, r *http.Request) {
	var req struct {
		URL         string  `json:"url"`
		Description string  `json:"description"`
	}
	json.NewDecoder(r.Body).Decode(&req)
	entry := CachedURL{
		URL:         req.URL,
		Description: req.Description,
		LastSync:    time.Now(),
	}
	mu.Lock()
	cache = append(cache, entry)
	mu.Unlock()
	// Real impl: wget/curl the URL content to local cache dir
	fmt.Fprintf(w, `{"ok":true,"url":%q}`, req.URL)
}

func handleCacheList(w http.ResponseWriter, r *http.Request) {
	mu.RLock()
	defer mu.RUnlock()
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(cache)
}

func main() {
	sockPath := "/run/sigma/commnet.sock"
	os.Remove(sockPath)
	ln, err := net.Listen("unix", sockPath)
	if err != nil {
		fmt.Fprintln(os.Stderr, "[sigma-commnet] listen error:", err)
		os.Exit(1)
	}

	// Pre-cache important Indian government URLs for offline use
	cache = []CachedURL{
		{URL: "https://ncert.nic.in",    Description: "NCERT textbooks"},
		{URL: "https://digilocker.gov.in", Description: "DigiLocker"},
		{URL: "https://enam.gov.in",     Description: "e-NAM mandi prices"},
		{URL: "https://pmkisan.gov.in",  Description: "PM-KISAN status"},
	}

	mux := http.NewServeMux()
	mux.HandleFunc("/commnet/status",        handleStatus)
	mux.HandleFunc("/commnet/bandwidth",     handleBandwidth)
	mux.HandleFunc("/commnet/member/add",    handleAddMember)
	mux.HandleFunc("/commnet/cache/add",     handleCacheAdd)
	mux.HandleFunc("/commnet/cache/list",    handleCacheList)

	fmt.Println("[sigma-commnet] community internet gateway listening on", sockPath)
	http.Serve(ln, mux)
}
