fn main() {
    let target = std::env::var("TARGET").unwrap();
    if target.contains("linux") {
        println!("cargo:rustc-link-lib=fontconfig");
        println!("cargo:rustc-link-lib=freetype");
    } 
}
