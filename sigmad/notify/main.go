// SPDX-License-Identifier: GPL-2.0-or-later
// sigma-notify: desktop notification daemon for SigmaOS
// Inspired by: freedesktop.org org.freedesktop.Notifications DBus spec,
//              dunst, mako — but using a Unix socket instead of DBus.

package main

import (
	"encoding/json"
	"log"
	"net"
	"os"
	"sync"
	"sync/atomic"
	"time"
)

// ─── Constants ───────────────────────────────────────────────────────────────

const (
	socketPath    = "/run/sigma/notifyd.sock"
	maxBodyLen    = 512  // max notification body bytes
	maxSummaryLen = 128  // max summary bytes
	maxAppLen     = 64   // max app name bytes
	maxPending    = 256  // max queued notifications
	defaultExpiry = 5000 // ms — auto-dismiss after 5 s if urgency < Critical
)

// ─── Urgency levels (matches freedesktop spec) ───────────────────────────────

type Urgency uint8

const (
	UrgencyLow      Urgency = 0
	UrgencyNormal   Urgency = 1
	UrgencyCritical Urgency = 2
)

// ─── Wire protocol ───────────────────────────────────────────────────────────

// NotifyRequest is sent by clients to trigger a notification.
type NotifyRequest struct {
	Op      string `json:"op"` // "notify" | "dismiss" | "list" | "history"
	AppName string `json:"app_name"`
	Summary string `json:"summary"`
	Body    string `json:"body"`
	Urgency uint8  `json:"urgency"` // 0=low 1=normal 2=critical
	Timeout int    `json:"timeout"` // ms; -1=persistent, 0=use default
	ID      uint32 `json:"id"`      // non-zero to replace existing notification
}

// NotifyResponse is returned to the client.
type NotifyResponse struct {
	OK      bool   `json:"ok"`
	Error   string `json:"error,omitempty"`
	ID      uint32 `json:"id,omitempty"`
	History []Notification `json:"history,omitempty"`
}

// Notification is an active or historical notification record.
type Notification struct {
	ID        uint32    `json:"id"`
	AppName   string    `json:"app_name"`
	Summary   string    `json:"summary"`
	Body      string    `json:"body"`
	Urgency   Urgency   `json:"urgency"`
	CreatedAt time.Time `json:"created_at"`
	ExpiresAt time.Time `json:"expires_at,omitempty"`
	Dismissed bool      `json:"dismissed"`
}

// ─── Daemon state ─────────────────────────────────────────────────────────────

type daemon struct {
	mu       sync.RWMutex
	active   map[uint32]*Notification // id → live notification
	history  []*Notification          // last 128 dismissed
	nextID   atomic.Uint32
	pending  chan *Notification
	dismiss  chan uint32
}

func newDaemon() *daemon {
	d := &daemon{
		active:  make(map[uint32]*Notification),
		pending: make(chan *Notification, maxPending),
		dismiss: make(chan uint32, maxPending),
	}
	d.nextID.Store(1)
	return d
}

// ─── Expiry watchdog ──────────────────────────────────────────────────────────

func (d *daemon) expireLoop() {
	ticker := time.NewTicker(500 * time.Millisecond)
	defer ticker.Stop()
	for range ticker.C {
		now := time.Now()
		d.mu.Lock()
		for id, n := range d.active {
			if !n.ExpiresAt.IsZero() && now.After(n.ExpiresAt) {
				n.Dismissed = true
				d.history = append(d.history, n)
				if len(d.history) > 128 {
					d.history = d.history[len(d.history)-128:]
				}
				delete(d.active, id)
				log.Printf("[notifyd] auto-expired id=%d app=%q summary=%q", id, n.AppName, n.Summary)
			}
		}
		d.mu.Unlock()
	}
}

// ─── Request handler ─────────────────────────────────────────────────────────

func (d *daemon) handle(req *NotifyRequest) NotifyResponse {
	switch req.Op {
	case "notify":
		return d.opNotify(req)
	case "dismiss":
		return d.opDismiss(req.ID)
	case "list":
		return d.opList()
	case "history":
		return d.opHistory()
	default:
		return NotifyResponse{Error: "unknown op: " + req.Op}
	}
}

func (d *daemon) opNotify(req *NotifyRequest) NotifyResponse {
	// Sanitise inputs — reject oversized fields
	if len(req.Summary) > maxSummaryLen {
		return NotifyResponse{Error: "summary too long"}
	}
	if len(req.Body) > maxBodyLen {
		return NotifyResponse{Error: "body too long"}
	}
	if len(req.AppName) > maxAppLen {
		return NotifyResponse{Error: "app_name too long"}
	}
	if req.Summary == "" {
		return NotifyResponse{Error: "summary required"}
	}

	urgency := Urgency(req.Urgency)
	if urgency > UrgencyCritical {
		urgency = UrgencyNormal
	}

	expiry := time.Duration(req.Timeout) * time.Millisecond
	if req.Timeout == 0 {
		if urgency < UrgencyCritical {
			expiry = time.Duration(defaultExpiry) * time.Millisecond
		}
		// Critical notifications persist until dismissed
	}

	d.mu.Lock()
	defer d.mu.Unlock()

	// Replace existing if ID supplied
	var id uint32
	if req.ID != 0 {
		if existing, ok := d.active[req.ID]; ok {
			existing.Summary = req.Summary
			existing.Body = req.Body
			existing.Urgency = urgency
			existing.CreatedAt = time.Now()
			if expiry > 0 {
				existing.ExpiresAt = time.Now().Add(expiry)
			}
			log.Printf("[notifyd] replaced id=%d app=%q", req.ID, req.AppName)
			return NotifyResponse{OK: true, ID: req.ID}
		}
		id = req.ID
	} else {
		id = d.nextID.Add(1)
	}

	n := &Notification{
		ID:        id,
		AppName:   req.AppName,
		Summary:   req.Summary,
		Body:      req.Body,
		Urgency:   urgency,
		CreatedAt: time.Now(),
	}
	if expiry > 0 {
		n.ExpiresAt = time.Now().Add(expiry)
	}
	d.active[id] = n
	log.Printf("[notifyd] notify id=%d urgency=%d app=%q summary=%q", id, urgency, req.AppName, req.Summary)
	return NotifyResponse{OK: true, ID: id}
}

func (d *daemon) opDismiss(id uint32) NotifyResponse {
	d.mu.Lock()
	defer d.mu.Unlock()
	n, ok := d.active[id]
	if !ok {
		return NotifyResponse{Error: "id not found"}
	}
	n.Dismissed = true
	d.history = append(d.history, n)
	if len(d.history) > 128 {
		d.history = d.history[len(d.history)-128:]
	}
	delete(d.active, id)
	log.Printf("[notifyd] dismissed id=%d", id)
	return NotifyResponse{OK: true}
}

func (d *daemon) opList() NotifyResponse {
	d.mu.RLock()
	defer d.mu.RUnlock()
	list := make([]Notification, 0, len(d.active))
	for _, n := range d.active {
		list = append(list, *n)
	}
	return NotifyResponse{OK: true, History: list}
}

func (d *daemon) opHistory() NotifyResponse {
	d.mu.RLock()
	defer d.mu.RUnlock()
	hist := make([]Notification, len(d.history))
	for i, n := range d.history {
		hist[i] = *n
	}
	return NotifyResponse{OK: true, History: hist}
}

// ─── Connection handler ───────────────────────────────────────────────────────

func (d *daemon) serveConn(conn net.Conn) {
	defer conn.Close()
	conn.SetDeadline(time.Now().Add(10 * time.Second))
	dec := json.NewDecoder(conn)
	enc := json.NewEncoder(conn)
	var req NotifyRequest
	if err := dec.Decode(&req); err != nil {
		log.Printf("[notifyd] decode error: %v", err)
		return
	}
	resp := d.handle(&req)
	if err := enc.Encode(resp); err != nil {
		log.Printf("[notifyd] encode error: %v", err)
	}
}

// ─── Main ─────────────────────────────────────────────────────────────────────

func main() {
	log.SetPrefix("[sigma-notifyd] ")
	log.SetFlags(log.LstdFlags | log.Lmicroseconds)

	if err := os.MkdirAll("/run/sigma", 0o750); err != nil {
		log.Fatalf("mkdir /run/sigma: %v", err)
	}
	os.Remove(socketPath) // stale socket

	ln, err := net.Listen("unix", socketPath)
	if err != nil {
		log.Fatalf("listen %s: %v", socketPath, err)
	}
	defer ln.Close()

	if err := os.Chmod(socketPath, 0o660); err != nil {
		log.Printf("chmod socket: %v", err)
	}

	d := newDaemon()
	go d.expireLoop()

	log.Printf("sigma-notifyd started on %s", socketPath)
	for {
		conn, err := ln.Accept()
		if err != nil {
			log.Printf("accept: %v", err)
			continue
		}
		go d.serveConn(conn)
	}
}
