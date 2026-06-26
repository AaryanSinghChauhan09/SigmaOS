// SPDX-License-Identifier: GPL-2.0-or-later
// sigmad/search/main.go — Global Search daemon (Spotlight-style)
//
// Indexes filesystem, app manifests, sysctl keys, and service names.
// Query via:  GET /search?q=ffmpeg&limit=10
// Returns ranked JSON results with source, title, path, score.
//
// sigma CLI: sigma search "ffmpeg"
// Extension: navigator.sigmaos.search.query("ffmpeg")

package main

import (
	"encoding/json"
	"fmt"
	"net"
	"net/http"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"sync"
	"time"
	"unicode"
)

// ── Result ────────────────────────────────────────────────────────────────────

type SearchResult struct {
	Title    string  `json:"title"`
	Path     string  `json:"path"`
	Source   string  `json:"source"` // "fs" | "app" | "sysctl" | "service" | "secret"
	Snippet  string  `json:"snippet,omitempty"`
	Score    float64 `json:"score"`
}

// ── Index ─────────────────────────────────────────────────────────────────────

type IndexEntry struct {
	Title   string
	Path    string
	Source  string
	Snippet string
	Keywords []string
}

type SearchIndex struct {
	mu      sync.RWMutex
	entries []IndexEntry
	built   time.Time
}

func normalize(s string) string {
	return strings.Map(func(r rune) rune {
		if unicode.IsSpace(r) { return ' ' }
		return unicode.ToLower(r)
	}, s)
}

func score(entry IndexEntry, query string) float64 {
	q := normalize(query)
	title := normalize(entry.Title)
	path  := normalize(entry.Path)
	s := 0.0
	// Exact title match = highest score
	if title == q             { return 100.0 }
	if strings.HasPrefix(title, q) { s += 80 }
	if strings.Contains(title, q)  { s += 50 }
	if strings.Contains(path,  q)  { s += 20 }
	for _, kw := range entry.Keywords {
		if strings.Contains(normalize(kw), q) { s += 10; break }
	}
	return s
}

func (idx *SearchIndex) Build() {
	entries := []IndexEntry{}

	// ── Filesystem: home + /sigma/bin + /sigma/data ───────────────────────
	scanDirs := []string{
		"/home/sigma", "/sigma/bin", "/sigma/sbin",
		"/sigma/data", "/sigma/etc",
	}
	for _, root := range scanDirs {
		filepath.Walk(root, func(p string, info os.FileInfo, err error) error {
			if err != nil || info.IsDir() { return nil }
			if info.Size() > 100*1024*1024 { return nil } // skip huge files
			entries = append(entries, IndexEntry{
				Title:   info.Name(),
				Path:    p,
				Source:  "fs",
				Snippet: fmt.Sprintf("%s  %d bytes", p, info.Size()),
				Keywords: []string{filepath.Ext(p), filepath.Dir(p)},
			})
			return nil
		})
	}

	// ── App manifests ─────────────────────────────────────────────────────
	appDirs := []string{"/sigma/apps", "/sigma/data/apps"}
	for _, d := range appDirs {
		entries_, _ := os.ReadDir(d)
		for _, e := range entries_ {
			mpath := filepath.Join(d, e.Name(), "sigma.json")
			b, err := os.ReadFile(mpath)
			if err != nil { continue }
			var m struct { Name, Description string; Caps []string }
			json.Unmarshal(b, &m)
			entries = append(entries, IndexEntry{
				Title:   m.Name,
				Path:    mpath,
				Source:  "app",
				Snippet: m.Description,
				Keywords: append(m.Caps, m.Name),
			})
		}
	}

	// ── Sysctl keys ───────────────────────────────────────────────────────
	sysctls := []string{
		"kernel.sched.timeslice_ms", "kernel.sched.rt_threshold",
		"security.aslr.enabled", "security.aslr.entropy_bits",
		"net.firewall.conntrack_max", "security.zerotrust.enabled",
	}
	for _, k := range sysctls {
		entries = append(entries, IndexEntry{
			Title:  k, Path: "sysctl:" + k, Source: "sysctl",
			Snippet: "sigma-sysctl " + k,
		})
	}

	idx.mu.Lock()
	idx.entries = entries
	idx.built   = time.Now()
	idx.mu.Unlock()
}

func (idx *SearchIndex) Query(q string, limit int) []SearchResult {
	idx.mu.RLock()
	defer idx.mu.RUnlock()

	type scored struct {
		entry IndexEntry
		score float64
	}
	var results []scored
	for _, e := range idx.entries {
		s := score(e, q)
		if s > 0 { results = append(results, scored{e, s}) }
	}
	sort.Slice(results, func(i, j int) bool { return results[i].score > results[j].score })
	if limit > 0 && len(results) > limit { results = results[:limit] }

	out := make([]SearchResult, len(results))
	for i, r := range results {
		out[i] = SearchResult{r.entry.Title, r.entry.Path,
			r.entry.Source, r.entry.Snippet, r.score}
	}
	return out
}

// ── HTTP handler ──────────────────────────────────────────────────────────────

func main() {
	idx := &SearchIndex{}

	// Build index on startup; rebuild every 5 minutes
	go func() {
		for {
			idx.Build()
			fmt.Printf("[sigma-search] index built: %d entries\n", len(idx.entries))
			time.Sleep(5 * time.Minute)
		}
	}()

	sockPath := "/run/sigma/search.sock"
	os.Remove(sockPath)
	ln, err := net.Listen("unix", sockPath)
	if err != nil { fmt.Fprintln(os.Stderr, err); os.Exit(1) }

	mux := http.NewServeMux()
	mux.HandleFunc("/search", func(w http.ResponseWriter, r *http.Request) {
		q := r.URL.Query().Get("q")
		if q == "" { w.WriteHeader(400); return }
		limit := 20
		results := idx.Query(q, limit)
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]interface{}{
			"query":   q,
			"count":   len(results),
			"results": results,
			"indexed": idx.built.Format(time.RFC3339),
		})
	})
	mux.HandleFunc("/search/reindex", func(w http.ResponseWriter, r *http.Request) {
		go idx.Build()
		fmt.Fprintln(w, `{"ok":true}`)
	})

	fmt.Println("[sigma-search] listening on", sockPath)
	http.Serve(ln, mux)
}
