// SPDX-License-Identifier: GPL-2.0-or-later
// sigmad/pkg/sigma_pkg_txn_lock.go — sysroot exclusive lock (rpm-ostree-inspired)
//
// Prevents concurrent sigma-pkg operations from corrupting the package database.
// flock() on /run/sigma/pkg.lock — any second install returns -EBUSY immediately.
// Client identity (exe, PID) is captured at lock acquisition for audit trail.

package main

import (
	"fmt"
	"os"
	"syscall"
	"time"
)

const pkgLockPath = "/run/sigma/pkg.lock"

type SysrootLock struct {
	fd          *os.File
	LockedAt    time.Time
	ClientPID   int
	ClientExe   string
	ClientUnit  string
	PackageName string
}

// Acquire exclusive sysroot lock. Returns error if another transaction is running.
func AcquireSysrootLock(packageName string) (*SysrootLock, error) {
	f, err := os.OpenFile(pkgLockPath, os.O_CREATE|os.O_WRONLY, 0o600)
	if err != nil {
		return nil, fmt.Errorf("cannot open pkg lock: %w", err)
	}

	// Non-blocking exclusive lock — fail immediately if busy (rpm-ostree pattern)
	err = syscall.Flock(int(f.Fd()), syscall.LOCK_EX|syscall.LOCK_NB)
	if err != nil {
		f.Close()
		return nil, fmt.Errorf("another sigma-pkg transaction is running (EBUSY)")
	}

	// Capture client identity for audit (rpm-ostree: client_description, sd_unit)
	clientExe := "unknown"
	if exe, err := os.Readlink("/proc/self/exe"); err == nil { clientExe = exe }

	clientUnit := os.Getenv("SYSTEMD_UNIT")
	if clientUnit == "" { clientUnit = fmt.Sprintf("manual pid=%d", os.Getpid()) }

	lock := &SysrootLock{
		fd:          f,
		LockedAt:    time.Now(),
		ClientPID:   os.Getpid(),
		ClientExe:   clientExe,
		ClientUnit:  clientUnit,
		PackageName: packageName,
	}

	// Write lock info to file (for diagnostic: cat /run/sigma/pkg.lock)
	fmt.Fprintf(f, "pid=%d unit=%s pkg=%s locked_at=%s\n",
		lock.ClientPID, lock.ClientUnit, packageName,
		lock.LockedAt.UTC().Format(time.RFC3339))

	return lock, nil
}

// Release the sysroot lock — called on txn commit or abort
func (l *SysrootLock) Release() {
	syscall.Flock(int(l.fd.Fd()), syscall.LOCK_UN)
	l.fd.Close()
	os.Remove(pkgLockPath)
}

// IsLocked checks if a transaction is in progress (for status display)
func IsLocked() bool {
	f, err := os.OpenFile(pkgLockPath, os.O_RDONLY, 0)
	if err != nil { return false }
	defer f.Close()
	err = syscall.Flock(int(f.Fd()), syscall.LOCK_EX|syscall.LOCK_NB)
	if err != nil { return true }  // couldn't acquire = locked
	syscall.Flock(int(f.Fd()), syscall.LOCK_UN)
	return false
}
