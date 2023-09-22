use std::collections::VecDeque;

#[cfg(test)]
#[test]
pub fn test_max_sliding_window(){
    // println!("{:?}", max_sliding_window(vec![1, 2, 4, 5, 6, 4, 5], 3));
    // println!("{:?}", max_sliding_window(vec![1], 1));
    // println!("{:?}", max_sliding_window(vec![1,2,2,1,5], 3));
    // println!("{:?}", max_sliding_window(vec![1,2,2,1,5], 3));
    // println!("{:?}", max_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3));
    println!("{:?}", max_sliding_window(vec![1,3,1,2,0,5], 3));
}

///在第二版的解法中主要耗时在这种情况：当前窗口最大值是左值，即将滑出，进来的右值比将滑出的最大值小，这时不得不对这个窗口
/// 切片并重新计算最大值。
/// 想到用空间换时间，维护一个长度为k的有序数组，但是一直对一个数组排序也挺耗时的。
///
/// 看了题解发现用队列，维护简单，时间复杂度低
///
/// 窗口在滑动时，每进入一个新值，与队列末尾比较，若比末尾大，则末尾出队，直至末尾比新值大，新值入队。
///
/// 这样队列就是一个有序的从大到小排列。那么问题又来了，如果滑出值是队列最大值怎么办，所以用队列里存储索引。
///
///
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    if nums.len()==1 { return vec![nums[0]] }
    let k = k as usize;
    let (mut left,mut right):(usize,usize) = (0,k-1);
    let mut deque = VecDeque::new();
    for i in 0..k{
        while !deque.is_empty() && nums[*deque.back().unwrap()]<nums[i]{
            deque.pop_back();
        }
        deque.push_back(i);
    }
    let mut max_vec = vec![];
    max_vec.push(nums[*deque.front().unwrap()]);
    while right < nums.len()-1 {
        right += 1;
        let i = nums[right];
        while !deque.is_empty() && nums[*deque.back().unwrap()]<i{
            deque.pop_back();
        }
        deque.push_back(right);
        //判断当前最大值是否有效
        if deque.front().unwrap()==&left {
            deque.pop_front();
        }
        max_vec.push(nums[*deque.front().unwrap()]);
        left += 1;
    }
    max_vec
}
pub fn compute_max_num(nums: &[i32]) ->(i32, i32, i32, i32){
    let mut max_num:(i32, i32, i32, i32) = if nums[0]>nums[1]{
        (nums[0],1,nums[1],1)
    }else {
        (nums[1],1,nums[0],1)
    };
    nums[2..].iter().for_each(|x|{
        if *x>max_num.0 {
            max_num = (*x,1,max_num.0,max_num.1);
        }else if *x == max_num.0{
            max_num.1 += 1;
        }else if *x<max_num.0&&*x>max_num.2 {
            max_num.2 = *x;
            max_num.3 = 1;
        }else if *x<max_num.0&&*x==max_num.2 {
            max_num.3 += 1;
        }
    });
    max_num
}
///右滑一下进入一个值a，左边同步出去一个值b，判断a是否比当前窗口内最大值还大。
///
/// 1.b是最大值，但是b滑出去后当前窗口内是否还有b？可以用（最大值，最大值数量）来记录
///
/// ①b大于一个，滑出去那么当前窗口最大值就是max（a，b）
///
/// ②b只有一个，滑出去了，若a>b还好，直接最大值是a，若a<b呢？当前窗口最大值以及最大值个数需要重新统计.
///
/// 时间 984ms 击败 5.66%使用 Rust 的用户
/// 内存 3.33MB 击败 66.98%使用 Rust 的用户
pub fn max_sliding_window2(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let (mut left,mut right):(usize,usize) = (0,k-1);
    // let mut max_num = *nums[0..k].iter().max().unwrap();
    let mut max_num = compute_max_num2(&nums[0..k]);
    // println!("{:?}",max_num);
    let mut max_vec = vec![];
    max_vec.push(max_num.0);
    while right < nums.len()-1  {
        right += 1;
        let i = nums[right];
        if i>max_num.0 {
            max_num = (i,1);
            max_vec.push(i);
        }else if i==max_num.0 {
            max_num.1+=1;
            max_vec.push(i);
        }else {
            //此时滑进来的值比窗口内最大值小，需要判断将滑出去的值是不是最大值。
            if nums[left] == max_num.0 {
                if max_num.1 >1  {
                    max_vec.push(max_num.0);
                    max_num.1 -= 1;
                }else {
                    max_num = compute_max_num2(&nums[left+1..right+1]);
                    max_vec.push(max_num.0);
                }
            }else {
                max_vec.push(max_num.0);
            }
        }
        left += 1;
    }
    max_vec
}
pub fn compute_max_num2(nums: &[i32]) ->(i32, i32){
    let mut max_num = (nums[0],1);
    nums[1..].iter().for_each(|x|{
        if *x>max_num.0 {
            max_num = (*x,1);
        }else if *x == max_num.0{
            max_num.1 += 1;
        }
    });
    max_num
}
///超时，每个滑动窗口做一个切片计算最大值
pub fn max_sliding_window3(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut left:usize = 0;
    let k = k as usize;
    let mut max_vec = vec![];
    while left + k <= nums.len()  {
        let x = &nums[left..left + k].iter().max().unwrap();
        max_vec.push(**x);
        left += 1;
    }
    max_vec
}