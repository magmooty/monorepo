use std::env;
use std::fs;
use std::path::PathBuf;

struct WhatsAppLinkingInfo<'a> {
    library_file_name: &'a str,
    relative_path: &'a str,
}

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
fn get_whatsapp_linking_info() -> WhatsAppLinkingInfo<'static> {
    println!("cargo:warning=Compiling for Apple Silicon");
    WhatsAppLinkingInfo {
        library_file_name: "libwhatsapp.a",
        relative_path: "../../whatsapp-bot/target/darwin-arm64",
    }
}

#[cfg(all(target_arch = "x86_64", target_os = "macos"))]
fn get_whatsapp_linking_info() -> WhatsAppLinkingInfo<'static> {
    println!("cargo:warning=Compiling for Apple Intel");
    WhatsAppLinkingInfo {
        library_file_name: "libwhatsapp.a",
        relative_path: "../../whatsapp-bot/target/darwin-x86_64",
    }
}

#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
fn get_whatsapp_linking_info() {
    println!("cargo:warning=Compiling for x86_64 Windows");
    WhatsAppLinkingInfo {
        library_file_name: "libwhatsapp.dll",
        relative_path: "../../whatsapp-bot/target/windows-x86_64",
    }
}

#[cfg(all(target_arch = "i686", target_os = "windows"))]
fn get_whatsapp_linking_info() {
    println!("cargo:warning=Compiling for i686 Windows");
    WhatsAppLinkingInfo {
        library_file_name: "libwhatsapp.dll",
        relative_path: "../../whatsapp-bot/target/windows-i686",
    }
}

fn link_whatsapp_library(whatsapp_linker_info: WhatsAppLinkingInfo) {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let relative_path: PathBuf = current_dir.join(whatsapp_linker_info.relative_path);
    let library_file_path = relative_path.join(whatsapp_linker_info.library_file_name);

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

fn copy_surreal_db() {
    let current_dir = env::current_dir().expect("Failed to get current directory");

    let surreal_build_path: PathBuf = current_dir.join("../../../surrealdb/target/release/surreal");
    let surreal_exec_path: PathBuf = current_dir.join("surreal");

    match fs::copy(surreal_build_path, surreal_exec_path) {
        Ok(_) => println!("File copied successfully!"),
        Err(e) => panic!(
            "Couldn't find surreal, please make sure to build surrealdb: {}",
            e
        ),
    }
}

fn main() {
    println!("cargo:rustc-link-lib=framework=Security");
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=11.0");
    link_whatsapp_library(get_whatsapp_linking_info());
    copy_surreal_db();
    tauri_build::build()
}
