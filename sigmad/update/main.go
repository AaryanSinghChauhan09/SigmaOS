// SPDX-License-Identifier: GPL-2.0-or-later
// sigma-updated: atomic A/B system updater for SigmaOS
//
// Inspired by:
//   • OSTree   — content-addressed object store, atomic deployment
//   • Flatcar / Bottlerocket — two-slot A/B rootfs with automated rollback
//   • ChromeOS update_engine — background download, verify, switch
//   • rpm-ostree — transactional layering on top of a base image
//   • systemd-sysupdate — declarative transfer spec

package main

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log"
	"net"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"sync"
	"time"
)

// ─── Constants ───────────────────────────────────────────────────────────────

const (
	socketPath     = "/run/sigma/updated.sock"
	updateMetaURL  = "https://updates.sigmaos.io/v1/meta.json"
	slotA          = "/sigma/slots/a"
	slotB          = "/sigma/slots/b"
	activeSlotFile = "/sigma/boot/active_slot"
	stagingDir     = "/sigma/staging"
	rollbackLimit  = 3               // keep last 3 generations
	downloadTimeout = 5 * time.Minute
	verifyTimeout   = 30 * time.Second
)

// ─── Update metadata (fetched from update server) ────────────────────────────

type UpdateMeta struct {
	Version     string `json:"version"`
	BuildDate   string `json:"build_date"`
	Channel     string `json:"channel"`    // stable | testing | canary
	ImageURL    string `json:"image_url"`
	ImageSHA256 string `json:"sha256"`
	ImageSize   int64  `json:"size_bytes"`
	MinVersion  string `json:"min_version"` // minimum version that can upgrade
	ReleaseNote string `json:"release_note"`
}

// ─── Slot state ───────────────────────────────────────────────────────────────

type Slot string

const (
	SlotA    Slot = "a"
	SlotB    Slot = "b"
	SlotNone Slot = ""
)

// ─── Daemon state ─────────────────────────────────────────────────────────────

type UpdateState string

const (
	StateIdle        UpdateState = "idle"
	StateChecking    UpdateState = "checking"
	StateDownloading UpdateState = "downloading"
	StateVerifying   UpdateState = "verifying"
	StateStaging     UpdateState = "staging"
	StateReady       UpdateState = "ready"   // staged, awaiting reboot
	StateRollingBack UpdateState = "rolling_back"
	StateError       UpdateState = "error"
)

type daemon struct {
	mu           sync.RWMutex
	state        UpdateState
	activeSlot   Slot
	pendingSlot  Slot
	lastChecked  time.Time
	lastMeta     *UpdateMeta
	progress     int    // 0–100 download progress
	errMsg       string
	httpClient   *http.Client
	cancelUpdate context.CancelFunc
}

func newDaemon() *daemon {
	d := &daemon{
		state: StateIdle,
		httpClient: &http.Client{
			Timeout: downloadTimeout,
		},
	}
	d.activeSlot = d.readActiveSlot()
	return d
}

// ─── Slot management ─────────────────────────────────────────────────────────

func (d *daemon) readActiveSlot() Slot {
	data, err := os.ReadFile(activeSlotFile)
	if err != nil {
		log.Printf("[updated] active slot file missing, defaulting to A: %v", err)
		return SlotA
	}
	s := Slot(filepath.Base(string(data)))
	if s != SlotA && s != SlotB {
		log.Printf("[updated] invalid slot %q, defaulting to A", s)
		return SlotA
	}
	return s
}

func (d *daemon) inactiveSlot() Slot {
	if d.activeSlot == SlotA {
		return SlotB
	}
	return SlotA
}

func slotPath(s Slot) string {
	if s == SlotA {
		return slotA
	}
	return slotB
}

func (d *daemon) commitSlotSwitch() error {
	newActive := d.inactiveSlot()
	tmp := activeSlotFile + ".tmp"
	if err := os.WriteFile(tmp, []byte(string(newActive)+"\n"), 0o644); err != nil {
		return fmt.Errorf("write tmp slot: %w", err)
	}
	if err := os.Rename(tmp, activeSlotFile); err != nil {
		return fmt.Errorf("rename slot file: %w", err)
	}
	log.Printf("[updated] slot switch committed: %s → %s", d.activeSlot, newActive)
	d.pendingSlot = newActive
	return nil
}

// ─── Update flow ─────────────────────────────────────────────────────────────

// checkForUpdate fetches update metadata and returns it if a newer version
// is available. Returns nil, nil if already up-to-date.
func (d *daemon) checkForUpdate(ctx context.Context) (*UpdateMeta, error) {
	d.setState(StateChecking, "")
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, updateMetaURL, nil)
	if err != nil {
		return nil, fmt.Errorf("build request: %w", err)
	}
	req.Header.Set("User-Agent", "sigma-updated/1.0")

	resp, err := d.httpClient.Do(req)
	if err != nil {
		return nil, fmt.Errorf("fetch meta: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("meta HTTP %d", resp.StatusCode)
	}

	var meta UpdateMeta
	if err := json.NewDecoder(io.LimitReader(resp.Body, 64*1024)).Decode(&meta); err != nil {
		return nil, fmt.Errorf("decode meta: %w", err)
	}

	d.mu.Lock()
	d.lastChecked = time.Now()
	d.lastMeta = &meta
	d.mu.Unlock()

	log.Printf("[updated] check: server=%s local=%s", meta.Version, currentVersion())
	if meta.Version == currentVersion() {
		d.setState(StateIdle, "")
		return nil, nil
	}
	return &meta, nil
}

// downloadAndVerify downloads the update image to staging and verifies SHA-256.
func (d *daemon) downloadAndVerify(ctx context.Context, meta *UpdateMeta) (string, error) {
	d.setState(StateDownloading, "")

	if err := os.MkdirAll(stagingDir, 0o700); err != nil {
		return "", fmt.Errorf("mkdir staging: %w", err)
	}
	dest := filepath.Join(stagingDir, "update.img")

	req, err := http.NewRequestWithContext(ctx, http.MethodGet, meta.ImageURL, nil)
	if err != nil {
		return "", fmt.Errorf("build dl request: %w", err)
	}
	req.Header.Set("User-Agent", "sigma-updated/1.0")

	resp, err := d.httpClient.Do(req)
	if err != nil {
		return "", fmt.Errorf("download: %w", err)
	}
	defer resp.Body.Close()

	f, err := os.OpenFile(dest, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0o600)
	if err != nil {
		return "", fmt.Errorf("open dest: %w", err)
	}
	defer f.Close()

	hasher := sha256.New()
	var downloaded int64
	buf := make([]byte, 256*1024) // 256 KB chunks
	for {
		select {
		case <-ctx.Done():
			return "", ctx.Err()
		default:
		}
		n, rerr := resp.Body.Read(buf)
		if n > 0 {
			if _, werr := f.Write(buf[:n]); werr != nil {
				return "", fmt.Errorf("write: %w", werr)
			}
			hasher.Write(buf[:n])
			downloaded += int64(n)
			if meta.ImageSize > 0 {
				d.setProgress(int(downloaded * 100 / meta.ImageSize))
			}
		}
		if rerr != nil {
			if errors.Is(rerr, io.EOF) {
				break
			}
			return "", fmt.Errorf("read: %w", rerr)
		}
	}

	d.setState(StateVerifying, "")
	got := hex.EncodeToString(hasher.Sum(nil))
	if got != meta.ImageSHA256 {
		os.Remove(dest)
		return "", fmt.Errorf("sha256 mismatch: got %s want %s", got, meta.ImageSHA256)
	}
	log.Printf("[updated] download+verify OK: %s (%d bytes)", meta.Version, downloaded)
	return dest, nil
}

// stageToInactiveSlot writes the verified image to the inactive slot.
func (d *daemon) stageToInactiveSlot(imagePath string) error {
	d.setState(StateStaging, "")
	target := slotPath(d.inactiveSlot())

	if err := os.MkdirAll(filepath.Dir(target), 0o755); err != nil {
		return fmt.Errorf("mkdir slot dir: %w", err)
	}

	// Atomic: copy to .tmp, then rename — prevents partial writes being booted
	tmp := target + ".tmp"
	src, err := os.Open(imagePath)
	if err != nil {
		return fmt.Errorf("open image: %w", err)
	}
	defer src.Close()

	dst, err := os.OpenFile(tmp, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0o600)
	if err != nil {
		return fmt.Errorf("open tmp slot: %w", err)
	}
	defer dst.Close()

	if _, err := io.Copy(dst, src); err != nil {
		os.Remove(tmp)
		return fmt.Errorf("copy to slot: %w", err)
	}
	dst.Close()

	if err := os.Rename(tmp, target); err != nil {
		os.Remove(tmp)
		return fmt.Errorf("rename slot: %w", err)
	}

	log.Printf("[updated] staged to slot %s (%s)", d.inactiveSlot(), target)
	return nil
}

// performUpdate runs the full update pipeline.
func (d *daemon) performUpdate(ctx context.Context) error {
	meta, err := d.checkForUpdate(ctx)
	if err != nil {
		d.setState(StateError, err.Error())
		return err
	}
	if meta == nil {
		log.Println("[updated] already up-to-date")
		return nil
	}

	imagePath, err := d.downloadAndVerify(ctx, meta)
	if err != nil {
		d.setState(StateError, err.Error())
		return err
	}
	defer os.Remove(imagePath)

	if err := d.stageToInactiveSlot(imagePath); err != nil {
		d.setState(StateError, err.Error())
		return err
	}

	if err := d.commitSlotSwitch(); err != nil {
		d.setState(StateError, err.Error())
		return err
	}

	d.setState(StateReady, "")
	log.Printf("[updated] update ready — reboot to activate slot %s", d.pendingSlot)
	return nil
}

// rollback switches back to the previously active slot.
func (d *daemon) rollback() error {
	d.setState(StateRollingBack, "")
	// Swap active slot back
	if d.activeSlot == SlotA {
		d.activeSlot = SlotB
	} else {
		d.activeSlot = SlotA
	}
	tmp := activeSlotFile + ".tmp"
	if err := os.WriteFile(tmp, []byte(string(d.activeSlot)+"\n"), 0o644); err != nil {
		return fmt.Errorf("write rollback slot: %w", err)
	}
	if err := os.Rename(tmp, activeSlotFile); err != nil {
		return fmt.Errorf("rename rollback slot: %w", err)
	}
	log.Printf("[updated] rolled back to slot %s", d.activeSlot)
	d.setState(StateIdle, "")
	return nil
}

// ─── State helpers ────────────────────────────────────────────────────────────

func (d *daemon) setState(s UpdateState, errMsg string) {
	d.mu.Lock()
	d.state = s
	d.errMsg = errMsg
	d.mu.Unlock()
}

func (d *daemon) setProgress(p int) {
	d.mu.Lock()
	d.progress = p
	d.mu.Unlock()
}

// ─── Wire protocol ────────────────────────────────────────────────────────────

type Request struct {
	Op string `json:"op"` // "check" | "update" | "rollback" | "status" | "cancel"
}

type Response struct {
	OK       bool        `json:"ok"`
	Error    string      `json:"error,omitempty"`
	State    UpdateState `json:"state"`
	Progress int         `json:"progress,omitempty"`
	Meta     *UpdateMeta `json:"meta,omitempty"`
	Slot     string      `json:"active_slot,omitempty"`
}

func (d *daemon) handleConn(conn net.Conn) {
	defer conn.Close()
	conn.SetDeadline(time.Now().Add(30 * time.Second))

	var req Request
	if err := json.NewDecoder(conn).Decode(&req); err != nil {
		log.Printf("[updated] decode: %v", err)
		return
	}

	var resp Response
	switch req.Op {
	case "status":
		d.mu.RLock()
		resp = Response{
			OK:       true,
			State:    d.state,
			Progress: d.progress,
			Meta:     d.lastMeta,
			Slot:     string(d.activeSlot),
		}
		d.mu.RUnlock()

	case "check":
		go func() {
			ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
			defer cancel()
			d.checkForUpdate(ctx) //nolint:errcheck
		}()
		resp = Response{OK: true, State: StateChecking}

	case "update":
		d.mu.Lock()
		if d.state != StateIdle && d.state != StateError {
			d.mu.Unlock()
			resp = Response{Error: "update already in progress", State: d.state}
			break
		}
		ctx, cancel := context.WithTimeout(context.Background(), 30*time.Minute)
		d.cancelUpdate = cancel
		d.mu.Unlock()
		go func() {
			defer cancel()
			if err := d.performUpdate(ctx); err != nil {
				log.Printf("[updated] update failed: %v", err)
			}
		}()
		resp = Response{OK: true, State: StateDownloading}

	case "rollback":
		if err := d.rollback(); err != nil {
			resp = Response{Error: err.Error()}
		} else {
			resp = Response{OK: true, State: StateIdle}
		}

	case "cancel":
		d.mu.Lock()
		if d.cancelUpdate != nil {
			d.cancelUpdate()
			d.cancelUpdate = nil
		}
		d.mu.Unlock()
		d.setState(StateIdle, "")
		resp = Response{OK: true, State: StateIdle}

	default:
		resp = Response{Error: "unknown op: " + req.Op}
	}

	json.NewEncoder(conn).Encode(resp) //nolint:errcheck
}

// ─── Version helper ───────────────────────────────────────────────────────────

func currentVersion() string {
	// Read /sigma/etc/os-release VERSION_ID field
	data, err := os.ReadFile("/sigma/etc/os-release")
	if err != nil {
		return "unknown"
	}
	for _, line := range splitLines(string(data)) {
		if len(line) > 11 && line[:11] == "VERSION_ID=" {
			v := line[11:]
			if len(v) >= 2 && v[0] == '"' {
				v = v[1 : len(v)-1]
			}
			return v
		}
	}
	return "unknown"
}

func splitLines(s string) []string {
	var lines []string
	start := 0
	for i, c := range s {
		if c == '\n' {
			lines = append(lines, s[start:i])
			start = i + 1
		}
	}
	if start < len(s) {
		lines = append(lines, s[start:])
	}
	return lines
}

// ─── Periodic background check ────────────────────────────────────────────────

func (d *daemon) periodicCheck() {
	ticker := time.NewTicker(6 * time.Hour)
	defer ticker.Stop()
	for range ticker.C {
		d.mu.RLock()
		s := d.state
		d.mu.RUnlock()
		if s != StateIdle {
			continue
		}
		ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
		if _, err := d.checkForUpdate(ctx); err != nil {
			log.Printf("[updated] periodic check failed: %v", err)
		}
		cancel()
	}
}

// ─── Reboot helper (called externally by sigma CLI) ───────────────────────────

func scheduleReboot(delaySec int) error {
	log.Printf("[updated] scheduling reboot in %d seconds", delaySec)
	return exec.Command("shutdown", "-r", fmt.Sprintf("+%d", delaySec/60),
		"SigmaOS update applied — rebooting").Run()
}

// ─── Main ─────────────────────────────────────────────────────────────────────

func main() {
	log.SetPrefix("[sigma-updated] ")
	log.SetFlags(log.LstdFlags | log.Lmicroseconds)

	if err := os.MkdirAll("/run/sigma", 0o750); err != nil {
		log.Fatalf("mkdir /run/sigma: %v", err)
	}
	os.Remove(socketPath)

	ln, err := net.Listen("unix", socketPath)
	if err != nil {
		log.Fatalf("listen %s: %v", socketPath, err)
	}
	defer ln.Close()
	os.Chmod(socketPath, 0o660) //nolint:errcheck

	d := newDaemon()
	log.Printf("sigma-updated started — active slot: %s", d.activeSlot)

	go d.periodicCheck()

	for {
		conn, err := ln.Accept()
		if err != nil {
			log.Printf("accept: %v", err)
			continue
		}
		go d.handleConn(conn)
	}
}
