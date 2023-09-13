use prost::Message;
use study_protobuf_use_02::proto_model::{person::PhoneNumber, person::PhoneType, Person};

fn main() {
    let a = 0;
    println!("main 函数的起始地址约在{:?}", &a as *const i32);

    let phones = vec![
        PhoneNumber::new("1831831818183", PhoneType::Mobile),
        PhoneNumber::new("1833333", PhoneType::Work),
    ];
    let p = Person::new("znycat", 1111, "znycat@qq.com", &phones);
    let p_point = &p as *const Person;
    println!("p = {:?}    p的地址 = {:?}", p, p_point);
    let v1 = p.encode_to_vec();
    println!("v1 = {:?}", v1);
    let person1 = Person::decode(v1.as_slice()).unwrap();
    assert_eq!(person1, p);
    println!(
        "person1 = {:?}     &person1地址 = {:?}",
        person1, &person1 as *const Person
    );

    // use std::{mem, ptr};
    // let person1_ptr = unsafe { mem::transmute::<_, *const Person>(&person1) };
    // let person1_address = ptr::addr_of!(*person1_ptr);
    // let person1_address = unsafe { ptr::addr_of!((*person1_ptr).name) };
    // println!("person1地址 = {:?}", person1_address);

    // let v2 = p.encode_length_delimited_to_vec();
    // println!("v2 = {:?}", v2);

    let a = 0;
    println!("main 函数的结束地址约在{:?}", &a as *const i32);
}
