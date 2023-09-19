fn main() {
    utf8_utf16();
    unicode();


}

#[allow(dead_code)]
fn unicode() {
    // rust 如何打印一个字符的 unicode 码点?
    let cat = '猫';
    // 我们看一下猫的unicode 码点吧
    assert_eq!(0b111001100101011, cat as u32);
    assert_eq!(0o71453, cat as u32);
    assert_eq!(29483, cat as u32);
    assert_eq!(0x732b, cat as u32);
    // unicode 编码
    println!(
        "猫的 unicode code point for \n:{:?} \n bin: {:b} \n hex:{:X} \n oct: {:o}",
        cat as i32, cat as i32, cat as i32, cat as i32
    );
}

#[allow(dead_code)]
fn utf8_utf16() {
    let cat = '猫';
    let mut cat_utf8: [u8; 4] = [0; 4];
    cat.encode_utf8(&mut cat_utf8);
    assert_eq!([231, 140, 171, 0], cat_utf8);
    println!("UTF-8 encoding of the character: {:?}", &cat_utf8);

    let mut cat_utf16: [u16; 2] = [0; 2];
    cat.encode_utf16(&mut cat_utf16);
    assert_eq!([29483, 0], cat_utf16);
    println!("UTF-16 encoding of the character: {:?}", &cat_utf16);
    // 对于utf16来说,一般使用两个或者4个字节来存储,
    // Unicode码点在0~FFFF 范围内的使用两个字节存储,并且是直接存储码点,不会进行编码转换

    // > 中文字符的Unicode编码范围是U+4E00至U+9FFF。
    // > 这个范围被称为中日韩统一表意文字区域（CJK Unified Ideographs），
    // > 用于表示中文、日文和韩文等东亚文字。
    // > 在这个范围内，包括了汉字、标点符号、部分注音符号和特殊字符。
    //
    // > 以下是一些常见的Unicode编码范围：
    //
    // > 中文（汉字）：U+4E00至U+9FFF
    // > 中文标点符号：U+3000至U+303F
    // > 中文注音符号：U+3100至U+312F

    // 编码转换
    // 0xxx xxxx 单字节编码形式，这和 ASCII 编码完全一样
    // 110xxxxx 10xxxxxx 双字节编码
    // 1110xxxx 10xxxxxx 10xxxxxx 三字节编码
    // 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx 四字节编码

    // utf32 是一种固定长度的编码方案,他始终使用4个字节来存储,也就是u32
    let cat_utf32: u32 = cat as u32;
    assert_eq!(0x732b, cat_utf32);
    println!("UTF-32 encoding of the character: {:?}", cat_utf32);
}
