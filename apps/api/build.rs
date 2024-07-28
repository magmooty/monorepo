use std::env;
use std::path::PathBuf;

struct WhatsAppLinkingInfo<'a> {
    library_file_name: &'a str,
    relative_path: &'a str,
}

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
fn get_whatsapp_linking_info() -> WhatsAppLinkingInfo<'static> {
    println!("cargo:warning=Compiling for Apple Silicon");
    println!("cargo:rustc-link-lib=framework=Security");
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=11.0");
    WhatsAppLinkingInfo {
        library_file_name: "libwhatsapp.a",
        relative_path: "./binaries/darwin-arm64",
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

fn main() {
    link_whatsapp_library(get_whatsapp_linking_info());
}
