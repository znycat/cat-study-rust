
fn main() {
    // Option<T>类型会自动实现IntoIterator trait 
    let a = Some("a str");
    let mut s1 = vec!["a", "b", "c"];
    s1.extend(a);
    println!("s1: {:?}", s1);

    let b = Some("b str");
    let s1 = vec!["a", "b", "c"];

    for item in s1.iter().chain(b.iter()) {
        println!("item: {}", item);
    }
}
