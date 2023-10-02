/// # 54 螺旋矩阵 (spiral-matrix)
///
///难度 ：Medium
///
///描述：<p>给你一个 <code>m</code> 行 <code>n</code> 列的矩阵 <code>matrix</code> ，请按照 <strong>顺时针螺旋顺序</strong> ，返回矩阵中的所有元素。</p>
///
///<p> </p>
///
///<p><strong>示例 1：</strong></p>
///<img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/spiral1.jpg" style="width: 242px; height: 242px;" />
///<pre>
///<strong>输入：</strong>matrix = [[1,2,3],[4,5,6],[7,8,9]]
///<strong>输出：</strong>[1,2,3,6,9,8,7,4,5]
///</pre>
///
///<p><strong>示例 2：</strong></p>
///<img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/spiral.jpg" style="width: 322px; height: 242px;" />
///<pre>
///<strong>输入：</strong>matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
///<strong>输出：</strong>[1,2,3,4,8,12,11,10,9,5,6,7]
///</pre>
///
///<p> </p>
///
///<p><strong>提示：</strong></p>
///
///<ul>
///	<li><code>m == matrix.length</code></li>
///	<li><code>n == matrix[i].length</code></li>
///	<li><code>1 <= m, n <= 10</code></li>
///	<li><code>-100 <= matrix[i][j] <= 100</code></li>
///</ul>
///
///
///url: https://leetcode-cn.com/problems/spiral-matrix/description
#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    #[test]
    fn test_spiral_matrix(){
        // println!("{:?}", spiral_order(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]));
        // println!("{:?}", spiral_order(vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12]]));
        // println!("{:?}", spiral_order(vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12]]));
        println!("{:?}", spiral_order(vec![vec![7],vec![9],vec![6]]));
        // println!("{}",1%4);
        // println!("{}",2%4);
        // println!("{}",3%4);
        // println!("{}",4%4);
        // println!("{}",5%4);
        // println!("{}",6%4);
        // println!("{}",7%4);
    }
    ///定义四个状态：0 第一排，直接全部入
    ///           1  最右边依次入
    ///           2  最下边倒序入
    ///           3  最左边从下往上倒序入
    /// 时间 0ms 击败 100.00%使用Rust的用户
    /// 内存 1.88MB 击败 85.52%使用Rust的用户
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        // let mut deque = matrix.into_iter().collect::<VecDeque<VecDeque<i32>>>();
        //a value of type `VecDeque<VecDeque<i32>>` cannot be built from an iterator over elements of type `Vec<i32>`
        //使用了 map 方法将每个 Vec<i32> 转换为 VecDeque<i32>，然后使用 collect方法将整个向量转换为 VecDeque<VecDeque<i32>>。
        let mut deque = matrix
            .into_iter()
            .map(|row| row.into_iter().collect::<VecDeque<i32>>())
            .collect::<VecDeque<VecDeque<i32>>>();
        let mut result = vec![];
        let mut flag = 4;
        while !deque.is_empty() {
            match flag%4 {
                0 => {
                    result.extend(deque.pop_front().unwrap().into_iter().collect::<Vec<i32>>());
                }
                1 => {
                    for d in deque.iter_mut() {
                        if let Some(value) = d.pop_back(){
                            result.push(value);
                        }
                    }
                }
                2 => {
                    let mut bottom = deque.pop_back().unwrap().into_iter().collect::<Vec<i32>>();
                    bottom.reverse();
                    result.extend_from_slice(&*bottom);
                }
                3 => {
                    println!("{:?}",deque.len());
                    for i in (0..deque.len()).rev() {
                        let left = deque.get_mut(i).unwrap();
                        if let Some(value) = left.pop_front(){
                            result.push(value);
                        }
                    }
                }
                _ => {}
            }
            flag += 1;
        }
        result
    }
}
   