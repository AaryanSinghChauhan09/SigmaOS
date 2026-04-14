#ifndef SIGMA_COMPAT_LAYER_H
#define SIGMA_COMPAT_LAYER_H

// SigmaOS Compatibility Layers & Native Porting
// For GCC, LLVM, and customized libraries

void compat_load_llvm_toolchain();
void compat_load_gcc_toolchain();
void compat_provide_posix_translation();

#endif // SIGMA_COMPAT_LAYER_H
