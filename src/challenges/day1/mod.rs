use crate::challenges::utils;

pub fn run() {
    let input = read_input("src/challenges/day1/input.txt");
    let res = count_depth_measurement_increases(input);
    println!("Day 1 result, phase 1 = {}", res);

    let input = read_input("src/challenges/day1/input-phase2.txt");
    let res = count_depth_measurement_increases_phase_2(input);
    println!("Day 1 result, phase 1 = {}", res);
}

fn read_input(filename: &str) -> Vec<i32> {
    utils::read_input_file_line_separated(filename).iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

fn count_depth_measurement_increases(input: Vec<i32>) -> i32 {
    input.windows(2).map(|window| {
        if window[1] > window[0] { 1 }
        else { 0 }
    }).sum()
}

fn count_depth_measurement_increases_phase_2(input: Vec<i32>) -> i32 {
    input.windows(4).map(|window| {
        let first: i32 = window[0..3].iter().sum();
        let second: i32 = window[1..4].iter().sum();
        if second > first { 1 } else { 0 }
    }).sum()
}

#[cfg(test)]
mod day1_tests {
    use crate::challenges::day1::count_depth_measurement_increases;
    use crate::challenges::day1::count_depth_measurement_increases_phase_2;

    #[test]
    fn part1_works_with_example() {
        let input = vec!(199, 200, 208, 210, 200, 207, 240, 269, 260, 263);
        let res = count_depth_measurement_increases(input);
        assert_eq!(res, 7);
    }

    #[test]
    fn part2_works_with_example() {
        let input = vec!(199,200,208,210,200,207,240,269,260,263);
        let res = count_depth_measurement_increases_phase_2(input);
        assert_eq!(res, 5)
    }
}
