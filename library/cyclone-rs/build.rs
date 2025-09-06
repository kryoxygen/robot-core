use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // 获取输出目录
    println!("cargo:rustc-rerun-if-changed=wrapper.h");

    // let dst = cmake::Config::new("msg").no_build_target(true).build();
    // println!("cargo:rustc-link-search=native={}/lib", dst.display());
    pkg_config::Config::new().atleast_version("0.10.0").probe("CycloneDDS").unwrap();

    let bindings = bindgen::Builder::default()
        // .clang_arg("-std=c++11") // 使用 C++11 标准
        .clang_arg("-xc") // 指定输入语言为 C（而不是 C++）
        .layout_tests(false) // 禁用为结构体生成布局测试
        .generate_comments(false) // 禁用在生成的绑定中包含注释
        .derive_default(true) // 为生成的类型自动派生 Default 特性
        .header("wrapper.h") // 头文件用于绑定生成
        .generate()
        .expect("Unable to generate bindings");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dylib=ddsc");

	cc::Build::new()
        .file("HelloWorldData.c")
        .include("./")
        .compile("HelloWorldData_lib");

    let out_path = PathBuf::from("./src/");
    let mut file = File::create(out_path.join("bindings.rs")).unwrap();
    // 先写入 warning 抑制
    writeln!(file, "#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, improper_ctypes, dead_code)]").unwrap();
    bindings.write(Box::new(file)).expect("Couldn't write bindings!");
}
