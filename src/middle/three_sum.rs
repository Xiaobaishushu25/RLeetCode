use std::collections::{ HashSet};
///给你一个整数数组 nums ，判断是否存在三元组 [nums[i], nums[j], nums[k]] 满足 i != j、i != k 且 j != k ，
/// 同时还满足 nums[i] + nums[j] + nums[k] == 0 。请你返回所有和为 0 且不重复的三元组。
/// 注意：答案中不可以包含重复的三元组。
/// https://leetcode.cn/problems/3sum/description/
#[cfg(test)]
#[test]
pub fn test_three_sum(){
    // println!("{:?}", three_sum(vec![-1, 0, 1, 2, -1, -4]));
    // println!("{:?}", three_sum(vec![0,0,0]));
    // println!("{:?}", three_sum(vec![0,1,1]));
    // println!("{:?}", three_sum(vec![-2,0,1,1,2]));
    println!("{:?}", three_sum(vec![0,-1,1]));
}
///先固定一个值，就变成两数之和了，想到用双指针但是不知道移动左还是右，后来一想排序后不久知道往左还是往右了吗。
/// 排序后从小到大，只要两数之和小于目标值我们就移动左指针，这样两数之和就会变大有可能符合要求。反之移动右指针。
/// 使用set去重。
/// 这样写出来后总是耗时200多ms，后来看了题解优化了移动指针的过程，当两个值一样时指针直接略过，
/// 这样降低到了40多ms。
/// 使用双指针必须有方向的移动指针
pub fn three_sum(mut num_list: Vec<i32>) -> Vec<Vec<i32>> {
    if num_list.len()<3 {
        return vec![]
    }
    num_list.sort_unstable();
    if num_list[0]>0 { return vec![] }
    let mut set:HashSet<Vec<i32>> = HashSet::new();
    // let mut list:Vec<Vec<i32>> = Vec::new();
    for (index,value) in num_list.iter().enumerate(){
        let target = 0-value;
        let mut left = index+1;
        let mut right = num_list.len()-1;
        while left < right {
            let i = num_list[left];
            let j = num_list[right];
            if i+j==target {
                let mut vec = vec![*value,i,j];
                vec.sort_unstable();
                set.insert(vec);
                //这里是耗时优化关键，跳过重复项
                while left < right&&num_list[left]==num_list[left+1] {
                    left+=1;
                }
                while left < right&&num_list[right]==num_list[right-1] {
                    right-=1;
                }
                left += 1;
                right -= 1;
            }else if i+j>target {
                right -= 1;
            }else {
                left += 1;
            }
        }
    }
    set.into_iter().collect::<Vec<Vec<i32>>>()
}