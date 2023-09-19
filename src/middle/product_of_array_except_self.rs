/// # 238 除自身以外数组的乘积 (product-of-array-except-self)
///
///难度 ：Medium
///
///描述：<p>给你一个整数数组&nbsp;<code>nums</code>，返回 <em>数组&nbsp;<code>answer</code>&nbsp;，其中&nbsp;<code>answer[i]</code>&nbsp;等于&nbsp;<code>nums</code>&nbsp;中除&nbsp;<code>nums[i]</code>&nbsp;之外其余各元素的乘积</em>&nbsp;。</p>
///
///<p>题目数据 <strong>保证</strong> 数组&nbsp;<code>nums</code>之中任意元素的全部前缀元素和后缀的乘积都在&nbsp; <strong>32 位</strong> 整数范围内。</p>
///
///<p>请<strong>不要使用除法，</strong>且在&nbsp;<code>O(<em>n</em>)</code> 时间复杂度内完成此题。</p>
///
///<p>&nbsp;</p>
///
///<p><strong>示例 1:</strong></p>
///
///<pre>
///<strong>输入:</strong> nums = <code>[1,2,3,4]</code>
///<strong>输出:</strong> <code>[24,12,8,6]</code>
///</pre>
///
///<p><strong>示例 2:</strong></p>
///
///<pre>
///<strong>输入:</strong> nums = [-1,1,0,-3,3]
///<strong>输出:</strong> [0,0,9,0,0]
///</pre>
///
///<p>&nbsp;</p>
///
///<p><strong>提示：</strong></p>
///
///<ul>
///	<li><code>2 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
///	<li><code>-30 &lt;= nums[i] &lt;= 30</code></li>
///	<li><strong>保证</strong> 数组&nbsp;<code>nums</code>之中任意元素的全部前缀元素和后缀的乘积都在&nbsp; <strong>32 位</strong> 整数范围内</li>
///</ul>
///
///<p>&nbsp;</p>
///
///<p><strong>进阶：</strong>你可以在 <code>O(1)</code>&nbsp;的额外空间复杂度内完成这个题目吗？（ 出于对空间复杂度分析的目的，输出数组<strong>不被视为</strong>额外空间。）</p>
///
///
///url: https://leetcode-cn.com/problems/product-of-array-except-self/description
#[cfg(test)]
mod tests {
    #[test]
    fn test_product_of_array_except_self(){
        println!("{:?}", product_except_self(vec![1, 2, 3, 4]));
    }
    ///最简单的方法自然是全部遍历一遍计算出所有元素积，然后除以每个元素自身就是需要的数组。
    /// 但是题目不让用除法。
    ///e 不会做 看题解，第一种:左右乘积列表
    pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut left = vec![0;len];
        let mut right = vec![0;len];
        left[0] = 1;
        right[len -1] = 1;
        for i in 1..len {
            left[i] = left[i-1]*nums[i-1]
        }
        for i in (0..len-1).rev(){
            right[i] = right[i+1] * nums[i+1];
        }
        for i in 0..len{
            nums[i] = left[i] * right[i]
        }
        nums
    }
}
   