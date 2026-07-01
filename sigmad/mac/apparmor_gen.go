// SPDX-License-Identifier: GPL-2.0-or-later
// sigmad/mac/apparmor_gen.go — Auto-generate AppArmor profiles from plug declarations
// (snapd interfaces/apparmor/apparmor.go-inspired)
//
// Every SigmaOS workload gets a deny-all AppArmor profile generated from its
// plug/slot declarations. Only declared interfaces open holes in the profile.
// AARE metacharacters are validated before embedding in the profile string.

package mac

import (
	"errors"
	"fmt"
	"strings"
	"text/template"
)

// AppArmor AARE metacharacters — must not appear in workload names (snapd pattern)
const aareChars = `?*[]{}^"` + "\x00"

func ValidateWorkloadName(name string) error {
	if strings.ContainsAny(name, aareChars) {
		return fmt.Errorf("workload name %q contains reserved AppArmor AARE char from %q",
			name, aareChars)
	}
	if len(name) == 0 { return errors.New("workload name must not be empty") }
	return nil
}

// WorkloadProfile describes a workload's security requirements
type WorkloadProfile struct {
	WorkloadName    string
	BinaryPath      string
	TrustLevel      string   // "KERNEL" | "SYSTEM" | "PRIVILEGED" | "USER" | "ISOLATED" | "UNTRUSTED"
	AllowedPaths    []string // from rpath plug declarations
	AllowNetwork    bool     // has sigma-net plug
	AllowUnixSocket bool     // has sigma-rpc plug
	AllowCamera     bool     // has hw:camera plug
	AllowAudio      bool     // has hw:audio plug
}

var profileTemplate = template.Must(template.New("aa").Parse(
`#include <tunables/global>

profile sigma-{{.WorkloadName}} {{.BinaryPath}} flags=(enforce) {
  #include <abstractions/base>

  # ── Default deny ──────────────────────────────────────────────────────
  deny /**              rwklmx,
  deny /proc/**         rwklmx,
  deny /sys/**          rwklmx,
  deny network,
  deny capability,

  # ── Own data directory ────────────────────────────────────────────────
  /sigma/data/{{.WorkloadName}}/    rw,
  /sigma/data/{{.WorkloadName}}/**  rw,

  # ── Own binary ────────────────────────────────────────────────────────
  {{.BinaryPath}}  mr,

  # ── Allowed paths (from rpath/wpath plug declarations) ────────────────
  {{- range .AllowedPaths}}
  {{.}}  r,{{- end}}

  {{- if .AllowNetwork}}
  # ── sigma-net plug ────────────────────────────────────────────────────
  network tcp,
  network udp,
  {{- end}}

  {{- if .AllowUnixSocket}}
  # ── sigma-rpc plug — connect to sigma-apid ────────────────────────────
  unix (connect) type=stream peer=(label=sigma-apid),
  {{- end}}

  {{- if .AllowCamera}}
  # ── hw:camera plug ────────────────────────────────────────────────────
  /dev/video*  rw,
  {{- end}}

  {{- if .AllowAudio}}
  # ── hw:audio plug ─────────────────────────────────────────────────────
  /dev/snd/**  rw,
  {{- end}}

  # ── Capability grants based on trust level ────────────────────────────
  {{- if eq .TrustLevel "PRIVILEGED"}}
  capability net_admin,
  capability sys_admin,
  capability sys_ptrace,
  {{- else if eq .TrustLevel "SYSTEM"}}
  capability net_admin,
  {{- end}}
}
`))

func GenerateProfile(w WorkloadProfile) (string, error) {
	if err := ValidateWorkloadName(w.WorkloadName); err != nil {
		return "", err
	}
	if w.BinaryPath == "" {
		return "", errors.New("binary path must not be empty")
	}
	// Validate all allowed paths too
	for _, p := range w.AllowedPaths {
		if strings.ContainsAny(p, aareChars) {
			return "", fmt.Errorf("allowed path %q contains AARE metachar", p)
		}
	}

	var buf strings.Builder
	if err := profileTemplate.Execute(&buf, w); err != nil {
		return "", fmt.Errorf("profile template: %w", err)
	}
	return buf.String(), nil
}

// LoadProfile writes and loads the generated profile into the running AppArmor LSM
func LoadProfile(profile string) error {
	// Real impl: write to /sys/kernel/security/apparmor/.replace
	// For now: stub that logs and returns success
	_ = profile
	return nil
}
