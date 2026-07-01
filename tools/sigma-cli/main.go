// SPDX-License-Identifier: GPL-2.0-or-later
// tools/sigma-cli/main.go — the sigma developer CLI
//
//   sigma init <name>     scaffold a new SigmaOS app
//   sigma sign <dir>      Ed25519 signature over bundle
//   sigma verify <dir>    verify bundle signature
//   sigma run <dir>       launch against local sigmad-process
//   sigma caps <dir>      list declared capabilities
//   sigma health          query sigma-healthd
//   sigma sysctl <key>[=val] read/write sigma sysctl

package main

import (
	"archive/tar"
	"crypto/ed25519"
	"crypto/rand"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net"
	"net/http"
	"os"
	"path/filepath"
	"strings"
	"time"
)

type Manifest struct {
	Name    string   `json:"name"`
	Version string   `json:"version"`
	Entry   string   `json:"entry"`
	Caps    []string `json:"caps"`
}

func usage() {
	fmt.Fprintln(os.Stderr, "usage: sigma <init|sign|verify|run|caps|health|sysctl> [args...]")
	os.Exit(2)
}

func main() {
	if len(os.Args) < 2 { usage() }
	var err error
	switch os.Args[1] {
	case "init":    err = cmdInit(os.Args[2:])
	case "sign":    err = cmdSign(os.Args[2:])
	case "verify":  err = cmdVerify(os.Args[2:])
	case "run":     err = cmdRun(os.Args[2:])
	case "caps":    err = cmdCaps(os.Args[2:])
	case "health":  err = cmdHealth()
	case "sysctl":  err = cmdSysctl(os.Args[2:])
	default:        usage()
	}
	if err != nil { fmt.Fprintln(os.Stderr, "error:", err); os.Exit(1) }
}

// ── init ─────────────────────────────────────────────────────────────────────
func cmdInit(args []string) error {
	if len(args) != 1 { return errors.New("sigma init <name>") }
	name := args[0]
	os.MkdirAll(name, 0o755)
	m := Manifest{Name: name, Version: "0.1.0", Entry: "index.html",
		Caps: []string{"fs:/home/sigma/Documents/"}}
	writeJSON(filepath.Join(name, "sigma.json"), &m)
	html := `<!doctype html><meta charset=utf-8><title>` + name + `</title>
<script type=module>
await navigator.sigmaos.runtime.ready();
document.body.textContent = 'hello from ' + navigator.sigmaos.runtime.appId;
</script>`
	os.WriteFile(filepath.Join(name, "index.html"), []byte(html), 0o644)
	fmt.Println("created", name+"/sigma.json", "and", name+"/index.html")
	return nil
}

// ── caps ──────────────────────────────────────────────────────────────────────
func cmdCaps(args []string) error {
	if len(args) != 1 { return errors.New("sigma caps <dir>") }
	m, err := readManifest(args[0])
	if err != nil { return err }
	for _, c := range m.Caps { fmt.Println(c) }
	return nil
}

// ── sign ──────────────────────────────────────────────────────────────────────
func cmdSign(args []string) error {
	if len(args) != 1 { return errors.New("sigma sign <dir>") }
	digest, err := bundleDigest(args[0])
	if err != nil { return err }
	key, err := loadOrGenerateKey()
	if err != nil { return err }
	sig := ed25519.Sign(key, digest)
	out := map[string]string{
		"digest":    hex.EncodeToString(digest),
		"signature": hex.EncodeToString(sig),
		"pubkey":    hex.EncodeToString(key.Public().(ed25519.PublicKey)),
		"signed_at": time.Now().UTC().Format(time.RFC3339),
	}
	writeJSON(filepath.Join(args[0], "sigma.sig"), out)
	fmt.Println("signed", args[0], "→ sigma.sig")
	return nil
}

// ── verify ────────────────────────────────────────────────────────────────────
func cmdVerify(args []string) error {
	if len(args) != 1 { return errors.New("sigma verify <dir>") }
	want, err := bundleDigest(args[0])
	if err != nil { return err }
	var sigDoc struct{ Digest, Signature, Pubkey string }
	if err := readJSON(filepath.Join(args[0], "sigma.sig"), &sigDoc); err != nil {
		return fmt.Errorf("no sigma.sig: %w", err)
	}
	if hex.EncodeToString(want) != sigDoc.Digest {
		return errors.New("digest mismatch — bundle was modified after signing")
	}
	sig, _  := hex.DecodeString(sigDoc.Signature)
	pub, _  := hex.DecodeString(sigDoc.Pubkey)
	gotD, _ := hex.DecodeString(sigDoc.Digest)
	if !ed25519.Verify(pub, gotD, sig) {
		return errors.New("signature INVALID")
	}
	fmt.Println("OK — pubkey:", sigDoc.Pubkey[:16]+"...")
	return nil
}

// ── run ───────────────────────────────────────────────────────────────────────
func cmdRun(args []string) error {
	if len(args) != 1 { return errors.New("sigma run <dir>") }
	m, err := readManifest(args[0])
	if err != nil { return err }
	body, _ := json.Marshal(map[string]any{
		"method": "shell.exec",
		"cmd":    "xdg-open",
		"args":   []string{filepath.Join(args[0], m.Entry)},
		"caps":   m.Caps,
		"origin": "cli://" + m.Name,
	})
	resp, err := http.Post("http://127.0.0.1:17382/process",
		"application/json", strings.NewReader(string(body)))
	if err != nil { return fmt.Errorf("sigmad-process unreachable: %w", err) }
	defer resp.Body.Close()
	io.Copy(os.Stdout, resp.Body)
	return nil
}

// ── health ────────────────────────────────────────────────────────────────────
func cmdHealth() error {
	sock := "/run/sigma/healthd.sock"
	client := &http.Client{Transport: &http.Transport{
		DialContext: func(_ interface{}, _, _ string) (net.Conn, error) {
			return net.Dial("unix", sock)
		},
	}}
	resp, err := client.Get("http://sigma/health")
	if err != nil { return fmt.Errorf("healthd unavailable (%s): %w", sock, err) }
	defer resp.Body.Close()

	var h map[string]interface{}
	json.NewDecoder(resp.Body).Decode(&h)

	overall, _ := h["overall"].(string)
	uptime, _  := h["uptime_seconds"].(float64)
	fmt.Printf("Node: %-20s  Uptime: %s  Overall: %s\n",
		h["node"], formatUptime(uptime), strings.ToUpper(overall))
	fmt.Println(strings.Repeat("─", 60))

	if subs, ok := h["subsystems"].([]interface{}); ok {
		for _, s := range subs {
			sm, _ := s.(map[string]interface{})
			status, _ := sm["status"].(string)
			name,   _ := sm["name"].(string)
			msg,    _ := sm["message"].(string)
			icon := "✓"
			if status != "ok" { icon = "✗" }
			fmt.Printf("%s %-15s %-10s %s\n", icon, name, status, msg)
		}
	}
	return nil
}

func formatUptime(secs float64) string {
	h := int(secs) / 3600
	m := (int(secs) % 3600) / 60
	if h > 0 { return fmt.Sprintf("%dh %dm", h, m) }
	return fmt.Sprintf("%dm", m)
}

// ── sysctl ────────────────────────────────────────────────────────────────────
func cmdSysctl(args []string) error {
	if len(args) != 1 { return errors.New("sigma sysctl <key>[=value]") }
	if strings.Contains(args[0], "=") {
		parts := strings.SplitN(args[0], "=", 2)
		body, _ := json.Marshal(map[string]string{"op": "set", "key": parts[0], "value": parts[1]})
		resp, err := http.Post("http://127.0.0.1:17382/sysctl",
			"application/json", strings.NewReader(string(body)))
		if err != nil { return err }
		defer resp.Body.Close()
		io.Copy(os.Stdout, resp.Body)
		fmt.Println()
	} else {
		resp, err := http.Get("http://127.0.0.1:17382/sysctl?key=" + args[0])
		if err != nil { return err }
		defer resp.Body.Close()
		io.Copy(os.Stdout, resp.Body)
		fmt.Println()
	}
	return nil
}

// ── helpers ───────────────────────────────────────────────────────────────────
func readManifest(dir string) (*Manifest, error) {
	var m Manifest
	return &m, readJSON(filepath.Join(dir, "sigma.json"), &m)
}
func writeJSON(path string, v any) error {
	b, _ := json.MarshalIndent(v, "", "  ")
	return os.WriteFile(path, b, 0o644)
}
func readJSON(path string, v any) error {
	b, err := os.ReadFile(path)
	if err != nil { return err }
	return json.Unmarshal(b, v)
}
func bundleDigest(dir string) ([]byte, error) {
	h := sha256.New()
	tw := tar.NewWriter(h)
	filepath.Walk(dir, func(p string, info os.FileInfo, err error) error {
		if err != nil || info.IsDir() { return err }
		rel, _ := filepath.Rel(dir, p)
		if rel == "sigma.sig" { return nil }
		data, err := os.ReadFile(p)
		if err != nil { return err }
		tw.WriteHeader(&tar.Header{Name: rel, Mode: 0o644, Size: int64(len(data))})
		tw.Write(data)
		return nil
	})
	tw.Close()
	return h.Sum(nil), nil
}
func loadOrGenerateKey() (ed25519.PrivateKey, error) {
	path := os.Getenv("SIGMA_SIGNING_KEY")
	if path == "" {
		home, _ := os.UserHomeDir()
		path = filepath.Join(home, ".sigmaos", "signing.key")
	}
	if b, err := os.ReadFile(path); err == nil { return ed25519.PrivateKey(b), nil }
	_, priv, err := ed25519.GenerateKey(rand.Reader)
	if err != nil { return nil, err }
	os.MkdirAll(filepath.Dir(path), 0o700)
	os.WriteFile(path, priv, 0o600)
	fmt.Fprintln(os.Stderr, "generated signing key:", path)
	return priv, nil
}
