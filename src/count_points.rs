use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::with_capacity(500);

        for qurire in queries {
            let mut sum = 0;
            for point in points.iter() {
                if Self::is_point_in_circle((point[0], point[1]), (qurire[0], qurire[1], qurire[2]))
                {
                    sum += 1;
                }
            }
            result.push(sum);
        }

        result
    }

    fn is_point_in_circle(point: (i32, i32), circle: (i32, i32, i32)) -> bool {
        (point.0 - circle.0).pow(2) + (point.1 - circle.1).pow(2) <= circle.2.pow(2)
    }
}

#[test]
fn test() {
    let res1 = Solution::count_points(
        vec![vec![1, 3], vec![3, 3], vec![5, 3], vec![2, 2]],
        vec![vec![2, 3, 1], vec![4, 3, 1], vec![1, 1, 2]],
    );

    println!("{:?}", res1);
    assert!(res1 == vec![3, 2, 2]);

    let res2 = Solution::count_points(
        vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]],
        vec![vec![1, 2, 2], vec![2, 2, 2], vec![4, 3, 2], vec![4, 3, 3]],
    );

    println!("{:?}", res2);
    assert!(res2 == vec![2, 3, 2, 4])
}
