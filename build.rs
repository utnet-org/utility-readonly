fn main() {
    cc::Build::new()
        .file("src/hash.c")
        .include("/opt/homebrew/Cellar/openssl@3/3.1.3/include") //确保此路径正确
        .compile("hash");
    println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/openssl@3/3.1.3/lib"); //确保此路径正确
    println!("cargo:rustc-link-lib=dylib=ssl");
    println!("cargo:rustc-link-lib=dylib=crypto");
}
