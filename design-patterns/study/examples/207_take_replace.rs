use std::mem;

fn main() {
    let mut a = MyEnum::A {
        name: "cat".to_string(),
        x: 0u8,
    };
    println!("a:{:?}", a);
    a_to_b(&mut a);
    println!("a:{:?}", a);

    let mut a = MyEnum::A {
        name: "cat1".to_string(),
        x: 0u8,
    };
    println!("a:{:?}", a);
    a_to_b1(&mut a);
    println!("a:{:?}", a);

    let mut a = MyEnum::A {
        name: "cat2".to_string(),
        x: 0u8,
    };
    println!("a:{:?}", a);
    a_to_b2(&mut a);
    println!("a:{:?}", a);

    let b: MyEnum = MyEnum::B {
        name: "cat3".to_string(),
    };
    println!("{:?}", b);
}

#[derive(Debug)]
enum MyEnum {
    A {
        name: String,
        x: u8,
    },
    #[allow(dead_code)]
    B {
        name: String,
    },
}

fn a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        *e = MyEnum::B { name: name.clone() };
    }
}

fn a_to_b1(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        *e = MyEnum::B {
            name: mem::take(name),
        };
    }
}

fn a_to_b2(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        *e = MyEnum::B {
            name: mem::replace(name, String::new()),
        };
    }
}
