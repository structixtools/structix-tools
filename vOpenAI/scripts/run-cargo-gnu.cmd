@echo off
setlocal
set "PATH=C:\Program Files\Rust stable LLVM 1.94\bin;C:\Program Files\Rust stable LLVM 1.94\lib\rustlib\x86_64-pc-windows-gnullvm\bin\gcc-ld;%PATH%"
set "LIB=C:\Users\Usuario\AppData\Local\Microsoft\WinGet\Packages\MartinStorsjo.LLVM-MinGW.UCRT_Microsoft.Winget.Source_8weky3d8bbwe\llvm-mingw-20260311-ucrt-x86_64\x86_64-w64-mingw32\lib;%LIB%"
set "RUSTFLAGS=-L native=C:\Users\Usuario\AppData\Local\Microsoft\WinGet\Packages\MartinStorsjo.LLVM-MinGW.UCRT_Microsoft.Winget.Source_8weky3d8bbwe\llvm-mingw-20260311-ucrt-x86_64\x86_64-w64-mingw32\lib"
set "CARGO_TARGET_X86_64_PC_WINDOWS_GNULLVM_LINKER=rust-lld.exe"
"C:\Program Files\Rust stable LLVM 1.94\bin\cargo.exe" %*
