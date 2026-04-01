"use strict";

/**
 * Σ SOVEREIGN VFS (Virtual File System)
 * High-performance, persistent sharded storage.
 */
export class SigmaVFS {
    constructor() {
        this.storageKey = 'SIGMAOS_VFS_ZENITH';
        this.fs = JSON.parse(localStorage.getItem(this.storageKey)) || this.getDefaultFS();
        this.sync();
    }

    getDefaultFS() {
        return {
            '/root': { type: 'dir', children: ['bin', 'kernel', 'userland', 'data', 'media', 'etc'] },
            '/root/bin': { type: 'dir', children: ['sigma_shell', 'sigmactl'] },
            '/root/kernel': { type: 'dir', children: ['sigma_core.asm', 'boot_master.c'] },
            '/root/userland': { type: 'dir', children: [] },
            '/root/data': { type: 'dir', children: ['industrial.json', 'audit.log', 'secret.txt', 'password.db'] },
            '/root/media': { type: 'dir', children: [] },
            '/root/etc': { type: 'dir', children: ['sigmaos.conf'] },
            '/root/data/industrial.json': { type: 'file', content: '{"status": "SOVEREIGN", "integrity": 100}' },
            '/root/etc/sigmaos.conf': { type: 'file', content: 'VERSION=160.0\nTHEME=ZENITH\nMODE=SUPREME' },
            '/root/data/secret.txt': { type: 'file', content: 'SIGMA_KEY=sh-4309-882' }
        };
    }

    sync() { localStorage.setItem(this.storageKey, JSON.stringify(this.fs)); }

    exists(path) { return !!this.fs[path]; }
    isDir(path) { return this.fs[path] && this.fs[path].type === 'dir'; }
    ls(path) { return this.fs[path] ? this.fs[path].children : []; }

    mkdir(path) {
        if (this.exists(path)) return false;
        const parts = path.split('/');
        const name = parts.pop();
        const parent = parts.join('/') || '/root';
        if (!this.isDir(parent)) return false;
        this.fs[path] = { type: 'dir', children: [] };
        if (!this.fs[parent].children.includes(name)) this.fs[parent].children.push(name);
        this.sync();
        return true;
    }

    write(path, content) {
        const parts = path.split('/');
        const name = parts.pop();
        const parent = parts.join('/') || '/root';
        if (!this.isDir(parent)) return false;
        if (!this.exists(path)) {
            if (!this.fs[parent].children.includes(name)) this.fs[parent].children.push(name);
        }
        this.fs[path] = { type: 'file', content };
        this.sync();
        return true;
    }

    read(path) { return this.fs[path] ? this.fs[path].content : null; }

    remove(path) {
        if (!this.exists(path)) return false;
        const parts = path.split('/');
        const name = parts.pop();
        const parent = parts.join('/') || '/root';
        this.fs[parent].children = this.fs[parent].children.filter(c => c !== name);
        delete this.fs[path];
        this.sync();
        return true;
    }

    snapshot(name) {
        const state = JSON.stringify(this.fs);
        localStorage.setItem(`SNAPSHOT_${name.toUpperCase()}`, state);
        return true;
    }

    rollback(name) {
        const state = localStorage.getItem(`SNAPSHOT_${name.toUpperCase()}`);
        if (!state) return false;
        this.fs = JSON.parse(state);
        this.sync();
        return true;
    }
}
