## First time installation steps
- First install `cargo-post`
```powershell
cargo install cargo-post
```
- Clone the repo
- Close any `IDE` you might be running like `vscode`, `clion` etc.
- Navigate to the repo with command line.
- Delete the `tools` directory if it exists
- Run `cargo post build --release`. It needs internet connection.
- This will take a lot of time based on your internet connection. It will download `libtorch` extract it to `tools`. Also will setup [AppPacker](https://github.com/SerGreen/Appacker)
- Run the app present at `Release/tch_packed.exe` (The app name will change if u rename the project). Note it is `Release/tch_packed.exe` not `tools/Release/tch_packed.exe`
- Uncomment the `#tch = "0.14.0"` from line 14 of `cargo.toml`
- Uncomment all commented lines in `main.rs`
- Run `cargo post build --release` again. Execute the binary at `.\Release\tch_packed.exe`
- If no errors present proceed with development of the app.
- If DLL error exist follow the below steps. For any other issue, submit a bug report.

## Solving Dll issues
- The missing DLLs can be found by running the app via file explorer.
- Add missing DLLs to the `REQ_DLL` list at `.cargo/config.toml`

## Deployment
- When you want to distribute the application run 
```powershell
cargo post build --release
```
- This will create an executable in `Release` folder. You can distribute it.

## Directory structure
```bash
.
|-- .cargo
|   `-- config.toml
|-- Release
|   `-- tch_packed.exe
|-- src
|   `-- main.rs
|-- target
|   |-- debug
|   |   |-- build
|   |   |-- deps
|   |   |-- examples
|   |   |-- .fingerprint
|   |   |-- incremental
|   |   `-- .cargo-lock
|   |-- release
|   |   |-- build
|   |   |-- deps
|   |   |-- examples
|   |   |-- .fingerprint
|   |   |-- incremental
|   |   |-- .cargo-lock
|   |   |-- tch_packed.d
|   |   `-- tch_packed.exe
|   |-- CACHEDIR.TAG
|   `-- .rustc_info.json
|-- tools
|   |-- libtorch
|   |   |-- bin
|   |   |-- cmake
|   |   |-- include
|   |   |-- lib
|   |   |-- share
|   |   |-- test
|   |   |-- build-hash
|   |   `-- build-version
|   |-- Release
|   |   |-- asmjit.dll
|   |   |-- c10.dll
|   |   |-- fbgemm.dll
|   |   |-- libiomp5md.dll
|   |   |-- MSVCP140.dll
|   |   |-- tch_packed.exe
|   |   |-- torch_cpu.dll
|   |   |-- uv.dll
|   |   `-- VCRUNTIME140.dll
|   |-- Appacker_v1.3.11.exe
|   `-- libtorch.zip
|-- build.rs
|-- Cargo.lock
|-- Cargo.toml
|-- .gitignore
|-- post_build.rs
`-- readme.md
```
- `tools\libtorch` contains the `libtorch` library
- `tools\Release` Contains *Temp files* needed by post build script to work.
- `Release` Contains the final binary
