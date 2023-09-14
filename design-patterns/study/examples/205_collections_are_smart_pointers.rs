use std::ops::Deref;

fn main() {
    let a: Vec<i32> = vec![1, 2, 3, 4];
    // 切片的集合
    let b: &[i32] = a.deref();
    println!("{:?}", b);
}

// impl<T> Deref for Vec<T> {
//     type Target = [T];
//     fn deref(&self) -> &[T] {
//         unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len()) }
//     }
// }
