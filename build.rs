fn main() {
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=SDL2main");
    println!("cargo:rustc-link-lib=SDL2_image");
}
