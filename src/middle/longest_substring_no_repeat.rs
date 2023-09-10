///3无重复字符的最长子串
/// 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。
/// 输入: s = "abcabcbb"
/// 输出: 3
/// 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
/// https://leetcode.cn/problems/longest-substring-without-repeating-characters/?envType=study-plan-v2&envId=top-100-liked
#[cfg(test)]
#[test]
pub fn test_length_of_longest_substring(){
    // println!("{}", length_of_longest_substring("abcabcbb".into()));
    println!("{}", length_of_longest_substring("pwwkew".into()));
}
///快慢指针，又叫滑动窗口？
/// 左右指针初始都置零，然后移动右指针，判断右指针指向的元素是否已经存在，若存在，移动左指针直到前面不存在该元素。
/// 时间 4ms 击败 72.00%使用 Rust 的用户
/// 内存 2.10MB 击败 58.12%使用 Rust 的用户
pub fn length_of_longest_substring(s: String) -> i32 {
    let chars = s.chars().collect::<Vec<char>>();
    let (mut left,mut right) = (0,0);
    let mut sub_string = String::new();
    let mut max_num:i32 = 0;
    while right < chars.len() {
        let char = chars[right];
        sub_string.push(char);
        if !sub_string.contains(char) {
            let len = sub_string.len() as i32;
            if len > max_num { max_num = len; }
        }else {
            loop {
                if chars[left] == char {
                    left += 1;
                    sub_string.remove(0);
                    break
                }
                left += 1;
                sub_string.remove(0);
            }
        }
        right += 1;
    }
    max_num
}