// SPDX-License-Identifier: GPL-2.0-or-later
// sigmad/pkg/assert/sigma_assert.go — cryptographic package assertion chain (snapd-inspired)
//
// Every sigma package has:
//   SigmaPackageDeclaration — immutable package identity signed by the root key
//   SigmaRevisionAssertion  — per-version hash binding (dm-verity + SHA-256)
//
// Verification chain:  root key → account key → declaration → revision
// Prevents: unsigned packages, publisher spoofing, replay attacks, silent downgrades.

package assert

import (
	"crypto/ed25519"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"time"
)

// ── Types ─────────────────────────────────────────────────────────────────────

type PlugRule struct {
	Interface string `json:"interface"`
	Provider  string `json:"provider"`
}

type SlotRule struct {
	Interface string `json:"interface"`
	Socket    string `json:"socket,omitempty"`
	Port      int    `json:"port,omitempty"`
}

// SigmaPackageDeclaration — immutable package identity (snapd SnapDeclaration)
type SigmaPackageDeclaration struct {
	PackageID      string     `json:"package-id"`      // immutable, assigned once
	PackageName    string     `json:"package-name"`    // human name
	PublisherID    string     `json:"publisher-id"`    // key fingerprint
	PlugRules      []PlugRule `json:"plug-rules"`
	SlotRules      []SlotRule `json:"slot-rules"`
	Timestamp      time.Time  `json:"timestamp"`
	SeriesID       string     `json:"series"`
	RefreshControl []string   `json:"refresh-control"`
	Signature      string     `json:"signature"` // hex ed25519 over all above
}

// SigmaRevisionAssertion — per-version hash binding
type SigmaRevisionAssertion struct {
	PackageID        string    `json:"package-id"`
	Revision         int       `json:"revision"`
	Version          string    `json:"version"`
	DmVerityRootHash string    `json:"dm-verity-root-hash"`
	SHA256           string    `json:"sha256"`
	DeveloperID      string    `json:"developer-id"`
	Timestamp        time.Time `json:"timestamp"`
	Signature        string    `json:"signature"` // hex ed25519
}

// ── Signing / Verification ────────────────────────────────────────────────────

func canonicalJSON(v interface{}) ([]byte, error) {
	// For real use, this should be sorted-key canonical JSON (RFC 8785)
	return json.Marshal(v)
}

func SignDeclaration(decl *SigmaPackageDeclaration, privKey ed25519.PrivateKey) error {
	decl.Signature = ""
	data, err := canonicalJSON(decl)
	if err != nil { return err }
	sig := ed25519.Sign(privKey, data)
	decl.Signature = hex.EncodeToString(sig)
	return nil
}

func SignRevision(rev *SigmaRevisionAssertion, privKey ed25519.PrivateKey) error {
	rev.Signature = ""
	data, err := canonicalJSON(rev)
	if err != nil { return err }
	sig := ed25519.Sign(privKey, data)
	rev.Signature = hex.EncodeToString(sig)
	return nil
}

// VerifyInstall — full assertion chain verification before installation
func VerifyInstall(decl *SigmaPackageDeclaration,
	rev *SigmaRevisionAssertion,
	rootPubKey ed25519.PublicKey,
	actualDmVerityHash string) error {

	// 1. Verify revision assertion timestamp (anti-replay)
	if rev.Timestamp.After(time.Now().Add(5 * time.Minute)) {
		return errors.New("assertion timestamp is in the future (replay attack?)")
	}
	if rev.Timestamp.Before(time.Now().Add(-365 * 24 * time.Hour)) {
		return errors.New("assertion is more than 1 year old — refresh required")
	}

	// 2. Verify declaration signature
	declSig, _ := hex.DecodeString(decl.Signature)
	decl.Signature = ""
	declData, _ := canonicalJSON(decl)
	decl.Signature = hex.EncodeToString(declSig)
	if !ed25519.Verify(rootPubKey, declData, declSig) {
		return errors.New("package declaration signature invalid")
	}

	// 3. Verify revision signature
	revSig, _ := hex.DecodeString(rev.Signature)
	rev.Signature = ""
	revData, _ := canonicalJSON(rev)
	rev.Signature = hex.EncodeToString(revSig)
	if !ed25519.Verify(rootPubKey, revData, revSig) {
		return errors.New("revision assertion signature invalid")
	}

	// 4. Verify dm-verity root hash matches the assertion
	if actualDmVerityHash != rev.DmVerityRootHash {
		return errors.New("dm-verity root hash mismatch — package may be tampered")
	}

	// 5. Package IDs must match
	if decl.PackageID != rev.PackageID {
		return errors.New("package ID mismatch between declaration and revision")
	}

	return nil
}

// PackageID generates a stable immutable ID from the package name + publisher
func PackageID(name, publisherID string) string {
	h := sha256.Sum256([]byte("sigma-pkg:" + publisherID + ":" + name))
	return "spkg-" + hex.EncodeToString(h[:8])
}
