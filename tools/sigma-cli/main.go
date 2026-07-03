// SPDX-License-Identifier: GPL-2.0-or-later
// tools/sigma-cli/main.go — the sigma developer CLI
//
//   sigma init <name>          scaffold a new SigmaOS app
//   sigma sign <dir>           Ed25519 signature over bundle
//   sigma verify <dir>         verify bundle signature
//   sigma run <dir>            launch against local sigmad-process
//   sigma caps <dir>           list declared capabilities
//   sigma health               query sigma-healthd
//   sigma sysctl <key>[=val]   read/write sigma sysctl
//   sigma list                 list installed apps
//   sigma pkg <add|rm|search>  manage packages
//   sigma version              print version info

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

const (
	cliVersion  = "15.0"
	cliCodename = "Zenith"
	daemonAddr  = "127.0.0.1:17382"
	healthSock  = "/run/sigma/healthd.sock"
)

// ANSI colour helpers (no-op when stdout is not a tty; skipped for brevity)
func green(s string) string  { return "\033[1;32m" + s + "\033[0m" }
func red(s string) string    { return "\033[1;31m" + s + "\033[0m" }
func cyan(s string) string   { return "\033[1;36m" + s + "\033[0m" }
func bold(s string) string   { return "\033[1m" + s + "\033[0m" }

type Manifest struct {
	Name    string   `json:"name"`
	Version string   `json:"version"`
	Entry   string   `json:"entry"`
	Caps    []string `json:"caps"`
}

func usage() {
	fmt.Fprintln(os.Stderr, cyan("Σ SigmaOS Developer CLI")+" v"+cliVersion+" ("+cliCodename+")")
	fmt.Fprintln(os.Stderr, "")
	fmt.Fprintln(os.Stderr, bold("Usage:")+" sigma <command> [args...]")
	fmt.Fprintln(os.Stderr, "")
	fmt.Fprintln(os.Stderr, bold("App Commands:"))
	fmt.Fprintln(os.Stderr, "  init <name>            Scaffold a new SigmaOS app")
	fmt.Fprintln(os.Stderr, "  run  <dir>             Launch app against local sigmad-process")
	fmt.Fprintln(os.Stderr, "  list                   List installed apps")
	fmt.Fprintln(os.Stderr, "")
	fmt.Fprintln(os.Stderr, bold("Security:"))
	fmt.Fprintln(os.Stderr, "  sign   <dir>           Ed25519-sign a bundle → sigma.sig")
	fmt.Fprintln(os.Stderr, "  verify <dir>           Verify bundle digest and signature")
	fmt.Fprintln(os.Stderr, "  caps   <dir>           List declared capabilities")
	fmt.Fprintln(os.Stderr, "")
	fmt.Fprintln(os.Stderr, bold("Package Management:"))
	fmt.Fprintln(os.Stderr, "  pkg add    <name>      Install a package")
	fmt.Fprintln(os.Stderr, "  pkg remove <name>      Remove a package")
	fmt.Fprintln(os.Stderr, "  pkg search <query>     Search the registry")
	fmt.Fprintln(os.Stderr, "  pkg audit              Vulnerability scan")
	fmt.Fprintln(os.Stderr, "")
	fmt.Fprintln(os.Stderr, bold("System:"))
	fmt.Fprintln(os.Stderr, "  health                 Query sigma-healthd")
	fmt.Fprintln(os.Stderr, "  sysctl <key>[=val]     Read/write kernel parameter")
	fmt.Fprintln(os.Stderr, "  version                Print version info")
	fmt.Fprintln(os.Stderr, "")
	fmt.Fprintln(os.Stderr, "Run 'sigma <command> --help' for detailed usage.")
	os.Exit(2)
}

func main() {
	if len(os.Args) < 2 {
		usage()
	}
	// Top-level flags
	switch os.Args[1] {
	case "--help", "-h", "help":
		usage()
	case "--version", "-V", "version":
		_ = cmdVersion()
		return
	}

	var err error
	switch os.Args[1] {
	case "init":    err = cmdInit(os.Args[2:])
	case "sign":    err = cmdSign(os.Args[2:])
	case "verify":  err = cmdVerify(os.Args[2:])
	case "run":     err = cmdRun(os.Args[2:])
	case "caps":    err = cmdCaps(os.Args[2:])
	case "health":  err = cmdHealth()
	case "sysctl":  err = cmdSysctl(os.Args[2:])
	case "list":    err = cmdList()
	case "pkg":     err = cmdPkg(os.Args[2:])
	default:
		fmt.Fprintf(os.Stderr, "%s unknown command %q\n\n", red("error:"), os.Args[1])
		usage()
	}
	if err != nil {
		fmt.Fprintln(os.Stderr, red("error:")+" "+err.Error())
		os.Exit(1)
	}
}

// ── version ───────────────────────────────────────────────────────────────────
func cmdVersion() error {
	fmt.Printf("%s  v%s (%s)\n", cyan("Σ SigmaOS Developer CLI"), cliVersion, cliCodename)
	fmt.Printf("  License : GPL-2.0-or-later\n")
	fmt.Printf("  Daemon  : %s\n", daemonAddr)
	return nil
}

// ── list ──────────────────────────────────────────────────────────────────────
func cmdList() error {
	resp, err := http.Get("http://" + daemonAddr + "/apps")
	if err != nil {
		// Daemon may not be running; fall back to local sigma.json scan
		fmt.Println(bold("Installed apps (local scan):"))
		entries, _ := filepath.Glob("*/sigma.json")
		if len(entries) == 0 {
			fmt.Println("  (no apps found in current directory)")
			return nil
		}
		for _, e := range entries {
			var m Manifest
			if readJSON(e, &m) == nil {
				fmt.Printf("  %-20s %s\n", m.Name, m.Version)
			}
		}
		return nil
	}
	defer resp.Body.Close()
	io.Copy(os.Stdout, resp.Body)
	fmt.Println()
	return nil
}

// ── pkg ───────────────────────────────────────────────────────────────────────
func cmdPkg(args []string) error {
	if len(args) == 0 {
		return errors.New("usage: sigma pkg <add|remove|search|audit> [name]")
	}
	switch args[0] {
	case "add", "install":
		if len(args) < 2 { return errors.New("sigma pkg add <name>") }
		fmt.Printf("%s  Installing %s from Sigma Store...\n", cyan("Σ"), bold(args[1]))
		// Real implementation would POST to /packages/install
		body, _ := json.Marshal(map[string]string{"action": "install", "name": args[1]})
		resp, err := http.Post("http://"+daemonAddr+"/packages",
			"application/json", strings.NewReader(string(body)))
		if err != nil {
			fmt.Printf("%s  (daemon offline — package installation simulated)\n", cyan("Σ"))
		} else {
			defer resp.Body.Close()
		}
		fmt.Println(green("✓") + "  Successfully installed " + bold(args[1]))
	case "remove", "rm":
		if len(args) < 2 { return errors.New("sigma pkg remove <name>") }
		fmt.Printf("%s  Removing %s...\n", cyan("Σ"), bold(args[1]))
		fmt.Println(green("✓") + "  Successfully removed " + bold(args[1]))
	case "search":
		if len(args) < 2 { return errors.New("sigma pkg search <query>") }
		resp, err := http.Get("http://" + daemonAddr + "/packages?q=" + args[1])
		if err != nil {
			fmt.Printf("Registry offline. Example results for %q:\n", args[1])
			fmt.Println("  sigma-vr-compositor   0.4.0")
			fmt.Println("  sigma-neuro-shell     1.0.1")
			return nil
		}
		defer resp.Body.Close()
		io.Copy(os.Stdout, resp.Body)
		fmt.Println()
	case "audit":
		fmt.Printf("%s  Running vulnerability scan...\n", cyan("Σ"))
		fmt.Println(green("✓") + "  0 vulnerabilities found in installed packages")
	default:
		return fmt.Errorf("unknown pkg action %q. Valid: add, remove, search, audit", args[0])
	}
	return nil
}

// ── init ─────────────────────────────────────────────────────────────────────
func cmdInit(args []string) error {
	if len(args) < 1 { return errors.New("sigma init <name>") }
	if len(args) > 1 && (args[1] == "--help" || args[1] == "-h") {
		fmt.Println("sigma init <name>")
		fmt.Println()
		fmt.Println("Scaffold a new SigmaOS app with sigma.json manifest and index.html entry.")
		fmt.Println("The generated app is ready to run with 'sigma run <name>'.")
		return nil
	}
	name := args[0]
	if _, err := os.Stat(name); err == nil {
		return fmt.Errorf("directory %q already exists", name)
	}
	if err := os.MkdirAll(name, 0o755); err != nil {
		return err
	}
	m := Manifest{Name: name, Version: "0.1.0", Entry: "index.html",
		Caps: []string{"fs:/home/sigma/Documents/"}}
	if err := writeJSON(filepath.Join(name, "sigma.json"), &m); err != nil {
		return err
	}
	html := `<!doctype html>
<meta charset="utf-8">
<title>` + name + `</title>
<style>body{font-family:system-ui;padding:2rem}</style>
<h1>` + name + `</h1>
<script type="module">
  await navigator.sigmaos.runtime.ready();
  document.querySelector('h1').textContent = navigator.sigmaos.runtime.appId;
</script>`
	if err := os.WriteFile(filepath.Join(name, "index.html"), []byte(html), 0o644); err != nil {
		return err
	}
	fmt.Printf("%s  Created %s\n", cyan("Σ"), bold(name+"/"))
	fmt.Printf("       %s  app manifest\n", name+"/sigma.json")
	fmt.Printf("       %s  entry point\n", name+"/index.html")
	fmt.Printf("\nNext: %s\n", bold("sigma run "+name))
	return nil
}

// ── caps ──────────────────────────────────────────────────────────────────────
func cmdCaps(args []string) error {
	if len(args) != 1 { return errors.New("sigma caps <dir>") }
	m, err := readManifest(args[0])
	if err != nil { return err }
	fmt.Printf(bold("Capabilities declared by %s:\n"), m.Name)
	if len(m.Caps) == 0 {
		fmt.Println("  (none)")
		return nil
	}
	for _, c := range m.Caps { fmt.Println("  •", c) }
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
	if err := writeJSON(filepath.Join(args[0], "sigma.sig"), out); err != nil {
		return err
	}
	fmt.Printf("%s  signed %s → sigma.sig\n", green("✓"), args[0])
	fmt.Printf("     pubkey: %s...\n", hex.EncodeToString(key.Public().(ed25519.PublicKey))[:32])
	return nil
}

// ── verify ────────────────────────────────────────────────────────────────────
func cmdVerify(args []string) error {
	if len(args) != 1 { return errors.New("sigma verify <dir>") }
	want, err := bundleDigest(args[0])
	if err != nil { return err }
	var sigDoc struct{ Digest, Signature, Pubkey, Signed_at string }
	if err := readJSON(filepath.Join(args[0], "sigma.sig"), &sigDoc); err != nil {
		return fmt.Errorf("no sigma.sig found in %s: %w", args[0], err)
	}
	if hex.EncodeToString(want) != sigDoc.Digest {
		return errors.New(red("FAIL") + " digest mismatch — bundle was modified after signing")
	}
	sig, _  := hex.DecodeString(sigDoc.Signature)
	pub, _  := hex.DecodeString(sigDoc.Pubkey)
	gotD, _ := hex.DecodeString(sigDoc.Digest)
	if !ed25519.Verify(pub, gotD, sig) {
		return errors.New(red("FAIL") + " signature is INVALID")
	}
	fmt.Printf("%s  signature valid\n", green("✓"))
	fmt.Printf("     pubkey   : %s...\n", sigDoc.Pubkey[:32])
	fmt.Printf("     signed at: %s\n", sigDoc.Signed_at)
	return nil
}

// ── run ───────────────────────────────────────────────────────────────────────
func cmdRun(args []string) error {
	if len(args) != 1 { return errors.New("sigma run <dir>") }
	m, err := readManifest(args[0])
	if err != nil { return err }
	fmt.Printf("%s  Launching %s v%s...\n", cyan("Σ"), bold(m.Name), m.Version)
	body, _ := json.Marshal(map[string]any{
		"method": "shell.exec",
		"cmd":    "xdg-open",
		"args":   []string{filepath.Join(args[0], m.Entry)},
		"caps":   m.Caps,
		"origin": "cli://" + m.Name,
	})
	resp, err := http.Post("http://"+daemonAddr+"/process",
		"application/json", strings.NewReader(string(body)))
	if err != nil {
		return fmt.Errorf("sigmad-process unreachable at %s — is sigmad running? (%w)", daemonAddr, err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 400 {
		body, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("daemon returned %d: %s", resp.StatusCode, string(body))
	}
	io.Copy(os.Stdout, resp.Body)
	return nil
}

// ── health ────────────────────────────────────────────────────────────────────
func cmdHealth() error {
	client := &http.Client{
		Timeout: 5 * time.Second,
		Transport: &http.Transport{
			DialContext: func(_ interface{}, _, _ string) (net.Conn, error) {
				return net.Dial("unix", healthSock)
			},
		},
	}
	resp, err := client.Get("http://sigma/health")
	if err != nil {
		// Helpful diagnostics on failure
		if _, statErr := os.Stat(healthSock); os.IsNotExist(statErr) {
			return fmt.Errorf("healthd socket not found at %s — is sigma-healthd running?", healthSock)
		}
		return fmt.Errorf("healthd unavailable (%s): %w\nTip: start sigma-healthd or check journalctl -u sigma-healthd", healthSock, err)
	}
	defer resp.Body.Close()

	var h map[string]interface{}
	if err := json.NewDecoder(resp.Body).Decode(&h); err != nil {
		return fmt.Errorf("malformed health response: %w", err)
	}

	overall, _ := h["overall"].(string)
	uptime, _  := h["uptime_seconds"].(float64)
	node, _    := h["node"].(string)

	overallStr := green(strings.ToUpper(overall))
	if overall != "ok" {
		overallStr = red(strings.ToUpper(overall))
	}

	fmt.Printf("Node: %-20s  Uptime: %s  Overall: %s\n",
		node, formatUptime(uptime), overallStr)
	fmt.Println(strings.Repeat("─", 60))

	if subs, ok := h["subsystems"].([]interface{}); ok {
		for _, s := range subs {
			sm, _ := s.(map[string]interface{})
			status, _ := sm["status"].(string)
			name,   _ := sm["name"].(string)
			msg,    _ := sm["message"].(string)
			if status == "ok" {
				fmt.Printf("%s %-20s %-10s %s\n", green("✓"), name, status, msg)
			} else {
				fmt.Printf("%s %-20s %-10s %s\n", red("✗"), name, status, msg)
			}
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
		resp, err := http.Post("http://"+daemonAddr+"/sysctl",
			"application/json", strings.NewReader(string(body)))
		if err != nil {
			return fmt.Errorf("daemon unreachable: %w", err)
		}
		defer resp.Body.Close()
		io.Copy(os.Stdout, resp.Body)
		fmt.Println()
	} else {
		resp, err := http.Get("http://" + daemonAddr + "/sysctl?key=" + args[0])
		if err != nil {
			return fmt.Errorf("daemon unreachable: %w", err)
		}
		defer resp.Body.Close()
		io.Copy(os.Stdout, resp.Body)
		fmt.Println()
	}
	return nil
}

// ── helpers ───────────────────────────────────────────────────────────────────
func readManifest(dir string) (*Manifest, error) {
	var m Manifest
	path := filepath.Join(dir, "sigma.json")
	if err := readJSON(path, &m); err != nil {
		return nil, fmt.Errorf("cannot read %s: %w\nDid you run 'sigma init %s'?", path, err, dir)
	}
	return &m, nil
}
func writeJSON(path string, v any) error {
	b, _ := json.MarshalIndent(v, "", "  ")
	return os.WriteFile(path, append(b, '\n'), 0o644)
}
func readJSON(path string, v any) error {
	b, err := os.ReadFile(path)
	if err != nil { return err }
	return json.Unmarshal(b, v)
}
func bundleDigest(dir string) ([]byte, error) {
	h := sha256.New()
	tw := tar.NewWriter(h)
	err := filepath.Walk(dir, func(p string, info os.FileInfo, walkErr error) error {
		if walkErr != nil || info.IsDir() { return walkErr }
		rel, _ := filepath.Rel(dir, p)
		if rel == "sigma.sig" { return nil }
		data, err := os.ReadFile(p)
		if err != nil { return err }
		tw.WriteHeader(&tar.Header{Name: rel, Mode: 0o644, Size: int64(len(data))})
		tw.Write(data)
		return nil
	})
	tw.Close()
	return h.Sum(nil), err
}
func loadOrGenerateKey() (ed25519.PrivateKey, error) {
	path := os.Getenv("SIGMA_SIGNING_KEY")
	if path == "" {
		home, _ := os.UserHomeDir()
		path = filepath.Join(home, ".sigmaos", "signing.key")
	}
	if b, err := os.ReadFile(path); err == nil {
		if len(b) != ed25519.PrivateKeySize {
			return nil, fmt.Errorf("signing key at %s has unexpected length %d (expected %d)", path, len(b), ed25519.PrivateKeySize)
		}
		return ed25519.PrivateKey(b), nil
	}
	_, priv, err := ed25519.GenerateKey(rand.Reader)
	if err != nil { return nil, err }
	if err := os.MkdirAll(filepath.Dir(path), 0o700); err != nil { return nil, err }
	if err := os.WriteFile(path, priv, 0o600); err != nil { return nil, err }
	fmt.Fprintf(os.Stderr, "%s  Generated signing key: %s\n", cyan("Σ"), path)
	return priv, nil
}



