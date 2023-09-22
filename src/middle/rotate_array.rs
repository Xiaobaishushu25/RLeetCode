/// # 189 轮转数组 (rotate-array)
///
///难度 ：Medium
///
///描述：<p>给定一个整数数组 <code>nums</code>，将数组中的元素向右轮转 <code>k</code><em>&nbsp;</em>个位置，其中&nbsp;<code>k</code><em>&nbsp;</em>是非负数。</p>
///
///<p>&nbsp;</p>
///
///<p><strong>示例 1:</strong></p>
///
///<pre>
///<strong>输入:</strong> nums = [1,2,3,4,5,6,7], k = 3
///<strong>输出:</strong> <code>[5,6,7,1,2,3,4]</code>
///<strong>解释:</strong>
///向右轮转 1 步: <code>[7,1,2,3,4,5,6]</code>
///向右轮转 2 步: <code>[6,7,1,2,3,4,5]
///</code>向右轮转 3 步: <code>[5,6,7,1,2,3,4]</code>
///</pre>
///
///<p><strong>示例&nbsp;2:</strong></p>
///
///<pre>
///<strong>输入：</strong>nums = [-1,-100,3,99], k = 2
///<strong>输出：</strong>[3,99,-1,-100]
///<strong>解释:</strong> 
///向右轮转 1 步: [99,-1,-100,3]
///向右轮转 2 步: [3,99,-1,-100]</pre>
///
///<p>&nbsp;</p>
///
///<p><strong>提示：</strong></p>
///
///<ul>
///	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
///	<li><code>-2<sup>31</sup> &lt;= nums[i] &lt;= 2<sup>31</sup> - 1</code></li>
///	<li><code>0 &lt;= k &lt;= 10<sup>5</sup></code></li>
///</ul>
///
///<p>&nbsp;</p>
///
///<p><strong>进阶：</strong></p>
///
///<ul>
///	<li>尽可能想出更多的解决方案，至少有 <strong>三种</strong> 不同的方法可以解决这个问题。</li>
///	<li>你可以使用空间复杂度为&nbsp;<code>O(1)</code> 的&nbsp;<strong>原地&nbsp;</strong>算法解决这个问题吗？</li>
///</ul>
///
///
///url: https://leetcode-cn.com/problems/rotate-array/description
#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    #[test]
    fn test_rotate_array(){
        rotate(&mut vec![1,2,3,4,6,7],3);
    }
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let mut deque = nums.drain(..).collect::<VecDeque<_>>();
        for _ in 0..k {
            let back = deque.pop_back().unwrap();
            deque.push_front(back);
        }
        *nums = deque.into_iter().collect::<Vec<_>>();
    }



    #[allow(dead_code)]
    pub fn rotate2(nums: &mut Vec<i32>, k: i32) {
        let mut deque = nums.iter().collect::<VecDeque<&i32>>();
        for _ in 0..k {
            let back = deque.pop_back().unwrap();
            deque.push_front(back);
        }
        //问题的原因在于deque.into_iter()的行为。当你调用into_iter()时，它会消耗掉deque并将其转换为一个迭代器。
        // 这个迭代器的元素类型与deque中的元素类型相同，即&i32。即使你调用了to_owned()，它也只是获取了迭代器中每个
        // 元素的所有权，而不是改变迭代器元素的类型。
        // 为了解决这个问题，你可以使用deque.into_iter().cloned().collect::<Vec<i32>>()来获取元素的所有权并将
        // 其收集到一个新的Vec<i32>中。这样，你就不会再遇到类型不匹配的问题
        // *nums = deque.into_iter().to_owned().collect::<Vec<i32>>();
        *nums = deque.into_iter().cloned().collect::<Vec<i32>>();
    }
}
   