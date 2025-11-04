use std::{env, path::PathBuf};

fn main() {
    let libfprint = pkg_config::probe_library("libfprint-2").unwrap();

    let bindings = bindgen::Builder::default()
        .allowlist_function("fp_device.*")
        .allowlist_function("fp_print.*")
        .allowlist_function("fp_context.*")
        .allowlist_function("fp_image.*")
        .allowlist_function("g_ptr_array_free")
        // GObject types and their get_type functions
        .allowlist_type("FpContextClass")
        .allowlist_type("_FpContextClass")
        .allowlist_type("FpDeviceClass")
        .allowlist_type("_FpDeviceClass")
        .allowlist_type("FpImageClass")
        .allowlist_type("_FpImageClass")
        .allowlist_type("FpPrintClass")
        .allowlist_type("_FpPrintClass")
        // GType functions
        .allowlist_function("fp_context_get_type")
        .allowlist_function("fp_device_get_type")
        .allowlist_function("fp_image_get_type")
        .allowlist_function("fp_print_get_type")
        // Other types
        .allowlist_type("FpPrint_autoptr")
        .allowlist_type("GType")
        // Allow all types that start with underscore followed by Fp
        .allowlist_type("_Fp.*")
        // Allow autoptr types
        .allowlist_type(".*_autoptr")
        .clang_args(
            libfprint
                .include_paths
                .iter()
                .map(|path| format!("-I{}", path.to_string_lossy())),
        )
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("libfprint.rs"))
        .expect("Couldn't write bindings!");
}
