"use strict";

export const SigmaSecurityHandlers = {
    executeAmnesicScrub: (system) => {
        const progress = document.getElementById('scrub-progress');
        if (!progress) return;
        progress.innerHTML = 'INITIATING FORENSIC OVERWRITE...';
        let i = 0;
        const interval = setInterval(() => {
            if (i >= 100) {
                clearInterval(interval);
                system.vfs.fs = {}; // Pure silicon zeroing
                progress.innerHTML = 'PURGE COMPLETE. VFS WIPED.';
                system.spawnToast('Amnesic: Forensic data scrub finished.');
                return;
            }
            i += 10;
            progress.innerHTML = `SCRUBBING RAM-DISK: ${i}%`;
        }, 300);
    },

    runKaliScan: (system) => {
        const log = document.getElementById('kali-log');
        if (!log) return;
        log.innerHTML = 'ENUMERATING LOCAL SILICON PORTS...';
        system.spawnToast('Kali Eq: Dispersing raw SYN port scanning kernel.');
        setTimeout(() => {
            log.innerHTML = '<span class="u-error-text">SCAN COMPLETE. Open Vectors: 22, 80, 443, 6500.</span>';
        }, 1600);
    }
};
