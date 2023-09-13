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
    println!("{:?}", find_anagrams("cbaebabacd".into(), "abc".into()));
    println!("{:?}", find_anagrams("abab".into(), "ab".into()));
}
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