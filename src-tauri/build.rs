fn main() {
    // Disable Windows resource compilation
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-cfg=windows_resource_disabled");
    }
    tauri_build::build()
}
