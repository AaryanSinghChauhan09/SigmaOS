# Contributing

Thank you for considering contributing to SigmaOS! This document explains how to contribute code, drivers, web apps, and documentation.

---

## Ways to Contribute

| Area | Skill Required | Good First Issue? |
|------|---------------|------------------|
| Kernel drivers | C++ / hardware knowledge | ⚠️ Intermediate |
| Go daemons | Go | ✅ Yes |
| Web apps | HTML/JS/CSS | ✅ Yes |
| Wiki / Docs | Writing | ✅ Yes |
| Build system | Bash / CMake | ⚠️ Intermediate |
| Test harnesses | C++ / Go | ✅ Yes |

Look for the `good-first-issue` label in [Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues).

---

## Development Setup

```bash
# 1. Fork and clone
git clone https://github.com/YOUR_USERNAME/SigmaOS.git
cd SigmaOS

# 2. Install C++ tools
sudo apt install -y build-essential cmake ninja-build clang

# 3. Install Go (≥ 1.21)
# See: https://go.dev/doc/install

# 4. Build the project
./build.sh          # Compiles kernel + daemons
./setup.sh          # Installs Buildroot dependencies

# 5. Run tests
cmake -B build -G Ninja && ninja -C build
./build/tests/kernel/test_vmm
./build/tests/kernel/test_tcp
```

---

## Coding Standards

### C++ (Kernel)

- **No POSIX, no libc**: Use only `sigma_stdio.h`, `sigma_types.h`, and klib headers from `klib/include/`.
- **No RTTI, no exceptions**: Use `sigma_assert()` and return codes instead.
- **`extern "C"` entry points**: All driver init functions must be `extern "C"`.
- Use `sigma_printf` (not `printf`), `sigma_malloc` (not `malloc`).

```cpp
// ✅ Correct
#include <sigma_types.h>
extern "C" void sigma_mydriver_init() {
    sigma_printf("[mydriver] Initializing...\n");
}

// ❌ Wrong — do not use standard library
#include <stdio.h>
void init() { printf("hello"); }
```

### Go (Daemons)

- All daemons must start with `http.HandleFunc` endpoints and a `log.Fatal(http.ListenAndServe(...))`.
- JSON request/response structs must be exported and documented.
- Cap checks must happen **before** any privileged operation.

### HTML/JS (Web Apps)

- All apps must include a `manifest.json` with a `sigmaos.permissions` array.
- Guard all `navigator.sigmaos` calls with existence checks.
- Use `navigator.sigmaos.fs.writeFile` for persistence — never `localStorage` for user data.

---

## Submitting a Driver

1. Create `kernel/drivers/<category>/sigma_<chipset>.cpp`.
2. Implement `extern "C" void sigma_<chipset>_init()`.
3. Register in `kernel/drivers/sigma_driver_registry.cpp`.
4. Add to `CMakeLists.txt` under the appropriate target.
5. Write a test in `tests/drivers/test_<chipset>.cpp`.
6. Update [Hardware-Support](Hardware-Support) wiki page.
7. Submit a PR — CI will run the test harness automatically.

---

## Submitting a Web App

1. Create `userland/web-shell/apps/<appname>/index.html` and `manifest.json`.
2. Declare all required `sigmaos.permissions` in the manifest.
3. Add your app to `userland/web-shell/store/apps.json`.
4. Test locally by running `busybox httpd -p 3000 -h userland/web-shell/` and opening your app.
5. Submit a PR.

---

## Pull Request Checklist

- [ ] Code follows the C++/Go/JS standards above
- [ ] New driver has an `extern "C"` init function
- [ ] New Go daemon code has capability checks before privileged ops
- [ ] Tests pass: `ninja -C build && ./build/tests/kernel/test_vmm`
- [ ] Wiki page updated if adding a new subsystem
- [ ] Commit message follows: `feat:`, `fix:`, `docs:`, `chore:`, `refactor:`

---

## Getting Help

- Open a [Discussion](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions) for design questions.
- Open an [Issue](https://github.com/AaryanSinghChauhan09/SigmaOS/issues) for bugs.
- Read the [Code of Conduct](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CODE_OF_CONDUCT.md).
