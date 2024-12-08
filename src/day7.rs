use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<(i64, Vec<i64>)>;

enum Op {
    Add,
    Mul,
    Cat,
}

impl Op {
    fn go(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Op::Add => lhs + rhs,
            Op::Mul => lhs * rhs,
            Op::Cat => format!("{}{}", lhs, rhs).parse::<i64>().unwrap(),
        }
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .filter_map(|line| {
            line.split_once(": ").map(|(key, values)| {
                let parsed_key = key.parse::<i64>().ok()?;
                let parsed_values = values
                    .split_whitespace()
                    .filter_map(|v| v.parse::<i64>().ok())
                    .collect::<Vec<_>>();
                Some((parsed_key, parsed_values))
            })?
        })
        .collect()
}

fn check(target: i64, nums: &[i64], current: i64, ops: &[Op]) -> bool {
    if nums.is_empty() {
        return current == target;
    }

    let (first, nums_remaining) = nums.split_at(1);
    let first_num = first[0];

    ops.iter()
        .any(|op| check(target, nums_remaining, op.go(current, first_num), ops))
}

#[aoc(day7, part1)]
fn part1(input: &Input) -> String {
    let ops = vec![Op::Add, Op::Mul];
    let acc: i64 = input
        .iter()
        .filter(|(key, values)| check(*key, values, 0, &ops))
        .map(|(k, _)| k)
        .sum();

    format!("{acc}")
}

#[aoc(day7, part2)]
fn part2(input: &Input) -> String {
    let ops = vec![Op::Add, Op::Mul, Op::Cat];
    let acc: i64 = input
        .iter()
        .filter(|(key, values)| check(*key, values, 0, &ops))
        .map(|(k, _)| k)
        .sum();

    format!("{acc}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            )),
            "3749"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            )),
            "11387"
        );
    }
}
