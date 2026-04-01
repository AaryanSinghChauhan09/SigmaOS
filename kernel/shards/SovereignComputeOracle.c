/**
 * Σ SIGMAOS: SOVEREIGN MICRO-ORACLE SHARD (Perplexity Computer local eq)
 * Industry Disruption: Localized knowledge-graph search primitive over VFS RAM-Disk.
 * Bypasses web APIs using pure string iteration over raw silicon memory.
 */



int sigma_str_len(const char* s) {
    const char* p = s;
    while (*p) p++;
    return (int)(p - s);
}

/**
 * SIGMA_SILICON_SEARCH
 * Extremely stripped back exact-match token indexer simulating local search without DB software.
 */
int sigma_silicon_search(const char* corpus, const char* token) {
    if (!token || !*token) return 0;
    
    int t_len = sigma_str_len(token);
    int matches = 0;
    
    for (const char* p = corpus; *p; p++) {
        int i;
        for (i = 0; i < t_len; i++) {
            if (p[i] != token[i]) break;
        }
        if (i == t_len) {
            matches++;
            p += t_len - 1; // Advance
        }
    }
    return matches;
}
