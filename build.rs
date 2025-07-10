// use winreg::{RegKey, HKEY_LOCAL_MACHINE};
fn main() {
    // Add current Arena SDK path(s) to the system env (for current session only)
    // to make it avalible for system lib loader
    println!(r"cargo:rustc-link-search=/usr/lib/arena-sdk/lib64");
    println!(r"cargo:rustc-link-search=src/infrostructure/arena/ArenaSDK_Linux_x64/lib64");
    // println!(r"cargo:rustc-link-search=src/infrostructure/arena/ArenaSDK_Linux_x64/GenICam/library/lib/Linux64_x64");
    // println!(r"cargo:rustc-link-search=src/infrostructure/arena/ArenaSDK_Linux_x64/ffmpeg");
    // Deppending on the OS...
    if cfg!(target_os = "linux") {
        // println!(r"cargo:rustc-link-search=/usr/lib/arena-sdk/lib64");
        // println!("cargo:rustc-env=LD_LIBRARY_PATH=/your/custom/path/");
        // println!("cargo:rustc-env=LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/your/custom/path/");
    } else if cfg!(target_os = "macos") {
        // export DYLD_FALLBACK_LIBRARY_PATH=/your/custom/path/:$DYLD_FALLBACK_LIBRARY_PATH
    } else if cfg!(target_os = "windows") {
        // set PATH=%PATH%;C:\your\path\here\
        // let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        // let (env, _) = hkcu.create_subkey("Environment").unwrap(); // create_subkey opens with write permissions
        // let env_key = hklm.create_subkey("System\\CurrentControlSet\\Control\\Session Manager\\Environment").unwrap();
        // env_key.set_value("MY_VAR", "my_value").unwrap();
    }
    println!("cargo:rustc-link-lib=arenac");
    println!("cargo:rustc-link-lib=arenad");
}
