#[cfg(test)]
mod tests {
    #[test]
    fn test_max_sub_array() {
        println!("{}", max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
        println!("{}", max_sub_array(vec![1]));
        println!("{}", max_sub_array(vec![5,4,-1,7,8]));
        println!("{}", max_sub_array(vec![-2,-1]));
        println!("{}", max_sub_array(vec![-1,-2]));
        println!("{}", max_sub_array(vec![-1,0]));
    }
    ///关键在于某个数是负数，要不要？
    /// 要的情况：这个负数可以与前面元素结合起来大于0
    /// 那么我们从第一个元素开始计和，直到这个和小于0，可以把前面的全丢掉
    ///时间 8ms 击败 96.83%使用 Rust 的用户
    /// 内存 2.92MB 击败 100.00%使用 Rust 的用户
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut sum = 0;
        for num in nums{
            sum += num;
            if num>0 {
                //加上一个正数，有可能是最大值
                max = sum.max(max);
            }else {
                if sum<=0 {
                    //考虑全负数的情况，此时max就是前面最大的一个负数
                    max = num.max(max);
                    //加上i后和小于0了，全部不要了
                    sum = 0;
                }
            }
        }
        max
    }
}