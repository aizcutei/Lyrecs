extern crate cc;

fn main() {
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=Foundation");
        cc::Build::new().file("extern/get_playing.m")
                        .compile("libgetplaying.a");
        println!("cargo:rustc-link-lib=static=getplaying");
        println!("cargo:rerun-if-changed=extern/get_playing.m");
    }
    tauri_build::build()
}
