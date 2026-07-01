// SPDX-License-Identifier: GPL-2.0-or-later
// sigmad/indexd/main.go — Attribute Index Server (Haiku index server-inspired)
//
// Maintains B-tree indexes for SemanticFS inode attributes.
// Notified by kernel on every attribute change; answers queries in O(log n).
// Socket: /run/sigma/indexd.sock
//
// Query API:
//   POST /indexd/query   {"conditions":[{"attr":"SIGMA:CLASS","op":"=","val":"2"},...]
//   POST /indexd/notify  {"inode":42,"attr":"SIGMA:TRUST","value":"5"}

package main

import (
	"encoding/json"
	"fmt"
	"net"
	"net/http"
	"os"
	"sort"
	"sync"
)

// ── Index structures ──────────────────────────────────────────────────────────

type IndexKey struct {
	Value  string
	Inodes []uint64
}

type AttributeIndex struct {
	AttrName string
	keys     []IndexKey // sorted by Value for binary search
	mu       sync.RWMutex
}

func (idx *AttributeIndex) add(inode uint64, value string) {
	idx.mu.Lock()
	defer idx.mu.Unlock()
	// Find or insert entry for this value
	i := sort.Search(len(idx.keys), func(i int) bool { return idx.keys[i].Value >= value })
	if i < len(idx.keys) && idx.keys[i].Value == value {
		idx.keys[i].Inodes = append(idx.keys[i].Inodes, inode)
	} else {
		entry := IndexKey{Value: value, Inodes: []uint64{inode}}
		idx.keys = append(idx.keys, entry)
		copy(idx.keys[i+1:], idx.keys[i:])
		idx.keys[i] = entry
	}
}

func (idx *AttributeIndex) lookup(value string) []uint64 {
	idx.mu.RLock()
	defer idx.mu.RUnlock()
	i := sort.Search(len(idx.keys), func(i int) bool { return idx.keys[i].Value >= value })
	if i < len(idx.keys) && idx.keys[i].Value == value {
		out := make([]uint64, len(idx.keys[i].Inodes))
		copy(out, idx.keys[i].Inodes)
		return out
	}
	return nil
}

// ── Registry ──────────────────────────────────────────────────────────────────

var (
	indices = map[string]*AttributeIndex{
		"SIGMA:TRUST":  {AttrName: "SIGMA:TRUST"},
		"SIGMA:CLASS":  {AttrName: "SIGMA:CLASS"},
		"SIGMA:SIGNER": {AttrName: "SIGMA:SIGNER"},
		"SIGMA:CREATOR":{AttrName: "SIGMA:CREATOR"},
		"SIGMA:MIME":   {AttrName: "SIGMA:MIME"},
	}
)

// ── HTTP handlers ─────────────────────────────────────────────────────────────

type Condition struct {
	Attr string `json:"attr"`
	Op   string `json:"op"`
	Val  string `json:"val"`
}

func handleQuery(w http.ResponseWriter, r *http.Request) {
	var req struct{ Conditions []Condition `json:"conditions"` }
	json.NewDecoder(r.Body).Decode(&req)

	if len(req.Conditions) == 0 { w.WriteHeader(400); return }

	// For each condition, get matching inode set; intersect all sets
	type inodeSet map[uint64]int
	counts := make(inodeSet)

	for _, cond := range req.Conditions {
		idx, ok := indices[cond.Attr]
		if !ok { continue }
		var matches []uint64
		if cond.Op == "=" || cond.Op == "eq" {
			matches = idx.lookup(cond.Val)
		}
		for _, inode := range matches { counts[inode]++ }
	}

	var result []uint64
	for inode, count := range counts {
		if count == len(req.Conditions) { result = append(result, inode) }
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{
		"count":  len(result),
		"inodes": result,
	})
}

func handleNotify(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Inode uint64 `json:"inode"`
		Attr  string `json:"attr"`
		Value string `json:"value"`
	}
	json.NewDecoder(r.Body).Decode(&req)

	if idx, ok := indices[req.Attr]; ok {
		idx.add(req.Inode, req.Value)
	}
	fmt.Fprintln(w, `{"ok":true}`)
}

// ── Main ──────────────────────────────────────────────────────────────────────

func main() {
	sockPath := "/run/sigma/indexd.sock"
	os.Remove(sockPath)
	ln, err := net.Listen("unix", sockPath)
	if err != nil { fmt.Fprintln(os.Stderr, err); os.Exit(1) }

	mux := http.NewServeMux()
	mux.HandleFunc("/indexd/query",  handleQuery)
	mux.HandleFunc("/indexd/notify", handleNotify)
	mux.HandleFunc("/indexd/health", func(w http.ResponseWriter, r *http.Request) {
		total := 0
		for _, idx := range indices { total += len(idx.keys) }
		fmt.Fprintf(w, `{"ok":true,"indexed_values":%d}`, total)
	})

	fmt.Println("[sigma-indexd] listening on", sockPath)
	http.Serve(ln, mux)
}
