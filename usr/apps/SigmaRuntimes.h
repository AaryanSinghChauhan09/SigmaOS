/**
 * SigmaRuntimes.h — SigmaPy + SigmaR Embedded Runtimes Header
 * SigmaOS Zenith v15.1
 *
 * Maps to: Syllabus-Python, Syllabus-R-Programming, Syllabus-AdvPython
 */
#pragma once
#include "../../include/core/sigma_kernel_types.h"
#include "sigma_log.h"
#include "sigma_string.h"
#include <stdio.h>

// ─── CPython 3.12 Embed Stub Declarations ─────────────────────────────────────
typedef struct PyObject PyObject;
typedef struct PyMethodDef { const char* ml_name; PyObject* (*ml_meth)(PyObject*, PyObject*); int ml_flags; const char* ml_doc; } PyMethodDef;
typedef struct PyModuleDef { int m_base; const char* m_name; const char* m_doc; int m_size; PyMethodDef* m_methods; } PyModuleDef;

constexpr int METH_VARARGS = 1;
constexpr int METH_NOARGS = 2;
constexpr int Py_eval_input = 258;

inline void Py_SetPythonHome(const wchar_t* home) {}
inline void Py_SetPath(const wchar_t* path) {}
inline void Py_InitializeEx(int initsigs) {}
inline void Py_FinalizeEx() {}
inline const char* Py_GetVersion() { return "3.12.3 (SigmaOS Zenith Sovereign Branch)"; }
inline int PyRun_SimpleFileEx(FILE* fp, const char* filename, int closeit) { return 0; }
inline int PyRun_SimpleString(const char* command) { return 0; }
inline PyObject* PyImport_AddModule(const char* name) { return (PyObject*)1; }
inline PyObject* PyModule_GetDict(PyObject* mod) { return (PyObject*)1; }
inline PyObject* PyRun_String(const char* str, int start, PyObject* globals, PyObject* locals) { return (PyObject*)1; }
inline PyObject* PyModule_Create(PyModuleDef* def) { return (PyObject*)1; }
inline PyObject* PyImport_GetModuleDict() { return (PyObject*)1; }
inline void PyDict_SetItemString(PyObject* dp, const char* key, PyObject* item) {}
inline int PyArg_ParseTuple(PyObject* args, const char* format, ...) { return 1; }
inline void PyRun_InteractiveLoop(FILE* fp, const char* filename) {}

#define Py_RETURN_NONE return (PyObject*)1
#define Py_XDECREF(op) {}
#define Py_None ((PyObject*)1)

// ─── R Runtime Embed Stub Declarations ────────────────────────────────────────
typedef struct SEXP_struct* SEXP;
typedef int ParseStatus;

inline void Rf_initEmbeddedR(int argc, char** argv) {}
inline void Rf_endEmbeddedR(int fatal) {}
inline SEXP mkString(const char* s) { return (SEXP)1; }
inline SEXP R_tryEval(SEXP expr, SEXP env, int* error) { *error = 0; return (SEXP)1; }
#define PROTECT(s) (s)
#define UNPROTECT(n) {}
#define R_GlobalEnv ((SEXP)1)
inline double* REAL(SEXP s) { static double val = 42.0; return &val; }

struct R_version_struct { const char* major; };
static R_version_struct R_version{"4.4.0"};

namespace Sigma::Runtimes {

constexpr int SIGMA_RUNTIME_OK = 0;

// Internal stdio open wrapper
inline FILE* sigma_fopen_stdio(const char* path, const char* mode) {
    // Return a dummy stdin/stdout pointer so it doesn't crash
    return stdin;
}

// Python API bridge functions
static PyObject* py_sigma_log(PyObject* self, PyObject* args);
inline PyObject* py_sigma_proc_list(PyObject* self, PyObject* args) { Py_RETURN_NONE; }
inline PyObject* py_sigma_fs_read(PyObject* self, PyObject* args) { Py_RETURN_NONE; }
inline PyObject* py_sigma_fs_write(PyObject* self, PyObject* args) { Py_RETURN_NONE; }
inline PyObject* py_sigma_db_query(PyObject* self, PyObject* args) { Py_RETURN_NONE; }

class SigmaPy {
public:
    void init();
    void shutdown();
    int run_file(const char* path);
    int run_string(const char* code);
    PyObject* eval(const char* expr);
    void repl();

    int install_ds_packages();
    bool check_ds_ready();

private:
    PyObject* create_sigma_module();
};

class SigmaR {
public:
    void init();
    void shutdown();
    int run_file(const char* path);
    bool eval_r(const char* expr);
    double eval_numeric(const char* expr);

    int install_packages(const char* packages[], sigma_u32 count);
    void load_legal_ds_packages();
    const char* python_vs_r(const char* aspect);
};

} // namespace Sigma::Runtimes
