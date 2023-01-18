pub struct Solution {}

impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let n = seats.len();
        let mut result = 0;
        let mut this_seats = seats.clone();
        let mut this_students = students.clone();
        this_seats.sort();
        this_students.sort();

        for i in 0..n {
            if this_seats[i] < this_students[i] {
                result += this_students[i] - this_seats[i];
            } else {
                result -= this_students[i] - this_seats[i];
            }
        }

        result
    }
}
