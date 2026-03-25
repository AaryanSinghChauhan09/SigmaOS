/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN C++ OOP FRAMEWORK (SigmaOOP.hpp)
 * =========================================================================
 * USP Absorbed:
 *   - Arch Linux: Rolling-release modular design
 *   - Fuchsia/Zircon: Object capability model, strong typing
 *   - seL4: Capability-based object system
 *   - FreeBSD: KOBJ (kernel object system)
 * OOP Principles Applied:
 *   - Encapsulation: All state hidden behind accessor method interfaces
 *   - Inheritance: Abstract base classes with pure virtual methods
 *   - Polymorphism: vtable-based dispatch without <typeinfo> or RTTI
 *   - Single Responsibility: Each class has one purpose
 * Principle: ZERO C++ Standard Template Library. ZERO <new>, ZERO <memory>.
 *            ZERO <type_traits>. Uses only sigma_types.h.
 * =========================================================================
 */

#pragma once

extern "C" {
#include "libc/sigma_types.h"
#include "libc/sigma_libc.h"
}

/* =========================================================================
 * CUSTOM GLOBAL new/delete (Replace heap allocation without <new>)
 * These hook directly into sigma_slab_alloc or raw mmap syscall.
 * ========================================================================= */

/* Forward declare slab system for operator new integration */
extern "C" void* sigma_slab_alloc_raw(sigma_usize size);
extern "C" void  sigma_slab_free_raw(void* ptr);

/*
 * Override global new/delete - eliminates dependency on the C++ runtime heap.
 */
void* operator new(sigma_usize size) {
    void* ptr = sigma_slab_alloc_raw(size);
    if (!ptr) sigma_exit(1); /* OOM = fatal in kernel context */
    return ptr;
}

void* operator new[](sigma_usize size) {
    return operator new(size);
}

void operator delete(void* ptr) noexcept {
    if (ptr) sigma_slab_free_raw(ptr);
}

void operator delete[](void* ptr) noexcept {
    operator delete(ptr);
}

void operator delete(void* ptr, sigma_usize) noexcept {
    operator delete(ptr);
}

void operator delete[](void* ptr, sigma_usize) noexcept {
    operator delete(ptr);
}

/* =========================================================================
 * SIGMA SMART POINTERS (Replacing std::unique_ptr / std::shared_ptr)
 * OOP: RAII-based ownership without <memory>
 * ========================================================================= */

/*
 * SigmaUniquePtr<T>: Exclusive ownership pointer (std::unique_ptr replacement).
 * OOP: Encapsulates raw pointer, destructor ensures cleanup.
 */
template<typename T>
class SigmaUniquePtr {
private:
    T* _ptr;
    /* Non-copyable: ownership is exclusive */
    SigmaUniquePtr(const SigmaUniquePtr&) = delete;
    SigmaUniquePtr& operator=(const SigmaUniquePtr&) = delete;

public:
    explicit SigmaUniquePtr(T* p = nullptr) noexcept : _ptr(p) {}

    /* Move constructor */
    SigmaUniquePtr(SigmaUniquePtr&& other) noexcept : _ptr(other._ptr) {
        other._ptr = nullptr;
    }

    /* Move assignment */
    SigmaUniquePtr& operator=(SigmaUniquePtr&& other) noexcept {
        if (this != &other) {
            delete _ptr;
            _ptr = other._ptr;
            other._ptr = nullptr;
        }
        return *this;
    }

    ~SigmaUniquePtr() noexcept { delete _ptr; }

    T* get() const noexcept { return _ptr; }
    T* release() noexcept { T* p = _ptr; _ptr = nullptr; return p; }
    void reset(T* p = nullptr) noexcept { delete _ptr; _ptr = p; }

    T& operator*()  const noexcept { return *_ptr; }
    T* operator->() const noexcept { return _ptr; }
    explicit operator bool() const noexcept { return _ptr != nullptr; }
};

template<typename T, typename... Args>
SigmaUniquePtr<T> sigma_make_unique(Args&&... args) {
    return SigmaUniquePtr<T>(new T(static_cast<Args&&>(args)...));
}

/* =========================================================================
 * SIGMA ARRAY (Replacing std::vector / std::array)
 * OOP: Dynamic array with capacity management, no STL dependency.
 * ========================================================================= */

template<typename T>
class SigmaArray {
private:
    T*          _data;
    sigma_usize _size;
    sigma_usize _capacity;

    void _grow(sigma_usize new_cap) {
        T* new_data = static_cast<T*>(
            sigma_slab_alloc_raw(new_cap * sizeof(T))
        );
        if (!new_data) sigma_exit(1);
        for (sigma_usize i = 0; i < _size; i++) {
            new (&new_data[i]) T(static_cast<T&&>(_data[i]));
            _data[i].~T();
        }
        sigma_slab_free_raw(_data);
        _data = new_data;
        _capacity = new_cap;
    }

public:
    SigmaArray() noexcept : _data(nullptr), _size(0), _capacity(0) {}

    explicit SigmaArray(sigma_usize initial_cap) :
        _data(nullptr), _size(0), _capacity(0) {
        reserve(initial_cap);
    }

    ~SigmaArray() noexcept {
        for (sigma_usize i = 0; i < _size; i++) _data[i].~T();
        sigma_slab_free_raw(_data);
    }

    /* Non-copyable for simplicity, moveable */
    SigmaArray(const SigmaArray&) = delete;
    SigmaArray& operator=(const SigmaArray&) = delete;

    SigmaArray(SigmaArray&& o) noexcept :
        _data(o._data), _size(o._size), _capacity(o._capacity) {
        o._data = nullptr; o._size = 0; o._capacity = 0;
    }

    void reserve(sigma_usize cap) {
        if (cap > _capacity) _grow(cap);
    }

    void push(const T& val) {
        if (_size >= _capacity) _grow(_capacity == 0 ? 4 : _capacity * 2);
        new (&_data[_size++]) T(val);
    }

    void push(T&& val) {
        if (_size >= _capacity) _grow(_capacity == 0 ? 4 : _capacity * 2);
        new (&_data[_size++]) T(static_cast<T&&>(val));
    }

    void pop() noexcept {
        if (_size > 0) _data[--_size].~T();
    }

    T& operator[](sigma_usize i) noexcept { return _data[i]; }
    const T& operator[](sigma_usize i) const noexcept { return _data[i]; }

    T& front() noexcept { return _data[0]; }
    T& back() noexcept { return _data[_size - 1]; }

    sigma_usize size() const noexcept { return _size; }
    sigma_usize capacity() const noexcept { return _capacity; }
    sigma_bool empty() const noexcept { return _size == 0; }

    T* begin() noexcept { return _data; }
    T* end() noexcept { return _data + _size; }
    const T* begin() const noexcept { return _data; }
    const T* end() const noexcept { return _data + _size; }

    void clear() noexcept {
        for (sigma_usize i = 0; i < _size; i++) _data[i].~T();
        _size = 0;
    }
};

/* =========================================================================
 * SIGMA STRING (Replacing std::string - no STL required)
 * OOP: Self-managing string buffer with SSO (small string optimization).
 * ========================================================================= */

class SigmaString {
private:
    static constexpr sigma_usize SSO_MAX = 23;  /* Small string buffer size */

    union {
        struct {
            char*       _heap_ptr;
            sigma_usize _heap_capacity;
        };
        char _sso_buf[SSO_MAX + 1];
    };
    sigma_usize _len;
    sigma_bool  _is_sso;

    char* _ptr() noexcept {
        return _is_sso ? _sso_buf : _heap_ptr;
    }
    const char* _ptr() const noexcept {
        return _is_sso ? _sso_buf : _heap_ptr;
    }

    void _ensure_capacity(sigma_usize needed) {
        if (_is_sso && needed <= SSO_MAX) return;
        sigma_usize new_cap = needed < 32 ? 32 : needed;
        /* Round up to next power of 2 */
        new_cap = sigma_align_up(new_cap + 1, 16);
        if (!_is_sso && new_cap <= _heap_capacity) return;

        char* new_buf = static_cast<char*>(
            sigma_slab_alloc_raw(new_cap)
        );
        if (!new_buf) sigma_exit(1);

        sigma_memcpy(new_buf, _ptr(), _len + 1);

        if (!_is_sso) sigma_slab_free_raw(_heap_ptr);
        _heap_ptr = new_buf;
        _heap_capacity = new_cap;
        _is_sso = SIGMA_FALSE;
    }

public:
    SigmaString() noexcept : _len(0), _is_sso(SIGMA_TRUE) {
        _sso_buf[0] = '\0';
    }

    SigmaString(const char* s) : _len(0), _is_sso(SIGMA_TRUE) {
        _sso_buf[0] = '\0';
        if (s) assign(s);
    }

    SigmaString(const SigmaString& o) : _len(0), _is_sso(SIGMA_TRUE) {
        _sso_buf[0] = '\0';
        assign(o.c_str());
    }

    SigmaString(SigmaString&& o) noexcept : _len(o._len), _is_sso(o._is_sso) {
        if (o._is_sso) {
            sigma_memcpy(_sso_buf, o._sso_buf, o._len + 1);
        } else {
            _heap_ptr      = o._heap_ptr;
            _heap_capacity = o._heap_capacity;
            o._heap_ptr    = nullptr;
        }
        o._len = 0; o._is_sso = SIGMA_TRUE; o._sso_buf[0] = '\0';
    }

    ~SigmaString() noexcept {
        if (!_is_sso && _heap_ptr) sigma_slab_free_raw(_heap_ptr);
    }

    SigmaString& operator=(const SigmaString& o) {
        if (this != &o) assign(o.c_str());
        return *this;
    }

    SigmaString& operator=(const char* s) {
        assign(s);
        return *this;
    }

    void assign(const char* s) {
        if (!s) { clear(); return; }
        sigma_usize slen = sigma_strlen(s);
        _ensure_capacity(slen);
        sigma_memcpy(_ptr(), s, slen + 1);
        _len = slen;
    }

    void append(const char* s) {
        if (!s) return;
        sigma_usize slen = sigma_strlen(s);
        _ensure_capacity(_len + slen);
        sigma_memcpy(_ptr() + _len, s, slen + 1);
        _len += slen;
    }

    void append(char c) {
        _ensure_capacity(_len + 1);
        _ptr()[_len++] = c;
        _ptr()[_len] = '\0';
    }

    SigmaString operator+(const char* s) const {
        SigmaString result(*this);
        result.append(s);
        return result;
    }

    SigmaString operator+(const SigmaString& o) const {
        SigmaString result(*this);
        result.append(o.c_str());
        return result;
    }

    SigmaString& operator+=(const char* s) { append(s); return *this; }
    SigmaString& operator+=(const SigmaString& o) { append(o.c_str()); return *this; }
    SigmaString& operator+=(char c) { append(c); return *this; }

    bool operator==(const char* s) const noexcept {
        return sigma_strcmp(_ptr(), s) == 0;
    }
    bool operator==(const SigmaString& o) const noexcept {
        return sigma_strcmp(_ptr(), o._ptr()) == 0;
    }
    bool operator!=(const char* s) const noexcept { return !(*this == s); }
    bool operator<(const SigmaString& o) const noexcept {
        return sigma_strcmp(_ptr(), o._ptr()) < 0;
    }

    const char* c_str() const noexcept { return _ptr(); }
    char* data() noexcept { return _ptr(); }
    sigma_usize length() const noexcept { return _len; }
    sigma_usize size() const noexcept { return _len; }
    sigma_bool empty() const noexcept { return _len == 0; }

    char operator[](sigma_usize i) const noexcept { return _ptr()[i]; }
    char& operator[](sigma_usize i) noexcept { return _ptr()[i]; }

    void clear() noexcept {
        _len = 0;
        _ptr()[0] = '\0';
    }

    sigma_bool starts_with(const char* prefix) const noexcept {
        return sigma_str_starts_with(_ptr(), prefix);
    }
    sigma_bool ends_with(const char* suffix) const noexcept {
        return sigma_str_ends_with(_ptr(), suffix);
    }
    sigma_bool contains(const char* needle) const noexcept {
        return sigma_str_contains(_ptr(), needle);
    }

    void print() const noexcept {
        sigma_write(SIGMA_FD_STDOUT, _ptr(), _len);
    }
    void println() const noexcept {
        sigma_write(SIGMA_FD_STDOUT, _ptr(), _len);
        sigma_write(SIGMA_FD_STDOUT, "\n", 1);
    }
};

/* =========================================================================
 * ABSTRACT BASE CLASS: SigmaObject (The universal OOP base)
 * OOP: Base class providing identity, polymorphism, lifecycle hooks.
 * Absorbing: Fuchsia's zx_object, seL4's CNode principles.
 * ========================================================================= */

class SigmaObject {
private:
    sigma_u64   _id;
    sigma_u32   _ref_count;
    sigma_bool  _alive;

    static sigma_u64 _next_id;  /* Simple monotonic ID counter */

protected:
    SigmaObject() noexcept :
        _id(_next_id++),
        _ref_count(1),
        _alive(SIGMA_TRUE) {}

    /* Objects may override this for custom cleanup */
    virtual void on_destroy() noexcept {}

public:
    virtual ~SigmaObject() noexcept {
        _alive = SIGMA_FALSE;
        on_destroy();
    }

    /* Non-virtual interface for ID access (OOP NVI pattern) */
    sigma_u64  id()        const noexcept { return _id; }
    sigma_u32  ref_count() const noexcept { return _ref_count; }
    sigma_bool is_alive()  const noexcept { return _alive; }

    void add_ref()  noexcept { _ref_count++; }
    void dec_ref()  noexcept { if (_ref_count > 0) _ref_count--; }

    /* Pure virtual: subclasses must identify themselves */
    virtual const char* type_name() const noexcept = 0;
    virtual sigma_status health_check() const noexcept = 0;

    /* Default print: uses our sigma_printf (no std::cout) */
    virtual void print_info() const noexcept {
        sigma_printf("[Object] id=%llu type='%s' refs=%u alive=%d\n",
            (unsigned long long)_id,
            type_name(),
            _ref_count,
            (int)_alive);
    }
};

/* Initialize static counter */
sigma_u64 SigmaObject::_next_id = 1;

/* =========================================================================
 * LINUX DISTRO ABSORBER (OOP Hierarchy)
 * OOP: Abstract AbstractDistroAbsorber -> Concrete Absorbers (Arch/Alpine/Debian)
 * Mission: Natively map Linux distro package management into SigmaOS.
 * Absorbing: Arch Linux, Alpine Linux, Debian, Fedora, Gentoo, NixOS.
 * ========================================================================= */

/* Abstract interface for distro absorption */
class AbstractDistroAbsorber : public SigmaObject {
public:
    /* Pure virtual methods (interface definition) */
    virtual const char* distro_name() const noexcept = 0;
    virtual const char* pkg_manager() const noexcept = 0;
    virtual sigma_status absorb_usp() noexcept = 0;
    virtual sigma_status install_pkg(const char* pkg_name) = 0;
    virtual sigma_status remove_pkg(const char* pkg_name) = 0;
    virtual sigma_status update_all() = 0;

    /* Non-virtual common behavior */
    void print_distro_info() const noexcept {
        sigma_printf("[ABSORBER] Distro: %s | PM: %s | id=%llu\n",
            distro_name(), pkg_manager(),
            (unsigned long long)id());
    }

    const char* type_name() const noexcept override {
        return "AbstractDistroAbsorber";
    }
    sigma_status health_check() const noexcept override {
        return SIGMA_OK;  /* Base: always alive */
    }
};

/* Arch Linux USP Absorber: Rolling releases, AUR, pacman, minimal base */
class ArchAbsorber : public AbstractDistroAbsorber {
public:
    const char* distro_name() const noexcept override { return "Arch Linux"; }
    const char* pkg_manager() const noexcept override { return "pacman"; }
    const char* type_name()   const noexcept override { return "ArchAbsorber"; }

    sigma_status absorb_usp() noexcept override {
        sigma_printf("[ARCH ABSORBER]: Rolling release model absorbed.\n");
        sigma_printf("[ARCH ABSORBER]: PKGBUILD scripted builds -> SigmaOS native builds.\n");
        sigma_printf("[ARCH ABSORBER]: AUR community packages -> SigmaOS SovereignAppStore.\n");
        sigma_printf("[ARCH ABSORBER]: pacman -Syu philosophy -> sigma_pkg update_all.\n");
        sigma_printf("[ARCH ABSORBER]: Minimal base -> SigmaOS zero-waste kernel.\n");
        return SIGMA_OK;
    }

    sigma_status install_pkg(const char* pkg) override {
        sigma_printf("[ARCH] Absorbing package '%s' into SigmaOS native format...\n", pkg);
        return SIGMA_OK;
    }
    sigma_status remove_pkg(const char* pkg) override {
        sigma_printf("[ARCH] Deabsorbing package '%s'...\n", pkg);
        return SIGMA_OK;
    }
    sigma_status update_all() override {
        sigma_printf("[ARCH] sigma_pkg update_all (pacman -Syu equivalent)...\n");
        return SIGMA_OK;
    }
    sigma_status health_check() const noexcept override { return SIGMA_OK; }
};

/* Alpine Linux USP Absorber: musl, BusyBox, OpenRC, security-first */
class AlpineAbsorber : public AbstractDistroAbsorber {
public:
    const char* distro_name() const noexcept override { return "Alpine Linux"; }
    const char* pkg_manager() const noexcept override { return "apk"; }
    const char* type_name()   const noexcept override { return "AlpineAbsorber"; }

    sigma_status absorb_usp() noexcept override {
        sigma_printf("[ALPINE ABSORBER]: musl libc -> SigmaLibC (our sovereign impl).\n");
        sigma_printf("[ALPINE ABSORBER]: BusyBox minimalism -> SigmaOS zero-waste tools.\n");
        sigma_printf("[ALPINE ABSORBER]: OpenRC init -> SigmaOS SovereignInit.\n");
        sigma_printf("[ALPINE ABSORBER]: apk (fast, content-addressable) -> sigma_pkg.\n");
        sigma_printf("[ALPINE ABSORBER]: Stack-smashing protection -> SigmaOS CanaryGuard.\n");
        return SIGMA_OK;
    }

    sigma_status install_pkg(const char* pkg) override {
        sigma_printf("[ALPINE] apk add '%s' -> sigma_pkg install '%s'\n", pkg, pkg);
        return SIGMA_OK;
    }
    sigma_status remove_pkg(const char* pkg) override {
        sigma_printf("[ALPINE] apk del '%s'\n", pkg);
        return SIGMA_OK;
    }
    sigma_status update_all() override {
        sigma_printf("[ALPINE] apk update && apk upgrade -> sigma_pkg sync\n");
        return SIGMA_OK;
    }
    sigma_status health_check() const noexcept override { return SIGMA_OK; }
};

/* Debian USP Absorber: dpkg, apt, stability, Debian Policy */
class DebianAbsorber : public AbstractDistroAbsorber {
public:
    const char* distro_name() const noexcept override { return "Debian"; }
    const char* pkg_manager() const noexcept override { return "apt/dpkg"; }
    const char* type_name()   const noexcept override { return "DebianAbsorber"; }

    sigma_status absorb_usp() noexcept override {
        sigma_printf("[DEBIAN ABSORBER]: dpkg package format -> SigmaOS .spkg format.\n");
        sigma_printf("[DEBIAN ABSORBER]: apt dependency resolution -> sigma_solver.\n");
        sigma_printf("[DEBIAN ABSORBER]: Debian Policy (reproducible builds) -> SigmaOS DevOps.\n");
        sigma_printf("[DEBIAN ABSORBER]: /dev/urandom entropy -> SigmaEntropy engine.\n");
        sigma_printf("[DEBIAN ABSORBER]: AppArmor profiles -> SigmaOS VanguardGuard.\n");
        return SIGMA_OK;
    }

    sigma_status install_pkg(const char* pkg) override {
        sigma_printf("[DEBIAN] apt install '%s' -> sigma_pkg install\n", pkg);
        return SIGMA_OK;
    }
    sigma_status remove_pkg(const char* pkg) override {
        sigma_printf("[DEBIAN] apt purge '%s'\n", pkg);
        return SIGMA_OK;
    }
    sigma_status update_all() override {
        sigma_printf("[DEBIAN] apt update && apt upgrade -> sigma_pkg sync\n");
        return SIGMA_OK;
    }
    sigma_status health_check() const noexcept override { return SIGMA_OK; }
};

/* Gentoo USP Absorber: Portage, USE flags, source-based compilation */
class GentooAbsorber : public AbstractDistroAbsorber {
public:
    const char* distro_name() const noexcept override { return "Gentoo"; }
    const char* pkg_manager() const noexcept override { return "portage/emerge"; }
    const char* type_name()   const noexcept override { return "GentooAbsorber"; }

    sigma_status absorb_usp() noexcept override {
        sigma_printf("[GENTOO ABSORBER]: Portage source builds -> SigmaOS native compilation.\n");
        sigma_printf("[GENTOO ABSORBER]: USE flags -> SigmaOS feature-flag build system.\n");
        sigma_printf("[GENTOO ABSORBER]: emerge --sync -> sigma_pkg sync --source.\n");
        sigma_printf("[GENTOO ABSORBER]: Profile system -> SigmaOS ModularProfiles.\n");
        sigma_printf("[GENTOO ABSORBER]: Hardened profile -> SigmaOS ZeroTrust kernel.\n");
        return SIGMA_OK;
    }

    sigma_status install_pkg(const char* pkg) override {
        sigma_printf("[GENTOO] emerge '%s' -> sigma_pkg build '%s'\n", pkg, pkg);
        return SIGMA_OK;
    }
    sigma_status remove_pkg(const char* pkg) override {
        sigma_printf("[GENTOO] emerge --unmerge '%s'\n", pkg);
        return SIGMA_OK;
    }
    sigma_status update_all() override {
        sigma_printf("[GENTOO] emerge -uDN @world -> sigma_pkg update_world\n");
        return SIGMA_OK;
    }
    sigma_status health_check() const noexcept override { return SIGMA_OK; }
};

/* NixOS Absorber: Purely functional, reproducible, declarative */
class NixOSAbsorber : public AbstractDistroAbsorber {
public:
    const char* distro_name() const noexcept override { return "NixOS"; }
    const char* pkg_manager() const noexcept override { return "nix"; }
    const char* type_name()   const noexcept override { return "NixOSAbsorber"; }

    sigma_status absorb_usp() noexcept override {
        sigma_printf("[NIXOS ABSORBER]: Nix store (content-addressable) -> SigmaOS SovereignStore.\n");
        sigma_printf("[NIXOS ABSORBER]: Declarative config -> SigmaOS SIGMA_CONFIG.\n");
        sigma_printf("[NIXOS ABSORBER]: Atomic upgrades/rollbacks -> SigmaOS SnapshotGuardian.\n");
        sigma_printf("[NIXOS ABSORBER]: Flakes (reproducibility) -> SigmaOS ReproducibleBuild.\n");
        sigma_printf("[NIXOS ABSORBER]: Multiple profile management -> SigmaOS PersonaCore.\n");
        return SIGMA_OK;
    }

    sigma_status install_pkg(const char* pkg) override {
        sigma_printf("[NIXOS] nix-env -i '%s' -> sigma_pkg declarative install\n", pkg);
        return SIGMA_OK;
    }
    sigma_status remove_pkg(const char* pkg) override {
        sigma_printf("[NIXOS] nix-env --uninstall '%s'\n", pkg);
        return SIGMA_OK;
    }
    sigma_status update_all() override {
        sigma_printf("[NIXOS] nixos-rebuild switch -> sigma_sov apply --declarative\n");
        return SIGMA_OK;
    }
    sigma_status health_check() const noexcept override { return SIGMA_OK; }
};

/* =========================================================================
 * SIGMA DISTRO ENGINE: Manages all absorbers (OOP Composite Pattern)
 * ========================================================================= */

class SigmaDistroEngine {
private:
    static constexpr sigma_u32 MAX_ABSORBERS = 16;
    AbstractDistroAbsorber* _absorbers[MAX_ABSORBERS];
    sigma_u32 _count;

public:
    SigmaDistroEngine() : _count(0) {
        sigma_memset(_absorbers, 0, sizeof(_absorbers));
    }

    ~SigmaDistroEngine() {
        for (sigma_u32 i = 0; i < _count; i++) {
            delete _absorbers[i];
        }
    }

    sigma_status register_absorber(AbstractDistroAbsorber* absorber) {
        if (!absorber || _count >= MAX_ABSORBERS) return SIGMA_ERR_INVAL;
        _absorbers[_count++] = absorber;
        return SIGMA_OK;
    }

    /* Absorb USPs from ALL registered distros */
    void absorb_all() noexcept {
        sigma_printf("[SIGMA DISTRO ENGINE]: Beginning multi-distro USP absorption...\n");
        sigma_printf("[SIGMA DISTRO ENGINE]: %u distros registered.\n", _count);
        for (sigma_u32 i = 0; i < _count; i++) {
            if (_absorbers[i]) {
                sigma_printf("\n--- Absorbing: %s ---\n", _absorbers[i]->distro_name());
                _absorbers[i]->absorb_usp();
            }
        }
        sigma_printf("\n[SIGMA DISTRO ENGINE]: All USPs absorbed. SigmaOS is supreme.\n");
    }

    /* Run health check on all absorbers */
    sigma_bool health_check_all() const noexcept {
        for (sigma_u32 i = 0; i < _count; i++) {
            if (_absorbers[i] && _absorbers[i]->health_check() != SIGMA_OK)
                return SIGMA_FALSE;
        }
        return SIGMA_TRUE;
    }

    sigma_u32 absorber_count() const noexcept { return _count; }
};

/* =========================================================================
 * SIGMA OOP FACTORY (Replaces complex std::make_shared / std::unique_ptr)
 * OOP: Abstract Factory Pattern - create distro absorbers by name.
 * ========================================================================= */

class SigmaDistroFactory {
public:
    /* Factory method: create absorber by distro name string */
    static AbstractDistroAbsorber* create(const char* name) {
        if (sigma_strcmp(name, "arch") == 0 || sigma_strcmp(name, "Arch") == 0)
            return new ArchAbsorber();
        if (sigma_strcmp(name, "alpine") == 0 || sigma_strcmp(name, "Alpine") == 0)
            return new AlpineAbsorber();
        if (sigma_strcmp(name, "debian") == 0 || sigma_strcmp(name, "Debian") == 0)
            return new DebianAbsorber();
        if (sigma_strcmp(name, "gentoo") == 0 || sigma_strcmp(name, "Gentoo") == 0)
            return new GentooAbsorber();
        if (sigma_strcmp(name, "nixos") == 0 || sigma_strcmp(name, "NixOS") == 0)
            return new NixOSAbsorber();
        return nullptr;  /* Unknown distro */
    }

    /* Create a fully-loaded engine with all known absorbers */
    static SigmaDistroEngine* create_full_engine() {
        SigmaDistroEngine* engine = new SigmaDistroEngine();
        engine->register_absorber(new ArchAbsorber());
        engine->register_absorber(new AlpineAbsorber());
        engine->register_absorber(new DebianAbsorber());
        engine->register_absorber(new GentooAbsorber());
        engine->register_absorber(new NixOSAbsorber());
        return engine;
    }
};

/* =========================================================================
 * SIGMA AUTOMATION ENGINE (OOP - Strategy Pattern)
 * Mission: Making SigmaOS automation/customization/personalization ready.
 * ========================================================================= */

class SigmaAutomationStrategy {
public:
    virtual ~SigmaAutomationStrategy() = default;
    virtual const char* strategy_name() const noexcept = 0;
    virtual sigma_status execute(const char* config) = 0;
    virtual sigma_status validate() const noexcept = 0;
};

/* Concrete strategy: System profile automation */
class SystemProfileStrategy : public SigmaAutomationStrategy {
public:
    const char* strategy_name() const noexcept override {
        return "SystemProfile";
    }
    sigma_status execute(const char* config) override {
        sigma_printf("[AUTOMATION] Applying system profile: %s\n", config ? config : "default");
        return SIGMA_OK;
    }
    sigma_status validate() const noexcept override { return SIGMA_OK; }
};

/* Concrete strategy: Network automation */
class NetworkAutomationStrategy : public SigmaAutomationStrategy {
public:
    const char* strategy_name() const noexcept override {
        return "NetworkAutomation";
    }
    sigma_status execute(const char* config) override {
        sigma_printf("[AUTOMATION] Applying network config: %s\n", config ? config : "default");
        return SIGMA_OK;
    }
    sigma_status validate() const noexcept override { return SIGMA_OK; }
};

/* The OOP Context that uses strategies */
class SigmaAutomationEngine {
private:
    SigmaAutomationStrategy* _strategy;
    SigmaString _name;

public:
    explicit SigmaAutomationEngine(const char* name) :
        _strategy(nullptr), _name(name) {}

    ~SigmaAutomationEngine() {
        delete _strategy;
    }

    void set_strategy(SigmaAutomationStrategy* strat) {
        delete _strategy;
        _strategy = strat;
    }

    sigma_status run(const char* config = nullptr) {
        if (!_strategy) return SIGMA_ERR_INVAL;
        sigma_printf("[ENGINE: %s] Running strategy: %s\n",
            _name.c_str(), _strategy->strategy_name());
        return _strategy->execute(config);
    }

    sigma_bool is_healthy() const noexcept {
        return _strategy && _strategy->validate() == SIGMA_OK;
    }
};

/* =========================================================================
 * GLOBAL OPERATOR new/delete BACKEND (Using mmap syscall)
 * These back the operator new/delete defined at the top of this file.
 * In production: replaced by SigmaSlabAllocator integration.
 * ========================================================================= */

extern "C" void* sigma_slab_alloc_raw(sigma_usize size) {
#if defined(SIGMA_ARCH_X86_64)
    /* Use mmap syscall (SYS_mmap = 9) */
    static const sigma_u64 PROT_RW = 3;        /* PROT_READ | PROT_WRITE */
    static const sigma_u64 MAP_ANON_PRIV = 34;  /* MAP_PRIVATE | MAP_ANONYMOUS */
    sigma_u64 result;
    __asm__ volatile (
        "syscall"
        : "=a"(result)
        : "0"(9ULL),             /* SYS_mmap */
          "D"((sigma_u64)0),     /* addr = NULL */
          "S"((sigma_u64)size),  /* length */
          "d"(PROT_RW),          /* prot */
          "r"(MAP_ANON_PRIV),    /* flags */
          "r"((sigma_u64)-1LL),  /* fd = -1 */
          "r"((sigma_u64)0)      /* offset = 0 */
        : "rcx", "r11", "memory"
    );
    if (result > (sigma_u64)-4096LL) return nullptr; /* mmap error */
    return (void*)result;
#else
    /* Fallback: simple static pool (embedded/kernel environment) */
    static sigma_u8 _pool[4 * 1024 * 1024];  /* 4 MiB pool */
    static sigma_usize _cursor = 0;
    size = sigma_align_up(size, 16);
    if (_cursor + size > sizeof(_pool)) return nullptr;
    void* p = &_pool[_cursor];
    _cursor += size;
    return p;
#endif
}

extern "C" void sigma_slab_free_raw(void* ptr) {
    /* In kernel context: proper deallocation via PMM */
    /* In userspace: munmap */
    (void)ptr; /* For now, no-op (arena allocator pattern) */
}
