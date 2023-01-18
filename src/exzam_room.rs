use std::collections::BTreeSet;

pub struct ExamRoom {
    n: usize,
    num: usize,
    left_seated_seat: usize,
    right_seated_seat: usize,
    seat_state: Vec<bool>,
    area_map: BTreeSet<(usize, usize, usize)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ExamRoom {
    pub fn new(n: i32) -> Self {
        let mut seat_state = Vec::new();
        for _ in 0..n {
            seat_state.push(false);
        }

        ExamRoom {
            n: n as usize,
            num: 0,
            seat_state,
            area_map: BTreeSet::new(),
            left_seated_seat: 0,
            right_seated_seat: 0,
        }
    }

    pub fn seat(&mut self) -> i32 {
        if self.num == 0 {
            self.seat_state[0] = true;
            self.num = 1;
            return 0;
        }
        if self.num == 1 {
            let curr_seated_seat = {
                let mut result = 0;
                for (index, i) in self.seat_state.iter().enumerate() {
                    if *i == true {
                        result = index;
                        break;
                    }
                }
                result
            };

            self.num = 2;
            let left_len = curr_seated_seat;
            let right_len = self.n - 1 - curr_seated_seat;
            if left_len >= right_len {
                self.seat_state[0] = true;
                self.area_map.insert((left_len, 0, curr_seated_seat));
                self.left_seated_seat = 0;
                self.right_seated_seat = curr_seated_seat;
                return 0;
            } else {
                self.seat_state[self.n - 1] = true;
                self.area_map
                    .insert((right_len, curr_seated_seat, self.n - 1));
                self.left_seated_seat = curr_seated_seat;
                self.right_seated_seat = self.n - 1;
                return (self.n - 1) as i32;
            }
        }

        let mut area = self.area_map.pop_last().expect("can not be empty");

        while self.seat_state[area.1] == false || self.seat_state[area.2] == false {
            area = self.area_map.pop_last().expect("can not be empty");
        }

        let (area_len, greatest_area_left, greatest_area_right): (usize, usize, usize) = area;
        let cur_len = area_len / 2;
        let left_len = self.left_seated_seat - 0;
        let right_len = self.n - 1 - self.right_seated_seat;

        if left_len > right_len && left_len > cur_len {
            self.num += 1;
            self.area_map.insert((left_len, 0, self.left_seated_seat));
            self.seat_state[0] = true;
            self.left_seated_seat = 0;
            return 0;
        } else if right_len > left_len && right_len > cur_len {
            self.num += 1;
            self.area_map
                .insert((right_len, self.left_seated_seat, self.n - 1));
            self.seat_state[self.n - 1] = true;
            self.right_seated_seat = self.n - 1;
            return (self.n - 1) as i32;
        } else {
            self.num += 1;
            let mid_index = (greatest_area_left + greatest_area_right) / 2;
            self.area_map.insert((
                mid_index - greatest_area_left,
                greatest_area_left,
                mid_index,
            ));
            self.area_map.insert((
                greatest_area_right - mid_index,
                mid_index,
                greatest_area_right,
            ));
            self.seat_state[mid_index] = true;
            return mid_index as i32;
        }
    }

    fn leave(&mut self, p: i32) {
        self.num -= 1;
        self.seat_state[p as usize] = false;
        if self.num <= 1 {
            return;
        }
        if self.left_seated_seat == p as usize {
            self.left_seated_seat = {
                let mut result = 0;
                for i in p as usize + 1..self.n {
                    if self.seat_state[i] {
                        result = i;
                        break;
                    }
                }
                result
            }
        }

        if self.right_seated_seat == p as usize {
            self.right_seated_seat = {
                let mut result = 0;
                for i in (0..(p - 1) as usize).rev() {
                    if self.seat_state[i] {
                        result = i;
                        break;
                    }
                }
                result
            }
        }
    }
}
