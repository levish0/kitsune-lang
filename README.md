# kitsune-lang
kitsune-lang

## ⚠ Windows Users: LLVM Setup Notice

`kitsune-lang` requires LLVM version **18.1.x** with the exact layout expected by the `llvm-sys` crate.

On Windows, prebuilt LLVM binaries often **do not include** `llvm-config.exe` or have incompatible directory structures.

**Therefore, Windows users must manually download the LLVM source code and build LLVM themselves.**

### Steps:

1. Download the LLVM 18.1.x source code from the official releases:  
   https://github.com/llvm/llvm-project/releases

2. Build LLVM on your Windows machine using CMake and Ninja (or another build system).

3. Set the environment variable `LLVM_SYS_181_PREFIX` to point to your LLVM build directory, which must contain `bin/llvm-config.exe`. For example:
   ```powershell
   $env:LLVM_SYS_181_PREFIX = "C:\path\to\your\llvm"
   ```
4. After that, running cargo build should detect your LLVM build correctly.

This manual build is necessary to ensure compatibility with llvm-sys and successful compilation of kitsune-lang on Windows.