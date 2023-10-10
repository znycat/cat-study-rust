fn main() {
    // let mut nums = vec![1, 1, 1, 2, 2, 3];
    let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    let v = Solution::remove_duplicates(&mut nums);
    println!("nums: {:?} res = {v}", nums);
}

struct Solution();

/*
给你一个有序数组 nums ，请你 原地 删除重复出现的元素，使得出现次数超过两次的元素只出现两次 ，返回删除后数组的新长度。

不要使用额外的数组空间，你必须在 原地 修改输入数组 并在使用 O(1) 额外空间的条件下完成。
*/

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 2 {
            return n as i32;
        }

        let mut slow: usize = 2;
        for fast in 2..n {
            let cursor = nums[fast];
            if nums[slow - 2] != cursor {
                nums[slow] = cursor;
                slow += 1;
            }
        }
        slow as i32
    }
    
    #[allow(dead_code)]
    pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
        let mut before_num = nums[0];

        let mut is_double = false;
        let mut re: usize = 1;

        for i in 1..nums.len() {
            let cursor = nums[i];
            if cursor == before_num {
                if is_double {
                } else {
                    nums[re] = cursor;
                    is_double = true;
                    re += 1;
                }
            } else {
                nums[re] = cursor;
                before_num = cursor;
                re += 1;
                is_double = false;
            }
        }

        re as i32
    }
}
