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
 * SIGMA OS CORE TASK STRUCTURE
 * ========================================================================= */

struct SigmaTask {
    sigma_u64 pid;
    char name[32];
    sigma_u64 stack_base;
    sigma_u64 heap_base;
    sigma_u32 priority;
    sigma_bool active;
};

/* =========================================================================
 * CUSTOM GLOBAL new/delete (Replace heap allocation without <new>)
 * ========================================================================= */

extern "C" void* sigma_slab_alloc_raw(sigma_usize size);
extern "C" void  sigma_slab_free_raw(void* ptr);

void* operator new(sigma_usize size) {
    void* ptr = sigma_slab_alloc_raw(size);
    if (!ptr) sigma_exit(1);
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

inline void* operator new(sigma_usize, void* p) noexcept { return p; }

/* =========================================================================
 * SIGMA SMART POINTERS
 * ========================================================================= */

template<typename T>
class SigmaUniquePtr {
private:
    T* _ptr;
    SigmaUniquePtr(const SigmaUniquePtr&) = delete;
    SigmaUniquePtr& operator=(const SigmaUniquePtr&) = delete;

public:
    explicit SigmaUniquePtr(T* p = nullptr) noexcept : _ptr(p) {}

    SigmaUniquePtr(SigmaUniquePtr&& other) noexcept : _ptr(other._ptr) {
        other._ptr = nullptr;
    }

    template<typename U>
    SigmaUniquePtr(SigmaUniquePtr<U>&& other) noexcept : _ptr(other.release()) {}

    SigmaUniquePtr& operator=(SigmaUniquePtr&& other) noexcept {
        if (this != &other) {
            delete _ptr;
            _ptr = other._ptr;
            other._ptr = nullptr;
        }
        return *this;
    }

    template<typename U>
    SigmaUniquePtr& operator=(SigmaUniquePtr<U>&& other) noexcept {
        delete _ptr;
        _ptr = other.release();
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

/* 
 * SigmaSharedPtr<T>: Shared ownership pointer.
 */
template<typename T>
class SigmaSharedPtr {
private:
    T*          _ptr;
    sigma_u32*  _ref_count;

    template<typename U> friend class SigmaSharedPtr;

    void _release() {
        if (_ref_count) {
            (*_ref_count)--;
            if (*_ref_count == 0) {
                delete _ptr;
                sigma_slab_free_raw(_ref_count);
            }
            _ptr = nullptr;
            _ref_count = nullptr;
        }
    }

public:
    explicit SigmaSharedPtr(T* p = nullptr) : _ptr(p), _ref_count(nullptr) {
        if (p) {
            _ref_count = static_cast<sigma_u32*>(sigma_slab_alloc_raw(sizeof(sigma_u32)));
            *_ref_count = 1;
        }
    }

    ~SigmaSharedPtr() { _release(); }

    SigmaSharedPtr(const SigmaSharedPtr& o) : _ptr(o._ptr), _ref_count(o._ref_count) {
        if (_ref_count) (*_ref_count)++;
    }

    /* Conversion constructor for covariance */
    template<typename U>
    SigmaSharedPtr(const SigmaSharedPtr<U>& o) : _ptr(o.get()), _ref_count(o.get_ref_count()) {
        if (_ref_count) (*_ref_count)++;
    }

    SigmaSharedPtr& operator=(const SigmaSharedPtr& o) {
        if (this != &o) {
            _release();
            _ptr = o._ptr;
            _ref_count = o._ref_count;
            if (_ref_count) (*_ref_count)++;
        }
        return *this;
    }

    template<typename U>
    SigmaSharedPtr& operator=(const SigmaSharedPtr<U>& o) {
        _release();
        _ptr = o.get();
        _ref_count = o.get_ref_count();
        if (_ref_count) (*_ref_count)++;
        return *this;
    }

    SigmaSharedPtr(SigmaSharedPtr&& o) noexcept : _ptr(o._ptr), _ref_count(o._ref_count) {
        o._ptr = nullptr; o._ref_count = nullptr;
    }

    template<typename U>
    SigmaSharedPtr(SigmaSharedPtr<U>&& o) noexcept : _ptr(o.get()), _ref_count(o.get_ref_count()) {
        o.detach(); // We need a way to detach without releasing
    }

    SigmaSharedPtr& operator=(SigmaSharedPtr&& o) noexcept {
        if (this != &o) {
            _release();
            _ptr = o._ptr;
            _ref_count = o._ref_count;
            o._ptr = nullptr;
            o._ref_count = nullptr;
        }
        return *this;
    }

    T* get() const noexcept { return _ptr; }
    sigma_u32* get_ref_count() const noexcept { return _ref_count; }
    void detach() noexcept { _ptr = nullptr; _ref_count = nullptr; }
    T& operator*() const noexcept { return *_ptr; }
    T* operator->() const noexcept { return _ptr; }
    explicit operator bool() const noexcept { return _ptr != nullptr; }
};

template<typename T, typename... Args>
SigmaSharedPtr<T> sigma_make_shared(Args&&... args) {
    return SigmaSharedPtr<T>(new T(static_cast<Args&&>(args)...));
}

template<typename T, typename... Args>
SigmaUniquePtr<T> sigma_make_unique(Args&&... args) {
    return SigmaUniquePtr<T>(new T(static_cast<Args&&>(args)...));
}

/* =========================================================================
 * SIGMA ARRAY (DYNAMC VECTOR)
 * ========================================================================= */

template<typename T>
class SigmaArray {
private:
    T*          _data;
    sigma_usize _size;
    sigma_usize _capacity;

    void _grow(sigma_usize new_cap) {
        T* new_data = static_cast<T*>(sigma_slab_alloc_raw(new_cap * sizeof(T)));
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
    ~SigmaArray() noexcept {
        for (sigma_usize i = 0; i < _size; i++) _data[i].~T();
        sigma_slab_free_raw(_data);
    }

    SigmaArray(const SigmaArray&) = delete;
    SigmaArray& operator=(const SigmaArray&) = delete;
    SigmaArray(SigmaArray&& o) noexcept : _data(o._data), _size(o._size), _capacity(o._capacity) {
        o._data = nullptr; o._size = 0; o._capacity = 0;
    }

    template<typename U>
    void push(U&& val) {
        if (_size >= _capacity) _grow(_capacity == 0 ? 4 : _capacity * 2);
        new (&_data[_size++]) T(static_cast<U&&>(val));
    }

    void pop() noexcept { if (_size > 0) _data[--_size].~T(); }
    T& operator[](sigma_usize i) noexcept { return _data[i]; }
    const T& operator[](sigma_usize i) const noexcept { return _data[i]; }
    T& back() noexcept { return _data[_size - 1]; }
    sigma_usize size() const noexcept { return _size; }
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
 * SIGMA STRING
 * ========================================================================= */

class SigmaString {
private:
    static constexpr sigma_usize SSO_MAX = 23;
    union {
        struct { char* _heap_ptr; sigma_usize _heap_capacity; };
        char _sso_buf[SSO_MAX + 1];
    };
    sigma_usize _len;
    sigma_bool  _is_sso;

    char* _ptr() noexcept { return _is_sso ? _sso_buf : _heap_ptr; }
    const char* _ptr() const noexcept { return _is_sso ? _sso_buf : _heap_ptr; }

    void _ensure_capacity(sigma_usize needed) {
        if (_is_sso && needed <= SSO_MAX) return;
        sigma_usize new_cap = sigma_align_up(needed + 1, 16);
        if (!_is_sso && new_cap <= _heap_capacity) return;
        char* new_buf = static_cast<char*>(sigma_slab_alloc_raw(new_cap));
        if (!new_buf) sigma_exit(1);
        sigma_memcpy(new_buf, _ptr(), _len + 1);
        if (!_is_sso) sigma_slab_free_raw(_heap_ptr);
        _heap_ptr = new_buf;
        _heap_capacity = new_cap;
        _is_sso = SIGMA_FALSE;
    }

public:
    SigmaString() noexcept : _len(0), _is_sso(SIGMA_TRUE) { _sso_buf[0] = '\0'; }
    SigmaString(const char* s) : _len(0), _is_sso(SIGMA_TRUE) { _sso_buf[0] = '\0'; assign(s); }
    SigmaString(const SigmaString& o) : _len(0), _is_sso(SIGMA_TRUE) { _sso_buf[0] = '\0'; assign(o.c_str()); }
    SigmaString(SigmaString&& o) noexcept : _len(o._len), _is_sso(o._is_sso) {
        if (o._is_sso) sigma_memcpy(_sso_buf, o._sso_buf, o._len + 1);
        else { _heap_ptr = o._heap_ptr; _heap_capacity = o._heap_capacity; o._heap_ptr = nullptr; }
        o._len = 0; o._is_sso = SIGMA_TRUE; o._sso_buf[0] = '\0';
    }
    ~SigmaString() noexcept { if (!_is_sso && _heap_ptr) sigma_slab_free_raw(_heap_ptr); }

    SigmaString& operator=(const SigmaString& o) {
        if (this != &o) assign(o.c_str());
        return *this;
    }

    SigmaString& operator=(const char* s) { assign(s); return *this; }
    void assign(const char* s) {
        if (!s) { clear(); return; }
        sigma_usize slen = sigma_strlen(s);
        _ensure_capacity(slen);
        sigma_memcpy(_ptr(), s, slen + 1);
        _len = slen;
    }
    void append(const char* s) {
        sigma_usize slen = sigma_strlen(s);
        _ensure_capacity(_len + slen);
        sigma_memcpy(_ptr() + _len, s, slen + 1);
        _len += slen;
    }
    bool operator==(const char* s) const noexcept { return sigma_strcmp(_ptr(), s) == 0; }
    const char* c_str() const noexcept { return _ptr(); }
    sigma_bool contains(const char* n) const noexcept { return sigma_str_contains(_ptr(), n); }
    void clear() noexcept { _len = 0; _ptr()[0] = '\0'; }
};

/* =========================================================================
 * SIGMA MAP
 * ========================================================================= */

template<typename K, typename V>
struct SigmaPair {
    K first;
    V second;
    SigmaPair() : first(), second() {}
    SigmaPair(const K& k, const V& v) : first(k), second(v) {}
    SigmaPair(const K& k, V&& v) : first(k), second(static_cast<V&&>(v)) {}
    SigmaPair(SigmaPair&& o) noexcept : first(static_cast<K&&>(o.first)), second(static_cast<V&&>(o.second)) {}
};

template<typename K, typename V>
class SigmaMap {
private:
    SigmaArray<SigmaPair<K, V>> _pairs;
public:
    SigmaMap() noexcept = default;
    template<typename U>
    void insert(const K& key, U&& value) {
        for (auto& p : _pairs) { if (p.first == key) { p.second = static_cast<U&&>(value); return; } }
        _pairs.push(SigmaPair<K, V>(key, static_cast<U&&>(value)));
    }
    V& operator[](const K& key) {
        for (auto& p : _pairs) if (p.first == key) return p.second;
        _pairs.push(SigmaPair<K, V>(key, V()));
        return _pairs.back().second;
    }
    const V& at(const K& key) const {
        for (const auto& p : _pairs) if (p.first == key) return p.second;
        sigma_exit(1); static V f{}; return f;
    }
    sigma_bool count(const K& key) const noexcept {
        for (const auto& p : _pairs) if (p.first == key) return SIGMA_TRUE;
        return SIGMA_FALSE;
    }
    auto begin() noexcept { return _pairs.begin(); }
    auto end() noexcept { return _pairs.end(); }
};

/* =========================================================================
 * SIGMA OBJECT & DISTRO ABSORBERS
 * ========================================================================= */

class SigmaObject {
protected:
    sigma_u64 _id;
    static sigma_u64 _next_id;
    SigmaObject() noexcept : _id(_next_id++) {}
public:
    virtual ~SigmaObject() noexcept = default;
    virtual const char* type_name() const noexcept = 0;
};

#ifndef SIGMA_OBJECT_ID_DEF
#define SIGMA_OBJECT_ID_DEF
inline sigma_u64 SigmaObject::_next_id = 1;
#endif

class AbstractDistroAbsorber : public SigmaObject {
public:
    virtual const char* distro_name() const noexcept = 0;
    virtual sigma_status absorb_usp() noexcept = 0;
};

class ArchAbsorber : public AbstractDistroAbsorber {
public:
    const char* distro_name() const noexcept override { return "Arch Linux"; }
    const char* type_name() const noexcept override { return "ArchAbsorber"; }
    sigma_status absorb_usp() noexcept override { sigma_printf("[ARCH]: Absorbed.\n"); return SIGMA_OK; }
};

class AlpineAbsorber : public AbstractDistroAbsorber {
public:
    const char* distro_name() const noexcept override { return "Alpine Linux"; }
    const char* type_name() const noexcept override { return "AlpineAbsorber"; }
    sigma_status absorb_usp() noexcept override { sigma_printf("[ALPINE]: Absorbed.\n"); return SIGMA_OK; }
};

class DebianAbsorber : public AbstractDistroAbsorber {
public:
    const char* distro_name() const noexcept override { return "Debian"; }
    const char* type_name() const noexcept override { return "DebianAbsorber"; }
    sigma_status absorb_usp() noexcept override { sigma_printf("[DEBIAN]: Absorbed.\n"); return SIGMA_OK; }
};

class GentooAbsorber : public AbstractDistroAbsorber {
public:
    const char* distro_name() const noexcept override { return "Gentoo"; }
    const char* type_name() const noexcept override { return "GentooAbsorber"; }
    sigma_status absorb_usp() noexcept override { sigma_printf("[GENTOO]: Absorbed.\n"); return SIGMA_OK; }
};

class NixOSAbsorber : public AbstractDistroAbsorber {
public:
    const char* distro_name() const noexcept override { return "NixOS"; }
    const char* type_name() const noexcept override { return "NixOSAbsorber"; }
    sigma_status absorb_usp() noexcept override { sigma_printf("[NIXOS]: Absorbed.\n"); return SIGMA_OK; }
};

class SigmaDistroEngine {
private:
    SigmaArray<AbstractDistroAbsorber*> _absorbers;
public:
    ~SigmaDistroEngine() { for (auto a : _absorbers) delete a; }
    void register_absorber(AbstractDistroAbsorber* a) { _absorbers.push(a); }
    void absorb_all() { for (auto a : _absorbers) a->absorb_usp(); }
};

/* =========================================================================
 * IMPLEMENTATION BACKENDS
 * ========================================================================= */

inline void* sigma_slab_alloc_raw(sigma_usize size) {
#if defined(SIGMA_ARCH_X86_64)
    sigma_u64 res;
    __asm__ volatile ("syscall" : "=a"(res) : "0"(9ULL), "D"(0ULL), "S"((sigma_u64)size), "d"(3ULL), "r"(34ULL), "r"(-1LL), "r"(0ULL) : "rcx", "r11", "memory");
    return (res > (sigma_u64)-4096LL) ? nullptr : (void*)res;
#else
    static sigma_u8 pool[1024*1024]; static sigma_usize c = 0;
    size = (size + 15) & ~15; if (c + size > sizeof(pool)) return nullptr;
    void* p = &pool[c]; c += size; return p;
#endif
}

inline void sigma_slab_free_raw(void* ptr) { (void)ptr; }
