fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=person.proto");
    prost_build::Config::new()
        .out_dir("src/proto_model")
        // .bytes(&["."])
        .btree_map(&["scores"])
        .type_attribute(&".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .field_attribute("data", "#[serde(rename = \"datasss\")]")
        .field_attribute("data", "#[serde(skip_serializing_if = \"Vec::is_empty\")]")
        .compile_protos(&["proto/person.proto"], &["."])
        .unwrap();

    /*

    */
}
