use std::ops::Div;
use crate::challenges::utils::{LINE_ENDING, read_file};

pub fn run() {
    let input = read_file("src/challenges/day3/input.txt");
    let report = parse_diagnostic_report(&input);
    let res = process_diagnostic_report(report);
    println!("Day 3 result, phase 1 = {}", res);
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

fn gamma_comparator(num_bits: u32, num_of_1s: u32) -> u32 {
    // between all the bits, gamma reduces the bits to the most common on
    if num_of_1s > num_bits.div(2) {
         1
    } else {
        0
    }
}

fn epsilon_comparator(num_bits: u32, num_of_1s: u32) -> u32 {
    // between all the bits, gamma reduces the bits to the most common on
    if num_of_1s < num_bits.div(2) {
        1
    } else {
        0
    }
}


struct BinaryNumber{
    bits: Vec<u32>
}

impl BinaryNumber {
    fn to_number(&self) -> u32 {
        let str: String = self.bits.iter().map(|x| x.to_string()).collect();
        u32::from_str_radix(str.as_str(), 2).unwrap()
    }
}

#[cfg(test)]
mod day3_tests {
    use crate::challenges::day3::{process_diagnostic_report, parse_diagnostic_report};

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
}
