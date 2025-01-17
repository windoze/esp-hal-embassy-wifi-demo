fn main() {
    if cfg!(feature = "esp32c6") || cfg!(feature = "esp32h2") {
        println!("cargo::rustc-link-arg=-Trom_coexist.x");
        println!("cargo::rustc-link-arg=-Trom_functions.x");
        println!("cargo::rustc-link-arg=-Trom_phy.x");
    } else {
        println!("cargo:rustc-link-arg-bins=-Trom_functions.x");
    }
}
