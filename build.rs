use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .clang_arg("-I/usr/include/python3.11/")
        .derive_debug(true)
        .derive_default(true)
        .generate_comments(false)
        .layout_tests(false)
        .header("wrapper.h")
        .allowlist_type("_PyRuntimeState")
        .allowlist_type("PyInterpreterState")
        .allowlist_type("PyFrameObject")
        .allowlist_type("PyThreadState")
        .allowlist_type("PyCodeObject")
        .allowlist_type("PyVarObject")
        .allowlist_type("PyBytesObject")
        .allowlist_type("PyASCIIObject")
        .allowlist_type("PyUnicodeObject")
        .allowlist_type("PyCompactUnicodeObject")
        .allowlist_type("PyTupleObject")
        .allowlist_type("PyListObject")
        .allowlist_type("PyLongObject")
        .allowlist_type("PyFloatObject")
        .allowlist_type("PyDictObject")
        .allowlist_type("PyDictKeysObject")
        .allowlist_type("PyDictKeyEntry")
        .allowlist_type("PyDictUnicodeEntry")
        .allowlist_type("PyObject")
        .allowlist_type("PyTypeObject")
        .allowlist_type("PyHeapTypeObject")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    
    // panic!("{:?}", out_dir);
}