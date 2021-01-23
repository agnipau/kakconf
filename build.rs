use std::{env, path::Path};

fn main() {
    println!("cargo:rustc-link-lib=editorconfig");

    let bindings = bindgen::Builder::default()
        .header("editorconfig.h")
        .generate()
        .expect("Failed to generate bindings");

    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_path = Path::new(&root_dir).join("src").join("editorconfig_sys.rs");

    let mut bindings = bindings.to_string();
    bindings.insert_str(
        0,
        "#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]\n",
    );
    std::fs::write(&out_path, bindings)
        .unwrap_or_else(|_| panic!("Failed to write bindings to {}", out_path.display()));
}
