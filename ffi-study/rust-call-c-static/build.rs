fn main() {
    println!("cargo:rustc-link-lib=cat_custom");
    // 使用相对路径
    // 这里的相对路径./ 是target 所在的目录
    println!("cargo:rustc-link-search=./rust-call-c-static/c_file/");
    // println!("cargo:rustc-link-search=../rust-call-c-2/c_file/");
}

// println!("cargo:rustc-link-lib=static=cat_custom");
// native 使用完全路径
//println!("cargo:rustc-link-search=native=/Users/naiyu3/Desktop/rust-call-c-2/");
