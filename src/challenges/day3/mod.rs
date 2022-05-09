use std::ops::Div;
use crate::challenges::utils::{LINE_ENDING, read_file};

pub fn run() {
    let input = read_file("src/challenges/day3/input.txt");
    let report = parse_diagnostic_report(&input);
    let res = process_diagnostic_report(report);
    println!("Day 3 result, phase 1 = {}", res);

    let input2 = read_file("src/challenges/day3/input-phase2.txt");
    let report2 = parse_diagnostic_report(&input2);
    let res2 = process_diagnostic_report_phase2(report2);
    println!("Day 3 result, phase 2 = {}", res2);
}

fn parse_diagnostic_report(content: &str) -> Vec<BinaryNumber> {
    content.trim()
        .split(LINE_ENDING)
        .map(String::from)
        .map(read_binary)
        .collect::<Vec<BinaryNumber>>()
}

fn read_binary(line: String) -> BinaryNumber {
    let bits: Vec<u32> = line.chars()
        .filter_map(|s| s.to_digit(2))
        .collect();
    BinaryNumber{bits}
}

fn process_diagnostic_report(report: Vec<BinaryNumber>) -> u32 {
    let gamma = compute_gamma(&report);
    let epsilon = compute_epsilon(&report);
    gamma * epsilon
}

fn process_diagnostic_report_phase2(report: Vec<BinaryNumber>) -> u32 {
    let oxygen_rating = compute_oxygen_generator_rating(&report);
    let c02_rating = compute_c02_scrubber_rating(&report);
    oxygen_rating * c02_rating
}

/** number of 1 in every number */
fn compute_bit_sum(report: &Vec<BinaryNumber>) -> Vec<u32> {
    let mut sum: Vec<u32> = vec!();
    for num in report.iter() {
        for (i, bit) in num.bits.iter().enumerate() {
            if sum.len() <= i {
                sum.push(*bit)
            } else {
                sum[i] += bit
            }
        }
    }
    sum
}


fn compute_gamma(report: &Vec<BinaryNumber>) -> u32 {
    let num_bits: u32 = report.len() as u32;
    let gamma_bits: Vec<u32> = compute_bit_sum(report)
        .into_iter()
        .map(|sum| gamma_comparator(num_bits, sum))
        .collect();
    let gamma_binary = BinaryNumber{bits: gamma_bits};
    gamma_binary.to_number()
}

fn compute_epsilon(report: &Vec<BinaryNumber>) -> u32 {
    let num_bits: u32 = report.len() as u32;
    let epsilon_bits: Vec<u32> = compute_bit_sum(report)
        .into_iter()
        .map(|sum| epsilon_comparator(num_bits, sum))
        .collect();
    let epsilon_binary = BinaryNumber{bits: epsilon_bits};
    epsilon_binary.to_number()
}

fn compute_rating(report: &Vec<BinaryNumber>, criteria: &dyn Fn(u32, u32) -> u32) -> u32{
    let mut current_bit_position = 0;
    let mut eligible_numbers = report.clone();

    while eligible_numbers.len() > 1 {
        let num_bits: u32 = eligible_numbers.len() as u32;
        let bit_sum = compute_bit_sum(&eligible_numbers);
        let most_common_bit: u32 = criteria(num_bits, bit_sum[current_bit_position]);
        eligible_numbers = eligible_numbers.iter().filter(|num| num.bits[current_bit_position] == most_common_bit).cloned().collect();

        // prepare next iteration
        current_bit_position += 1;
    }

    eligible_numbers[0].to_number()
}

fn compute_oxygen_generator_rating(report: &Vec<BinaryNumber>) -> u32 {
    compute_rating(report, &oxygen_generator_rating_criteria)
}

fn compute_c02_scrubber_rating(report: &Vec<BinaryNumber>) -> u32 {
    compute_rating(report, &co2_scrubber_rating_criteria)
}

fn gamma_comparator(num_bits: u32, num_of_1s: u32) -> u32 {
    // between all the bits, reduces the bits to the most common on
    if num_of_1s > num_bits.div(2) {
         1
    } else {
        0
    }
}

fn epsilon_comparator(num_bits: u32, num_of_1s: u32) -> u32 {
    // between all the bits, reduces the bits to the least common on
    if num_of_1s < num_bits.div(2) {
        1
    } else {
        0
    }
}

fn oxygen_generator_rating_criteria(num_bits: u32, num_of_1s: u32) -> u32 {
    // between all the bits, reduces the bits to the most common on
    if num_of_1s as f64 >= (num_bits as f64).div(2.0) {
        1
    } else {
        0
    }
}

fn co2_scrubber_rating_criteria(num_bits: u32, num_of_1s: u32) -> u32 {
    // between all the bits, reduces the bits to the least common on
    if num_of_1s as f64 >= (num_bits as f64).div(2.0) {
        0
    } else {
        1
    }
}

#[derive(Clone)]
struct BinaryNumber{
    bits: Vec<u32>
}

impl BinaryNumber {
    fn to_number(&self) -> u32 {
        let str: String = self.bits.iter().map(|x| x.to_string()).collect();
        u32::from_str_radix(str.as_str(), 2).unwrap()
    }

    fn most_common_bit(&self) -> u32 {
        let num_1s: u32 = self.bits.iter().sum();
        if num_1s > self.bits.len().div(2) as u32 {
            1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod day3_tests {
    use crate::challenges::day3::{process_diagnostic_report, parse_diagnostic_report, process_diagnostic_report_phase2};

    #[test]
    fn part1_works_with_example() {
        let input = "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";
        let report = parse_diagnostic_report(input);
        let res = process_diagnostic_report(report);
        assert_eq!(res, 198);
    }

    #[test]
    fn part2_works_with_example() {
        let input = "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";
        let report = parse_diagnostic_report(input);
        let res = process_diagnostic_report_phase2(report);
        assert_eq!(res, 230);
    }
}
