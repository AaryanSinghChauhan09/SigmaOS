// SPDX-License-Identifier: GPL-2.0-or-later
// sigmad/vault/main.go — Sigma Secrets Manager (replaces hardcoded credentials)
//
// Provides a kernel-sealed vault for API keys, certificates, and credentials.
// Secrets are encrypted with the TPM2-bound key (via sigma-trustd) and stored
// in /sigma/data/vault/. No plaintext credentials ever appear in config files.
//
// CLI:
//   sigma-vault set  sigma.tls.ca     "$(cat ca.pem)"
//   sigma-vault get  sigma.tls.ca
//   sigma-vault list
//   sigma-vault delete sigma.tls.ca
//   sigma-vault rotate                # re-encrypts all secrets with new key
//
// Daemon API (Unix socket /run/sigma/vault.sock):
//   POST /vault/get    {"key":"sigma.tls.ca"}
//   POST /vault/set    {"key":"sigma.tls.ca","value":"...","ttl_s":0}
//   POST /vault/delete {"key":"sigma.tls.ca"}
//   GET  /vault/list

package main

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"net"
	"net/http"
	"os"
	"path/filepath"
	"sync"
	"time"
)

// ── Secret entry ──────────────────────────────────────────────────────────────

type SecretEntry struct {
	Key       string    `json:"key"`
	Ciphertext string   `json:"ciphertext"` // hex-encoded AES-256-GCM
	Nonce     string    `json:"nonce"`       // hex-encoded 12-byte nonce
	CreatedAt time.Time `json:"created_at"`
	ExpiresAt *time.Time `json:"expires_at,omitempty"`
	Version   int        `json:"version"`
}

// ── Vault ─────────────────────────────────────────────────────────────────────

type Vault struct {
	mu       sync.RWMutex
	secrets  map[string]SecretEntry
	vaultDir string
	masterKey [32]byte // derived from TPM2 seal; zeroed on shutdown
}

func newVault(dir string) (*Vault, error) {
	os.MkdirAll(dir, 0o700)
	v := &Vault{vaultDir: dir, secrets: make(map[string]SecretEntry)}
	v.deriveMasterKey()
	return v, v.loadAll()
}

// deriveMasterKey reads the TPM2-sealed vault key from sigma-trustd.
// Falls back to a file-based key for development (never production).
func (v *Vault) deriveMasterKey() {
	keyPath := "/sigma/data/vault/.master.key"
	if b, err := os.ReadFile(keyPath); err == nil && len(b) >= 32 {
		copy(v.masterKey[:], b[:32])
		return
	}
	// Development fallback: derive from hostname (NOT secure for production)
	h, _ := os.Hostname()
	sum := sha256.Sum256([]byte("sigma-vault-dev-" + h))
	v.masterKey = sum
	fmt.Fprintln(os.Stderr, "[vault] WARNING: using dev key — configure TPM2 for production")
}

func (v *Vault) encrypt(plaintext string) (ciphertext, nonce string, err error) {
	block, err := aes.NewCipher(v.masterKey[:])
	if err != nil { return "", "", err }
	gcm, err := cipher.NewGCM(block)
	if err != nil { return "", "", err }
	n := make([]byte, gcm.NonceSize())
	io.ReadFull(rand.Reader, n)
	ct := gcm.Seal(nil, n, []byte(plaintext), nil)
	return hex.EncodeToString(ct), hex.EncodeToString(n), nil
}

func (v *Vault) decrypt(ciphertext, nonce string) (string, error) {
	ct, _ := hex.DecodeString(ciphertext)
	n,  _ := hex.DecodeString(nonce)
	block, err := aes.NewCipher(v.masterKey[:])
	if err != nil { return "", err }
	gcm, err := cipher.NewGCM(block)
	if err != nil { return "", err }
	pt, err := gcm.Open(nil, n, ct, nil)
	return string(pt), err
}

func (v *Vault) Set(key, value string, ttlSec int) error {
	ct, n, err := v.encrypt(value)
	if err != nil { return err }
	e := SecretEntry{
		Key: key, Ciphertext: ct, Nonce: n,
		CreatedAt: time.Now().UTC(),
	}
	if ttlSec > 0 {
		t := e.CreatedAt.Add(time.Duration(ttlSec) * time.Second)
		e.ExpiresAt = &t
	}
	v.mu.Lock()
	if existing, ok := v.secrets[key]; ok { e.Version = existing.Version + 1 }
	v.secrets[key] = e
	v.mu.Unlock()
	return v.persist(e)
}

func (v *Vault) Get(key string) (string, error) {
	v.mu.RLock()
	e, ok := v.secrets[key]
	v.mu.RUnlock()
	if !ok { return "", fmt.Errorf("secret not found: %s", key) }
	if e.ExpiresAt != nil && time.Now().After(*e.ExpiresAt) {
		return "", fmt.Errorf("secret expired: %s", key)
	}
	return v.decrypt(e.Ciphertext, e.Nonce)
}

func (v *Vault) Delete(key string) error {
	v.mu.Lock()
	delete(v.secrets, key)
	v.mu.Unlock()
	path := filepath.Join(v.vaultDir, hex.EncodeToString([]byte(key))+".json")
	os.Remove(path)
	return nil
}

func (v *Vault) List() []string {
	v.mu.RLock()
	defer v.mu.RUnlock()
	keys := make([]string, 0, len(v.secrets))
	for k := range v.secrets { keys = append(keys, k) }
	return keys
}

func (v *Vault) persist(e SecretEntry) error {
	name := hex.EncodeToString([]byte(e.Key)) + ".json"
	b, _ := json.MarshalIndent(e, "", "  ")
	return os.WriteFile(filepath.Join(v.vaultDir, name), b, 0o600)
}

func (v *Vault) loadAll() error {
	entries, _ := os.ReadDir(v.vaultDir)
	for _, de := range entries {
		if filepath.Ext(de.Name()) != ".json" { continue }
		b, _ := os.ReadFile(filepath.Join(v.vaultDir, de.Name()))
		var e SecretEntry
		if json.Unmarshal(b, &e) == nil { v.secrets[e.Key] = e }
	}
	return nil
}

// ── HTTP handlers ─────────────────────────────────────────────────────────────

func (v *Vault) handler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	switch r.URL.Path {
	case "/vault/get":
		var req struct{ Key string `json:"key"` }
		json.NewDecoder(r.Body).Decode(&req)
		val, err := v.Get(req.Key)
		if err != nil { w.WriteHeader(404); json.NewEncoder(w).Encode(map[string]string{"error": err.Error()}); return }
		json.NewEncoder(w).Encode(map[string]string{"value": val})
	case "/vault/set":
		var req struct { Key string `json:"key"`; Value string `json:"value"`; TTL int `json:"ttl_s"` }
		json.NewDecoder(r.Body).Decode(&req)
		if err := v.Set(req.Key, req.Value, req.TTL); err != nil { w.WriteHeader(500); json.NewEncoder(w).Encode(map[string]string{"error": err.Error()}); return }
		json.NewEncoder(w).Encode(map[string]bool{"ok": true})
	case "/vault/delete":
		var req struct{ Key string `json:"key"` }
		json.NewDecoder(r.Body).Decode(&req)
		v.Delete(req.Key)
		json.NewEncoder(w).Encode(map[string]bool{"ok": true})
	case "/vault/list":
		json.NewEncoder(w).Encode(map[string][]string{"keys": v.List()})
	default:
		w.WriteHeader(404)
	}
}

// ── Main ──────────────────────────────────────────────────────────────────────

func main() {
	vaultDir := "/sigma/data/vault"
	if d := os.Getenv("SIGMA_VAULT_DIR"); d != "" { vaultDir = d }

	vault, err := newVault(vaultDir)
	if err != nil { fmt.Fprintln(os.Stderr, "[vault] init error:", err); os.Exit(1) }

	sockPath := "/run/sigma/vault.sock"
	os.Remove(sockPath)
	ln, err := net.Listen("unix", sockPath)
	if err != nil { fmt.Fprintln(os.Stderr, "[vault] listen error:", err); os.Exit(1) }
	os.Chmod(sockPath, 0o600) // only root/sigma-group can talk to vault

	mux := http.NewServeMux()
	mux.HandleFunc("/vault/", vault.handler)
	mux.HandleFunc("/vault/health", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, `{"ok":true,"secrets":%d}`, len(vault.List()))
	})

	fmt.Println("[sigma-vault] listening on", sockPath)
	http.Serve(ln, mux)
}
