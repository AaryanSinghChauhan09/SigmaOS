# sigpkg Packaging Templates

To lower the barrier to entry and reduce contributor friction, SigmaOS provides officially maintained templates for packaging common languages and applications.

## 1. Rust Application Template
```yaml
name: "sigma-rust-app"
version: "1.0.0"
architecture: "x86_64"
epoch: 1

build:
  - sigma-dev-toolchain rust
  - cargo build --release --target x86_64-sigma-none

install:
  - cp target/x86_64-sigma-none/release/app $SIGPKG_STAGING/bin/app
```

## 2. C/C++ Application Template
```yaml
name: "sigma-c-app"
version: "1.0.0"
architecture: "x86_64"
epoch: 1

build:
  - sigma-dev-toolchain gcc
  - make -j$(nproc)

install:
  - make install DESTDIR=$SIGPKG_STAGING
```

## 3. Node.js Application Template
```yaml
name: "sigma-node-app"
version: "1.0.0"
architecture: "noarch"
epoch: 1

build:
  - sigma-dev-toolchain node
  - npm install

install:
  - cp -r . $SIGPKG_STAGING/opt/node-app
```
