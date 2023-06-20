use std::{env, path::PathBuf};

fn main() {
    let libfprint = pkg_config::probe_library("libfprint-2").unwrap();
    let glib = pkg_config::probe_library("glib-2.0").unwrap();

    let glib_arg = glib
        .include_paths
        .iter()
        .map(|path| format!("-I{}", path.to_string_lossy()));
    let bindings = bindgen::Builder::default()
        .clang_args(
            libfprint
                .include_paths
                .iter()
                .map(|path| format!("-I{}", path.to_string_lossy()))
                .chain(glib_arg),
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
