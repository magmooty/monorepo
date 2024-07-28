use std::env;
use std::path::PathBuf;

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
fn link_whatsapp_library() {
    println!("cargo:warning=Compiling for Apple Silicon");
    println!("cargo:rustc-link-lib=framework=Security");
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=11.0");

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let relative_path: PathBuf = current_dir.join("./binaries/darwin-arm64");
    let library_file_path = relative_path.join("libwhatsapp.a");

    // Check if the file exists
    if !library_file_path.exists() {
        panic!(
            "WhatsApp library file not built: {}",
            library_file_path.display()
        );
    }

    let link_search_path = relative_path
        .to_str()
        .expect("Failed to convert path to string");

    println!("cargo:rustc-link-search=native={}", link_search_path);
    println!(
        "cargo:warning=WhatsApp library search path: {}",
        link_search_path
    );
    println!("cargo:rustc-link-lib=static=whatsapp");
}

#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
fn link_whatsapp_library() {
    println!("cargo:warning=Compiling for Windows x86_64");

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let relative_path: PathBuf = current_dir.join("./binaries/windows-x86_64");
    let library_file_path = relative_path.join("libwhatsapp.a");

    // Check if the file exists
    if !library_file_path.exists() {
        panic!(
            "WhatsApp library file not built: {}",
            library_file_path.display()
        );
    }

    let link_search_path = relative_path
        .to_str()
        .expect("Failed to convert path to string");

    println!("cargo:rustc-link-search=native={}", link_search_path);
    println!(
        "cargo:warning=WhatsApp library search path: {}",
        link_search_path
    );
    println!("cargo:rustc-link-lib=static=whatsapp");
}

fn main() {
    link_whatsapp_library();
}
