use std::cmp::min;
use std::fmt::Alignment::Right;

///给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。
/// 输入：height = [0,1,0,2,1,0,1,3,2,1,2,1]
/// 输出：6
/// 解释：上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。
/// https://leetcode.cn/problems/trapping-rain-water/description/
#[cfg(test)]
#[test]
pub fn test_trap(){
    println!("{}", trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
    // println!("{}", trap(vec![4,2,0,3,2,5]));
    // println!("{}", trap(vec![5,4,1,2]));
    // println!("{}", trap(vec![5,5,1,7,1,1,5,2,7,6]));
    // println!("{}", trap(vec![0,1,2,0,3,0,1,2,0,0,4,2,1,2,5,0,1,2,0,2]));
    // println!("{}", trap(vec![4,9,4,5,3,2]));
}
//非常的复杂繁琐，我是维护了四个指针，左边两个右边两个分别计算左右面积，最后结算一个中间的面积，有更好的写法
pub fn trap(mut height: Vec<i32>) -> i32 {
    if height.len()<2 { return 0 }
    //去除首尾的0元素
    while height[0] == 0 {
        height.remove(0);
    }
    while height.last()==Some(&0){
        height.pop();
    }
    if height.len()<2 { return 0 }
    let (mut left,mut right) = (0,height.len()-1);
    // let mut right = height.len()-1;
    let mut area = 0;
    let (mut first,mut second) = (0, right);
    // let mut second = right;
    let (mut left_flag, mut right_flag) = (true, true);
    while left <= right {
        loop {
            left +=1;
            //在左边找不到更高的元素，左边结束
            if left >= right{
                left = first;
                left_flag = false;
                break
            }
            //从左边开始找到一个更高的元素就停止
            if height[first] < height[left] {
                break
            }
        }
        loop {
            right -=1;
            if left >= right{
                right = second;
                right_flag = false;
                break
            }
            if height[right] > height[second] {
                break
            }
        }
        // println!("左边是{},右边是{}",left,right);
        // if left_flag {
        //     let min_item = min(height[first], height[left]);
        //     let mut block = 0;
        //     &height[first+1..left].iter().for_each( |x|{
        //         if *x>min_item { block+= min_item }else { block += *x };
        //     });
        //     let area1 = min_item * ((left - first-1) as i32) - block;
        //     area += area1;
        // }
        // if right_flag{
        //     let min_item = min(height[right], height[second]);
        //     let mut block = 0;
        //     &height[right+1..second].iter().for_each( |x|{
        //         if *x>min_item { block+= min_item }else { block += *x };
        //     });
        //     let area2 = min_item * ((second - right-1) as i32) - block;
        //     area +=area2;
        // }
        //
        // if !left_flag&&!right_flag{
        //     let min_item = min(height[first], height[second]);
        //     let mut block = 0;
        //     &height[first+1..second].iter().for_each( |x|{
        //         if *x>min_item { block+= min_item }else { block += *x };
        //     });
        //     let area1 = min_item * ((second - first-1) as i32) - block;
        //     area += area1;
        //     break
        // }
        if left_flag { area += compute_area(first,left,&height) }
        if right_flag { area += compute_area(right,second,&height);}
        if !left_flag&&!right_flag {
            area += compute_area(first,second,&height);
            break
        }
        first = left;
        second = right;
    }
    area
}
pub fn compute_area(left:usize,right:usize,height:&Vec<i32>) -> i32{
    if left == right { return 0 }
    let min_item = min(height[left], height[right]);
    let mut block = 0;
    &&height[left+1..right].iter().for_each( |x|{
        if *x>min_item { block+= min_item }else { block += *x };
    });
    let area = min_item * ((right - left-1) as i32) - block;
    // println!("左x{} 左高{} 右x{} 右高{} 底长{} 面积{}", left, height[left], right, height[right], right-left-1, area);
    area
}
pub fn trap2(mut height: Vec<i32>) -> i32 {
    let mut res = 0;
    let (mut l, mut r) = (0, height.len() - 1);
    while l < r {
        if height[l] < height[r] {
            if height[l + 1] < height[l] {
                res += height[l] - height[l + 1];
                height[l + 1] = height[l];
            }
            l += 1;
        } else {
            if height[r - 1] < height[r] {
                res += height[r] - height[r - 1];
                height[r - 1] = height[r];
            }
            r -= 1;
        }
    }
    res
}