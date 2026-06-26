// SPDX-License-Identifier: GPL-2.0-or-later
// sigmad/ds/main.go — Service Discovery Data Store (MINIX 3 ds server-inspired)
//
// Replaces hardcoded socket paths and PIDs in every daemon.
// Services publish: sigma-ds publish sigma.trustd.socket /run/sigma/trustd.sock
// Services query:   sigma-ds get sigma.trustd.socket
// Services watch:   sigma-ds subscribe sigma.pkg.version
//
// sigma-init populates well-known entries as each service becomes ready.

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

// ── Types ─────────────────────────────────────────────────────────────────────

type DSEntry struct {
	Key       string `json:"key"`
	Value     string `json:"value"`
	Publisher string `json:"publisher"`
	UpdatedAt string `json:"updated_at"`
	TTL_ms    int64  `json:"ttl_ms"` // 0 = permanent
}

type DSRequest struct {
	Op    string `json:"op"`    // "publish" | "get" | "delete" | "list"
	Key   string `json:"key"`
	Value string `json:"value"`
	From  string `json:"from"` // caller's service name
}

type DSResponse struct {
	OK    bool     `json:"ok"`
	Value string   `json:"value,omitempty"`
	Keys  []string `json:"keys,omitempty"`
	Error string   `json:"error,omitempty"`
}

// ── Store ────────────────────────────────────────────────────────────────────

var (
	store   = make(map[string]DSEntry)
	storeMu sync.RWMutex
	watches = make(map[string][]chan string) // key → list of watchers
)

func dsPublish(key, value, publisher string) {
	storeMu.Lock()
	defer storeMu.Unlock()
	store[key] = DSEntry{
		Key:       key,
		Value:     value,
		Publisher: publisher,
		UpdatedAt: time.Now().UTC().Format(time.RFC3339),
	}
	// Notify watchers
	if chs, ok := watches[key]; ok {
		for _, ch := range chs {
			select {
			case ch <- value:
			default:
			}
		}
	}
}

func dsGet(key string) (string, bool) {
	storeMu.RLock()
	defer storeMu.RUnlock()
	e, ok := store[key]
	return e.Value, ok
}

// ── HTTP handlers ─────────────────────────────────────────────────────────────

func handleDS(w http.ResponseWriter, r *http.Request) {
	var req DSRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		json.NewEncoder(w).Encode(DSResponse{Error: err.Error()})
		return
	}

	var resp DSResponse
	switch req.Op {
	case "publish":
		dsPublish(req.Key, req.Value, req.From)
		resp.OK = true
	case "get":
		v, ok := dsGet(req.Key)
		if !ok {
			resp.Error = "key not found: " + req.Key
		} else {
			resp.OK = true
			resp.Value = v
		}
	case "delete":
		storeMu.Lock()
		delete(store, req.Key)
		storeMu.Unlock()
		resp.OK = true
	case "list":
		storeMu.RLock()
		for k := range store {
			resp.Keys = append(resp.Keys, k)
		}
		storeMu.RUnlock()
		resp.OK = true
	default:
		resp.Error = "unknown op: " + req.Op
	}
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(resp)
}

// ── Main ──────────────────────────────────────────────────────────────────────

func main() {
	sockPath := "/run/sigma/ds.sock"
	os.Remove(sockPath)

	ln, err := net.Listen("unix", sockPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "[sigma-ds] listen failed: %v\n", err)
		os.Exit(1)
	}

	// Bootstrap well-known entries
	dsPublish("sigma.ds.version",     "1.0", "sigma-ds")
	dsPublish("sigma.ds.socket",      sockPath, "sigma-ds")
	dsPublish("sigma.kernel.version", "SigmaOS 0.1.0-alpha", "sigma-ds")

	mux := http.NewServeMux()
	mux.HandleFunc("/ds", handleDS)
	mux.HandleFunc("/ds/health", func(w http.ResponseWriter, r *http.Request) {
		storeMu.RLock()
		count := len(store)
		storeMu.RUnlock()
		fmt.Fprintf(w, `{"ok":true,"entries":%d}`, count)
	})

	fmt.Println("[sigma-ds] listening on", sockPath)
	http.Serve(ln, mux)
}
