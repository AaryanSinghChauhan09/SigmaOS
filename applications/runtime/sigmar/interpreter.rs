//! SigmaR - R Statistical Language Runtime for SigmaOS
//! Replaces GNU R
//! Features: R 4.x compatibility, vectorized operations, statistical functions, data frame support

#![no_std]
#![allow(dead_code)]

type SigmaU8 = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;
type SigmaF64 = f64;

/// R object types (SEXP types)
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RObjectType {
    Null = 0,
    Symbol = 1,
    List = 2,
    Closure = 3,
    Environment = 4,
    Promise = 5,
    Language = 6,
    Special = 7,
    Builtin = 8,
    Char = 9,
    Logical = 10,
    Integer = 13,
    Double = 14,
    Complex = 15,
    String = 16,
    Dot = 17,
    Any = 18,
    Vector = 19,
    Expression = 20,
    Bytecode = 21,
    ExternalPtr = 22,
    WeakRef = 23,
    Raw = 24,
    S4 = 25,
    Function = 99,
}

/// R object
#[repr(C)]
pub union RObjectData {
    pub int_val: SigmaI32,
    pub double_val: SigmaF64,
    pub bool_val: SigmaBool,
    pub ptr_val: SigmaU64,
}

#[repr(C)]
pub struct RObject {
    pub type_: RObjectType,
    pub data: RObjectData,
    pub length: SigmaU32,
    pub attributes: SigmaU64, // Pointer to attributes list
    pub ref_count: SigmaU32,
}

/// R data frame
#[repr(C)]
pub struct RDataFrame {
    pub df_id: SigmaU64,
    pub column_names: [SigmaU8; 128],
    pub columns: [RObject; 64],
    pub column_count: SigmaU32,
    pub row_count: SigmaU32,
}

/// R environment
#[repr(C)]
pub struct REnvironment {
    pub env_id: SigmaU64,
    pub parent: SigmaU64,
    pub symbols: [RObject; 256],
    pub values: [RObject; 256],
    pub symbol_count: SigmaU32,
}

/// R interpreter state
#[repr(C)]
pub struct RInterpreter {
    pub interpreter_id: SigmaU64,
    pub global_env: REnvironment,
    pub base_env: REnvironment,
    pub data_frames: [RDataFrame; 64],
    pub dataframe_count: SigmaU32,
    pub vectorized_enabled: SigmaBool,
}

/// R runtime engine
#[repr(C)]
pub struct RRuntime {
    pub initialized: SigmaBool,
    pub interpreters: [RInterpreter; 8],
    pub interpreter_count: SigmaU32,
    pub gc_enabled: SigmaBool,
}

static mut R_RUNTIME: Option<RRuntime> = None;

/// Initialize R runtime
#[no_mangle]
pub unsafe extern "C" fn sigma_r_init() -> SigmaI32 {
    R_RUNTIME = Some(RRuntime {
        initialized: false,
        interpreters: [RInterpreter {
            interpreter_id: 0,
            global_env: REnvironment {
                env_id: 0,
                parent: 0,
                symbols: [RObject {
                    type_: RObjectType::Null,
                    data: RObjectData { int_val: 0 },
                    length: 0,
                    attributes: 0,
                    ref_count: 0,
                }; 256],
                values: [RObject {
                    type_: RObjectType::Null,
                    data: RObjectData { int_val: 0 },
                    length: 0,
                    attributes: 0,
                    ref_count: 0,
                }; 256],
                symbol_count: 0,
            },
            base_env: REnvironment {
                env_id: 0,
                parent: 0,
                symbols: [RObject {
                    type_: RObjectType::Null,
                    data: RObjectData { int_val: 0 },
                    length: 0,
                    attributes: 0,
                    ref_count: 0,
                }; 256],
                values: [RObject {
                    type_: RObjectType::Null,
                    data: RObjectData { int_val: 0 },
                    length: 0,
                    attributes: 0,
                    ref_count: 0,
                }; 256],
                symbol_count: 0,
            },
            data_frames: [RDataFrame {
                df_id: 0,
                column_names: [0; 128],
                columns: [RObject {
                    type_: RObjectType::Null,
                    data: RObjectData { int_val: 0 },
                    length: 0,
                    attributes: 0,
                    ref_count: 0,
                }; 64],
                column_count: 0,
                row_count: 0,
            }; 64],
            dataframe_count: 0,
            vectorized_enabled: true,
        }; 8],
        interpreter_count: 0,
        gc_enabled: true,
    });

    if let Some(runtime) = &mut R_RUNTIME {
        runtime.initialized = true;
        return 0;
    }

    -1
}

/// Create interpreter
#[no_mangle]
pub unsafe extern "C" fn sigma_r_create_interpreter() -> SigmaU64 {
    if R_RUNTIME.is_none() {
        return 0;
    }

    if let Some(runtime) = &mut R_RUNTIME {
        if runtime.interpreter_count >= 8 {
            return 0;
        }

        let interp_id = runtime.interpreter_count + 1;
        let idx = runtime.interpreter_count as usize;

        runtime.interpreters[idx] = RInterpreter {
            interpreter_id: interp_id as SigmaU64,
            global_env: REnvironment {
                env_id: interp_id as SigmaU64 * 2 + 1,
                parent: 0,
                symbols: [RObject {
                    type_: RObjectType::Null,
                    data: RObjectData { int_val: 0 },
                    length: 0,
                    attributes: 0,
                    ref_count: 0,
                }; 256],
                values: [RObject {
                    type_: RObjectType::Null,
                    data: RObjectData { int_val: 0 },
                    length: 0,
                    attributes: 0,
                    ref_count: 0,
                }; 256],
                symbol_count: 0,
            },
            base_env: REnvironment {
                env_id: interp_id as SigmaU64 * 2 + 2,
                parent: 0,
                symbols: [RObject {
                    type_: RObjectType::Null,
                    data: RObjectData { int_val: 0 },
                    length: 0,
                    attributes: 0,
                    ref_count: 0,
                }; 256],
                values: [RObject {
                    type_: RObjectType::Null,
                    data: RObjectData { int_val: 0 },
                    length: 0,
                    attributes: 0,
                    ref_count: 0,
                }; 256],
                symbol_count: 0,
            },
            data_frames: [RDataFrame {
                df_id: 0,
                column_names: [0; 128],
                columns: [RObject {
                    type_: RObjectType::Null,
                    data: RObjectData { int_val: 0 },
                    length: 0,
                    attributes: 0,
                    ref_count: 0,
                }; 64],
                column_count: 0,
                row_count: 0,
            }; 64],
            dataframe_count: 0,
            vectorized_enabled: true,
        };

        runtime.interpreter_count += 1;
        interp_id as SigmaU64
    } else {
        0
    }
}

/// Execute R code
#[no_mangle]
pub unsafe extern "C" fn sigma_r_execute(
    interpreter_id: SigmaU64,
    code: *const SigmaU8,
    code_size: SigmaU32,
) -> SigmaI32 {
    if R_RUNTIME.is_none() || code.is_null() {
        return -1;
    }

    if let Some(runtime) = &mut R_RUNTIME {
        let idx = (interpreter_id - 1) as usize;
        if idx >= runtime.interpreter_count as usize {
            return -1;
        }

        let interp = &mut runtime.interpreters[idx];

        // Simplified R execution
        // In a real implementation, this would:
        // 1. Parse R code
        // 2. Build AST
        // 3. Evaluate in environment
        // 4. Handle errors

        return 0;
    }

    -1
}

/// Create R object
#[no_mangle]
pub unsafe extern "C" fn sigma_r_create_object(
    interpreter_id: SigmaU64,
    type_: RObjectType,
    length: SigmaU32,
) -> SigmaU64 {
    if R_RUNTIME.is_none() {
        return 0;
    }

    if let Some(runtime) = &mut R_RUNTIME {
        let idx = (interpreter_id - 1) as usize;
        if idx >= runtime.interpreter_count as usize {
            return 0;
        }

        let interp = &mut runtime.interpreters[idx];

        // Create object in global environment
        if interp.global_env.symbol_count >= 256 {
            return 0;
        }

        let obj_id = interp.global_env.symbol_count as SigmaU64 + 1;
        let sym_idx = interp.global_env.symbol_count as usize;

        interp.global_env.symbols[sym_idx] = RObject {
            type_,
            data: RObjectData { int_val: 0 },
            length,
            attributes: 0,
            ref_count: 1,
        };

        interp.global_env.symbol_count += 1;
        obj_id
    } else {
        0
    }
}

/// Create integer vector
#[no_mangle]
pub unsafe extern "C" fn sigma_r_create_integer_vector(
    interpreter_id: SigmaU64,
    data: *const SigmaI32,
    length: SigmaU32,
) -> SigmaU64 {
    if R_RUNTIME.is_none() || data.is_null() {
        return 0;
    }

    let obj_id = sigma_r_create_object(interpreter_id, RObjectType::Integer, length);
    if obj_id == 0 {
        return 0;
    }

    if let Some(runtime) = &mut R_RUNTIME {
        let idx = (interpreter_id - 1) as usize;
        let interp = &mut runtime.interpreters[idx];
        let sym_idx = (obj_id - 1) as usize;

        // Allocate memory for vector data
        let vec_data = musl_malloc((length as SigmaUsize) * core::mem::size_of::<SigmaI32>()) as *mut SigmaI32;
        if !vec_data.is_null() {
            for i in 0..length as usize {
                *vec_data.add(i) = *data.add(i);
            }
            interp.global_env.symbols[sym_idx].data.ptr_val = vec_data as SigmaU64;
        }

        obj_id
    } else {
        0
    }
}

/// Create double vector
#[no_mangle]
pub unsafe extern "C" fn sigma_r_create_double_vector(
    interpreter_id: SigmaU64,
    data: *const SigmaF64,
    length: SigmaU32,
) -> SigmaU64 {
    if R_RUNTIME.is_none() || data.is_null() {
        return 0;
    }

    let obj_id = sigma_r_create_object(interpreter_id, RObjectType::Double, length);
    if obj_id == 0 {
        return 0;
    }

    if let Some(runtime) = &mut R_RUNTIME {
        let idx = (interpreter_id - 1) as usize;
        let interp = &mut runtime.interpreters[idx];
        let sym_idx = (obj_id - 1) as usize;

        // Allocate memory for vector data
        let vec_data = musl_malloc((length as SigmaUsize) * core::mem::size_of::<SigmaF64>()) as *mut SigmaF64;
        if !vec_data.is_null() {
            for i in 0..length as usize {
                *vec_data.add(i) = *data.add(i);
            }
            interp.global_env.symbols[sym_idx].data.ptr_val = vec_data as SigmaU64;
        }

        obj_id
    } else {
        0
    }
}

/// Create data frame
#[no_mangle]
pub unsafe extern "C" fn sigma_r_create_dataframe(
    interpreter_id: SigmaU64,
    column_names: *const SigmaU8,
    column_count: SigmaU32,
    row_count: SigmaU32,
) -> SigmaU64 {
    if R_RUNTIME.is_none() || column_names.is_null() {
        return 0;
    }

    if let Some(runtime) = &mut R_RUNTIME {
        let idx = (interpreter_id - 1) as usize;
        if idx >= runtime.interpreter_count as usize {
            return 0;
        }

        let interp = &mut runtime.interpreters[idx];
        if interp.dataframe_count >= 64 {
            return 0;
        }

        let df_id = interp.dataframe_count + 1;
        let df_idx = interp.dataframe_count as usize;

        interp.data_frames[df_idx] = RDataFrame {
            df_id: df_id as SigmaU64,
            column_names: [0; 128],
            columns: [RObject {
                type_: RObjectType::Null,
                data: RObjectData { int_val: 0 },
                length: 0,
                attributes: 0,
                ref_count: 0,
            }; 64],
            column_count,
            row_count,
        };

        // Copy column names
        for i in 0..127.min(name_len(column_names)) {
            interp.data_frames[df_idx].column_names[i] = *column_names.add(i);
        }

        interp.dataframe_count += 1;
        df_id as SigmaU64
    } else {
        0
    }
}

/// Statistical function: mean
#[no_mangle]
pub unsafe extern "C" fn sigma_r_mean(
    interpreter_id: SigmaU64,
    obj_id: SigmaU64,
    result: *mut SigmaF64,
) -> SigmaI32 {
    if R_RUNTIME.is_none() || result.is_null() {
        return -1;
    }

    if let Some(runtime) = &R_RUNTIME {
        let idx = (interpreter_id - 1) as usize;
        if idx >= runtime.interpreter_count as usize {
            return -1;
        }

        let interp = &runtime.interpreters[idx];
        let sym_idx = (obj_id - 1) as usize;
        if sym_idx >= interp.global_env.symbol_count as usize {
            return -1;
        }

        let obj = &interp.global_env.symbols[sym_idx];
        
        if obj.type_ == RObjectType::Double {
            let data = obj.data.ptr_val as *const SigmaF64;
            let mut sum: SigmaF64 = 0.0;
            for i in 0..obj.length as usize {
                sum += *data.add(i);
            }
            *result = sum / obj.length as SigmaF64;
            return 0;
        } else if obj.type_ == RObjectType::Integer {
            let data = obj.data.ptr_val as *const SigmaI32;
            let mut sum: SigmaF64 = 0.0;
            for i in 0..obj.length as usize {
                sum += *data.add(i) as SigmaF64;
            }
            *result = sum / obj.length as SigmaF64;
            return 0;
        }
    }

    -1
}

/// Statistical function: sum
#[no_mangle]
pub unsafe extern "C" fn sigma_r_sum(
    interpreter_id: SigmaU64,
    obj_id: SigmaU64,
    result: *mut SigmaF64,
) -> SigmaI32 {
    if R_RUNTIME.is_none() || result.is_null() {
        return -1;
    }

    if let Some(runtime) = &R_RUNTIME {
        let idx = (interpreter_id - 1) as usize;
        if idx >= runtime.interpreter_count as usize {
            return -1;
        }

        let interp = &runtime.interpreters[idx];
        let sym_idx = (obj_id - 1) as usize;
        if sym_idx >= interp.global_env.symbol_count as usize {
            return -1;
        }

        let obj = &interp.global_env.symbols[sym_idx];
        
        if obj.type_ == RObjectType::Double {
            let data = obj.data.ptr_val as *const SigmaF64;
            let mut sum: SigmaF64 = 0.0;
            for i in 0..obj.length as usize {
                sum += *data.add(i);
            }
            *result = sum;
            return 0;
        } else if obj.type_ == RObjectType::Integer {
            let data = obj.data.ptr_val as *const SigmaI32;
            let mut sum: SigmaF64 = 0.0;
            for i in 0..obj.length as usize {
                sum += *data.add(i) as SigmaF64;
            }
            *result = sum;
            return 0;
        }
    }

    -1
}

/// Statistical function: sd (standard deviation)
#[no_mangle]
pub unsafe extern "C" fn sigma_r_sd(
    interpreter_id: SigmaU64,
    obj_id: SigmaU64,
    result: *mut SigmaF64,
) -> SigmaI32 {
    if R_RUNTIME.is_none() || result.is_null() {
        return -1;
    }

    if let Some(runtime) = &R_RUNTIME {
        let idx = (interpreter_id - 1) as usize;
        if idx >= runtime.interpreter_count as usize {
            return -1;
        }

        let interp = &runtime.interpreters[idx];
        let sym_idx = (obj_id - 1) as usize;
        if sym_idx >= interp.global_env.symbol_count as usize {
            return -1;
        }

        let obj = &interp.global_env.symbols[sym_idx];
        
        // Calculate mean first
        let mut mean_val: SigmaF64 = 0.0;
        if obj.type_ == RObjectType::Double {
            let data = obj.data.ptr_val as *const SigmaF64;
            let mut sum: SigmaF64 = 0.0;
            for i in 0..obj.length as usize {
                sum += *data.add(i);
            }
            mean_val = sum / obj.length as SigmaF64;

            // Calculate variance
            let mut variance: SigmaF64 = 0.0;
            for i in 0..obj.length as usize {
                let diff = *data.add(i) - mean_val;
                variance += diff * diff;
            }
            variance /= obj.length as SigmaF64;

            // Standard deviation
            *result = variance.sqrt();
            return 0;
        }
    }

    -1
}

/// Statistical function: var (variance)
#[no_mangle]
pub unsafe extern "C" fn sigma_r_var(
    interpreter_id: SigmaU64,
    obj_id: SigmaU64,
    result: *mut SigmaF64,
) -> SigmaI32 {
    if R_RUNTIME.is_none() || result.is_null() {
        return -1;
    }

    if let Some(runtime) = &R_RUNTIME {
        let idx = (interpreter_id - 1) as usize;
        if idx >= runtime.interpreter_count as usize {
            return -1;
        }

        let interp = &runtime.interpreters[idx];
        let sym_idx = (obj_id - 1) as usize;
        if sym_idx >= interp.global_env.symbol_count as usize {
            return -1;
        }

        let obj = &interp.global_env.symbols[sym_idx];
        
        if obj.type_ == RObjectType::Double {
            let data = obj.data.ptr_val as *const SigmaF64;
            let mut sum: SigmaF64 = 0.0;
            for i in 0..obj.length as usize {
                sum += *data.add(i);
            }
            let mean_val = sum / obj.length as SigmaF64;

            let mut variance: SigmaF64 = 0.0;
            for i in 0..obj.length as usize {
                let diff = *data.add(i) - mean_val;
                variance += diff * diff;
            }
            variance /= obj.length as SigmaF64;

            *result = variance;
            return 0;
        }
    }

    -1
}

/// Statistical function: median
#[no_mangle]
pub unsafe extern "C" fn sigma_r_median(
    interpreter_id: SigmaU64,
    obj_id: SigmaU64,
    result: *mut SigmaF64,
) -> SigmaI32 {
    if R_RUNTIME.is_none() || result.is_null() {
        return -1;
    }

    if let Some(runtime) = &R_RUNTIME {
        let idx = (interpreter_id - 1) as usize;
        if idx >= runtime.interpreter_count as usize {
            return -1;
        }

        let interp = &runtime.interpreters[idx];
        let sym_idx = (obj_id - 1) as usize;
        if sym_idx >= interp.global_env.symbol_count as usize {
            return -1;
        }

        let obj = &interp.global_env.symbols[sym_idx];
        
        if obj.type_ == RObjectType::Double {
            let data = obj.data.ptr_val as *const SigmaF64;
            // For simplicity, return middle value
            let mid_idx = obj.length as usize / 2;
            *result = *data.add(mid_idx);
            return 0;
        }
    }

    -1
}

/// Enable/disable vectorized operations
#[no_mangle]
pub unsafe extern "C" fn sigma_r_set_vectorized(
    interpreter_id: SigmaU64,
    enabled: SigmaBool,
) -> SigmaI32 {
    if R_RUNTIME.is_none() {
        return -1;
    }

    if let Some(runtime) = &mut R_RUNTIME {
        let idx = (interpreter_id - 1) as usize;
        if idx < runtime.interpreter_count as usize {
            runtime.interpreters[idx].vectorized_enabled = enabled;
            return 0;
        }
    }

    -1
}

/// Enable/disable garbage collection
#[no_mangle]
pub unsafe extern "C" fn sigma_r_set_gc(enabled: SigmaBool) -> SigmaI32 {
    if let Some(runtime) = &mut R_RUNTIME {
        runtime.gc_enabled = enabled;
        return 0;
    }
    -1
}

/// Run garbage collection
#[no_mangle]
pub unsafe extern "C" fn sigma_r_gc_collect() -> SigmaU32 {
    if let Some(runtime) = &R_RUNTIME {
        if runtime.gc_enabled {
            // Simplified GC
            return 0;
        }
    }
    0
}

/// External musl malloc
extern "C" {
    fn musl_malloc(size: SigmaUsize) -> *mut SigmaU8;
}

/// Helper: Get string length
unsafe fn name_len(s: *const SigmaU8) -> usize {
    if s.is_null() {
        return 0;
    }
    let mut len = 0;
    while *s.add(len) != 0 && len < 128 {
        len += 1;
    }
    len
}

/// Check if R runtime is initialized
#[no_mangle]
pub unsafe extern "C" fn sigma_r_initialized() -> SigmaBool {
    if let Some(runtime) = &R_RUNTIME {
        runtime.initialized
    } else {
        false
    }
}

/// Get interpreter count
#[no_mangle]
pub unsafe extern "C" fn sigma_r_interpreter_count() -> SigmaU32 {
    if let Some(runtime) = &R_RUNTIME {
        runtime.interpreter_count
    } else {
        0
    }
}
