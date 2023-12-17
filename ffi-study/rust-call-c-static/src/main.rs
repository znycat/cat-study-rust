fn main() {
    unsafe {
        let num = add(1, 2);
        println!("{}", num);
    }
}

#[link(name = "cat_custom")]
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}
