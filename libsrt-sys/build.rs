use std::{env, path::PathBuf};

fn main() {
    if cfg!(unix) {
        let mut lib_dir = {
            let mut config = cmake::Config::new("libsrt");
            config.define("ENABLE_APPS", "OFF");
            #[cfg(not(debug_assertions))]
            config.define("ENABLE_DEBUG", "OFF");
            config.define("ENABLE_SHARED", "OFF");
            config.define("USE_STATIC_LIBSTDCXX", "ON");
            config.build()
        };
        lib_dir.push("lib");
        println!("cargo:rustc-link-search={}", lib_dir.display());
        println!("cargo:rustc-link-lib=static=srt");
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=dylib=crypto");
    } else if cfg!(windows) {
        let dst = cmake::Config::new("libsrt")
            .generator("Visual Studio 16 2019")
            .cxxflag("/EHs")
            .define("ENABLE_STDCXX_SYNC", "ON")
            .define("ENABLE_APPS", "OFF")
            .build();
        let mut lib_dir = dst.clone();
        lib_dir.push("lib");
        let mut bin_dir = dst;
        bin_dir.push("bin");
        println!("cargo:rustc-link-search={}", lib_dir.display());
        println!("cargo:rustc-link-search={}", bin_dir.display());
        println!("cargo:rustc-link-lib=srt");
    }

    println!("cargo:rerun-if-changed=wrapper.h");

    let mut include_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    include_path.push("include");
    include_path.push("srt");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("--include-directory={}", include_path.display()))
        .size_t_is_usize(true)
        .allowlist_function("srt_.*")
        .allowlist_type("SRT.*")
        .allowlist_var("SRT.*")
        .bitfield_enum("SRT_EPOLL_OPT")
        .default_enum_style(bindgen::EnumVariation::NewType { is_bitfield: false })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
}
