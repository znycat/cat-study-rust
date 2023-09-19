fn main() {
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
    assert_eq!("this is old old new23", s.replacen(char::is_numeric, "new", 1));
}
