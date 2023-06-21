use std::{env, path::PathBuf};

fn main() {
    let libfprint = pkg_config::probe_library("libfprint-2").unwrap();

    let bindings = bindgen::Builder::default()
        .clang_args(
            libfprint
                .include_paths
                .iter()
                .map(|path| format!("-I{}", path.to_string_lossy())),
        )
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("libfprint.rs"))
        .expect("Couldn't write bindings!");
}
