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
    use std::cmp::min;
    use std::collections::{HashMap, VecDeque};

    #[test]
    fn test_min_window() {
        // println!("{}", min_window("ADOBECODEBANC".into(), "ABC".into()));
        println!("{}", min_window("acbbaca".into(), "aba".into()));
        // println!("{}", min_window("ab".into(), "a".into()));
    }
    pub fn min_window(s: String, t: String) -> String {
        if t.len()>s.len() { return "".into() }
        // let (mut left,mut right) = (0,0);
        let mut right = 0;
        let mut min_size = s.len()+1;
        let mut now_sum = 0;
        // let mut flag = s.len()+1;
        let mut result = String::from("");
        let mut deque = VecDeque::new();
        let mut map = HashMap::with_capacity(t.len());
        for char in t.chars(){
            map.insert(char,map.get(&char).unwrap_or(&0)+1);
        }
        let mut now_map = HashMap::with_capacity(t.len());
        let chars = s.chars().collect::<Vec<char>>();
        while right<chars.len() {
            let x = chars[right];
            deque.push_back(x);
            if map.contains_key(&x){
                now_map.insert(x,now_map.get(&x).unwrap_or(&0)+1);
                now_sum+=1;
            }
            loop {
                if let Some(front) = deque.front(){
                    if map.contains_key(&front){
                        if let Some(i) = now_map.get(&front){
                            let need_i = map.get(&front).unwrap();
                            // println!("当前{}的数量是{}，需要的数量{}",front,i,need_i);
                            if i-1>=*need_i {
                                now_map.insert(*front,now_map.get(&front).unwrap()-1);
                                now_sum-=1;
                                deque.pop_front();
                            }else {
                                break
                            }
                        }
                    }else {
                        deque.pop_front();
                    }
                }else {
                    // println!("队列为空");
                    break
                }
            }
            if min_size >deque.len()&&now_sum>=t.len() {
                min_size = deque.len();
                result = deque.iter().collect::<String>();
            }
            // println!("{:?}",deque);
            right+=1;
        }
        result
    }
}