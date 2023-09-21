/// # 56 合并区间 (merge-intervals)
///
///难度 ：Medium
///
///描述：<p>以数组 <code>intervals</code> 表示若干个区间的集合，其中单个区间为 <code>intervals[i] = [start<sub>i</sub>, end<sub>i</sub>]</code> 。请你合并所有重叠的区间，并返回&nbsp;<em>一个不重叠的区间数组，该数组需恰好覆盖输入中的所有区间</em>&nbsp;。</p>
///
///<p>&nbsp;</p>
///
///<p><strong>示例 1：</strong></p>
///
///<pre>
///<strong>输入：</strong>intervals = [[1,3],[2,6],[8,10],[15,18]]
///<strong>输出：</strong>[[1,6],[8,10],[15,18]]
///<strong>解释：</strong>区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].
///</pre>
///
///<p><strong>示例&nbsp;2：</strong></p>
///
///<pre>
///<strong>输入：</strong>intervals = [[1,4],[4,5]]
///<strong>输出：</strong>[[1,5]]
///<strong>解释：</strong>区间 [1,4] 和 [4,5] 可被视为重叠区间。</pre>
///
///<p>&nbsp;</p>
///
///<p><strong>提示：</strong></p>
///
///<ul>
///	<li><code>1 &lt;= intervals.length &lt;= 10<sup>4</sup></code></li>
///	<li><code>intervals[i].length == 2</code></li>
///	<li><code>0 &lt;= start<sub>i</sub> &lt;= end<sub>i</sub> &lt;= 10<sup>4</sup></code></li>
///</ul>
///
///
///url: https://leetcode-cn.com/problems/merge-intervals/description
#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    #[test]
    fn test_merge(){
        // println!("{:?}", merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]));
        // println!("{:?}", merge(vec![vec![1, 4], vec![4, 5]]));
        // println!("{:?}", merge(vec![vec![1, 4], vec![0, 4]]));
        // println!("{:?}", merge(vec![vec![1, 4], vec![0, 1]]));//[[0,4]]
        // println!("{:?}", merge(vec![vec![1, 4], vec![5, 6]]));
        println!("{:?}", merge(vec![vec![1, 4], vec![0, 0]])); //[[0,0],[1,4]]
    }
    ///队列为空，直接push
    /// 队列不为空，拿出队列尾,若大
    pub fn merge2(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len()==1 { return intervals }
        let mut result = vec![];
        // let mut i = 0;
        let mut deque = VecDeque::new();
        for interval in intervals {
            if deque.is_empty() {
                deque.push_back(interval);
            }else if deque.back().unwrap()[1]<interval[0] {
                deque.push_back(interval);
            }else if deque.front().unwrap()[0]>interval[1]{
                deque.push_front(interval);
            }else {
                while !(deque.back().unwrap()[1] > interval[1] && deque.back().unwrap()[0] < interval[1]) {
                    result.push(deque.pop_back().unwrap());
                }
                let right = deque.pop_back().unwrap();
                //去掉中间覆盖的
                while !(deque.back().unwrap()[1] > interval[0] && deque.back().unwrap()[0] < interval[0]) {
                    deque.pop_back();
                }
                let left = deque.pop_back().unwrap();
                deque.push_back(vec![left[0],right[1]]);
                while !result.is_empty() {
                    deque.push_back(result.pop().unwrap());
                }
            }
        }
        deque.into_iter().collect()
    }
    //我自己想的，不知道是什么思想
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len()==1 { return intervals }
        let mut result = vec![];
        let mut record = Vec::new();
        for interval in intervals {
            while record.len() <= interval[1] as usize {
                record.push(0);
            }
            for i in interval[0]..interval[1]+1{
                if i==interval[1]&&record[i as usize]!=1 {
                    record[i as usize] = 2;
                }else {
                    record[i as usize] = 1;
                }
            }
        }
        let mut flag = false;
        let (mut left,mut right) = (0,0);
        // println!("{:?}",record);
        for i in 0..record.len(){
            if record[i] ==1&&!flag {
                left = i;
                flag = true
            }else if record[i] ==2 &&flag {
                right = i;
                flag = false;
                result.push(vec![left as i32,right as i32])
            }else if record[i]==2 &&!flag{
                result.push(vec![i as i32,i as i32]);
            }
            // if record[i] ==1&&!flag {
            //     left = i;
            //     flag = true
            // }else if record[i] ==0 &&flag {
            //     right = i-1;
            //     flag = false;
            //     result.push(vec![left as i32,right as i32])
            // }else if record[i]==2&&i==record.len()-1 {
            //     result.push(vec![left as i32,i as i32])
            // }
        }
        result
    }
    pub fn merge3(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len()==1 { return intervals }
        let mut result = vec![];
        let mut i = 1;
        if !(intervals[1][0] <= intervals[0][1]) {
            result.push(intervals[0].clone());
        }
        let mut flag = false;
        while i<intervals.len() {
            while i<intervals.len() && intervals[i][0] <= intervals[i - 1][1] &&intervals[i][0] >intervals[i-1][0]  {
                intervals[i][0] = intervals[i][0].min(intervals[i - 1][0]);
                intervals[i][1] = intervals[i][1].max(intervals[i - 1][1]);
                i += 1;
                flag = true;
            }
            if flag {
                result.push(intervals[i-1].clone());
                flag = false;
            }else {
                result.push(intervals[i].clone());
                i += 1;
            }
        }
        result
    }
}
   