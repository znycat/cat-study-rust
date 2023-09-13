use study_protobuf_use_02::proto_model::{person::PhoneNumber, person::PhoneType, Person};

fn main() {
    let phones = vec![
        PhoneNumber::new("1831831818183", PhoneType::Mobile),
        PhoneNumber::new("1833333", PhoneType::Work),
    ];
    let p = Person::new("znycat", 1111, "znycat@qq.com", &phones);

    let json = serde_json::to_string_pretty(&p).unwrap();
    println!("json = {:?}", json);

}
