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
    use std::iter::MapWhile;
    use crate::middle::find_all_anagrams::find_anagrams;

    #[test]
    fn test_first_missing_positive(){
        // println!("{}", first_missing_positive(vec![1, 2, 0]));
        // println!("{}", first_missing_positive(vec![3,4,-1,1]));
        println!("{}", first_missing_positive(vec![7,8,9,11,12]));
    }
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut record = vec![];
        for num in nums {
            // println!("{},{num}",record.len());
            while record.len()<(num.abs()+1) as usize {
                record.push(false)
            }
            if num>=0 {
                println!("{},{num}",record.len());
                record[num as usize] = true;
            }
        }
        record.push(false);
        for(index,value) in record.iter().enumerate(){
            // println!("{index} {value}");
            if !*value&&index!=0 {
                return index as i32;
            }
        }
        0
    }
}
   