///438. 找到字符串中所有字母异位词
/// 给定两个字符串 s 和 p，找到 s 中所有 p 的 异位词 的子串，返回这些子串的起始索引。不考虑答案输出的顺序。
///
/// 异位词 指由相同字母重排列形成的字符串（包括相同的字符串）。
/// 输入: s = "cbaebabacd", p = "abc"
/// 输出: [0,6]
/// 解释:
/// 起始索引等于 0 的子串是 "cba", 它是 "abc" 的异位词。
/// 起始索引等于 6 的子串是 "bac", 它是 "abc" 的异位词。
/// https://leetcode.cn/problems/find-all-anagrams-in-a-string/?envType=study-plan-v2&envId=top-100-liked
#[cfg(test)]
#[test]
pub fn test_find_anagrams(){
    // println!("{:?}", find_anagrams("cbaebabacd".into(), "abc".into()));
    // println!("{:?}", find_anagrams("abab".into(), "ab".into()));
    println!("{:?}", find_anagrams2("abab".into(), "ab".into()));
}
///时间 736 ms 击败 13.33%使用 Rust 的用户
/// 内存 2.3MB 击败 28.89%使用 Rust 的用户
pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let mut target = p.chars().collect::<Vec<char>>();
    target.sort_unstable();
    let mut left = 0;
    let mut indexs = Vec::new();
    let len = target.len();
    while left + len <= s.len() {
        let slice = &mut s[left.. left + len].chars().collect::<Vec<char>>();
        slice.sort_unstable();
        if slice==&target {
            indexs.push(left as i32);
        }
        left += 1;
    }
    indexs
}
///时间 20ms 击败 20.00%使用 Rust 的用户
/// 内存 2.16MB 击败 51.56%使用 Rust 的用户
/// 我用map<char,i32>来记录每个元素的数量，更好的方法是用一个数组，把char转byte作为数组下标
pub fn find_anagrams2(s: String, p: String) -> Vec<i32> {
    if s.len()<p.len() { return vec![] }
    let mut nums:Vec<i32> = vec![];
    let mut target_map:HashMap<char,i32> = HashMap::new();
    for char in p.chars() {
        if let Some(num) = target_map.get(&char){
            target_map.insert(char, num+1);
        }else {
            target_map.insert(char, 1);
        }
    }
    let mut now_map:HashMap<char,i32> = HashMap::with_capacity(target_map.len());
    let (mut left,mut right) = (0,0);
    let len = p.len();
    let chars = s.chars().collect::<Vec<char>>();
    while right < s.len() {
        while (right - left) != len {
            let x = chars[right];
            if let Some(num) = now_map.get(&x){
                now_map.insert(x,num+1);
            }else {
                now_map.insert(x,1);
            }
            right += 1;
        }
        if now_map == target_map {
            nums.push(left as i32)
        }
        let x = chars[left];
        if let Some(num) = now_map.get(&x){
            if *num>1{
                now_map.insert(x,num-1);
            }else {
                now_map.remove(&x);
            }
        }
        left += 1;
    }
    nums
}