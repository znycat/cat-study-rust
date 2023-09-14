
/// Add two numbers
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    let c = a + b;
    return c;
}

/// cbindgen:field-names=[x, y]
/// cbindgen:derive-eq
#[repr(C)]
pub struct Point(pub f32, pub f32);

/// Print a message
#[no_mangle]
pub extern "C" fn foo(a: i32, b: bool) {
    println!("hello foo a:{:?} b:{:?} ", a, b);
}
