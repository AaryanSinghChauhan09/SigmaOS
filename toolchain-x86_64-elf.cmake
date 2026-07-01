# SigmaOS Cross-Compiler Toolchain Configuration
# Targets x86_64 bare-metal environment (Sovereign Architecture)

set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR x86_64)

# Ensure the cross-compiler is in your PATH.
# e.g., built via crosstool-NG or downloaded pre-built x86_64-elf tools.
set(CMAKE_C_COMPILER x86_64-elf-gcc)
set(CMAKE_CXX_COMPILER x86_64-elf-g++)
set(CMAKE_ASM_COMPILER x86_64-elf-as)

# Core Compiler Flags (Freestanding, No Stdlib, Position Independent Code)
set(COMMON_FLAGS "-ffreestanding -fPIC -mno-red-zone -mcmodel=large -fno-stack-protector")

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} ${COMMON_FLAGS} -std=c11 -Wall -Wextra" CACHE STRING "" FORCE)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${COMMON_FLAGS} -std=c++20 -Wall -Wextra -fno-exceptions -fno-rtti" CACHE STRING "" FORCE)

# Linker Flags (No default libc, custom linker script)
set(CMAKE_EXE_LINKER_FLAGS "-nostdlib -Wl,-T${CMAKE_SOURCE_DIR}/linker.ld -Wl,-z,max-page-size=0x1000" CACHE STRING "" FORCE)

# Don't try to link C/C++ executables during CMake compiler test (we have no libc)
set(CMAKE_TRY_COMPILE_TARGET_TYPE STATIC_LIBRARY)
