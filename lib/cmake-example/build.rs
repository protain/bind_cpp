use cmake;
use bindgen;

fn main() {
    // CMakeLists.txtが存在するディレクトリを指定
    //
    let dst = cmake::build("src/cpp");
    println!("cargo:rustc-link-search=native={}", dst.display());

    // staticライブラリとして他に利用するライブラリはない事を表明する
    println!("cargo:rustc-link-lib=static=cmake-example");

    // C++ソースは必ず追加(→実は不要)
    //println!("cargo:rustc-link-lib=dylib=stdc++");

    // pkg-configでヒットしないライブラリは以下のように直接パス指定することが可能
    let soil_lib_dir = "/usr/lib";
    println!("cargo:rustc-link-search={}", soil_lib_dir);
    //println!("cargo:rustc-link-lib=dylib=SOIL");

    let bindings = bindgen::Builder::default()
        .header("src/cpp/test.h")
        .generate()
        .expect("unable to generate bindings.");
    bindings
        .write_to_file("src/bindings.rs")
        .expect("could not write bindings.");
}
