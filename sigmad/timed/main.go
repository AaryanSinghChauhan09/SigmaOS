// SPDX-License-Identifier: GPL-2.0-or-later
// sigma-timed: NTP/SNTP time synchronisation daemon for SigmaOS
//
// Inspired by:
//   • OpenNTPD (Henning Brauer) — minimal, privilege-separated NTP
//   • chrony (Richard Curnow)   — slew-based clock discipline
//   • RFC 4330 (SNTP v4)        — simple NTP client algorithm
//   • systemd-timesyncd         — single-file NTP client
//
// Algorithm:
//   1. Query pool.ntp.org (4 servers in parallel)
//   2. Apply RFC 5905 clock filter: discard the 3 worst samples
//   3. Compute offset and round-trip delay
//   4. Slew the clock with adjtime(2) — never step unless offset > 128ms
//      (stepping breaks TLS cert validity, log correlation, PQC key expiry)
//   5. Re-sync every poll_interval (64s default, adapts on error)

package main

import (
	"encoding/binary"
	"errors"
	"fmt"
	"log"
	"math"
	"net"
	"os"
	"sync"
	"syscall"
	"time"
)

// ─── Constants ───────────────────────────────────────────────────────────────

const (
	socketPath   = "/run/sigma/timed.sock"
	ntpPort      = "123"
	ntpEpochDiff = 2208988800 // seconds between 1900 and 1970
	maxOffset    = 128e-3     // step threshold: 128ms
	minPoll      = 16         // minimum poll interval in seconds
	maxPoll      = 1024
	defaultPoll  = 64
	sampleCount  = 8 // NTP filter window size
)

var defaultServers = []string{
	"0.pool.ntp.org",
	"1.pool.ntp.org",
	"2.pool.ntp.org",
	"3.pool.ntp.org",
}

// ─── NTP packet (48 bytes, RFC 4330) ─────────────────────────────────────────

type ntpPacket struct {
	LiVnMode       uint8
	Stratum        uint8
	Poll           int8
	Precision      int8
	RootDelay      uint32
	RootDispersion uint32
	ReferenceID    uint32
	RefTimeSec     uint32
	RefTimeFrac    uint32
	OrigTimeSec    uint32
	OrigTimeFrac   uint32
	RxTimeSec      uint32
	RxTimeFrac     uint32
	TxTimeSec      uint32
	TxTimeFrac     uint32
}

func ntpTimestamp(sec, frac uint32) float64 {
	return float64(sec) - ntpEpochDiff + float64(frac)/math.Pow(2, 32)
}

// ─── Single server query ──────────────────────────────────────────────────────

type sample struct {
	offset float64 // seconds (positive = system is behind)
	delay  float64 // round-trip delay in seconds
	stratum uint8
}

func queryServer(server string) (sample, error) {
	conn, err := net.DialTimeout("udp", net.JoinHostPort(server, ntpPort), 5*time.Second)
	if err != nil {
		return sample{}, err
	}
	defer conn.Close()
	conn.SetDeadline(time.Now().Add(5 * time.Second))

	req := ntpPacket{LiVnMode: 0x1B} // LI=0, VN=3, Mode=3 (client)
	t1 := time.Now()

	if err := binary.Write(conn, binary.BigEndian, &req); err != nil {
		return sample{}, err
	}

	var resp ntpPacket
	if err := binary.Read(conn, binary.BigEndian, &resp); err != nil {
		return sample{}, err
	}
	t4 := time.Now()

	if (resp.LiVnMode>>3)&0x7 != 4 { // Mode must be 4 (server)
		return sample{}, errors.New("invalid NTP mode")
	}
	if resp.Stratum == 0 {
		return sample{}, errors.New("kiss-o-death from server")
	}

	t1f := float64(t1.Unix()) + float64(t1.Nanosecond())/1e9
	t4f := float64(t4.Unix()) + float64(t4.Nanosecond())/1e9
	t2f := ntpTimestamp(resp.RxTimeSec, resp.RxTimeFrac)
	t3f := ntpTimestamp(resp.TxTimeSec, resp.TxTimeFrac)

	// RFC 5905 §8: offset = ((t2-t1) + (t3-t4)) / 2
	offset := ((t2f - t1f) + (t3f - t4f)) / 2
	// delay  = (t4-t1) - (t3-t2)
	delay := (t4f - t1f) - (t3f - t2f)

	return sample{offset: offset, delay: delay, stratum: resp.Stratum}, nil
}

// ─── Clock discipline ─────────────────────────────────────────────────────────

// slew adjusts the system clock gradually using adjtime(2).
// Linux adjtime moves the clock at ~500ppm until the delta is consumed.
func slewClock(offsetSec float64) error {
	tv := syscall.Timeval{
		Sec:  int64(offsetSec),
		Usec: int64((offsetSec - math.Trunc(offsetSec)) * 1e6),
	}
	// syscall.Adjtime is not in stdlib — use raw syscall
	_, _, errno := syscall.Syscall(syscall.SYS_ADJTIME,
		uintptr(unsafe.Pointer(&tv)), 0, 0)
	if errno != 0 {
		return errno
	}
	return nil
}

// stepClock sets the clock directly — only for large offsets at startup.
func stepClock(offsetSec float64) error {
	now := time.Now()
	adjusted := now.Add(time.Duration(offsetSec * float64(time.Second)))
	tv := syscall.Timeval{
		Sec:  adjusted.Unix(),
		Usec: int64(adjusted.Nanosecond() / 1000),
	}
	_, _, errno := syscall.Syscall(syscall.SYS_SETTIMEOFDAY,
		uintptr(unsafe.Pointer(&tv)), 0, 0)
	if errno != 0 {
		return errno
	}
	log.Printf("[timed] step clock by %.3fs", offsetSec)
	return nil
}

// ─── Daemon state ─────────────────────────────────────────────────────────────

type daemon struct {
	mu          sync.RWMutex
	lastOffset  float64
	lastDelay   float64
	lastSync    time.Time
	pollSec     int
	syncCount   uint64
	errorCount  uint64
	stratum     uint8
	servers     []string
}

func newDaemon() *daemon {
	return &daemon{
		pollSec: defaultPoll,
		servers: defaultServers,
	}
}

func (d *daemon) sync() {
	results := make(chan sample, len(d.servers))
	for _, s := range d.servers {
		go func(srv string) {
			sm, err := queryServer(srv)
			if err != nil {
				log.Printf("[timed] %s: %v", srv, err)
				results <- sample{}
				return
			}
			results <- sm
		}(s)
	}

	var good []sample
	for range d.servers {
		sm := <-results
		if sm.stratum > 0 && sm.delay > 0 {
			good = append(good, sm)
		}
	}

	if len(good) == 0 {
		d.mu.Lock()
		d.errorCount++
		if d.pollSec < maxPoll {
			d.pollSec *= 2
		}
		d.mu.Unlock()
		log.Printf("[timed] no usable responses — backing off to %ds", d.pollSec)
		return
	}

	// RFC 5905 clock filter: pick sample with minimum delay
	best := good[0]
	for _, sm := range good[1:] {
		if sm.delay < best.delay {
			best = sm
		}
	}

	offset := best.offset
	log.Printf("[timed] offset=%.6fs delay=%.6fs stratum=%d", offset, best.delay, best.stratum)

	var err error
	if math.Abs(offset) > maxOffset {
		err = stepClock(offset)
	} else {
		err = slewClock(offset)
	}

	d.mu.Lock()
	if err != nil {
		d.errorCount++
		log.Printf("[timed] clock adjust error: %v", err)
	} else {
		d.syncCount++
		d.lastOffset = offset
		d.lastDelay  = best.delay
		d.lastSync   = time.Now()
		d.stratum    = best.stratum + 1
		d.pollSec    = defaultPoll // reset on success
	}
	d.mu.Unlock()
}

func (d *daemon) run() {
	// Initial sync on startup
	d.sync()

	for {
		d.mu.RLock()
		poll := d.pollSec
		d.mu.RUnlock()
		time.Sleep(time.Duration(poll) * time.Second)
		d.sync()
	}
}

// ─── Status socket ────────────────────────────────────────────────────────────

type statusResp struct {
	LastOffset float64   `json:"last_offset_sec"`
	LastDelay  float64   `json:"last_delay_sec"`
	LastSync   time.Time `json:"last_sync"`
	SyncCount  uint64    `json:"sync_count"`
	ErrorCount uint64    `json:"error_count"`
	PollSec    int       `json:"poll_interval_sec"`
	Stratum    uint8     `json:"stratum"`
}

func (d *daemon) serveStatus(ln net.Listener) {
	import "encoding/json"
	for {
		conn, err := ln.Accept()
		if err != nil {
			continue
		}
		d.mu.RLock()
		resp := statusResp{
			LastOffset: d.lastOffset,
			LastDelay:  d.lastDelay,
			LastSync:   d.lastSync,
			SyncCount:  d.syncCount,
			ErrorCount: d.errorCount,
			PollSec:    d.pollSec,
			Stratum:    d.stratum,
		}
		d.mu.RUnlock()
		json.NewEncoder(conn).Encode(resp) //nolint:errcheck
		conn.Close()
	}
}

// ─── Main ─────────────────────────────────────────────────────────────────────

func main() {
	log.SetPrefix("[sigma-timed] ")
	log.SetFlags(log.LstdFlags | log.Lmicroseconds)

	os.MkdirAll("/run/sigma", 0o750) //nolint:errcheck
	os.Remove(socketPath)

	ln, err := net.Listen("unix", socketPath)
	if err != nil {
		log.Fatalf("listen %s: %v", socketPath, err)
	}
	os.Chmod(socketPath, 0o660) //nolint:errcheck

	d := newDaemon()
	go d.serveStatus(ln)
	log.Printf("sigma-timed started (servers: %v)", d.servers)
	d.run()
}
