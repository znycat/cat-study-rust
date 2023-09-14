fn main() {
    println!("Hello, world!");
    {
        let p = Person {
            first_name: String::from("Naiyu"),
            last_name: String::from("z"),
        };
        println!("box: {:?}", p);
        let a = Box::new(p);
        println!("box: {:?}", a);
    }

    println!("Hello, world!end");
    
}

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("Dropping Person!");
    }
}

