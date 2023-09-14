#![crate_type = "staticlib"]

enum SimpleError {
    IoError = 1,
    FileCorrupted = 2,
}

/// 可以从SimpleError转换到c语言的整数类型
impl From<SimpleError> for libc::c_int {
    fn from(error: SimpleError) -> libc::c_int {
        (error as i8).into()
    }
}

enum SimpleError2 {
    IoError(String),
    FileCorrupted(String),
}

/// 可以从SimpleError转换到c语言的整数类型
impl From<SimpleError2> for libc::c_int {
    fn from(error: SimpleError2) -> libc::c_int {
        match error {
            SimpleError2::IoError(_) => 1,
            SimpleError2::FileCorrupted(_) => 2,
        }
    }
}

struct SimpleError3 {
    expected: char,
    line: i32,
}

#[repr(C)]
pub struct ParseError {
    pub expected: libc::c_char,
    pub line: libc::c_int,
}

/// 可以从SimpleError转换到c语言的整数类型
impl From<SimpleError3> for ParseError {
    fn from(s: SimpleError3) -> ParseError {
        let SimpleError3 { expected, line } = s;

        ParseError {
            expected: expected as libc::c_char,
            line: line as libc::c_int,
        }
    }
}

fn produce_err(t: u64) -> SimpleError {
    match t {
        1 => SimpleError::IoError,
        _ => SimpleError::FileCorrupted,
    }
}

fn produce_err2(t: u64) -> SimpleError2 {
    match t {
        1 => SimpleError2::IoError("IoError".to_string()),
        _ => SimpleError2::FileCorrupted("FileCorrupted".to_string()),
    }
}

fn produce_err3(t: u64) -> SimpleError3 {
    match t {
        1 => SimpleError3 {
            expected: 'a',
            line: 1,
        },
        _ => SimpleError3 {
            expected: 'b',
            line: 2,
        },
    }
}

#[no_mangle]
pub extern "C" fn return_err1(t: libc::c_int) -> libc::c_int {
    let err = produce_err(t as u64);
    libc::c_int::from(err)
}

#[no_mangle]
pub extern "C" fn return_err2(t: libc::c_int) -> *mut libc::c_char {
    let err = produce_err2(t as u64);
    let err_str = match err {
        SimpleError2::IoError(e) => format!("IoError:{:?}", e),
        SimpleError2::FileCorrupted(e) => format!("FileCorrupted:{:?}", e),
    };

    let c_error: *mut i8 = unsafe {
        let malloc: *mut u8 = libc::malloc(err_str.len() + 1) as *mut u8;
        if malloc.is_null() {
            return std::ptr::null_mut();
        }
        let src = err_str.as_bytes().as_ptr();
        std::ptr::copy_nonoverlapping(src, malloc, err_str.len());
        std::ptr::write(malloc.add(err_str.len()), 0);
        malloc as *mut libc::c_char
    };
    c_error
}

#[no_mangle]
pub extern "C" fn return_err3() -> ParseError {
    let err: SimpleError3 = produce_err3(1);
    ParseError::from(err)
}
