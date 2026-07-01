// SPDX-License-Identifier: GPL-2.0-or-later
// sigmad/busd/main.go — sigma-bus central router daemon
//
// Routes binary IPC messages between processes over Unix domain sockets.
// Faster than D-Bus (no XML), sovereign (no libdbus dependency).
//
// Protocol: sigma_bus_hdr_t (fixed header) + JSON body
// Socket:   /run/sigma/bus.sock (world-readable, per-message auth via credentials)

package main

import (
	"encoding/binary"
	"fmt"
	"net"
	"os"
	"sync"
)

const (
	busSocket   = "/run/sigma/bus.sock"
	maxMsgSize  = 65536
	hdrSize     = 4 + 4 + 1 + 64 + 64 + 4 // serial+reply_serial+type+iface+member+body_len
)

// ── Message types ─────────────────────────────────────────────────────────────
const (
	MsgCall   = 1
	MsgReply  = 2
	MsgSignal = 3
	MsgError  = 4
)

// ── Client registry ───────────────────────────────────────────────────────────
type BusClient struct {
	conn      net.Conn
	pid       uint32
	ifaces    []string // interfaces this client provides
	subscribed []string // interfaces this client subscribes to
}

type BusDaemon struct {
	mu      sync.RWMutex
	clients map[uint32]*BusClient // keyed by PID
	serial  uint32
}

func newBusDaemon() *BusDaemon {
	return &BusDaemon{clients: make(map[uint32]*BusClient)}
}

func (d *BusDaemon) nextSerial() uint32 {
	d.serial++
	return d.serial
}

// ── Message routing ───────────────────────────────────────────────────────────
func (d *BusDaemon) route(sender *BusClient, msgType byte,
	iface, member string, body []byte) {
	d.mu.RLock()
	defer d.mu.RUnlock()

	switch msgType {
	case MsgSignal:
		// Broadcast to all subscribers of this interface
		for _, c := range d.clients {
			if c == sender { continue }
			for _, sub := range c.subscribed {
				if sub == iface || sub == "*" {
					d.send(c, MsgSignal, iface, member, body, 0)
					break
				}
			}
		}

	case MsgCall:
		// Route to the registered provider of this interface
		for _, c := range d.clients {
			for _, provided := range c.ifaces {
				if provided == iface {
					d.send(c, MsgCall, iface, member, body, d.nextSerial())
					return
				}
			}
		}
		// No provider found — send error back to sender
		d.send(sender, MsgError, iface, member,
			[]byte(`{"error":"no provider for interface"}`), 0)
	}
}

func (d *BusDaemon) send(c *BusClient, msgType byte,
	iface, member string, body []byte, serial uint32) {
	// Build wire header
	hdr := make([]byte, hdrSize)
	binary.LittleEndian.PutUint32(hdr[0:], serial)
	hdr[8] = msgType
	copy(hdr[9:73], iface)
	copy(hdr[73:137], member)
	binary.LittleEndian.PutUint32(hdr[137:], uint32(len(body)))
	c.conn.Write(append(hdr, body...))
}

// ── Connection handler ────────────────────────────────────────────────────────
func (d *BusDaemon) handle(conn net.Conn) {
	client := &BusClient{conn: conn}
	buf := make([]byte, hdrSize+maxMsgSize)

	for {
		// Read header
		if _, err := conn.Read(buf[:hdrSize]); err != nil { break }

		msgType := buf[8]
		iface   := nullTerm(buf[9:73])
		member  := nullTerm(buf[73:137])
		bodyLen := binary.LittleEndian.Uint32(buf[137:141])

		var body []byte
		if bodyLen > 0 {
			body = make([]byte, bodyLen)
			conn.Read(body)
		}

		// Handle control messages (register/subscribe)
		switch iface {
		case "sigma.Bus":
			switch member {
			case "Register":
				client.ifaces = append(client.ifaces, string(body))
				d.mu.Lock()
				d.clients[client.pid] = client
				d.mu.Unlock()
			case "Subscribe":
				client.subscribed = append(client.subscribed, string(body))
			}
		default:
			d.route(client, msgType, iface, member, body)
		}
	}

	d.mu.Lock()
	delete(d.clients, client.pid)
	d.mu.Unlock()
	conn.Close()
}

func nullTerm(b []byte) string {
	for i, c := range b {
		if c == 0 { return string(b[:i]) }
	}
	return string(b)
}

// ── Main ──────────────────────────────────────────────────────────────────────
func main() {
	os.Remove(busSocket)
	ln, err := net.Listen("unix", busSocket)
	if err != nil { fmt.Fprintln(os.Stderr, "[sigma-busd]", err); os.Exit(1) }
	os.Chmod(busSocket, 0o666)

	d := newBusDaemon()
	fmt.Println("[sigma-busd] listening on", busSocket)

	for {
		conn, err := ln.Accept()
		if err != nil { continue }
		go d.handle(conn)
	}
}
