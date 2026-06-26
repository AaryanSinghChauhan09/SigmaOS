// SPDX-License-Identifier: GPL-2.0-or-later
// sigmad/webhook/main.go — Event-driven webhook dispatcher
//
// Subscribes to sigma-ds events and POSTs JSON payloads to registered URLs.
// Usage:
//   sigma-webhook register --event service.crashed --url https://hooks.slack.com/...
//   sigma-webhook list
//   sigma-webhook delete <id>

package main

import (
	"bytes"
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"net"
	"net/http"
	"os"
	"sync"
	"time"
)

type WebhookRegistration struct {
	ID        string    `json:"id"`
	Event     string    `json:"event"`     // "service.crashed" | "pkg.installed" | "*"
	URL       string    `json:"url"`
	Secret    string    `json:"secret"`    // HMAC-SHA256 signing key
	CreatedAt time.Time `json:"created_at"`
	Retries   int       `json:"retries"`
}

type WebhookEvent struct {
	ID        string      `json:"id"`
	Event     string      `json:"event"`
	Timestamp string      `json:"timestamp"`
	Payload   interface{} `json:"payload"`
}

var (
	hooks   []WebhookRegistration
	hooksMu sync.RWMutex
	hooksFile = "/sigma/data/webhooks.json"
)

func loadHooks() {
	b, err := os.ReadFile(hooksFile)
	if err != nil { return }
	json.Unmarshal(b, &hooks)
}

func saveHooks() {
	b, _ := json.MarshalIndent(hooks, "", "  ")
	os.WriteFile(hooksFile, b, 0o600)
}

func dispatch(event WebhookEvent) {
	hooksMu.RLock()
	matching := []WebhookRegistration{}
	for _, h := range hooks {
		if h.Event == "*" || h.Event == event.Event { matching = append(matching, h) }
	}
	hooksMu.RUnlock()

	for _, h := range matching {
		go func(hook WebhookRegistration) {
			body, _ := json.Marshal(event)

			// Sign with HMAC-SHA256 if secret is set
			sig := ""
			if hook.Secret != "" {
				mac := hmac.New(sha256.New, []byte(hook.Secret))
				mac.Write(body)
				sig = "sha256=" + hex.EncodeToString(mac.Sum(nil))
			}

			for attempt := 0; attempt <= hook.Retries; attempt++ {
				req, _ := http.NewRequest("POST", hook.URL, bytes.NewReader(body))
				req.Header.Set("Content-Type", "application/json")
				req.Header.Set("X-Sigma-Event",     event.Event)
				req.Header.Set("X-Sigma-Delivery",  event.ID)
				req.Header.Set("X-Sigma-Timestamp", event.Timestamp)
				if sig != "" { req.Header.Set("X-Sigma-Signature-256", sig) }

				client := &http.Client{Timeout: 10 * time.Second}
				resp, err := client.Do(req)
				if err == nil && resp.StatusCode < 300 {
					fmt.Printf("[webhook] delivered %s → %s (attempt %d)\n",
						event.Event, hook.URL, attempt+1)
					return
				}
				if attempt < hook.Retries {
					time.Sleep(time.Duration(1<<uint(attempt)) * time.Second) // exp backoff
				}
			}
			fmt.Fprintf(os.Stderr, "[webhook] FAILED to deliver %s → %s after %d attempts\n",
				event.Event, hook.URL, hook.Retries+1)
		}(h)
	}
}

func main() {
	loadHooks()
	sockPath := "/run/sigma/webhook.sock"
	os.Remove(sockPath)
	ln, _ := net.Listen("unix", sockPath)

	mux := http.NewServeMux()

	mux.HandleFunc("/webhook/register", func(w http.ResponseWriter, r *http.Request) {
		var h WebhookRegistration
		json.NewDecoder(r.Body).Decode(&h)
		h.ID = fmt.Sprintf("wh_%d", time.Now().UnixNano())
		h.CreatedAt = time.Now().UTC()
		if h.Retries == 0 { h.Retries = 3 }
		hooksMu.Lock(); hooks = append(hooks, h); saveHooks(); hooksMu.Unlock()
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(h)
	})

	mux.HandleFunc("/webhook/list", func(w http.ResponseWriter, r *http.Request) {
		hooksMu.RLock(); defer hooksMu.RUnlock()
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(hooks)
	})

	mux.HandleFunc("/webhook/fire", func(w http.ResponseWriter, r *http.Request) {
		var ev WebhookEvent
		json.NewDecoder(r.Body).Decode(&ev)
		ev.ID = fmt.Sprintf("evt_%d", time.Now().UnixNano())
		if ev.Timestamp == "" { ev.Timestamp = time.Now().UTC().Format(time.RFC3339) }
		go dispatch(ev)
		w.Header().Set("Content-Type", "application/json")
		fmt.Fprintf(w, `{"ok":true,"id":"%s"}`, ev.ID)
	})

	fmt.Println("[sigma-webhook] listening on", sockPath)
	http.Serve(ln, mux)
}
