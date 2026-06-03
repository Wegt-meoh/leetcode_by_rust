pub struct Solution;

impl Solution {
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        Self::solve(
            &land_start_time,
            &water_start_time,
            &land_duration,
            &water_duration,
        )
        .min(Self::solve(
            &water_start_time,
            &land_start_time,
            &water_duration,
            &land_duration,
        ))
    }

    fn solve(start1: &[i32], start2: &[i32], duration1: &[i32], duration2: &[i32]) -> i32 {
        let mut finish1 = i32::MAX;
        start1.iter().enumerate().for_each(|(index, start_time)| {
            finish1 = finish1.min(*start_time + duration1[index]);
        });

        let mut finish2 = i32::MAX;
        start2.iter().enumerate().for_each(|(index, start_time)| {
            finish2 = finish2.min(start_time.max(&finish1) + duration2[index]);
        });

        finish2
    }
}
