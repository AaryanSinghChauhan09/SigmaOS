/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN OOP FRAMEWORK (v20.0 - C11 ZENITH)
 * =========================================================================
 * Mission: Provide pure-C object-oriented primitives for Sovereign Shards.
 * Principle: Zero-latency. Zero-vtable overhead. Absolute Sovereignty.
 * =========================================================================
 */

#ifndef SIGMA_OOP_H
#define SIGMA_OOP_H

#include "suites/S01_Genesis/shards/sigma_types.h"

/* 
 * CLASS_DECLARE: Define a struct and its associated typedef.
 * Usage: CLASS_DECLARE(MyClass) { ... fields ... };
 */
#define CLASS_DECLARE(name) typedef struct name name##_t; struct name

/*
 * VIRTUAL: Define a function pointer field (simulated virtual method).
 * Usage: VIRTUAL(void, my_method, MyClass_t* self, int arg);
 */
#define VIRTUAL(ret, name, ...) ret (*name)(__VA_ARGS__)

/* 
 * SigmaObject: The root base object for all sovereign shards.
 */
CLASS_DECLARE(SigmaObject) {
    const char* class_name;
    sigma_u32   object_id;
    VIRTUAL(void, destroy, struct SigmaObject* self);
};

/*
 * sigma_object_init: Basic constructor for the base object shard.
 */
static inline void sigma_object_init(SigmaObject_t* obj, const char* name, sigma_u32 id) {
    if (!obj) return;
    obj->class_name = name;
    obj->object_id = id;
    obj->destroy = (void*)0; // Default: No-op destructor
}

#endif /* SIGMA_OOP_H */
