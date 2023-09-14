use std::fs;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = "-";
    let (mut stdin_read, mut file_read);

    let readable: &mut dyn io::Read = match arg {
        "-" => {
            stdin_read = io::stdin();
            &mut stdin_read
        }
        path => {
            file_read = fs::File::open(path)?;
            &mut file_read
        }
    };

    let mut buf: Vec<u8> = Vec::new();
    println!("{:?}", readable.read_to_end(&mut buf));

    let a: A<u8> = A { a: 1u8 };
    println!("{}", a.a);

    Ok(())
}

struct A<T> {
    a: T,
}
