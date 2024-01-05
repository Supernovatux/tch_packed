## First time installation steps
- First install `cargo-post`
```powershell
cargo install cargo-post
```
- Clone the repo
- Close any `IDE` you might be running like `vscode`, `clion` etc.
- Navigate to the repo with command line.
- Delete the `tools` directory if it exists
- Run `cargo post build --release`
- This will take a lot of time based on your internet connection. It will download `libtorch` extract it to `tools`. Also will setup [AppPacker](https://github.com/SerGreen/Appacker)
- run the app present at `Release/tch_packed.exe` (The app name will change if u rename the project). Note it is `Release/tch_packed.exe` not `tools/Release/tch_packed.exe`
- If no errors present proceed with development of the app.
- If dll error exist follow the below steps. For any other issue report a bug.

## Solving Dll issues
- The missing dlls can be found by running the app via file explorer.
- Add missing dlls to the `REQ_DLL` list at `.cargo/config.toml`

## Deployement
- When you want to distribute the application run 
```powershell
cargo post build --release
```
- This will create a executable in `Release` folder. You can distribute it.