///给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。
///
/// 有效字符串需满足：
///
/// 左括号必须用相同类型的右括号闭合。
/// 左括号必须以正确的顺序闭合。
/// 每个右括号都有一个对应的相同类型的左括号。
/// 输入：s = "()"
/// 输出：true
/// 输入：s = "(]"
/// 输出：false
/// https://leetcode.cn/problems/valid-parentheses/description/
#[cfg(test)]
#[test]
pub fn test_is_valid(){
    println!("{}", is_valid("{()[]{}".into()));
    // let string = "nihao".to_string();
    // let mut chars = string.chars().to_owned();
    // println!("{:?}", chars.nth(1));
}
/**
 构造一个栈，遍历原字符串，遇到左括号就入栈，右括号就出栈一个字符并检查是否匹配。结束后栈长度应该为0。
*/
pub fn is_valid(s: String) -> bool {
    let mut chars = Vec::with_capacity(s.len());
    for char in s.chars(){
        match char {
            '('|'['|'{' =>{
                chars.push(char)
            }
            ')' => {
                if let Some('(') = chars.pop() {  } else { return false }
                // match chars.pop() {
                //     None => {return false}
                //     Some(left) => { if left !='('{
                //         return false
                //     }}
                // }
            }
            ']' => {
                match chars.pop() {
                    None => {return false}
                    Some(left) => { if left !='['{
                        return false
                    }}
                }
            }
            '}' => {
                match chars.pop() {
                    None => {return false}
                    Some(left) => { if left !='{'{
                        return false
                    }}
                }
            }
            _ => {}
        }
    }
    chars.len()==0
    // if chars.len()!=0 { return false }
    // true
}
pub fn is_valid0(s: String) -> bool {
    let mut light = 0;
    let mut right = s.len()-1 ;
    // let mut chars = s.chars().to_owned();
    // let chars = s.into_bytes();
    let chars = s.chars().collect::<Vec<char>>();
    while light < right {
        match chars[light] {
            '(' => {
                while chars[right]!=')' {
                    right = right-1;
                    if light==right { return false }
                }
            }
            '[' => {
                while chars[right]!=']' {
                    right = right-1;
                    if light==right { return false }

                }
            }
            '{' => {
                while chars[right] != '}' {
                    right = right-1;
                    if light==right { return false }

                }
            }
            ')'|']'|'}' => {
                return false
            }
            _ => {}
        }
        light += 1;
    }
    return true
}