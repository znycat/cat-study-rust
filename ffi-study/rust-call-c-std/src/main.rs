fn main() {
    call_c_std_abs();
    call_c_add_custom();
}

// 调用c语言标准款的abs函数
// extern 需要使用unsafe才能调用
fn call_c_std_abs() {
    unsafe {
        let num = abs(-20);
        println!("{}", num);
    }
}

// 通过 build.rs  cc libc 来自动将c文件编译到静态库中 就可以直接调用了
/*

``` 
build = "build.rs"

[dependencies]
libc = "0.2.151"

[build-dependencies]
cc = "1.0.83"
``` 
*/
fn call_c_add_custom() {
    unsafe {
        let num = add(1, 2);
        println!("{}", num);
    }
}


extern "C" {
    fn add(a: i32, b: i32) -> i32;

    fn abs(a: i32) -> i32;
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works1() {
        call_c_std_abs();
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works2() {
        call_c_add_custom();
        assert_eq!(3 + 3, 6);
    }
}
