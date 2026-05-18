/**
 * SigmaRuntimes.cpp — SigmaPy + SigmaR Embedded Runtimes
 * SigmaOS Zenith v15.1
 *
 * Maps to: Syllabus-Python, Syllabus-R-Programming, Syllabus-AdvPython
 * Implements: SigmaPy (CPython 3.12 embed), SigmaR (R runtime bridge)
 */
#include "SigmaRuntimes.h"

namespace Sigma::Runtimes {

// ═════════════════════════════════════════════════════════════════════════════
// SIGMA PY — Embedded Python 3.12 Runtime
// ═════════════════════════════════════════════════════════════════════════════

void SigmaPy::init() {
    // Configure Python home to SigmaOS Python directory
    Py_SetPythonHome(L"/sigma/runtime/python312");
    Py_SetPath(L"/sigma/runtime/python312/lib:/sigma/apps/python:/sigma/scripts");

    // Initialize interpreter
    Py_InitializeEx(0); // 0 = don't install signal handlers

    // Inject SigmaOS Python module
    PyObject* sigma_module = create_sigma_module();
    PyDict_SetItemString(PyImport_GetModuleDict(), "sigma", sigma_module);

    sigma_klog(sigma_printf, "[SigmaPy] CPython %s initialized\n", Py_GetVersion());
}

void SigmaPy::shutdown() {
    Py_FinalizeEx();
    sigma_klog(sigma_printf, "[SigmaPy] CPython finalized\n");
}

int SigmaPy::run_file(const char* path) {
    FILE* f = sigma_fopen_stdio(path, "r");
    if (!f) {
        sigma_klog(sigma_printf, "[SigmaPy] Cannot open: %s\n", path);
        return -1;
    }
    int rc = PyRun_SimpleFileEx(f, path, 1);
    sigma_klog(sigma_printf, "[SigmaPy] %s → rc=%d\n", path, rc);
    return rc;
}

int SigmaPy::run_string(const char* code) {
    return PyRun_SimpleString(code);
}

PyObject* SigmaPy::eval(const char* expr) {
    PyObject* main_module = PyImport_AddModule("__main__");
    PyObject* global_dict = PyModule_GetDict(main_module);
    return PyRun_String(expr, Py_eval_input, global_dict, global_dict);
}

// Create sigma OS module for Python scripts
PyObject* SigmaPy::create_sigma_module() {
    static PyMethodDef sigma_methods[] = {
        {"log",      py_sigma_log,       METH_VARARGS, "Log a message"},
        {"proc_list",py_sigma_proc_list, METH_NOARGS,  "List processes"},
        {"fs_read",  py_sigma_fs_read,   METH_VARARGS, "Read file"},
        {"fs_write", py_sigma_fs_write,  METH_VARARGS, "Write file"},
        {"db_query", py_sigma_db_query,  METH_VARARGS, "Run SQL query"},
        {nullptr, nullptr, 0, nullptr}
    };
    static PyModuleDef sigma_module = {
        PyModuleDef_HEAD_INIT, "sigma", "SigmaOS Python API",
        -1, sigma_methods
    };
    return PyModule_Create(&sigma_module);
}

// Python → SigmaOS log bridge
static PyObject* py_sigma_log(PyObject*, PyObject* args) {
    const char* msg;
    int level = sigma_printf;
    if (!PyArg_ParseTuple(args, "s|i", &msg, &level)) return nullptr;
    sigma_klog(level, "[SigmaPy] %s\n", msg);
    Py_RETURN_NONE;
}

// ─── REPL Mode ─────────────────────────────────────────────────────────────────
void SigmaPy::repl() {
    sigma_klog(sigma_printf, "[SigmaPy] Starting Python REPL\n");
    PyRun_InteractiveLoop(stdin, "<sigma-py>");
}

// ─── Data Science Packages ────────────────────────────────────────────────────
int SigmaPy::install_ds_packages() {
    // Execute: pip install numpy pandas matplotlib scikit-learn seaborn
    const char* install_cmd =
        "import subprocess; subprocess.run(['pip','install','numpy','pandas',"
        "'matplotlib','scikit-learn','seaborn','jupyter'], check=True)";
    return run_string(install_cmd);
}

bool SigmaPy::check_ds_ready() {
    const char* check = "import numpy, pandas, matplotlib, sklearn; print('OK')";
    PyObject* result = eval(check);
    bool ok = (result != nullptr && result != Py_None);
    Py_XDECREF(result);
    return ok;
}

// ═════════════════════════════════════════════════════════════════════════════
// SIGMA R — Embedded R Runtime Bridge
// ═════════════════════════════════════════════════════════════════════════════

void SigmaR::init() {
    // Initialize R via Rembedded API
    static const char* r_args[] = {"R", "--no-save", "--no-restore", "--quiet"};
    int r_argc = 4;
    Rf_initEmbeddedR(r_argc, (char**)r_args);

    // Set R library path to SigmaOS R runtime
    eval_r("'.libPaths(c(\"/sigma/runtime/R/library\", .libPaths()))'");
    sigma_klog(sigma_printf, "[SigmaR] R %s runtime initialized\n", R_version.major);
}

void SigmaR::shutdown() {
    Rf_endEmbeddedR(0);
    sigma_klog(sigma_printf, "[SigmaR] R runtime finalized\n");
}

int SigmaR::run_file(const char* path) {
    char cmd[512];
    sigma_printf(cmd, sizeof(cmd), "source('%s')", path);
    return eval_r(cmd) ? 0 : -1;
}

bool SigmaR::eval_r(const char* expr) {
    ParseStatus status;
    SEXP e, result;
    int error;
    PROTECT(e = mkString(expr));
    PROTECT(result = R_tryEval(e, R_GlobalEnv, &error));
    UNPROTECT(2);
    return !error;
}

double SigmaR::eval_numeric(const char* expr) {
    ParseStatus status;
    SEXP e, result;
    int error;
    PROTECT(e = mkString(expr));
    PROTECT(result = R_tryEval(e, R_GlobalEnv, &error));
    double val = error ? 0.0 : REAL(result)[0];
    UNPROTECT(2);
    return val;
}

// Install R packages via sigma package manager integration
int SigmaR::install_packages(const char* packages[], sigma_u32 count) {
    for (sigma_u32 i = 0; i < count; i++) {
        char cmd[256];
        sigma_printf(cmd, sizeof(cmd),
            "if (!require('%s', quietly=TRUE)) install.packages('%s', repos='https://cran.r-project.org')",
            packages[i], packages[i]);
        if (!eval_r(cmd)) {
            sigma_klog(sigma_printf, "[SigmaR] Failed to install: %s\n", packages[i]);
        }
    }
    return SIGMA_RUNTIME_OK;
}

// Load required packages for legal data science
void SigmaR::load_legal_ds_packages() {
    static const char* pkgs[] = {
        "stringr", "tm", "wordcloud", "igraph",
        "e1071", "class", "ggplot2", "jsonlite", "readxl"
    };
    install_packages(pkgs, 9);
    for (sigma_u32 i = 0; i < 9; i++) {
        char cmd[128];
        sigma_printf(cmd, sizeof(cmd), "library(%s)", pkgs[i]);
        eval_r(cmd);
    }
    sigma_klog(sigma_printf, "[SigmaR] Legal data science packages loaded\n");
}

// Python vs R comparison API (answers syllabus question)
const char* SigmaR::python_vs_r(const char* aspect) {
    struct { const char* aspect; const char* answer; } table[] = {
        {"syntax",       "Python: general-purpose, clean. R: statistical-focused, vectorized."},
        {"performance",  "Python: faster with NumPy/C. R: optimized for statistical ops."},
        {"ml",           "Python: sklearn, TensorFlow, PyTorch. R: caret, mlr."},
        {"visualization","Python: Matplotlib, Seaborn. R: ggplot2 (superior grammar of graphics)."},
        {"data",         "Python: Pandas DataFrames. R: native data frames, tibbles."},
        {"legal",        "R preferred for legal text mining (tm, stringr ecosystem)."},
        {nullptr, nullptr}
    };
    for (int i = 0; table[i].aspect; i++)
        if (sigma_strcmp(aspect, table[i].aspect) == 0) return table[i].answer;
    return "Unknown aspect";
}

} // namespace Sigma::Runtimes
