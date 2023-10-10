
pub mod array;
pub mod array_generic;

mod study01 {
    #[allow(dead_code)]
    pub fn run() {
        /* 初始化数组 */
        // 无初始值
        let arr: Vec<i32> = vec![0; 5]; // [0, 0, 0, 0, 0]
        println!("arr: {:?}", arr);

        //给定初始值
        let nums: Vec<i32> = vec![1, 3, 2, 5, 4];
        println!("nums: {:?}", nums);
    }

    #[test]
    fn test_study01() {
        run();
    }
}

