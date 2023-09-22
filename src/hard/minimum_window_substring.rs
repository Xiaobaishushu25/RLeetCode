/// 76. 最小覆盖子串
/// 给你一个字符串 s 、一个字符串 t 。返回 s 中涵盖 t 所有字符的最小子串。如果 s 中不存在涵盖 t 所有字符的子串，
///
/// 则返回空字符串 "" 。
/// 对于 t 中重复字符，我们寻找的子字符串中该字符数量必须不少于 t 中该字符数量。
///
/// 如果 s 中存在这样的子串，我们保证它是唯一的答案。
/// 示例 1：
///
/// 输入：s = "ADOBECODEBANC", t = "ABC"
/// 输出："BANC"
/// 解释：最小覆盖子串 "BANC" 包含来自字符串 t 的 'A'、'B' 和 'C'。
///
/// https://leetcode.cn/problems/minimum-window-substring/description/?envType=study-plan-v2&envId=top-100-liked
#[cfg(test)]
mod tests {
    use std::collections::{HashMap};

    #[test]
    fn test_min_window() {
        // println!("{}", min_window2("ADOBECODEBANC".into(), "ABC".into()));
        println!("{}", min_window("ADOBECODEBANC".into(), "ABC".into()));
        // println!("{}", min_window("a".into(), "b".into()));
        // println!("{}", min_window("ab".into(), "a".into()));
        // println!("{}", min_window("ab".into(), "b".into()));
        // let vec1 = vec![1, 2, 3, 4];
        // println!("{:?}",&vec1[0..4]);
    }
    ///时间 16ms 击败 54.35%使用 Rust 的用户
    ///
    /// 内存2.93MB 击败 5.43%使用 Rust 的用户
    ///
    /// 基本思路就是左右俩指针维护一个滑动窗口。我们先用一个map<char,char个数>来记录子串的字符以及该字符个数。
    /// 先移动右指针，检查进入窗口的元素是否是我们需要的，也就是在不在map中：
    /// 若不在，右指针继续右移。
    /// 若在，我们将其加入now_map中，也就是当前窗口中我们关注的char以及其数量。
    /// 如果进入了一个需要的元素，也就意味着左指针可能可以移动以删除多余的元素。
    /// 我们在一个循环中反复检查第一个元素也就是left元素，left元素有两个可能：
    ///
    /// ①不是我们关注的元素，左指针直接左移就行
    ///
    /// ②是我们关注的元素，那么需要检查移除该元素后是否满足字串要求，如果满足就直接移除，不满足就退出循环以保证当前窗口
    /// 一定是包含字串。
    ///
    pub fn min_window(s: String, t: String) -> String {
        if t.len()>s.len() { return "".into() }
        let (mut left,mut right) = (0,0);//左右俩指针，维护出一个滑动窗口
        let mut min_size = s.len()+1;
        let mut result = String::new();
        let mut map = HashMap::with_capacity(t.len());
        for char in t.chars(){
            *map.entry(char).or_insert(0) +=1;
            // map.insert(char,map.get(&char).unwrap_or(&0)+1);
        }
        let mut need_map = map.clone();//这个map仅仅为了确定当前是否已经覆盖字串
        let mut now_map = HashMap::with_capacity(t.len());
        let chars = s.chars().collect::<Vec<char>>();
        while right<chars.len() {
            let x = chars[right];
            right+=1;
            if map.contains_key(&x){
                *now_map.entry(x).or_insert(0) += 1;
                // now_map.insert(x,now_map.get(&x).unwrap_or(&0)+1);
                if need_map.len()!=0 {
                    if let Some(i) = need_map.get_mut(&x){
                        *i -=1;
                        if *i ==0{
                            need_map.remove(&x);
                        }
                    }
                }
                loop {
                    if left == chars.len()||left==right{ break }
                    let y = chars[left];
                    if map.contains_key(&y){
                        if let Some(i) = now_map.get_mut(&y){
                            let need_i = map.get(&y).unwrap();
                            if *i-1>=*need_i {
                                left += 1;
                                *i -=1;
                            }else { break }
                        }
                    }else {
                        left += 1;
                    }
                }
                if min_size >(right-left)&&need_map.len()==0 {
                    min_size = right-left;
                    result = chars[left..right].iter().collect::<String>();
                }
            }
        }
        result
    }
    ///复制来的，比我快的有限，不看了 测了一次14ms ，我最好是16ms
    #[allow(dead_code)]
    pub fn min_window2(s: String, t: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut overflow = HashMap::new();
        println!("{:?}",t.as_bytes());
        let mut lack = HashMap::with_capacity(t.as_bytes().len());
        let (mut l, mut res_l, mut res_r) = (0, 0, s.len());
        for c in t.chars() {
            *lack.entry(c).or_insert(0) += 1;
        }
        for (r, c) in s.iter().enumerate() {
            if let Some(old) = lack.get_mut(c) {
                *old -= 1;
                if *old == 0 {
                    lack.remove(c);
                }
            } else {
                *overflow.entry(*c).or_insert(0) += 1;
            }
            while let Some(old) = s.get(l).and_then(|lc| overflow.get_mut(lc)) {
                *old -= 1;
                if *old == 0 {
                    overflow.remove(&s[l]);
                }
                l += 1;
            }
            if lack.is_empty() && r - l < res_r - res_l {
                res_r = r;
                res_l = l;
            }
        }
        if res_r == s.len() {
            "".to_string()
        } else {
            s[res_l..=res_r].iter().collect()
        }
    }
}
