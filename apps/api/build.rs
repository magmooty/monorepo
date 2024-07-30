use std::env;
use std::path::PathBuf;

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
static BINARIES_DIRECTORY: &str = "./binaries/darwin-arm64";

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
static BINARIES_DIRECTORY: &str = "./binaries/linux-arm64";

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
static BINARIES_DIRECTORY: &str = "./binaries/linux-x86_64";

#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
static BINARIES_DIRECTORY: &str = "./binaries/windows-x86_64";

static WHATSAPP_LIBRARY_FILE_NAME: &str = "libwhatsapp.a";
static WHATSAPP_LIBRARY_NAME: &str = "whatsapp";

fn link_whatsapp_library() {
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    println!("cargo:warning=Compiling for Apple Silicon");

    #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
    println!("cargo:warning=Compiling for Linux Arm64");

    #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
    println!("cargo:warning=Compiling for Linux x86_64");

    #[cfg(all(target_arch = "x86_64", target_os = "windows"))]
    println!("cargo:warning=Compiling for Windows x86_64");

    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    println!("cargo:rustc-link-lib=framework=Security");

    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=11.0");

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let relative_path: PathBuf = current_dir.join(BINARIES_DIRECTORY);
    let library_file_path = relative_path.join(WHATSAPP_LIBRARY_FILE_NAME);

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

    println!(
        "cargo:warning=WhatsApp library search path: {}",
        link_search_path
    );
    println!("cargo:rustc-link-search=native={}", link_search_path);
    println!("cargo:rustc-link-lib=static={}", WHATSAPP_LIBRARY_NAME);
}

fn main() {
    link_whatsapp_library();
}
