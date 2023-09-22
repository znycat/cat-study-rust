fn main() {
    contains();
    find();
    split();
    matches();
    trim();
    replace();
}

/// 存在性判断
#[allow(dead_code)]
fn contains() {
    // 存在性判断
    let str = "znycat";
    assert!(str.contains("cat"));
    assert!(str.contains(char::is_lowercase));
    assert!(str.starts_with("zny"));
    assert!(str.ends_with("cat"));
}

/// 位置匹配
#[allow(dead_code)]
fn find() {
    // 位置匹配
    let str = "znY 猫猫 cat";
    assert_eq!(Some(0), str.find('z'));
    assert_eq!(Some(1), str.find('n'));
    assert_eq!(Some(12), str.find('a'));
    assert_eq!(Some(12), str.rfind('a'));
    // 找第一个小写
    assert_eq!(Some(0), str.find(char::is_lowercase));
    // 找第一个大写
    assert_eq!(Some(2), str.find(char::is_uppercase));
    // 找第一个空格
    assert_eq!(Some(3), str.find(char::is_whitespace));
}
/// 分割字符串
#[allow(dead_code)]
fn split() {
    let s = "hello world";
    let mut iter = s.split_whitespace();
    assert_eq!(Some("hello"), iter.next());
    assert_eq!(Some("world"), iter.next());
    assert_eq!(None, iter.next());

    let s2: Vec<&str> = "hello world".split(|c| c == ' ').collect();
    println!("{:?}", s2);

    let s3: Vec<&str> = "hello world".split(" ").collect();
    println!("{:?}", s3);

    let s4: Vec<&str> = "A..B..".split(".").collect();
    println!("{:?}", s4);
}

/// 捕获匹配
#[allow(dead_code)]
fn matches() {
    let s: Vec<&str> = "zny 123 zny 123 zny 123".matches("zny").collect();
    println!("{:?}", s);

    let s1: Vec<&str> = "zny 123 zny 123 zny 123"
        .rmatches(char::is_numeric)
        .collect();
    println!("{:?}", s1);

    // 返回一个元组数组,元组的两个值一个是索引,一个是字符串
    let s2: Vec<(usize, &str)> = "zny 123 zny 123 zny 123".match_indices("zny").collect();
    println!("{:?}", s2);

    let s3: Vec<(usize, &str)> = "zny 123 zny 123 zny 123"
        .rmatch_indices(char::is_numeric)
        .collect();
    println!("{:?}", s3);
}
/// 删除匹配
#[allow(dead_code)]
fn trim() {
    let s = "  zny 123 zny 123 zny 123  ".trim();
    assert_eq!("zny 123 zny 123 zny 123", s);
    let s = "  zny 123 zny 123 zny 123  ".trim_start();
    assert_eq!("zny 123 zny 123 zny 123  ", s);

    let s = "  zny 123 zny 123 zny 123  ".trim_end();
    assert_eq!("  zny 123 zny 123 zny 123", s);
}

/// 替代匹配
#[allow(dead_code)]
fn replace() {
    let s = "Hello world\t";
    // replace 从左到右依次替换
    assert_eq!("Hello world ", s.replace("\t", " "));
    // trim()去除空格
    assert_eq!("Hello world", s.replace("\t", " ".trim()));

    let s = "this is old old 123";
    // replace 会替换多次
    assert_eq!("this is new new 123", s.replace("old", "new"));
    // replacen 可以指定替换次数
    assert_eq!("this is new old 123", s.replacen("old", "new", 1));
    assert_eq!("this is ald ald 123", s.replacen("o", "a", 3));
    // char::is_numeric 判断是否为数字, 让替换从第一个数字开始
    assert_eq!(
        "this is old old new23",
        s.replacen(char::is_numeric, "new", 1)
    );
}
