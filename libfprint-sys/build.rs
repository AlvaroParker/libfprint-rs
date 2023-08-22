use std::{env, path::PathBuf};

fn main() {
    let libfprint = pkg_config::probe_library("libfprint-2").unwrap();

    let bindings = bindgen::Builder::default()
        .allowlist_function("fp_device.*")
        .allowlist_function("fp_print.*")
        .allowlist_function("fp_context.*")
        .allowlist_function("fp_image.*")
        .allowlist_function("g_ptr_array_free")
        .allowlist_type("FpContextClass")
        .allowlist_type("FpPrint_autoptr")
        .allowlist_type("FpDeviceClass")
        .allowlist_type("FpImageClass")
        .allowlist_type("FpPrintClass")
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
