extern crate gcc;

fn main() {
    // Necessary for using library.
    println!("cargo:rustc-link-lib=flint");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=mpfr");
    println!("cargo:rustc-link-lib=gmp");

    gcc::Config::new()
        .file("src/flint_wrapper.c")
        .flag("-lflint -lmpfr -lgmp -lpthread")
        .include("/usr/local/include/flint")
        .compile("libflint_wrapper.a");
}
