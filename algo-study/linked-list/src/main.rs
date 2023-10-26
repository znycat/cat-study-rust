fn main() {
    test();
}

// 问题, map 返回的是引用,而for 返回的是值

fn test() {
    let arr = vec![1, 2, 3, 4, 5];
    arr.iter().for_each(|e| {
        println!("e: {:?}", e);
    });

    let arr2 = vec![1, 2, 3, 4, 5];
    for e in arr2 {
        println!("e: {:?}", e);
    }

    let arr3 = vec![1, 2, 3, 4, 5];
    arr3.into_iter().for_each(|e| {
        println!("e: {:?}", e);
    });

    let mut arr4 = vec![1, 2, 3, 4, 5];
    arr4.iter_mut().for_each(|e| {
        println!("e: {:?}", e);
        *e = *e * 2;
    });
    println!("arr4: {:?}", arr4);
}
