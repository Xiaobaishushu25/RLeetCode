#[cfg(test)]
#[test]
pub fn test_max_sliding_window(){
    // println!("{:?}", max_sliding_window(vec![1, 2, 4, 5, 6, 4, 5], 3));
    // println!("{:?}", max_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3));
    println!("{:?}", max_sliding_window(vec![1], 1));
}
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut left:usize = 0;
    let k = k as usize;
    let mut max_num = *nums[0..k].iter().max().unwrap();

    let mut max_vec = vec![];
    while left + k <= nums.len()  {
        let x = &nums[left..left + k].iter().max().unwrap();
        max_vec.push(**x);
        left += 1;
    }
    max_vec
}
