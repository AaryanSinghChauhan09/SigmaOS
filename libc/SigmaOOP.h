/*
 * =========================================================================
 * Σ SIGMAOS ZENITH SUPREME: OOP ABSTRACTION LAYER (C11)
 * =========================================================================
 * Mission: Zero-Dependency Object-Oriented paradigm using raw C structs.
 * Design: Native pointers, encapsulations, and polymorphic interface bounds.
 * =========================================================================
 */

#ifndef SIGMA_OOP_H
#define SIGMA_OOP_H

#include "SovereignLibC.h"

#ifndef _SIGMA_UINT32_T_DEFINED
#define _SIGMA_UINT32_T_DEFINED
typedef unsigned int uint32_t;
#endif

// -------------------------------------------------------------------------
// SigmaOOP Base Interface
// -------------------------------------------------------------------------

/*
 * CLASS_DECLARE: Helper macro to define a standard OOP struct.
 * Usage:
 *    CLASS_DECLARE(MyClass) {
 *        int data;
 *        void (*print)(struct MyClass* self);
 *    };
 */
#define CLASS_DECLARE(name) typedef struct name name##_t; struct name

/*
 * VIRTUAL: Macro indicating a function pointer intended to be polymorphic.
 */
#define VIRTUAL(ret, name, ...) ret (*name)(__VA_ARGS__)

// -------------------------------------------------------------------------
// Standard Object Core
// -------------------------------------------------------------------------

CLASS_DECLARE(SigmaObject) {
    const char* class_name;
    sigma_u32 object_id;
    VIRTUAL(void, destroy, struct SigmaObject* self);
};

// -------------------------------------------------------------------------
// Global OOP Factory Utilities
// -------------------------------------------------------------------------

static inline void sigma_object_init(SigmaObject_t* obj, const char* name, sigma_u32 id) {
    if (obj) {
        obj->class_name = name;
        obj->object_id = id;
        obj->destroy = 0; // To be overridden
    }
}

#endif // SIGMA_OOP_H
