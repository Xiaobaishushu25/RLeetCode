#[cfg(test)]
#[test]
pub fn test_move_zeros(){
    move_zeroes(&mut vec![0,1,0,3,12])
}
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let len = nums.len();
    nums.retain(|x| *x != 0);
    nums.resize(len, 0);
    println!("{:?}",nums)
}