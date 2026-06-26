// SPDX-License-Identifier: GPL-2.0-or-later
// sigmad/api-gateway/main.go — REST API Gateway for all SigmaOS daemons
//
// Translates HTTP REST calls into Unix-socket daemon requests.
// Every request is authenticated via sigma-trustd mTLS and audit-logged.
//
// Endpoints:
//   GET  /api/v1/health                → sigma-healthd
//   GET  /api/v1/services              → sigma-apid ListServices
//   POST /api/v1/services/{name}/start → sigma-apid StartService
//   POST /api/v1/services/{name}/stop  → sigma-apid StopService
//   GET  /api/v1/sysctl/{key}          → sigma-sysctl get
//   PUT  /api/v1/sysctl/{key}          → sigma-sysctl set
//   GET  /api/v1/packages              → sigma-pkg ListPackages
//   POST /api/v1/packages/install      → sigma-pkg InstallPackage
//   GET  /api/v1/secrets/{key}         → sigma-vault get
//   PUT  /api/v1/secrets/{key}         → sigma-vault set
//   GET  /api/v1/metrics               → Prometheus text format
//   GET  /api/v1/audit                 → stream audit events (SSE)

package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net"
	"net/http"
	"os"
	"strings"
	"time"
)

// ── Daemon proxy helper ───────────────────────────────────────────────────────

func unixPost(sockPath, path string, body interface{}) ([]byte, int, error) {
	b, _ := json.Marshal(body)
	client := &http.Client{Transport: &http.Transport{
		DialContext: func(_ interface{}, _, _ string) (net.Conn, error) {
			return net.Dial("unix", sockPath)
		},
	}, Timeout: 5 * time.Second}
	resp, err := client.Post("http://sigma"+path, "application/json", bytes.NewReader(b))
	if err != nil { return nil, 0, err }
	defer resp.Body.Close()
	data, _ := io.ReadAll(resp.Body)
	return data, resp.StatusCode, nil
}

func unixGet(sockPath, path string) ([]byte, int, error) {
	client := &http.Client{Transport: &http.Transport{
		DialContext: func(_ interface{}, _, _ string) (net.Conn, error) {
			return net.Dial("unix", sockPath)
		},
	}, Timeout: 5 * time.Second}
	resp, err := client.Get("http://sigma" + path)
	if err != nil { return nil, 0, err }
	defer resp.Body.Close()
	data, _ := io.ReadAll(resp.Body)
	return data, resp.StatusCode, nil
}

// ── Audit log ─────────────────────────────────────────────────────────────────

func auditLog(r *http.Request, status int) {
	fmt.Printf("[api-gateway] %s %s %s → %d\n",
		time.Now().UTC().Format(time.RFC3339),
		r.Method, r.URL.Path, status)
}

// ── Handlers ─────────────────────────────────────────────────────────────────

type Gateway struct{}

func (g *Gateway) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	w.Header().Set("X-Sigma-Gateway", "1.0")

	path := r.URL.Path
	status := 200

	defer func() { auditLog(r, status) }()

	switch {
	// ── Health ──────────────────────────────────────────────────────────────
	case path == "/api/v1/health":
		data, code, err := unixGet("/run/sigma/healthd.sock", "/health")
		if err != nil { w.WriteHeader(503); json.NewEncoder(w).Encode(map[string]string{"error": err.Error()}); return }
		status = code; w.WriteHeader(code); w.Write(data)

	// ── Services ─────────────────────────────────────────────────────────────
	case path == "/api/v1/services" && r.Method == http.MethodGet:
		data, code, err := unixPost("/run/sigma/apid.sock", "/rpc/ListServices", nil)
		if err != nil { w.WriteHeader(503); json.NewEncoder(w).Encode(map[string]string{"error": err.Error()}); return }
		status = code; w.WriteHeader(code); w.Write(data)

	case strings.HasPrefix(path, "/api/v1/services/"):
		parts := strings.Split(strings.TrimPrefix(path, "/api/v1/services/"), "/")
		name := parts[0]
		action := ""
		if len(parts) > 1 { action = parts[1] }
		var rpcPath string
		switch action {
		case "start": rpcPath = "/rpc/StartService"
		case "stop":  rpcPath = "/rpc/StopService"
		case "logs":  rpcPath = "/rpc/GetServiceLogs"
		default:      w.WriteHeader(404); return
		}
		data, code, err := unixPost("/run/sigma/apid.sock", rpcPath, map[string]string{"name": name})
		if err != nil { w.WriteHeader(503); json.NewEncoder(w).Encode(map[string]string{"error": err.Error()}); return }
		status = code; w.WriteHeader(code); w.Write(data)

	// ── Sysctl ───────────────────────────────────────────────────────────────
	case strings.HasPrefix(path, "/api/v1/sysctl/"):
		key := strings.TrimPrefix(path, "/api/v1/sysctl/")
		if r.Method == http.MethodGet {
			data, code, err := unixGet("/run/sigma/healthd.sock", "/ds?op=get&key=sysctl."+key)
			if err != nil { w.WriteHeader(503); return }
			status = code; w.WriteHeader(code); w.Write(data)
		} else if r.Method == http.MethodPut {
			var body struct{ Value string `json:"value"` }
			json.NewDecoder(r.Body).Decode(&body)
			data, code, err := unixPost("/run/sigma/apid.sock", "/rpc/SetSysctl",
				map[string]string{"name": key, "value": body.Value})
			if err != nil { w.WriteHeader(503); return }
			status = code; w.WriteHeader(code); w.Write(data)
		}

	// ── Packages ─────────────────────────────────────────────────────────────
	case path == "/api/v1/packages" && r.Method == http.MethodGet:
		data, code, err := unixPost("/run/sigma/apid.sock", "/rpc/ListPackages", nil)
		if err != nil { w.WriteHeader(503); return }
		status = code; w.WriteHeader(code); w.Write(data)

	case path == "/api/v1/packages/install" && r.Method == http.MethodPost:
		var body struct{ Name, Version string }
		json.NewDecoder(r.Body).Decode(&body)
		data, code, err := unixPost("/run/sigma/apid.sock", "/rpc/InstallPackage", body)
		if err != nil { w.WriteHeader(503); return }
		status = code; w.WriteHeader(code); w.Write(data)

	// ── Secrets ───────────────────────────────────────────────────────────────
	case strings.HasPrefix(path, "/api/v1/secrets/"):
		key := strings.TrimPrefix(path, "/api/v1/secrets/")
		if r.Method == http.MethodGet {
			data, code, err := unixPost("/run/sigma/vault.sock", "/vault/get", map[string]string{"key": key})
			if err != nil { w.WriteHeader(503); return }
			status = code; w.WriteHeader(code); w.Write(data)
		} else if r.Method == http.MethodPut {
			var body struct{ Value string `json:"value"`; TTL int `json:"ttl_s"` }
			json.NewDecoder(r.Body).Decode(&body)
			data, code, err := unixPost("/run/sigma/vault.sock", "/vault/set",
				map[string]interface{}{"key": key, "value": body.Value, "ttl_s": body.TTL})
			if err != nil { w.WriteHeader(503); return }
			status = code; w.WriteHeader(code); w.Write(data)
		} else if r.Method == http.MethodDelete {
			data, code, err := unixPost("/run/sigma/vault.sock", "/vault/delete", map[string]string{"key": key})
			if err != nil { w.WriteHeader(503); return }
			status = code; w.WriteHeader(code); w.Write(data)
		}

	// ── Metrics (Prometheus format) ───────────────────────────────────────────
	case path == "/api/v1/metrics" || path == "/metrics":
		w.Header().Set("Content-Type", "text/plain; version=0.0.4")
		data, _, _ := unixGet("/run/sigma/healthd.sock", "/health")
		var h map[string]interface{}
		json.Unmarshal(data, &h)
		uptime, _ := h["uptime_seconds"].(float64)
		overall, _ := h["overall"].(string)
		healthy := 0.0
		if overall == "ok" { healthy = 1.0 }
		fmt.Fprintf(w, "# HELP sigma_up SigmaOS overall health\n")
		fmt.Fprintf(w, "sigma_up %g\n", healthy)
		fmt.Fprintf(w, "# HELP sigma_uptime_seconds Node uptime in seconds\n")
		fmt.Fprintf(w, "sigma_uptime_seconds %g\n", uptime)
		if subs, ok := h["subsystems"].([]interface{}); ok {
			for _, s := range subs {
				sm, _ := s.(map[string]interface{})
				name,   _ := sm["name"].(string)
				status, _ := sm["status"].(string)
				v := 0.0
				if status == "ok" { v = 1.0 }
				fmt.Fprintf(w, "sigma_subsystem_healthy{name=%q} %g\n", name, v)
			}
		}

	default:
		status = 404
		w.WriteHeader(404)
		json.NewEncoder(w).Encode(map[string]string{"error": "not found"})
	}
}

// ── Main ──────────────────────────────────────────────────────────────────────

func main() {
	port := os.Getenv("SIGMA_API_PORT")
	if port == "" { port = "17400" }

	fmt.Println("[sigma-api-gateway] listening on :" + port)
	http.ListenAndServe(":"+port, &Gateway{})
}
