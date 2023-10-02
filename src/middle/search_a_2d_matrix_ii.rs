/// # 240 搜索二维矩阵 II (search-a-2d-matrix-ii)
///
///难度 ：Medium
///
///描述：<p>编写一个高效的算法来搜索&nbsp;<code><em>m</em>&nbsp;x&nbsp;<em>n</em></code>&nbsp;矩阵 <code>matrix</code> 中的一个目标值 <code>target</code> 。该矩阵具有以下特性：</p>
///
///<ul>
///	<li>每行的元素从左到右升序排列。</li>
///	<li>每列的元素从上到下升序排列。</li>
///</ul>
///
///<p>&nbsp;</p>
///
///<p><b>示例 1：</b></p>
///<img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/11/25/searchgrid2.jpg" />
///<pre>
///<b>输入：</b>matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 5
///<b>输出：</b>true
///</pre>
///
///<p><b>示例 2：</b></p>
///<img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/11/25/searchgrid.jpg" />
///<pre>
///<b>输入：</b>matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 20
///<b>输出：</b>false
///</pre>
///
///<p>&nbsp;</p>
///
///<p><strong>提示：</strong></p>
///
///<ul>
///	<li><code>m == matrix.length</code></li>
///	<li><code>n == matrix[i].length</code></li>
///	<li><code>1 &lt;= n, m &lt;= 300</code></li>
///	<li><code>-10<sup>9</sup>&nbsp;&lt;= matrix[i][j] &lt;= 10<sup>9</sup></code></li>
///	<li>每行的所有元素从左到右升序排列</li>
///	<li>每列的所有元素从上到下升序排列</li>
///	<li><code>-10<sup>9</sup>&nbsp;&lt;= target &lt;= 10<sup>9</sup></code></li>
///</ul>
///
///
///url: https://leetcode-cn.com/problems/search-a-2d-matrix-ii/description
#[cfg(test)]
mod tests {
    #[test]
    fn test_search_a_2d_matrix_ii(){
        println!("{}", search_matrix(vec![vec![1, 4, 7, 11, 15], vec![2, 5, 8, 12, 19], vec![3, 6, 9, 16, 22], vec![10, 13, 14, 17, 24]],10));
        // println!("{}", search_matrix( vec![vec![-5]],-5));
    }
    ///用时0ms
    /// 不是最好解法，最好解法是类似二叉树深度搜索
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix[0][0]>target||matrix.last().unwrap().last().unwrap()< &target {
            return false;
        }
        for i in 0..matrix.len() {
            let m = &matrix[i];
                if (m.last().unwrap()>= &target) && (m[0]<=target) {
                    for j in 0..m.len() {
                        if m[j]==target {
                            return true
                        }else if m[j+1]>target&&m[j]<target {
                            for k in (0..j+1).rev() {
                                for l in i+1..matrix.len() {
                                    if matrix[l][k]==target {
                                        return true
                                    }
                                }
                            }
                            return false
                        }
                    }
                }
        }
        false
    }
}
   