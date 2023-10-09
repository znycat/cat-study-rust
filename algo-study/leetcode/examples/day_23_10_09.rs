fn main() {
    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let v = Solution::remove_element(&mut nums, 2);
    println!("nums: {:?} res = {v}", nums);
}

struct Solution();

/*
给你一个数组 nums 和一个值 val，你需要 原地 移除所有数值等于 val 的元素，并返回移除后数组的新长度。
不要使用额外的数组空间，你必须仅使用 O(1) 额外空间并 原地 修改输入数组。
元素的顺序可以改变。你不需要考虑数组中超出新长度后面的元素。
*/

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut index: usize = 0;
        for i in 0..nums.len() {
            let a = nums[i];
            if a == val {
            } else {
                nums[index] = a;
                index += 1;
            }
        }
        return index as i32;
    }
}
