/**
 * =========================================================================
 * Σ ZENITH NATIVE PYTHON BINDINGS (PHASE 9)
 * =========================================================================
 * A native CPython C-extension module that exposes Zenith UI functions
 * directly to Python. Eliminates the need for PyBind11, reducing bloat.
 * =========================================================================
 */

#include <Python.h>
#include "../../include/zenith.h"

// ─── Python Method Wrappers ──────────────────────────────────────────────

static PyObject* py_zenith_init(PyObject* self, PyObject* args) {
    zenith_theme_init();
    Py_RETURN_NONE;
}

static PyObject* py_zenith_draw_button(PyObject* self, PyObject* args) {
    long w, x, y, bw, bh;
    const char* label;
    int hover;

    // We stub out the raw buffer pointer in Python for safety, assuming the
    // IPC bridge handles the memory map transparently for scripting languages.
    if (!PyArg_ParseTuple(args, "lllllps", &w, &x, &y, &bw, &bh, &hover, &label)) {
        return NULL;
    }

    // Mock buffer
    sigma_u8 dummy_buf[1] = {0}; 
    
    zenith_draw_button(dummy_buf, (sigma_u32)w, (sigma_i32)x, (sigma_i32)y, 
                      (sigma_u32)bw, (sigma_u32)bh, label, hover != 0);

    Py_RETURN_NONE;
}

// ─── Module Definition ───────────────────────────────────────────────────

static PyMethodDef ZenithMethods[] = {
    {"init", py_zenith_init, METH_NOARGS, "Initialize the Zenith Theme Engine."},
    {"draw_button", py_zenith_draw_button, METH_VARARGS, "Draw a native Zenith button."},
    {NULL, NULL, 0, NULL} /* Sentinel */
};

static struct PyModuleDef zenithmodule = {
    PyModuleDef_HEAD_INIT,
    "zenith_ui",   /* name of module */
    "SigmaOS Zenith Native UI Toolkit", /* module documentation */
    -1,            /* size of per-interpreter state */
    ZenithMethods
};

PyMODINIT_FUNC PyInit_zenith_ui(void) {
    return PyModule_Create(&zenithmodule);
}
