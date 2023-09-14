fn main() {
    let s = "world";
    println!("{}", say_hello1(s));
    println!("{}", say_hello2(s));
}

fn say_hello1(name: &str) -> String {
    let mut result = "hello ".to_owned();
    result.push_str(name);
    result.push_str("!");
    result
}

fn say_hello2(name: &str) -> String {
    format!("hello {}!", name)
}
