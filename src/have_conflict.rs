pub struct Solution {}

impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        if event2[0] > event1[1] || event2[1] < event1[0] {
            return false;
        }
        true
    }
}

// 给你两个字符串数组 event1 和 event2 ，表示发生在同一天的两个闭区间时间段事件，其中：

// event1 = [startTime1, endTime1] 且
// event2 = [startTime2, endTime2]
// 事件的时间为有效的 24 小时制且按 HH:MM 格式给出。

// 当两个事件存在某个非空的交集时（即，某些时刻是两个事件都包含的），则认为出现 冲突 。

// 如果两个事件之间存在冲突，返回 true ；否则，返回 false 。

// 来源：力扣（LeetCode）
// 链接：https://leetcode.cn/problems/determine-if-two-events-have-conflict
// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

#[test]
fn test() {
    let event1 = vec!["01:15".to_string(), "02:00".to_string()];
    let event2 = vec!["02:00".to_string(), "03:00".to_string()];
    let ans = true;
    let result = Solution::have_conflict(event1, event2);
    assert!(ans == result);

    let event1 = vec!["01:00".to_string(), "02:00".to_string()];
    let event2 = vec!["01:20".to_string(), "03:00".to_string()];
    let ans = true;
    let result = Solution::have_conflict(event1, event2);
    assert!(ans == result);

    let event1 = vec!["10:00".to_string(), "11:00".to_string()];
    let event2 = vec!["14:00".to_string(), "15:00".to_string()];
    let ans = false;
    let result = Solution::have_conflict(event1, event2);
    assert!(ans == result);
}
