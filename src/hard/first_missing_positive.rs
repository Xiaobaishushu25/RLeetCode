/// # 41 缺失的第一个正数 (first-missing-positive)
///
///难度 ：Hard
///
///描述：<p>给你一个未排序的整数数组 <code>nums</code> ，请你找出其中没有出现的最小的正整数。</p>
///请你实现时间复杂度为 <code>O(n)</code> 并且只使用常数级别额外空间的解决方案。
///
///<p> </p>
///
///<p><strong>示例 1：</strong></p>
///
///<pre>
///<strong>输入：</strong>nums = [1,2,0]
///<strong>输出：</strong>3
///</pre>
///
///<p><strong>示例 2：</strong></p>
///
///<pre>
///<strong>输入：</strong>nums = [3,4,-1,1]
///<strong>输出：</strong>2
///</pre>
///
///<p><strong>示例 3：</strong></p>
///
///<pre>
///<strong>输入：</strong>nums = [7,8,9,11,12]
///<strong>输出：</strong>1
///</pre>
///
///<p> </p>
///
///<p><strong>提示：</strong></p>
///
///<ul>
///	<li><code>1 <= nums.length <= 5 * 10<sup>5</sup></code></li>
///	<li><code>-2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1</code></li>
///</ul>
///
///
///url: https://leetcode-cn.com/problems/first-missing-positive/description
#[cfg(test)]
mod tests {

    #[test]
    fn test_first_missing_positive(){
        println!("{}", first_missing_positive(vec![1, 2, 0]));
        println!("{}", first_missing_positive(vec![3,4,-1,1]));
        println!("{}", first_missing_positive(vec![7,8,9,11,12]));
    }
    ///不会做，看了题解写的..
    ///数组长度为N，那么缺失的第一个正数必定在[1,N+1]范围。
    /// 若数组为1到N的连续整数，那么缺失的就是N+1，否则缺失的正整数就在[1,N]范围。
    /// 要求使用常数级别额外空间的解决方案，那么只能在原数组上操作。
    /// 我们可以把数组元素调整位置，值为x的元素应该在x-1的位置上。排好后就是[1,2,3,-1...]
    /// 当不满足nums[i]= i+1,就是缺失的第一个正整数。
    /// 那么就是遍历数组，如果x在[1,N]的范围，就把x放到num[x-1]的位置上，注意换过来的可能也需要交换。
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            loop {
                let num = nums[i];
                if 1<=num && num <= nums.len() as i32 {
                    if nums[(num-1)as usize] == num { break }
                    nums[i] = nums[(num-1)as usize];
                    nums[(num-1)as usize] = num;
                }else {
                    break;
                }
            }
        }
        for i in 1..nums.len()+1 {
             if nums[i-1] != i as i32 {
                 return i as i32
             }
        }
        (nums.len() + 1) as i32
    }
}
   