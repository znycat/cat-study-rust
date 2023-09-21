fn main() {
    chars_bytes();

    // 字符串修改一般要使用String类型
    let mut cat = String::from("猫猫");
    println!("cat: {:?}", cat);

    // 追加
    cat.push('1');
    println!("cat: {:?}", cat);
    cat.push_str("2");
    println!("cat: {:?}", cat);
    cat.extend(['3', '4'].iter());
    println!("cat: {:?}", cat);
    cat.extend("567".chars());
    println!("cat: {:?}", cat);

    // 插入
    // insert insert_str 使用的都是字节来处理的,我们需要先找到正确的位置
    // 比如我们需要插入到1的位置,也就是第一个猫的后面
    // char_indices()方法将字符串转换为一个包含字符索引和字符本身的元组的迭代器
    if let Some((byte_index, char)) = cat.char_indices().nth(1) {
        println!("byte_index: {:?}, char: {:?}", byte_index, char);
        cat.insert_str(byte_index, "插入的字符串");
        println!("cat: {:?}", cat);
    }

    // 连接
    let cat1 = "zny".to_string();
    let cat2 = String::from("cat");
    let mut cat3 = cat1 + &cat2;
    println!("cat3: {:?}", cat3);
    cat3 += "123";
    println!("cat3: {:?}", cat3);

    // 更新
    let s = String::from("foobar");
    let mut result = s.into_bytes();
    // 用索引来更新操作
    (0..result.len()).for_each(|i| {
        if i % 2 == 0 {
            result[i] = result[i].to_ascii_uppercase();
        } else {
            result[i] = result[i].to_ascii_lowercase();
        }
    });
    println!("result: {:?}", result,);
    println!(
        "result_String: {:?}",
        String::from_utf8(result).unwrap_or("def".to_string())
    );

    let s = String::from("foobar");
    // 按照字符迭代来处理
    let s: String = s
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 0 {
                c.to_ascii_lowercase().to_string()
            } else {
                c.to_ascii_uppercase().to_string()
            }
        })
        .collect();
    println!("s: {:?}", s);

    // 删除

    let mut s = String::from("foobar");
    s.remove(3);
    assert_eq!(s, "fooar");

    let mut s = String::from("foobar");
    s.truncate(3);
    assert_eq!(s, "foo");

    let mut s = String::from("foobar");
    s.clear();
    assert_eq!(s, "");

    let mut s = String::from("foobbbbar");
    let beta_offset = s.find('b').unwrap_or(s.len());
    let t: String = s.drain(..beta_offset).collect();
    println!("t: {:?}", t);
    assert_eq!(t, "foo");

    let mut s = String::from("foobar");
    s.drain(..);
    assert_eq!(s, "");



}

#[allow(dead_code)]
fn chars_bytes() {
    // 字符串处理的两种方式
    // 1. 按照字节处理
    let cat = "猫猫123";
    let chars = cat.chars();
    println!("chars: {:?}", chars);
    println!("chars.count(): {:?}", chars.count());

    // 2. 按照字符处理
    let bytes = cat.bytes();
    println!("bytes: {:?}", bytes);
    println!("bytes.count(): {:?}", bytes.count());
}
