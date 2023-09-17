use std::env;

fn main() -> miette::Result<()> {
    // let out_dir = compile();
    // println!("OUT_DIR: {}", out_dir);

    // let path = std::path::PathBuf::from("src"); // include path
    let path2 = std::path::PathBuf::from("faiss/"); // include path
    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&path2])
        .extra_clang_args(&["-std=c++20", "-Iomg"])
        .build()?;

    env::set_var("CXX", "/opt/homebrew/opt/llvm/bin/clang++");

    b.flag_if_supported("-std=c++20")
        .define("FINTEGER", "int")
        .flag("-Wno-deprecated-declarations")
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-function")
        .flag("-Wno-unused-const-variable")
        .flag("-Wno-unused-but-set-variable")
        .flag("-Wno-unused-variable")
        .flag("-Wno-mismatched-tags")
        .files(glob::glob("faiss/faiss/*.cpp").unwrap().map(|x| x.unwrap()))
        .files(glob::glob("faiss/faiss/utils/*.cpp").unwrap().map(|x| x.unwrap()))
        .files(glob::glob("faiss/faiss/impl/*.cpp").unwrap().map(|x| x.unwrap()))
        .files(glob::glob("faiss/faiss/invlists/*.cpp").unwrap().map(|x| x.unwrap()))
        // .opt_level(0)
        .emit_rerun_if_env_changed(false)
        .compile("faiss"); // arbitrary library name, pick anything
    println!("cargo:rerun-if-changed=src/lib.rs");
    Ok(())
}
