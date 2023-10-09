fn main() {
    let mut nums1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
    let mut nums2: Vec<i32> = vec![2, 5, 6];
    Solution::merge(&mut nums1, 3, &mut nums2, 3);
    println!("nums1:{:?}", nums1);

}

struct Solution();

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let tmp = nums1[..(m as usize)].to_vec();
        let mut i: usize = 0;
        let mut j: usize = 0;
        nums1.clear();
        while (i as i32) < m && (j as i32) < n {
            if tmp[i] < nums2[j] {
                nums1.push(tmp[i]);
                i += 1;
            } else {
                nums1.push(nums2[j]);
                j += 1;
            }
        }
        if (i as i32) == m {
            nums1.extend(nums2[j..].iter());
        } else {
            nums1.extend(tmp[i..].iter());
        }
    }
}
