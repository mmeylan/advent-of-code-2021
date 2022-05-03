use std::cmp;
use crate::challenges::utils;

pub fn run() {
    let input = read_input("src/challenges/day2/input.txt");
    let mut submarine = SubmarineV1::build();
    let res = process_commands(&mut submarine,input);
    println!("Day 2 result, phase 1 = {}", res);

    let input = read_input("src/challenges/day2/input-phase2.txt");
    let mut submarine2 = SubmarineV2::build();
    let res = process_commands(&mut submarine2,input);
    println!("Day 2 result, phase 2 = {}", res);
}

fn read_input(filename: &str) -> Vec<Command> {
    utils::read_input_file_line_separated(filename).iter()
        .map(|str| split_command_input(str))
        .collect::<Vec<Command>>()
}

fn split_command_input(line: &str) -> Command {
    let cmd_arg: Vec<&str> = line.split_whitespace().take(2).collect();
    if cmd_arg.len() != 2 {
        panic!("Invalid input {:?}", cmd_arg)
    }
    let amount = match cmd_arg[1].parse::<i32>() {
        Ok(value) => value,
        Err(err) => panic!("Invalid input {}", err.to_string())
    };
    let cmd = match cmd_arg[0] {
        "forward" => Command::Forward {amount},
        "down" => Command::Down {amount},
        "up" => Command::Up {amount},
        _ => panic!("Invalid input {}", cmd_arg[0])
    };
    cmd
}

fn process_commands<T: Submarine>(submarine: &mut T, commands: Vec<Command>) -> i32 {
    for cmd in commands {
        submarine.steer(cmd)
    }
    submarine.position()
}

#[cfg(test)]
mod day2_tests {
    use crate::challenges::day2::{Command, process_commands, split_command_input, SubmarineV1, SubmarineV2};

    fn read_input(cmds: Vec<&str>) -> Vec<Command> {
        cmds.iter().map(|str| split_command_input(str)).collect::<Vec<Command>>()
    }

    #[test]
    fn part1_works_with_example() {
        let input: Vec<&str> = vec!("forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2");
        let cmds = read_input(input);
        let mut submarine = SubmarineV1::build();
        let res = process_commands(&mut submarine, cmds);
        assert_eq!(res, 150);
    }

    #[test]
    fn part2_works_with_example() {
        let input: Vec<&str> = vec!("forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2");
        let cmds = read_input(input);
        let mut submarine = SubmarineV2::build();
        let res = process_commands(&mut submarine, cmds);
        assert_eq!(res, 900);
    }
}

enum Command {
    Forward { amount: i32 },
    Down { amount: i32 },
    Up { amount: i32 },
}

trait Submarine {
    fn steer(&mut self, cmd: Command) -> ();
    fn position(&self) -> i32;
}

struct SubmarineV1 {
    position: i32,
    depth: i32,
}

impl SubmarineV1 {
    fn build() -> SubmarineV1 {
        SubmarineV1{depth: 0, position: 0}
    }
}

impl Submarine for SubmarineV1 {
    fn steer(&mut self, cmd: Command) {
        match cmd {
            Command::Forward {amount} => self.position = self.position + amount,
            Command::Up {amount} => self.depth = cmp::max(0, self.depth - amount),
            Command::Down {amount} => self.depth = self.depth + amount,
        }
    }
    fn position(&self) -> i32 {
        self.depth * self.position
    }
}

struct SubmarineV2 {
    position: i32,
    depth: i32,
     aim: i32,
}

impl SubmarineV2 {
    fn build() -> SubmarineV2 {
        SubmarineV2{depth: 0, position: 0, aim: 0}
    }
    fn forward(&mut self, amount: i32) {
        self.position = self.position + amount;
        self.depth = cmp::max(0, self.depth + self.aim * amount)
    }
    fn up(&mut self, amount: i32) {
        self.aim = self.aim - amount
    }
    fn down(&mut self, amount: i32) {
        self.aim = self.aim + amount
    }
}

impl Submarine for SubmarineV2 {
    fn steer(&mut self, cmd: Command) {
        match cmd {
            Command::Forward {amount} => self.forward(amount),
            Command::Up {amount} => self.up(amount),
            Command::Down {amount} => self.down(amount),
        }
    }
    fn position(&self) -> i32 {
        self.depth * self.position
    }
}
