/// # 48 旋转图像 (rotate-image)
///
///难度 ：Medium
///
///描述：<p>给定一个 <em>n&nbsp;</em>×&nbsp;<em>n</em> 的二维矩阵&nbsp;<code>matrix</code> 表示一个图像。请你将图像顺时针旋转 90 度。</p>
///
///<p>你必须在<strong><a href="https://baike.baidu.com/item/%E5%8E%9F%E5%9C%B0%E7%AE%97%E6%B3%95" target="_blank"> 原地</a></strong> 旋转图像，这意味着你需要直接修改输入的二维矩阵。<strong>请不要 </strong>使用另一个矩阵来旋转图像。</p>
///
///<p>&nbsp;</p>
///
///<p><strong>示例 1：</strong></p>
///<img alt="" src="https://assets.leetcode.com/uploads/2020/08/28/mat1.jpg" style="height: 188px; width: 500px;" />
///<pre>
///<strong>输入：</strong>matrix = [[1,2,3],[4,5,6],[7,8,9]]
///<strong>输出：</strong>[[7,4,1],[8,5,2],[9,6,3]]
///</pre>
///
///<p><strong>示例 2：</strong></p>
///<img alt="" src="https://assets.leetcode.com/uploads/2020/08/28/mat2.jpg" style="height: 201px; width: 500px;" />
///<pre>
///<strong>输入：</strong>matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
///<strong>输出：</strong>[[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]
///</pre>
///
///<p>&nbsp;</p>
///
///<p><strong>提示：</strong></p>
///
///<ul>
///	<li><code>n == matrix.length == matrix[i].length</code></li>
///	<li><code>1 &lt;= n &lt;= 20</code></li>
///	<li><code>-1000 &lt;= matrix[i][j] &lt;= 1000</code></li>
///</ul>
///
///<p>&nbsp;</p>
///
///
///url: https://leetcode-cn.com/problems/rotate-image/description
#[cfg(test)]
mod tests {
    #[test]
    fn test_rotate_image(){
        // let mut n = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
        // let mut n = vec![vec![4,8],vec![3,6]];
        let mut n = vec![vec![5,1,9,11],vec![2,4,8,10],vec![13,3,6,7],vec![15,14,12,16]];
        rotate(&mut n);
        println!("{:?}",n);
    }
    ///一层一层的旋转
    /// 用一个n记录由外向内排好几层了
    /// 用一个index记录每次要操作的元素
    /// flag:0 -> 左边转到上边
    ///      1 -> 下边转到左边
    ///      2 -> 右边转到下边
    ///      3 -> 上边转到右边
    ///
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut temp;
        let mut index = 0;
        let mut n = 0;
        let len = matrix[0].len();
        let mut flag = 0;
        while len -2*n>1 {
            temp = (&matrix[n][n..(len-n)]).to_vec();
            while flag != 4 {
                index = n;
                match flag {
                    //左边转到上边
                    0 => {
                        //注意index是一直在变，n变得比较慢
                        for _ in 0..temp.len() {
                            matrix[n][index] = matrix[len-index-1][n];
                            index += 1;
                        }
                        log(matrix);
                    }
                    //下边转到左边
                    1 => {
                        for _ in 0..temp.len() {
                            matrix[index][n] = matrix[len-n-1][index];
                            index += 1;
                        }
                        log(matrix);
                    }
                    //右边转到下边
                    2 => {
                        for _ in 0..temp.len() {
                            matrix[len-n-1][index] = matrix[len-index-1][len-n-1];
                            index += 1;
                        }
                        log(matrix);
                    }
                    //上边转到右边,上面已经变了，用的暂存的temp
                    3 => {
                        for _ in 0..temp.len() {
                            matrix[index][len-n-1] =temp[index-n];
                            index += 1;
                        }
                        log(matrix);
                    }
                    _ => {}
                }
                flag +=1;
            }
            flag = 0;
            n +=1;
        }
    }
    fn log(matrix: &mut Vec<Vec<i32>>){
        println!("************");
        for v in matrix{
            println!("{:?}",v);
        }
        println!("************");
    }
}
   