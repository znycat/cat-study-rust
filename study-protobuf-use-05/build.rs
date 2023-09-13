fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=person.proto");
    prost_build::Config::new()
        .out_dir("src/proto_model")
        // 用之前是 data: ::prost::alloc::vec::Vec<u8>,
        // 用之后是pub data: ::prost::bytes::Bytes,
        // .bytes(&["."])
        // 所有叫scrores的map都会变成btree_map
        .btree_map(&["scores"])
        .type_attribute(&".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&["proto/person.proto"], &["."])
        .unwrap();

    /*
    .bytes
        // 用之前是 data: ::prost::alloc::vec::Vec<u8>,
        // 用之后是pub data: ::prost::bytes::Bytes,
    .btreee_map
        pub scores: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
        pub scores: ::prost::alloc::collections::BTreeMap
        <
        ::prost::alloc::string::String,
        i32,
        >,
    derive是有限的,如果需要增加更多的derive,比如需要添加serde的 #[derive(Serialize,Deserialize)]
    那么就需要使用 .type_attribute()
    有意思的是,在加完之后,发现bytes是不能Serialize , 需要去解决冲突

    */
}
