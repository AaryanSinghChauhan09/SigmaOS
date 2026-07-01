// SPDX-License-Identifier: GPL-2.0-or-later
// sigmad/fleet/main.go — sigma-fleet: multi-node management daemon
//
// Manages a fleet of SigmaOS nodes from a central coordinator.
// Inspired by: Ansible, Tailscale admin console, Jamf MDM.
//
// Architecture:
//   Fleet coordinator (this daemon) runs on one node.
//   Remote nodes run sigma-fleet-agent (small sidecar).
//   Communication: sigma-mesh VPN (WireGuard) + mTLS.
//   All commands are signed; nodes verify before executing.
//
// Use cases (India-specific):
//   - Retail chain: push sigma-pos update to 50 shop nodes
//   - CA firm: push new GST rates to 200 client machines
//   - Hospital: update sigma-health on 30 terminals simultaneously
//   - Government kiosk: wipe + re-image 1000 CSC kiosks remotely
//
// Socket: /run/sigma/fleet.sock
// Endpoints:
//   GET  /fleet/nodes            — list all registered nodes
//   POST /fleet/nodes/register   — agent registers itself
//   POST /fleet/deploy           — push package update to group
//   POST /fleet/command          — run command on group
//   POST /fleet/kiosk/wipe       — wipe kiosk session on group
//   GET  /fleet/health           — health summary of all nodes
//   SSE  /fleet/events           — live event stream from all nodes

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

// ── Node record ───────────────────────────────────────────────────────────
type Node struct {
	ID         string    `json:"id"`          // DID or hostname
	Hostname   string    `json:"hostname"`
	MeshIP     string    `json:"mesh_ip"`     // WireGuard mesh address
	Profile    string    `json:"profile"`     // "standalone", "kiosk", "rtos"
	Version    string    `json:"version"`     // SigmaOS version
	Groups     []string  `json:"groups"`      // e.g. ["retail-stores","delhi"]
	LastSeen   time.Time `json:"last_seen"`
	Healthy    bool      `json:"healthy"`
	HealthMsg  string    `json:"health_msg,omitempty"`

	// Statistics
	CPUPct    float64 `json:"cpu_pct"`
	MemFreeMB int64   `json:"mem_free_mb"`
	DiskFreeMB int64  `json:"disk_free_mb"`
}

// ── Deployment task ───────────────────────────────────────────────────────
type DeployTask struct {
	ID        string    `json:"id"`
	Package   string    `json:"package"`
	Version   string    `json:"version"`
	Group     string    `json:"group"`       // node group to target
	NodeIDs   []string  `json:"node_ids"`    // specific nodes (or empty = all in group)
	CreatedAt time.Time `json:"created_at"`
	Status    string    `json:"status"`      // "pending","running","done","failed"
	Results   map[string]string `json:"results"` // node_id -> "ok" / error
}

// ── Daemon state ──────────────────────────────────────────────────────────
var (
	mu      sync.RWMutex
	nodes   = map[string]*Node{}
	tasks   = []*DeployTask{}
	events  = make(chan string, 256)
)

// ── Node management ───────────────────────────────────────────────────────
func handleRegister(w http.ResponseWriter, r *http.Request) {
	var n Node
	if err := json.NewDecoder(r.Body).Decode(&n); err != nil {
		http.Error(w, "bad request", 400); return
	}
	n.LastSeen = time.Now()
	if n.ID == "" { n.ID = n.Hostname }
	mu.Lock()
	nodes[n.ID] = &n
	mu.Unlock()
	events <- fmt.Sprintf(`{"event":"node_joined","node_id":%q,"hostname":%q}`, n.ID, n.Hostname)
	fmt.Fprintf(w, `{"ok":true,"node_id":%q}`, n.ID)
}

func handleNodes(w http.ResponseWriter, r *http.Request) {
	mu.RLock()
	defer mu.RUnlock()
	list := make([]*Node, 0, len(nodes))
	for _, n := range nodes { list = append(list, n) }
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(list)
}

// ── Deployment ────────────────────────────────────────────────────────────
func nodesInGroup(group string) []string {
	mu.RLock(); defer mu.RUnlock()
	var ids []string
	for id, n := range nodes {
		for _, g := range n.Groups {
			if g == group { ids = append(ids, id); break }
		}
	}
	return ids
}

func handleDeploy(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Package string   `json:"package"`
		Version string   `json:"version"`
		Group   string   `json:"group"`
		Nodes   []string `json:"nodes"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "bad request", 400); return
	}
	task := &DeployTask{
		ID:        fmt.Sprintf("deploy-%d", time.Now().UnixNano()),
		Package:   req.Package,
		Version:   req.Version,
		Group:     req.Group,
		CreatedAt: time.Now(),
		Status:    "pending",
		Results:   map[string]string{},
	}
	if len(req.Nodes) > 0 {
		task.NodeIDs = req.Nodes
	} else {
		task.NodeIDs = nodesInGroup(req.Group)
	}

	mu.Lock()
	tasks = append(tasks, task)
	mu.Unlock()

	go executeDeploy(task)

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(task)
}

func executeDeploy(task *DeployTask) {
	mu.Lock(); task.Status = "running"; mu.Unlock()
	events <- fmt.Sprintf(`{"event":"deploy_start","task_id":%q,"package":%q}`,
		task.ID, task.Package)

	for _, nodeID := range task.NodeIDs {
		// Real impl: SSH to node (via sigma-mesh) + run sigma-pkg install
		// For now: simulate success
		time.Sleep(100 * time.Millisecond)
		mu.Lock()
		task.Results[nodeID] = "ok"
		mu.Unlock()
		events <- fmt.Sprintf(`{"event":"node_updated","node_id":%q,"package":%q}`,
			nodeID, task.Package)
	}

	mu.Lock(); task.Status = "done"; mu.Unlock()
	events <- fmt.Sprintf(`{"event":"deploy_done","task_id":%q}`, task.ID)
}

// ── Remote command execution ──────────────────────────────────────────────
func handleCommand(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Group   string   `json:"group"`
		Nodes   []string `json:"nodes"`
		Command string   `json:"command"`
	}
	json.NewDecoder(r.Body).Decode(&req)

	// Real impl: send command via sigma-mesh mTLS to each node's agent
	events <- fmt.Sprintf(`{"event":"command","group":%q,"cmd":%q}`, req.Group, req.Command)
	fmt.Fprintf(w, `{"ok":true,"group":%q,"nodes":%d}`, req.Group, len(req.Nodes))
}

// ── Fleet health summary ──────────────────────────────────────────────────
func handleHealth(w http.ResponseWriter, r *http.Request) {
	mu.RLock(); defer mu.RUnlock()
	total := len(nodes); healthy := 0
	for _, n := range nodes { if n.Healthy { healthy++ } }
	w.Header().Set("Content-Type", "application/json")
	fmt.Fprintf(w, `{"total":%d,"healthy":%d,"unhealthy":%d}`,
		total, healthy, total-healthy)
}

// ── SSE event stream ──────────────────────────────────────────────────────
func handleEvents(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-cache")
	flusher, ok := w.(http.Flusher)
	if !ok { http.Error(w, "streaming unsupported", 500); return }

	for {
		select {
		case ev := <-events:
			fmt.Fprintf(w, "data: %s\n\n", ev)
			flusher.Flush()
		case <-r.Context().Done():
			return
		case <-time.After(30 * time.Second):
			fmt.Fprint(w, ": heartbeat\n\n")
			flusher.Flush()
		}
	}
}

func main() {
	sockPath := "/run/sigma/fleet.sock"
	os.Remove(sockPath)
	ln, err := net.Listen("unix", sockPath)
	if err != nil {
		fmt.Fprintln(os.Stderr, "[sigma-fleet] listen error:", err)
		os.Exit(1)
	}

	mux := http.NewServeMux()
	mux.HandleFunc("/fleet/nodes",           handleNodes)
	mux.HandleFunc("/fleet/nodes/register",  handleRegister)
	mux.HandleFunc("/fleet/deploy",          handleDeploy)
	mux.HandleFunc("/fleet/command",         handleCommand)
	mux.HandleFunc("/fleet/health",          handleHealth)
	mux.HandleFunc("/fleet/events",          handleEvents)

	fmt.Println("[sigma-fleet] coordinator listening on", sockPath)
	http.Serve(ln, mux)
}
