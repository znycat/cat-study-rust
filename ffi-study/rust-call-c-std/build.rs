extern crate cc;
fn main() {
    cc::Build::new()
    .file("c_file/cat_custom.c")
        .compile("libcat_custom.a");
}
