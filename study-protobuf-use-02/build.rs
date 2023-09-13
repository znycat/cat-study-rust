fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=person.proto");
    prost_build::Config::new()
        .out_dir("src/proto_model")
        .compile_protos(&["proto/person.proto"], &["."])
        .unwrap();
}
