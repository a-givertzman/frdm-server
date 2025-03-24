// use winreg::{RegKey, HKEY_LOCAL_MACHINE};
fn main() {
    // Default search paths...
    println!(r"cargo:rustc-link-search=src/infrostructure/arena/ArenaSDK_Linux_x64/lib64");
    println!(r"cargo:rustc-link-search=src/infrostructure/arena/ArenaSDK_Linux_x64/GenICam/library/lib/Linux64_x64");
    println!(r"cargo:rustc-link-search=src/infrostructure/arena/ArenaSDK_Linux_x64/ffmpeg");

    // Add current Arena SDK path(s) to the system env (for current session only)
    // to make it avalible for system lib loader
    // Deppending on the OS...
    if cfg!(target_os = "linux") {
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

    // println!("cargo:rustc-link-search=ArenaCApi");
    // println!("cargo:rustc-link-lib=libarenac");
    // println!(r"cargo:rustc-link-search=native=src/infrostructure/arena/ArenaSDK_Linux_x64/lib64");
    // println!(r"cargo:rustc-link-search=src/infrostructure/arena/ArenaSDK_Linux_x64/include/ArenaC");
    // println!("cargo:rustc-link-lib=dylib=arenac");
    // println!("cargo:rustc-link-lib=libarenac");
    // println!("cargo:rustc-link-lib=arena");
    // println!("cargo:rustc-link-lib=avcore");
    println!("cargo:rustc-link-lib=arenac");
    println!("cargo:rustc-link-lib=arenad");
    // println!("cargo:rustc-link-lib=gentl");
    // src/infrostructure/arena/ArenaSDK_Linux_x64/lib64/GenTL_LUCID.cti
    // println!("cargo:rustc-link-lib=./src/infrostructure/arena/ArenaSDK_Linux_x64/lib64/libarenac.so");
    // println!("cargo:rustc-link-search=./src/infrostructure/arena/ArenaSDK_Linux_x64/lib64/");
    // println!(r"RUSTFLAGS='-Clink-args=-Wl,-rpath=/home/lobanov/code/rust/frdm-server/src/infrostructure/arena/ArenaSDK_Linux_x64/lib64/'");
    // println!("cargo:rustc-link-lib=/home/lobanov/code/rust/frdm-server/src/infrostructure/arena/ArenaSDK_Linux_x64/lib64/libarenac.so");

    // Custom install path.
    // if let Some(lucid_arena_sdk_lib_path) = option_env!("LUCID_ARENA_SDK_LIB_PATH") {
    //     println!("cargo:rustc-link-search={}", lucid_arena_sdk_lib_path);
    // }

    // Tell cargo to tell rustc to link the Arena shared library.
    // if let Some(lucid_arena_sdk_lib_name) = option_env!("LUCID_ARENA_SDK_LIB_NAME") {
    //     println!("cargo:rustc-link-lib={}", lucid_arena_sdk_lib_name);
    // } else {
    //     println!("cargo:rustc-link-lib=arena/ArenaSDK_Linux_x64/lib64/libarenac");
    // }

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    // println!("cargo:rerun-if-changed=wrapper.h");
    // println!("cargo:rerun-if-changed=src/infrostructure/arena/mod.rs");
}
