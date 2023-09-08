use std::cmp::min;
///给定一个长度为 n 的整数数组 height 。有 n 条垂线，第 i 条线的两个端点是 (i, 0) 和 (i, height[i]) 。
/// 找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。
/// 返回容器可以储存的最大水量。
/// https://leetcode.cn/problems/container-with-most-water/description/?envType=study-plan-v2&envId=top-100-liked
#[cfg(test)]
#[test]
pub fn test_max_area(){
    // println!("{}", max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    // println!("{}", max_area(vec![1, 1]));
    println!("{}", max_area(vec![1,3,2,5,25,24,5]));
}
/// 双指针
/// 时间 8ms  击败 84.49%使用 Rust 的用户
/// 内存 2.78MB 击败 77.00%使用 Rust 的用户
/// 一开始想到用双指针做，但是在指针移动上出现了问题，我想的是先假设先后移动左和右指针，比较哪次移动后的面积更大就选择实际移动哪个
/// 会出现问题，看了答案改成哪个边短就移动哪个，因为底移动后肯定变小1，只有两条边有操作空间，只有移动短的边可能会获得更大的面积
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len()-1;
    let mut maxArea = min(height[0],height[right])*(right as i32);
    while left < right {
        println!("选择的左是{} 右是{}",height[left],height[right]);
        // if height[left] > height[right] {
        //     right -= 1;
        // }else {
        //     left += 1;
        // }
        // maxArea = maxArea.max(min(height[left],height[right])*((right-left) as i32));
        let left_area = compute_area(left + 1, right, &height);
        let right_area = compute_area(left, right - 1, &height);
        maxArea = maxArea.max(left_area.max(right_area));
        if left_area > right_area {
            left+=1
        }else {
            right-=1
        }
    }
    maxArea
}
pub fn compute_area(left:usize,right:usize,height:&Vec<i32>) -> i32{
    min(height[left],height[right])*((right-left) as i32)
}