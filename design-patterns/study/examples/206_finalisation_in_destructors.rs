fn main() {
    let a = bar().unwrap();
    println!("{:?}", a);
}

fn bar() -> Result<(), std::io::Error> {
    struct Foo;
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Dropping Foo");
        }
    }

    let _exit = Foo;

    Ok(())
}
