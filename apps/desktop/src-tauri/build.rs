#[cfg(target_os = "windows")]
use std::{env, path::PathBuf};

#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
static BINARIES_DIRECTORY: &str = r"binaries\windows-x86_64";

#[cfg(target_os = "windows")]
fn link_libraries() {
    #[cfg(all(target_arch = "x86_64", target_os = "windows"))]
    println!("cargo:warning=Compiling for Windows x86_64");

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let relative_path: PathBuf = current_dir.join(BINARIES_DIRECTORY);

    // Find binaries search path
    let link_search_path = relative_path
        .to_str()
        .expect("Failed to convert path to string");

    // Set binaries search path
    println!("cargo:warning=Libraries search path: {}", link_search_path);
    println!("cargo:rustc-link-search=native={}", link_search_path);
}

fn main() {
    #[cfg(target_os = "windows")]
    link_libraries();

    tauri_build::build();
}
