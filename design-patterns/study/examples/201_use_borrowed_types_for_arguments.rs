fn main() {
    let a = "hello".to_string();
    print_use_string(a);
    //println!("{}", a); //  |                    ^ value borrowed here after move

    let b = "hello world".to_string();
    print_use_string1(&b);
    println!("{}", b);

    let c = "hello yaaaa";
    print_use_string2(c);
    println!("{}", c);
}

fn print_use_string(s: String) {
    println!("{}", s);
}

fn print_use_string1(s: &String) {
    println!("{}", s);
}

fn print_use_string2(s: &str) {
    println!("{}", s);
}
