use std::path::{Path, PathBuf};
use std::{env, fs};

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
static BINARIES_DIRECTORY: &str = "./binaries/darwin-arm64";

#[cfg(all(target_arch = "aarch64", target_os = "linux"))]
static BINARIES_DIRECTORY: &str = "./binaries/linux-arm64";

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
static BINARIES_DIRECTORY: &str = "./binaries/linux-x86_64";

#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
static BINARIES_DIRECTORY: &str = r"binaries\windows-x86_64";

fn link_libraries() {
    // Compiler logs
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    println!("cargo:warning=Compiling for Apple Silicon");

    #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
    println!("cargo:warning=Compiling for Linux Arm64");

    #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
    println!("cargo:warning=Compiling for Linux x86_64");

    #[cfg(all(target_arch = "x86_64", target_os = "windows"))]
    println!("cargo:warning=Compiling for Windows x86_64");

    // MacOS specific flags
    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    println!("cargo:rustc-link-lib=framework=Security");

    #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
    println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=14.5");

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let relative_path: PathBuf = current_dir.join(BINARIES_DIRECTORY);

    let out_path = env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_path = Path::new(&out_path)
        .ancestors()
        .nth(3)
        .unwrap()
        .to_path_buf();

    println!(
        "cargo:warning=Build directory: {}",
        &out_path.to_str().unwrap()
    );

    // Find binaries search path
    let link_search_path = relative_path
        .to_str()
        .expect("Failed to convert path to string");

    // Set binaries search path
    println!("cargo:warning=Libraries search path: {}", link_search_path);
    println!("cargo:rustc-link-search=native={}", link_search_path);
    
    // Select binaries to statically link
    println!("cargo:rustc-link-lib=static={}", "whatsapp");
    
    // Select binaries to dynamically link
    println!("cargo:rustc-link-lib=dylib={}", "tdjson");
    
    // Copy dynamically linked tdjson
    #[cfg(target_os = "windows")]
    let tdjson_file_name = "tdjson.dll";

    #[cfg(target_os = "linux")]
    let tdjson_file_name = "libtdjson.so.1.8.29";
    
    #[cfg(target_os = "macos")]
    let tdjson_file_name = "libtdjson.1.8.34.dylib";
    
    let tdjson_path = relative_path.join(tdjson_file_name);
    
    println!(
        "cargo:warning=tdjson dynamic library: {}",
        &tdjson_path.to_str().unwrap()
    );
    println!("cargo:rerun-if-changed={}", tdjson_path.display());
    
    fs::copy(tdjson_path, out_path.join(tdjson_file_name)).unwrap();
}

fn main() {
    link_libraries();
}
