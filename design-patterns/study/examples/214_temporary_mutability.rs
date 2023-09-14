fn main() {
    let mut data = vec![2, 1, 4, 10, 3, 5];
    data.sort();
    let data = data; // 重新绑定data 变为不可变的变量

    println!("{:?}", data[2]);

    // data.push(4);

    let data = {
        let mut data = vec![2, 1, 4, 10, 3, 5];
        data.sort();
        data
    };
    println!("{:?}", data[2]);
}
